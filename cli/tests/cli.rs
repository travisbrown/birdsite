use std::process::Command;

/// Quiet mode suppresses diagnostics, but keeps machine-readable summaries and exit statuses.
#[test]
fn validation_reports_problems_with_the_shared_verbosity_flags() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("invalid.zst");
    std::fs::write(
        &input,
        zstd::encode_all(&b"not a snapshot\n"[..], 0).unwrap(),
    )
    .unwrap();

    for flags in [
        vec![],
        vec!["-v"],
        vec!["-vv"],
        vec!["-vvv"],
        vec!["--quiet"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_birdsite-cli"))
            .arg("validate")
            .args(&flags)
            .arg("--input")
            .arg(&input)
            .output()
            .unwrap();

        assert_eq!(output.status.code(), Some(1));
        let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(summary["line_count"], 1);
        assert_eq!(summary["schema_errors"].as_array().unwrap().len(), 1);
        let stderr = String::from_utf8(output.stderr).unwrap();
        if flags == ["--quiet"] {
            assert!(stderr.is_empty(), "{stderr}");
        } else {
            assert!(stderr.contains("Validation found problems"), "{stderr}");
            assert_eq!(stderr.contains("Validated 1 lines"), !flags.is_empty());
        }
    }
}

#[test]
fn successful_validation_returns_zero() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("empty.zst");
    std::fs::write(&input, zstd::encode_all(&b""[..], 0).unwrap()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_birdsite-cli"))
        .args(["-q", "validate", "--input"])
        .arg(input)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(output.stderr.as_slice(), b"");
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["line_count"], 0);
}

/// Operational failures include the error's sources and remain visible in quiet mode.
#[test]
fn operational_errors_return_two_and_report_the_error_chain() {
    let directory = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_birdsite-cli"))
        .args(["--quiet", "validate", "--input"])
        .arg(directory.path().join("missing.zst"))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.as_slice(), b"");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("validation error: I/O error:"), "{stderr}");
}

#[test]
fn quiet_and_verbose_flags_conflict() {
    let output = Command::new(env!("CARGO_BIN_EXE_birdsite-cli"))
        .args(["validate", "-q", "-v", "--input", "unused.zst"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cannot be used with"), "{stderr}");
}
