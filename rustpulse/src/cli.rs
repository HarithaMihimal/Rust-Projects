
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
    Test {
        /// Target API URL
        #[arg(long)]
        url: String,

        /// Total number of requests
        #[arg(long, default_value_t = 1000)]
        requests: u32,

        /// Number of concurrent requests
        #[arg(long, default_value_t = 10)]
        concurrency: u32,
    },

    /// View previous test reports
    Report,
}
