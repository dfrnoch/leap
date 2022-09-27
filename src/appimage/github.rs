
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub id: u16,
}

//Fetches 10 last releases from github 
pub fn fetch_release(repo: &str) -> Result<Vec<Release>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/releases?per_page=10", repo);
    let resp = reqwest::blocking::get(&url)?;
    let releases: Vec<Release> = resp.json()?;
    Ok(releases)
}