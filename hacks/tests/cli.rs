use std::process::Command;

/// Global verbosity flags after nested subcommands control diagnostics without changing stdout.
#[test]
fn nested_commands_accept_the_shared_verbosity_flags() {
    let directory = tempfile::tempdir().unwrap();

    for flags in [
        vec![],
        vec!["-v"],
        vec!["-vv"],
        vec!["-vvv"],
        vec!["--quiet"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_birdsite-hacks"))
            .args(["wxj", "extract"])
            .args(&flags)
            .arg("--input")
            .arg(directory.path())
            .output()
            .unwrap();

        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout.as_slice(), b"");
        let stderr = String::from_utf8(output.stderr).unwrap();
        if flags.is_empty() || flags == ["--quiet"] {
            assert!(stderr.is_empty(), "{stderr}");
        } else {
            assert_eq!(stderr.matches("Loaded 0 paths").count(), 1, "{stderr}");
        }
    }
}

#[test]
fn operational_errors_return_two_and_remain_visible_in_quiet_mode() {
    let directory = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_birdsite-hacks"))
        .args(["--quiet", "wxj", "validate", "--input"])
        .arg(directory.path().join("missing.zst"))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout.as_slice(), b"");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("I/O error:"), "{stderr}");
}
