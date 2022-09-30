use std::error::Error;

use crate::appimage::{catalog::fetch_catalog, install};
use dialoguer::{theme::ColorfulTheme, Confirm};
use futures_util::Future;

use super::*;

#[derive(Clone, Debug, Parser)]
#[clap(alias = "i", about = "Install an appimage.")]
pub struct Install {
    #[arg(short, long)]
    pub link: Option<String>,

    #[arg(short, long)]
    pub github: Option<String>,

    pub name: Option<String>,
}

// #[derive(Copy, Clone)]
// pub struct Appimage {
//     pub name: &'a str,
//     pub link: &'a str,
// }

pub struct Appimage {
    pub name: String,
    pub link: String,
}

impl Install {
    pub async fn install(&self) -> Result<(), Box<dyn Error>> {
        let mut appimage: Option<Appimage> = None;

        match self {
            Install {
                name: Some(_name),
                link: None,
                github: None,
            } => {
                let catalog = fetch_catalog().await?;
                let name = self.name.as_ref().unwrap();
                let app = catalog
                    .into_iter()
                    .find(|x| {
                        x.title.to_lowercase().contains(&name.to_lowercase())
                            || name.to_lowercase().contains(&x.title.to_lowercase())
                    })
                    .ok_or(format!(
                        "Could not find app with name {} in the catalog",
                        name
                    ))
                    .unwrap();
                appimage = Some(Appimage {
                    name: app.title,
                    link: app.url,
                });
            }
            Install {
                name,
                link: Some(link),
                github: None,
            } => {
                let name = name
                    .to_owned()
                    .ok_or("Please provide a name \nusage: leap -l <LINK> <APP_NAME>")
                    .unwrap();

                appimage = Some(Appimage {
                    name: name,
                    link: link.clone(),
                });
            }
            Install {
                name: None,
                link: None,
                github: Some(repo),
            } => {
                log::info!("Fetching latest release from github");
                let releases = crate::appimage::github::fetch_release(repo);
                //todo
            }
            _ => return Err("Invalid arguments".into()),
        }

        let result = appimage.unwrap();

        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Do you want to install {}?", result.name))
            .interact_opt()
            .unwrap()
        {
            Some(true) => install::download_file(result.name, result.link).await,
            _ => Err("Installation cancelled".into()),
        }
    }
}
