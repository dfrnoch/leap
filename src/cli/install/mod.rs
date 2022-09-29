use std::error::Error;

use crate::appimage::{catalog::fetch_catalog, install};
use dialoguer::{theme::ColorfulTheme, Confirm};

use super::*;

#[derive(Clone, Debug, Parser)]
pub struct Install {
    pub name: Option<String>,

    #[clap(short, long)]
    pub link: Option<String>,

    #[clap(short, long)]
    pub github: Option<String>,
}

struct Appimage {
    name: String,
    link: String,
}

impl Install {
    pub fn install(&self) -> Result<(), Box<dyn Error>> {
        let mut appimage: Option<Appimage> = None;

        match self {
            Install {
                name: Some(name),
                link: None,
                github: None,
            } => {
                let catalog = fetch_catalog()?;
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
                    ))?;
                appimage = Some(Appimage {
                    name: app.title,
                    link: app.url,
                });
            }
            Install {
                name: None,
                link: Some(link),
                github: None,
            } => {
                appimage = Some(Appimage {
                    name: link.clone(),
                    link: link.clone(),
                });
            }
            Install {
                name: None,
                link: None,
                github: Some(repo),
            } => {
                let releases = crate::appimage::github::fetch_release(repo)?;
                //todo
            }
            _ => {
                return Err("Invalid arguments".into());
            }
        }

        let result = appimage.unwrap();

        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Do you want to install {}?", result.name))
            .interact_opt()
            .unwrap()
        {
            Some(true) => println!("Looks like you want to continue"),
            Some(false) => println!("nevermind then :("),
            None => println!("Ok, we can start over later"),
        }

        install::download(&result.link)?;

        Ok(())
    }
}
