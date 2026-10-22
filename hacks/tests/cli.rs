use std::process::Command;

/// Global verbosity flags remain available after nested subcommands.
#[test]
fn nested_commands_accept_the_shared_verbosity_flags() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("empty.zst");
    std::fs::write(&input, zstd::encode_all(&b""[..], 0).unwrap()).unwrap();

    for flags in [
        vec![],
        vec!["-v"],
        vec!["-vv"],
        vec!["-vvv"],
        vec!["--quiet"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_birdsite-hacks"))
            .args(["wxj", "validate"])
            .args(flags)
            .arg("--input")
            .arg(&input)
            .output()
            .unwrap();

        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout.as_slice(), b"");
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
