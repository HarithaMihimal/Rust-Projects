use std::process::Command;

#[test]
fn displays_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustpulse"))
        .arg("--help")
        .output()
        .expect("Failed to execute RustPulse");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("test"));
    assert!(stdout.contains("report"));
}

#[test]
fn displays_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustpulse"))
        .arg("--version")
        .output()
        .expect("Failed to execute RustPulse");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("rustpulse"));
}
