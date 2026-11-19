pub mod latency;
pub mod statistics;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestReport {
    pub timestamp: String,
    pub target: String,
    pub method: String,

    pub total_requests: u32,
    pub successful_requests: u32,
    pub failed_requests: u32,

    pub success_rate: f64,

    pub duration_seconds: f64,
    pub requests_per_second: f64,

    pub average_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,

    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,

    pub status_codes: BTreeMap<String, u32>,
    pub errors: Vec<String>,
}
