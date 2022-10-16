use std::error::Error;

use crate::appimage::{catalog::fetch_catalog, AppImage};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

#[derive(Clone, Debug, Parser)]
#[clap(alias = "i", about = "Install an appimage.")]
pub struct Install {
    #[arg(short, long)]
    pub link: Option<String>,

    #[arg(short, long)]
    pub github: Option<String>,

    pub name: Option<String>,
}

impl Install {
    pub async fn install(&self) -> Result<(), Box<dyn Error>> {

        let mut appimage;

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
                appimage = AppImage::new(app.title, app.url);
            }

            //Install from link
            Install {
                name,
                link: Some(link),
                github: None,
            } => {
                let name = name
                    .to_owned()
                    .ok_or("Please provide a name \nusage: leap i -l <LINK> <APP_NAME>")
                    .unwrap();

                appimage = AppImage::new(name, link.to_owned());
            }

            //Github installation
            Install {
                name: None,
                link: None,
                github: Some(repo),
            } => {
                let name = repo.split('/').last().unwrap().to_owned();

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
                    appimage = AppImage::new(
                        name,
                        assets[0].browser_download_url.to_owned(),
                    );
                } else {
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select asset")
                        .default(0)
                        .items(&assets.iter().map(|x| x.name.as_str()).collect::<Vec<_>>())
                        .interact()
                        .unwrap();

                    appimage = AppImage::new(
                        name,
                        assets[selection].browser_download_url.to_owned(),
                    );
                }
            }
            _ => return Err("Invalid arguments".into()),
        }


        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Do you want to install {}?", appimage.name))
            .interact_opt()
            .unwrap()
        {
            // install or download?
            Some(true) => Ok(appimage.download().await?),
            _ => Err("Installation cancelled".into()),
        }
    }
}
