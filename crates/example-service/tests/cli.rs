//! End-to-end tests for the binary output.
//!
//! 针对二进制输出的端到端测试。

use std::process::Command;

#[test]
fn binary_prints_banner() {
    let output = Command::new(env!("CARGO_BIN_EXE_example-service"))
        .output()
        .expect("binary should run");
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.stderr.is_empty(),
        "stderr should be empty: {stderr:?}"
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert_eq!(stdout.trim_end(), "Hello, example-service!");
}
