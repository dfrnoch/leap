pub mod install;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub args: Args,

}

#[derive(Parser)]
pub enum Args {
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
