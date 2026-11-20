pub mod json;
pub mod terminal;

use crate::cli::OutputFormat;
use crate::metrics::TestReport;

use anyhow::Result;
use std::path::PathBuf;

pub fn save_report(
    report: &TestReport,
    format: OutputFormat,
    custom_path: Option<PathBuf>,
) -> Result<PathBuf> {
    let extension = match format {
        OutputFormat::Json => "json",
        OutputFormat::Csv => "csv",
    };

    let path = match custom_path {
        Some(path) => path,
        None => {
            std::fs::create_dir_all("reports")?;

            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");

            PathBuf::from(format!("reports/rustpulse_{timestamp}.{extension}"))
        }
    };

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    match format {
        OutputFormat::Json => {
            json::save_json(report, &path)?;
        }

        OutputFormat::Csv => {
            save_csv(report, &path)?;
        }
    }

    Ok(path)
}

fn save_csv(report: &TestReport, path: &std::path::Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(["Metric", "Value"])?;

    macro_rules! record {
        ($name:expr, $value:expr) => {
            writer.write_record([$name.to_string(), $value.to_string()])?;
        };
    }

    record!("Timestamp", &report.timestamp);
    record!("Target", &report.target);
    record!("Method", &report.method);

    record!("Total Requests", report.total_requests);
    record!("Successful Requests", report.successful_requests);
    record!("Failed Requests", report.failed_requests);

    record!("Success Rate (%)", report.success_rate);
    record!("Duration (seconds)", report.duration_seconds);
    record!("Requests Per Second", report.requests_per_second);

    record!("Average Latency (ms)", report.average_latency_ms);
    record!("Minimum Latency (ms)", report.min_latency_ms);
    record!("Maximum Latency (ms)", report.max_latency_ms);

    record!("P50 Latency (ms)", report.p50_latency_ms);
    record!("P95 Latency (ms)", report.p95_latency_ms);
    record!("P99 Latency (ms)", report.p99_latency_ms);

    for (status, count) in &report.status_codes {
        record!(format!("HTTP {status} responses"), count);
    }

    for error in &report.errors {
        record!("Network Error", error);
    }

    writer.flush()?;

    Ok(())
}
