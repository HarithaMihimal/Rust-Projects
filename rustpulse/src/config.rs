use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::Method;

use crate::cli::{HttpMethod, OutputFormat, TestArgs};

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub url: String,
    pub requests: u32,
    pub concurrency: usize,
    pub timeout: u64,
    pub method: Method,
    pub headers: HeaderMap,
    pub body: Option<String>,
    pub format: OutputFormat,
    pub output: Option<std::path::PathBuf>,
}

impl TestConfig {
    pub fn from_args(args: TestArgs) -> Result<Self> {
        reqwest::Url::parse(&args.url).map_err(|e| anyhow!("Invalid URL: {e}"))?;

        if args.requests == 0 {
            return Err(anyhow!("Request count must be greater than zero"));
        }

        if args.concurrency == 0 {
            return Err(anyhow!("Concurrency must be greater than zero"));
        }

        if args.timeout == 0 {
            return Err(anyhow!("Timeout must be greater than zero"));
        }

        if args.graphql && args.body.is_none() {
            return Err(anyhow!(
                "GraphQL mode requires a JSON request body using --body"
            ));
        }

        let method = match args.method {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Patch => Method::PATCH,
            HttpMethod::Delete => Method::DELETE,
        };

        let mut headers = HeaderMap::new();

        for header in args.headers {
            let (key, value) = header
                .split_once(':')
                .ok_or_else(|| anyhow!("Invalid header format: {header}"))?;

            let name = HeaderName::from_bytes(key.trim().as_bytes())
                .map_err(|e| anyhow!("Invalid header name: {e}"))?;

            let value = HeaderValue::try_from(value.trim())
                .map_err(|e| anyhow!("Invalid header value: {e}"))?;

            headers.insert(name, value);
        }

        if args.graphql && !headers.contains_key(CONTENT_TYPE) {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        Ok(Self {
            url: args.url,
            requests: args.requests,
            concurrency: args.concurrency,
            timeout: args.timeout,
            method,
            headers,
            body: args.body,
            format: args.format,
            output: args.output,
        })
    }
}
