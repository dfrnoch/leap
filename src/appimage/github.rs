use serde::Deserialize;
use serde_json::{from_str};
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}


pub fn fetch_release(repo: &str) -> Result<Vec<Release>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/releases?per_page=10", repo);
    let resp = reqwest::blocking::get(&url)?.text()?;

    let releases: Vec<Release> = from_str(&resp)?;

    Ok(releases)
}


//TODO: Parse links from github