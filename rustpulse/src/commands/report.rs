use crate::cli::ReportArgs;

use anyhow::{Context, Result};

pub fn run(args: ReportArgs) -> Result<()> {
    let directory = args.dir;

    if !directory.exists() {
        println!("No reports directory found.");
        return Ok(());
    }

    let entries = std::fs::read_dir(&directory)
        .with_context(|| format!("Unable to read directory: {}", directory.display()))?;

    let mut reports = Vec::new();

    for entry in entries {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() {
            let is_report = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("json") || ext.eq_ignore_ascii_case("csv"))
                .unwrap_or(false);

            if is_report {
                reports.push(path);
            }
        }
    }

    reports.sort();

    println!();
    println!("RustPulse - Previous Reports");
    println!("------------------------------------------");

    if reports.is_empty() {
        println!("No reports found.");
        return Ok(());
    }

    for (index, report) in reports.iter().enumerate() {
        println!("{}. {}", index + 1, report.display());
    }

    println!("------------------------------------------");
    println!("Total reports: {}", reports.len());

    Ok(())
}
