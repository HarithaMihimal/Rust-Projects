use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "rustpulse",
    version,
    about = "A high-performance API load testing CLI tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run API load testing
    Test(TestArgs),

    /// View previous test reports
    Report(ReportArgs),
}

#[derive(clap::Args, Debug)]
pub struct TestArgs {
    /// Target API URL
    #[arg(long)]
    pub url: String,

    /// Total number of requests
    #[arg(long, default_value_t = 1000)]
    pub requests: u32,

    /// Maximum concurrent requests
    #[arg(long, default_value_t = 10)]
    pub concurrency: usize,

    /// HTTP request timeout in seconds
    #[arg(long, default_value_t = 10)]
    pub timeout: u64,

    /// HTTP method
    #[arg(long, value_enum, default_value = "get")]
    pub method: HttpMethod,

    /// HTTP headers, e.g. Authorization: Bearer token
    #[arg(long = "header", value_name = "KEY: VALUE")]
    pub headers: Vec<String>,

    /// Request body
    #[arg(long)]
    pub body: Option<String>,

    /// Send a GraphQL JSON request
    #[arg(long)]
    pub graphql: bool,

    /// Report output format
    #[arg(long, value_enum, default_value = "json")]
    pub format: OutputFormat,

    /// Custom report output path
    #[arg(long)]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Csv,
}

#[derive(clap::Args, Debug)]
pub struct ReportArgs {
    /// Directory containing previous reports
    #[arg(long, default_value = "reports")]
    pub dir: PathBuf,
}
