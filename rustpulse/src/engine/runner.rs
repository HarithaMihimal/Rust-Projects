use crate::config::TestConfig;
use crate::engine::worker::execute_request;
use crate::metrics::statistics::{build_report, RequestResult};
use crate::metrics::TestReport;

use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;

use reqwest::Client;
use tokio::task::JoinSet;

pub async fn run_load_test(config: TestConfig) -> Result<TestReport> {
    tracing::info!("Starting load test: {}", config.url);

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout))
        .build()?;

    let total_requests = config.requests as usize;
    let concurrency = config.concurrency;

    let config = Arc::new(config);

    let start = Instant::now();

    let mut tasks = JoinSet::new();
    let mut results: Vec<RequestResult> = Vec::with_capacity(total_requests);

    let mut next_request = 0;

    while next_request < total_requests || !tasks.is_empty() {
        // Launch tasks only while under the concurrency limit.
        while next_request < total_requests && tasks.len() < concurrency {
            let client = client.clone();
            let config = Arc::clone(&config);

            tasks.spawn(async move { execute_request(client, config).await });

            next_request += 1;
        }

        // Wait for one task to complete before launching more.
        if let Some(result) = tasks.join_next().await {
            let result = result?;

            results.push(result);
        }
    }

    let duration = start.elapsed();

    tracing::info!("Completed {} requests", results.len());

    Ok(build_report(&config, &results, duration))
}
