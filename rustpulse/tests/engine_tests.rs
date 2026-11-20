use std::process::Command;

#[test]
fn rejects_zero_concurrency() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustpulse"))
        .args(["test", "--url", "https://example.com", "--concurrency", "0"])
        .output()
        .expect("Failed to execute RustPulse");

    assert!(!output.status.success());
}

#[test]
fn rejects_zero_requests() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustpulse"))
        .args(["test", "--url", "https://example.com", "--requests", "0"])
        .output()
        .expect("Failed to execute RustPulse");

    assert!(!output.status.success());
}
