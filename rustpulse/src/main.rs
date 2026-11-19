
mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Test {
            url,
            requests,
            concurrency,
        } => {
            println!("Starting RustPulse...");
            println!("Target URL: {}", url);
            println!("Total Requests: {}", requests);
            println!("Concurrency: {}", concurrency);
        }

        Commands::Report => {
            println!("Displaying previous test reports...");
        }
    }
}
