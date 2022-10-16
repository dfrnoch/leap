use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

pub async fn fetch_release(repo: &str) -> Result<Vec<Release>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/releases?per_page=10", repo);
    let resp = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36")
        .send()
        .await?
        .text()
        .await?;

    let releases: Vec<Release> = from_str(&resp)?;

    Ok(releases)
}

//TODO: Parse links from github
