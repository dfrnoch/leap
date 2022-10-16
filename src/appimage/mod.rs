use std::path::PathBuf;

// Managing AppImages from default database
pub mod catalog;
// Looking up fro Appimages on github releases
pub mod github;

pub mod install;

pub mod remove;

pub mod update;

pub struct AppImage {
    pub name: String,
    pub link: String,
    pub path: PathBuf,
    pub data: Data,
}
pub struct Data {
    pub icon: Option<PathBuf>,
    pub desktop: Option<PathBuf>,
}

impl AppImage {
    pub fn new(name: String, link: String) -> Self {
        let path = super::dirs::data_dir(Some(&name));
        Self {
            name,
            link,
            path,
            data: Data {
                icon: None,
                desktop: None,
            },
        }
    }
}
