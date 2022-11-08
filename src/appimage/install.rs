use async_process::Command;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    os::unix::prelude::PermissionsExt,
};
use tokio::fs::rename;

use crate::dirs::{bin_dir, desktop_dir};

use super::AppImage;

impl AppImage {
    pub async fn install(&mut self) -> Result<(), Box<dyn Error>> {
        let path = &self.path;
        let app_path = path.join(format!("{}.AppImage", self.name));

        log::info!("Adding executable permissions");
        let mut perms = fs::metadata(&app_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&app_path, perms)?;

        log::info!("creating symlink");
        if let Err(e) = std::os::unix::fs::symlink(&app_path, &bin_dir().join(&self.name)) {
            log::warn!("Failed to create symlink: {}", e);
        }

        Command::new(&app_path)
            .arg("--appimage-extract")
            .current_dir(&path)
            .status()
            .await?;

        fs::read_dir(&path.join("squashfs-root"))
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .for_each(|entry| {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(".desktop") {
                    self.data.desktop = Some(path);
                } else if file_name.ends_with(".png") {
                    self.data.icon = Some(path);
                }
            });

        if let Some(desktop) = &self.data.desktop {
            log::info!("Moving desktop file");

            let mut file = File::open(desktop)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let exec = contents.find("Exec=").unwrap();
            let end = contents[exec..].find("\n").unwrap();
            contents.replace_range(exec + 5..exec + end, &self.name);

            let name = contents.find("Name=").unwrap();
            let end = contents[name..].find("\n").unwrap();
            let name = &contents[name + 5..name + end];
            contents = contents.replace(
                format!("Icon={}", name).as_str(),
                format!("Icon={}", self.name).as_str(),
            );

            let mut file = File::create(path.join(format!("{}.desktop", self.name)))?;
            file.write_all(contents.as_bytes())?;

            fs::copy(
                &path.join(format!("{}.desktop", self.name)),
                desktop_dir().join(format!("{}.desktop", self.name)),
            )?;
        } else {
            log::warn!("No desktop file found");
        }

        if let Some(icon) = &self.data.icon {
            log::info!("Moving icon file");
            rename(&icon, &path.join(format!("{}.png", self.name))).await?;
        }

        fs::remove_dir_all(&path.join("squashfs-root"))?;

        Command::new("xdg-icon-resource")
            .arg("install")
            .arg("--novendor")
            .arg("--size")
            .arg("128")
            .arg(format!(
                "{}.png",
                self.path.join(format!("{}", self.name)).to_str().unwrap()
            ))
            .arg(&self.name)
            .status()
            .await?;

        Ok(())
    }

    pub async fn download(&mut self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();
        let res = client
            .get(&self.link)
            .send()
            .await
            .or(Err(format!("Failed to GET from '{}'", &self.link)))?;
        let total_size = res.content_length().ok_or(format!(
            "Failed to get content length from '{}'",
            &self.link
        ))?;

        // Indicatif setup
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap().progress_chars("#>-"));

        // download chunks
        {
            let mut file = File::create(&self.path.join(format!("{}.AppImage", self.name)))
                .or(Err(format!("Failed to create file '{:?}'", &self.path)))?;
            let mut downloaded: u64 = 0;
            let mut stream = res.bytes_stream();

            //TODO: prob change how fast it updates
            let mut display_every: u64 = 0;

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.or(Err(format!("Failed to get chunk from '{}'", &self.link)))?;
                file.write_all(&chunk)
                    .or(Err(format!("Failed to write to file '{:?}'", self.path)))?;

                downloaded += chunk.len() as u64;
                if display_every <= downloaded {
                    pb.set_position(downloaded);
                    display_every += 1024 * 1024 / 10;
                }
            }
            pb.finish_with_message(format!("Downloaded {} to {:?}", self.name, self.path));
        }

        self.install().await?;


        Ok(())
    }
}
