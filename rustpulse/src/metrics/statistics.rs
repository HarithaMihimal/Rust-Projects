use crate::config::TestConfig;
use crate::metrics::latency::percentile;
use crate::metrics::TestReport;

use std::collections::BTreeMap;
use std::time::Duration;

pub struct RequestResult {
    pub latency_ms: f64,
    pub success: bool,
    pub status_code: Option<u16>,
    pub error: Option<String>,
}

pub fn build_report(
    config: &TestConfig,
    results: &[RequestResult],
    duration: Duration,
) -> TestReport {
    let total = results.len() as u32;

    let successful = results.iter().filter(|r| r.success).count() as u32;

    let failed = total - successful;

    let latencies: Vec<f64> = results.iter().map(|r| r.latency_ms).collect();

    let average = if latencies.is_empty() {
        0.0
    } else {
        latencies.iter().sum::<f64>() / latencies.len() as f64
    };

    let min = latencies.iter().copied().reduce(f64::min).unwrap_or(0.0);

    let max = latencies.iter().copied().reduce(f64::max).unwrap_or(0.0);

    let elapsed = duration.as_secs_f64().max(f64::EPSILON);

    let mut status_codes = BTreeMap::new();

    for result in results {
        if let Some(status) = result.status_code {
            *status_codes.entry(status.to_string()).or_insert(0) += 1;
        }
    }

    let errors = results
        .iter()
        .filter_map(|r| r.error.clone())
        .take(20)
        .collect();

    TestReport {
        timestamp: chrono::Utc::now().to_rfc3339(),
        target: config.url.clone(),
        method: config.method.as_str().to_string(),

        total_requests: total,
        successful_requests: successful,
        failed_requests: failed,

        success_rate: if total == 0 {
            0.0
        } else {
            successful as f64 / total as f64 * 100.0
        },

        duration_seconds: duration.as_secs_f64(),
        requests_per_second: total as f64 / elapsed,

        average_latency_ms: average,
        min_latency_ms: min,
        max_latency_ms: max,

        p50_latency_ms: percentile(&latencies, 50.0),
        p95_latency_ms: percentile(&latencies, 95.0),
        p99_latency_ms: percentile(&latencies, 99.0),

        status_codes,
        errors,
    }
}
