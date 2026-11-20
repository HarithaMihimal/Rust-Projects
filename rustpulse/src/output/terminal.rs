use crate::metrics::TestReport;

pub fn print_report(report: &TestReport) {
    println!();
    println!("==========================================");
    println!("       RustPulse - Performance Report");
    println!("==========================================");

    println!();

    println!("Target:          {}", report.target);
    println!("Method:          {}", report.method);
    println!("Total Requests:  {}", report.total_requests);

    println!();

    println!("REQUEST RESULTS");
    println!("------------------------------------------");

    println!("Successful:      {}", report.successful_requests);

    println!("Failed:          {}", report.failed_requests);

    println!("Success Rate:    {:.2}%", report.success_rate);

    println!();

    println!("LATENCY");
    println!("------------------------------------------");

    println!("Average:         {:.2} ms", report.average_latency_ms);

    println!("Minimum:         {:.2} ms", report.min_latency_ms);

    println!("Maximum:         {:.2} ms", report.max_latency_ms);

    println!("P50:             {:.2} ms", report.p50_latency_ms);

    println!("P95:             {:.2} ms", report.p95_latency_ms);

    println!("P99:             {:.2} ms", report.p99_latency_ms);

    println!();

    println!("THROUGHPUT");
    println!("------------------------------------------");

    println!("Duration:        {:.2} seconds", report.duration_seconds);

    println!("Requests/sec:    {:.2}", report.requests_per_second);

    println!();

    println!("HTTP STATUS CODES");
    println!("------------------------------------------");

    for (status, count) in &report.status_codes {
        println!("{status}: {count}");
    }

    if !report.errors.is_empty() {
        println!();
        println!("NETWORK ERRORS (first 20)");
        println!("------------------------------------------");

        for error in &report.errors {
            println!("- {error}");
        }
    }

    println!();
    println!("==========================================");
}
