use clap::Parser;
use cli::{Args, Cli};

pub mod appimage;
pub mod cli;
pub mod dirs;
pub mod logging;

fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Set up logging
    logging::set_up_logging();

    match cli.args {
        Args::Install(opts) => {
            if let Err(e) = opts.install() {
                log::error!("{}", e);
            }
        }
    }
}
