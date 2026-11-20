mod cli;
mod commands;
mod config;
mod engine;
mod metrics;
mod output;

use clap::Parser;
use cli::{Cli, Commands};

use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Test(args) => {
            commands::test::run(args).await?;
        }

        Commands::Report(args) => {
            commands::report::run(args)?;
        }
    }

    Ok(())
}
