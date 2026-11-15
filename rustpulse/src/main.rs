
mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Test => {
            println!("Running API load testing...");
        }

        Commands::Report => {
            println!("Displaying previous test reports...");
        }
    }
}
