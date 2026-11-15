
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustpulse")]
#[command(version = "0.1.0")]
#[command(about = "A high-performance API load testing CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run API load testing
    Test,

    /// View previous test reports
    Report,
}
