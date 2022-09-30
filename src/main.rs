use clap::Parser;
use cli::{Cli, Commands};

pub mod appimage;
pub mod cli;
pub mod dirs;
pub mod logging;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Set up logging
    logging::set_up_logging();

    match cli.command {
        Commands::Install(opts) => {
            if let Err(e) = opts.install().await {
                log::error!("{}", e);
            }
        }
    }
}
