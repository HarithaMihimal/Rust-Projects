use crate::metrics::TestReport;

use anyhow::Result;
use std::path::Path;

pub fn save_json(report: &TestReport, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;

    std::fs::write(path, json)?;

    Ok(())
}
