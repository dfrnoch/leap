use std::{cmp::min, fs, fs::File, io::Write};

use crate::dirs::data_dir;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

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

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {:?}", name, path));

    log::info!("Extracting data");
    install_file(&name).unwrap();

    return Ok(());
}

//TODO: Make this async, Symlink the file to .local/bin, permissions
//FIXME: BUSY FILE
fn install_file(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = data_dir(Some("apps"));

    Command::new("chmod")
        .arg(format!("+x {:?}", path.join(name)).as_str())
        .output()
        .expect("Failed to chmod");

    Command::new(path.join(name))
        .arg("--appimage-extract")
        .output()
        .unwrap();

    let mut desktop_file = String::new();
    let mut icon_file = String::new();

    fs::read_dir(path.join("squashfs-root"))
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .for_each(|entry| {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".desktop") {
                desktop_file = file_name.to_string();
            } else if file_name.ends_with(".png") || file_name.ends_with(".svg") {
                icon_file = file_name.to_string();
            }
        });

    fs::rename(
        path.join("squashfs-root").join(desktop_file),
        path.join(name.to_string() + ".desktop"),
    )
    .unwrap();

    fs::rename(
        path.join("squashfs-root").join(icon_file),
        path.join(name.to_string() + ".png"),
    )
    .unwrap();

    fs::remove_dir_all(path.join("squashfs-root")).unwrap();

    return Ok(());
}
