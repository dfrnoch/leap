use std::{fs, fs::File, io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};

use crate::dirs::data_dir;
use async_process::Command;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn download_file(name: String, link: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = data_dir(Some(name.as_str()));

    let name = format!("{}.AppImage", name);

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
    let mut file =
        File::create(&path.join(&name)).or(Err(format!("Failed to create file '{:?}'", path)))?;
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

    install_file(&name.replace(".AppImage", ""), path).await?;

    return Ok(());
}

struct ExtractData {
    icon: Option<PathBuf>,
    desktop: Option<PathBuf>,
}

//TODO: Make this async, Symlink the file to .local/bin
//FIXME: BUSY FILE
async fn install_file(name: &str, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let app_path = path.join(format!("{}.AppImage", name));

    let mut perms = fs::metadata(&app_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&app_path, perms)?;

    log::info!("Extracting file");
    Command::new(&app_path)
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
                data.desktop = Some(path.join(file_name));
            } else if file_name.ends_with(".png") || file_name.ends_with(".svg") {
                data.icon = Some(path.join(file_name));
            }
        });

    extract(name, data);

    return Ok(());
}

fn extract(name: &str, data: ExtractData) -> Result<(), Box<dyn std::error::Error>> {
    //leap/{name}/

    Ok(())
}
