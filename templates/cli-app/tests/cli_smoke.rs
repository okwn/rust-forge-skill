use std::process::Command;

#[test]
fn test_doctor_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "doctor"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to run cargo");

    // Doctor should succeed and show rustc/cargo versions
    assert!(
        output.status.success(),
        "doctor command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rustc"), "should show rustc version");
    assert!(stdout.contains("cargo"), "should show cargo version");
}

#[test]
fn test_echo_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "echo", "hello"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to run cargo");

    assert!(
        output.status.success(),
        "echo command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello"), "should echo 'hello'");
}

#[test]
fn test_echo_uppercase() {
    let output = Command::new("cargo")
        .args(["run", "--", "echo", "hello", "--uppercase"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to run cargo");

    assert!(
        output.status.success(),
        "echo --uppercase failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HELLO"), "should echo 'HELLO'");
}

#[test]
fn test_help_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to run cargo");

    assert!(output.status.success(), "help flag should work");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("doctor") && stdout.contains("echo"));
}