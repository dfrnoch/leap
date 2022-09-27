const DB_URL: &str = "https://appimage.github.io/search.json";

pub fn get_db() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(DB_URL)?;
    let body = resp.text()?;
    let v: serde_json::Value = serde_json::from_str(&body)?;
    
    Ok(v)
}
