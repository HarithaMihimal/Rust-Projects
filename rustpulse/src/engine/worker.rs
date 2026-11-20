use crate::config::TestConfig;
use crate::metrics::statistics::RequestResult;

use std::sync::Arc;
use std::time::Instant;

use reqwest::Client;

pub async fn execute_request(client: Client, config: Arc<TestConfig>) -> RequestResult {
    let start = Instant::now();

    let mut request = client
        .request(config.method.clone(), &config.url)
        .headers(config.headers.clone());

    if let Some(body) = &config.body {
        request = request.body(body.clone());
    }

    let result = request.send().await;

    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    match result {
        Ok(response) => {
            let status = response.status();

            RequestResult {
                latency_ms,
                success: status.is_success(),
                status_code: Some(status.as_u16()),
                error: None,
            }
        }

        Err(error) => RequestResult {
            latency_ms,
            success: false,
            status_code: None,
            error: Some(error.to_string()),
        },
    }
}
