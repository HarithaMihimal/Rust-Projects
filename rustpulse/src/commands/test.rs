use crate::cli::TestArgs;
use crate::config::TestConfig;
use crate::engine::runner::run_load_test;
use crate::output::{save_report, terminal};

use anyhow::Result;

pub async fn run(args: TestArgs) -> Result<()> {
    let config = TestConfig::from_args(args)?;

    println!("Starting RustPulse API Load Test...");
    println!("Target: {}", config.url);
    println!("Requests: {}", config.requests);
    println!("Concurrency: {}", config.concurrency);
    println!("Timeout: {} seconds", config.timeout);

    println!();

    let report = run_load_test(config.clone()).await?;

    terminal::print_report(&report);

    let report_path = save_report(&report, config.format, config.output.clone())?;

    println!("Report saved to: {}", report_path.display());

    Ok(())
}
