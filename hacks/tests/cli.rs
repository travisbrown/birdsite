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

#[test]
fn wxj_errors_report_the_input_line_and_parse_error() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("invalid.zst");
    let valid = concat!(
        r#"{"content":{"data":{"id":"1","conversation_id":"1","#,
        r#""created_at":"2026-11-18T00:00:00Z","lang":"en","#,
        r#""possibly_sensitive":false,"public_metrics":{"retweet_count":0,"#,
        r#""reply_count":0,"like_count":0,"quote_count":0},"#,
        r#""reply_settings":"everyone","text":"hello"},"includes":{"users":[]}}}"#
    );
    let invalid = valid.replace(r#""lang":"en""#, r#""lang":"invalid""#);
    let contents = format!("{valid}\n{invalid}\n");
    std::fs::write(&input, zstd::encode_all(contents.as_bytes(), 0).unwrap()).unwrap();

    for flags in [vec![], vec!["--quiet"]] {
        let output = Command::new(env!("CARGO_BIN_EXE_birdsite-hacks"))
            .args(["wxj", "validate"])
            .args(flags)
            .arg("--input")
            .arg(&input)
            .output()
            .unwrap();

        assert_eq!(output.status.code(), Some(2));
        assert_eq!(output.stdout.as_slice(), b"");
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("WXJ error on line 2: unknown variant `invalid`"),
            "{stderr}"
        );
        assert_eq!(
            stderr.matches("unknown variant `invalid`").count(),
            1,
            "{stderr}"
        );
    }
}
