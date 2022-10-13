use std::{fs, fs::File, io::Write, os::unix::prelude::PermissionsExt};

use crate::dirs::data_dir;
use async_process::Command;
use file_mode::{ModePath, User};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn download_file(name: String, link: String) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("{}.AppImage", name);

    let path = data_dir(Some("apps")).join(&name);

    // Reqwest setup
    let client = reqwest::Client::new();
    let res = client
        .get(&link)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &link)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &link))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap().progress_chars("#>-"));

    // download chunks
    let mut file = File::create(&path).or(Err(format!("Failed to create file '{:?}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    //TODO: prob change how fast it updates
    let mut display_every: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.or(Err(format!("Failed to get chunk from '{}'", &link)))?;
        file.write_all(&chunk)
            .or(Err(format!("Failed to write to file '{:?}'", path)))?;

        downloaded += chunk.len() as u64;
        if display_every <= downloaded {
            pb.set_position(downloaded);
            display_every += 1024 * 1024 / 10;
        }
    }

    pb.finish_with_message(format!("Downloaded {} to {:?}", name, path));

    log::info!("Extracting data");
    install_file(&name).await?;

    return Ok(());
}

struct ExtractData {
    icon: Option<String>,
    desktop: Option<String>,
}

//TODO: Make this async, Symlink the file to .local/bin, permissions
//FIXME: BUSY FILE
async fn install_file(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = data_dir(Some("apps"));
    let name_path = path.join(name);


    name_path.set_mode(0o755)?;

    log::info!("Extracting file");
    Command::new(name_path)
        .arg("--appimage-extract")
        .arg(&path)
        .output()
        .await?;

    let mut data = ExtractData {
        icon: None,
        desktop: None,
    };

    fs::read_dir(path.join("squashfs-root"))
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .for_each(|entry| {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".desktop") {
                data.desktop = Some(file_name.to_string());
            } else if file_name.ends_with(".png") || file_name.ends_with(".svg") {
                data.icon = Some(file_name.to_string());
            }
        });

    extract(data);

    return Ok(());
}

fn extract(data: ExtractData) -> Result<(), Box<dyn std::error::Error>> {
    // fs::rename(
    //     path.join("squashfs-root").join(desktop_file),
    //     path.join(name.to_string() + ".desktop"),
    // )
    // .unwrap();

    // fs::rename(
    //     path.join("squashfs-root").join(icon_file),
    //     path.join(name.to_string() + ".png"),
    // )
    // .unwrap();
    // fs::remove_dir_all(path.join("squashfs-root")).unwrap();
    Ok(())
}
