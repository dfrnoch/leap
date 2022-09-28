use std::error::Error;

use super::*;

#[derive(Clone, Debug, Parser)]
pub struct Install {
    pub name: String,
}

impl Install {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        log::info!("Installing {}", self.name);
        Ok(())
    }
}
