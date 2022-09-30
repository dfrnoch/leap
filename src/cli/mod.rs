pub mod install;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install an appimage.
    Install(install::Install),
    // /// Uninstall an appimage.
    // Uninstall(Uninstall),

    // /// List installed appimages.
    // List(List),

    // /// Update installed appimages.
    // Update(Update),

    // /// Download from github
    // Github(Github),
}
