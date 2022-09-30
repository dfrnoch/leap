use std::{cmp::min, fs::File, io::Write};

use crate::dirs::data_dir;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn download_file(name: String, link: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = data_dir().join("apps").join(&name);

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
    return Ok(());
}
