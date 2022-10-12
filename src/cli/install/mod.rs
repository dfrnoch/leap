use std::error::Error;

use crate::appimage::{catalog::fetch_catalog, install};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

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

pub struct Appimage {
    pub name: String,
    pub link: String,
}

impl Install {
    pub async fn install(&self) -> Result<(), Box<dyn Error>> {
        let mut appimage: Option<Appimage> = None;

        match self {
            //Install from db
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

            //Install from link
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
                    name,
                    link: link.clone(),
                });
            }

            //Github Isntallaion
            Install {
                name: None,
                link: None,
                github: Some(repo),
            } => {
                let name = repo.split("/").last().unwrap().to_owned();

                log::info!("Fetching latest release from github");
                let releases = crate::appimage::github::fetch_release(repo).await?;

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select release")
                    .default(0)
                    .items(
                        &releases
                            .iter()
                            .map(|x| x.tag_name.as_str())
                            .collect::<Vec<_>>(),
                    )
                    .interact()
                    .unwrap();

                let assets = releases[selection]
                    .assets
                    .iter()
                    .filter(|x| x.name.ends_with(".AppImage"))
                    .collect::<Vec<_>>();

                if assets.len() == 1 {
                    appimage = Some(Appimage {
                        name,
                        link: assets[0].browser_download_url.clone(),
                    });
                } else {
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select asset")
                        .default(0)
                        .items(&assets.iter().map(|x| x.name.as_str()).collect::<Vec<_>>())
                        .interact()
                        .unwrap();

                    appimage = Some(Appimage {
                        name,
                        link: assets[selection].browser_download_url.clone(),
                    });
                }
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
