use std::{error::Error, fs};

use crate::dirs::bin_dir;

use super::AppImage;

impl AppImage {
    pub fn remove(&mut self) -> Result<(), Box<dyn Error>> {
        let path = &self.path;
        let app_path = path.join(format!("{}.AppImage", self.name));

        log::info!("Removing AppImage");
        fs::remove_file(&app_path)?;

        log::info!("Removing symlink");
        fs::remove_file(&bin_dir().join(&self.name))?;

        Ok(())
    }
}
