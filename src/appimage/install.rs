use crate::cli::install::Appimage;



pub fn download(appimage: Appimage) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(appimage.link)?;
    let mut file = std::fs::File::create("appimage")?;
    std::io::copy(&mut resp, &mut file)?;



    

    Ok(())
}
