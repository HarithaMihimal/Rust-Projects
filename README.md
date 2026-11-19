
# RustPulse

A high-performance API load testing and performance monitoring
CLI tool built with Rust.

## Features

- Concurrent HTTP request execution
- REST API testing
- GraphQL JSON request support
- Configurable request count and concurrency
- GET, POST, PUT, PATCH and DELETE
- Custom HTTP headers
- Request timeout configuration
- Average, minimum and maximum latency
- P50, P95 and P99 latency percentiles
- Requests per second calculation
- HTTP status code statistics
- JSON and CSV report generation
- Previous report listing

## Technology Stack

- Rust
- Tokio
- Reqwest
- Clap
- Serde
- Serde JSON
- Chrono
- Tracing

## Installation

Clone the repository:

    git clone https://github.com/YOUR_USERNAME/rustpulse.git

Navigate to the project:

    cd rustpulse

Build:

    cargo build --release

## Usage

Display help:

    cargo run -- --help

Run API load testing:

    cargo run -- test --url https://example.com

Configure request count and concurrency:

    cargo run -- test \
      --url https://example.com \
      --requests 1000 \
      --concurrency 50

Use POST:

    cargo run -- test \
      --url https://example.com/api \
      --method post \
      --body '{"name":"RustPulse"}'

Add custom headers:

    cargo run -- test \
      --url https://example.com/api \
      --header "Authorization: Bearer YOUR_TOKEN"

Generate CSV:

    cargo run -- test \
      --url https://example.com \
      --format csv

Generate JSON:

    cargo run -- test \
      --url https://example.com \
      --format json

View previous reports:

    cargo run -- report

## Architecture

    src/
      cli.rs
      config.rs
      commands/
      engine/
      metrics/
      output/
      main.rs

## Development

Run tests:

    cargo test

Check formatting:

    cargo fmt --all -- --check

Run Clippy:

    cargo clippy --all-targets --all-features -- -D warnings

Build optimized release:

    cargo build --release

## License

MIT
