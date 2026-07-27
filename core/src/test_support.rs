//! Shared helpers for exercising the model types against example data.
//!
//! Example data is kept in two tiers, so that a clone of the repository is self-contained while
//! the much larger archives used during development stay out of version control.
//!
//! # Fixtures
//!
//! A fixture is a small, curated document committed under `<package>/tests/data` and included at
//! compile time with `include_str!`. Fixtures live inside the package directory because
//! `cargo package` only ships files from there, so a path escaping it would be missing from the
//! published crate. Each is distilled from a corpus by taking the fewest documents covering the
//! most field paths and value variants, which keeps the committed volume to a few kilobytes per
//! format.
//!
//! # Corpora
//!
//! A corpus is the full, uncommitted collection a fixture was distilled from. Corpora are read at
//! run time from the directory named by `BIRDSITE_CORPUS_DIR`, defaulting to `examples` beside the
//! workspace root, and are addressed by a name mirroring the fixture path (`wxj/flat` matches
//! either `wxj/flat.jsonl` or a `wxj/flat` directory). Tests that use one pass trivially when it
//! is absent, so setting `BIRDSITE_REQUIRE_CORPUS` turns that skip into a failure for runs where
//! the data is expected to be installed.
//!
//! Every test taking a corpus has a fixture counterpart, so a clone with no archives installed
//! still exercises each format. The README describes the arrangement from a user's perspective.

use std::path::{Path, PathBuf};

/// Names the directory holding the local example corpora.
const CORPUS_DIR_VAR: &str = "BIRDSITE_CORPUS_DIR";

/// When set, a missing corpus fails the test instead of skipping it.
const REQUIRE_CORPUS_VAR: &str = "BIRDSITE_REQUIRE_CORPUS";

/// Yields the non-blank lines of a JSONL document alongside their one-based line numbers.
pub fn numbered_lines(source: &str) -> impl Iterator<Item = (usize, &str)> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| (index + 1, line))
}

/// Asserts that every line of a JSONL document deserializes as `T`, returning the line count.
///
/// # Panics
///
/// Panics if the document is empty, or if any line fails to deserialize, reporting `label` and
/// the line number.
pub fn parse_jsonl<'a, T: serde::de::Deserialize<'a>>(label: &str, source: &'a str) -> usize {
    let mut checked = 0;

    for (number, line) in numbered_lines(source) {
        serde_json::from_str::<T>(line)
            .unwrap_or_else(|error| panic!("{label} line {number}: {error}"));
        checked += 1;
    }

    assert!(checked > 0, "{label}: no documents");

    checked
}

/// Asserts that every line of a JSONL document deserializes as `T` and serializes back to an
/// equivalent document, returning the line count.
///
/// Comparing the emitted JSON against its input catches shape changes that a value-level
/// round-trip cannot see, since a two-element array written as a `start`/`end` map still reparses
/// into an equal value.
///
/// # Panics
///
/// Panics if the document is empty, or if any line fails to deserialize, fails to serialize, or
/// does not survive the round-trip, reporting `label` and the line number.
pub fn round_trip_jsonl<'a, T: serde::de::Deserialize<'a> + serde::ser::Serialize>(
    label: &str,
    source: &'a str,
) -> usize {
    let mut checked = 0;

    for (number, line) in numbered_lines(source) {
        let value = serde_json::from_str::<T>(line)
            .unwrap_or_else(|error| panic!("{label} line {number}: {error}"));
        let serialized = serde_json::to_string(&value)
            .unwrap_or_else(|error| panic!("{label} line {number}: {error}"));

        assert_same_json(&format!("{label} line {number}"), line, &serialized);
        checked += 1;
    }

    assert!(checked > 0, "{label}: no documents");

    checked
}

/// Asserts that two JSON documents are equivalent, ignoring fields whose value is `null`.
///
/// Absent optional fields are serialized as explicit nulls rather than omitted, so this
/// normalization is what lets re-serialized output be compared against its input.
///
/// # Panics
///
/// Panics if either document is invalid JSON or if the two differ.
pub fn assert_same_json(label: &str, expected: &str, actual: &str) {
    let mut expected_value =
        serde_json::from_str::<serde_json::Value>(expected).expect("valid expected JSON");
    let mut actual_value =
        serde_json::from_str::<serde_json::Value>(actual).expect("valid actual JSON");

    strip_nulls(&mut expected_value);
    strip_nulls(&mut actual_value);

    // Documents run to tens of kilobytes, so report the offending path rather than both sides.
    if let Some((path, expected_at, actual_at)) =
        first_difference(&expected_value, &actual_value, "")
    {
        panic!(
            "{label}: wire-format mismatch at {path}: expected {expected_at}, found {actual_at}"
        );
    }
}

/// Stands in for a field one side does not have, which normalization makes equivalent to `null`.
const ABSENT: &serde_json::Value = &serde_json::Value::Null;

/// Returns the path of the first difference between two JSON values, with both sides rendered.
fn first_difference(
    expected: &serde_json::Value,
    actual: &serde_json::Value,
    path: &str,
) -> Option<(String, String, String)> {
    match (expected, actual) {
        (serde_json::Value::Object(left), serde_json::Value::Object(right)) => left
            .iter()
            .map(|(key, value)| (key, value, right.get(key).unwrap_or(ABSENT)))
            .chain(
                right
                    .iter()
                    .filter(|(key, _)| !left.contains_key(key.as_str()))
                    .map(|(key, value)| (key, ABSENT, value)),
            )
            .find_map(|(key, left_value, right_value)| {
                first_difference(left_value, right_value, &format!("{path}.{key}"))
            }),
        (serde_json::Value::Array(left), serde_json::Value::Array(right))
            if left.len() == right.len() =>
        {
            left.iter()
                .zip(right)
                .enumerate()
                .find_map(|(index, (left_value, right_value))| {
                    first_difference(left_value, right_value, &format!("{path}[{index}]"))
                })
        }
        _ => {
            (expected != actual).then(|| (path.to_owned(), truncated(expected), truncated(actual)))
        }
    }
}

/// Renders a value for a failure message, shortening anything too long to read.
fn truncated(value: &serde_json::Value) -> String {
    const LIMIT: usize = 120;

    let rendered = value.to_string();

    if rendered.chars().count() > LIMIT {
        format!("{}...", rendered.chars().take(LIMIT).collect::<String>())
    } else {
        rendered
    }
}

/// Returns the path and contents of every document making up the named local corpus.
///
/// The name is resolved against the corpus root as a directory, as a file, and finally as a file
/// with a `.jsonl` extension. The result is empty when no corpus is installed, which makes the
/// calling test a no-op unless [`REQUIRE_CORPUS_VAR`] is set.
///
/// # Panics
///
/// Panics if the corpus is missing while [`REQUIRE_CORPUS_VAR`] is set, or if an installed corpus
/// cannot be read.
pub fn local_corpus(name: &str) -> Vec<(String, String)> {
    let root = std::env::var_os(CORPUS_DIR_VAR).map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../examples"),
        PathBuf::from,
    );
    let base = root.join(name);

    let paths = if base.is_dir() {
        let mut paths = std::fs::read_dir(&base)
            .unwrap_or_else(|error| panic!("Unreadable corpus {}: {error}", base.display()))
            .map(|entry| entry.expect("Readable corpus entry").path())
            .filter(|path| path.is_file())
            .collect::<Vec<_>>();

        // `read_dir` yields entries in an unspecified order, so sort for reproducible reports.
        paths.sort();
        paths
    } else if base.is_file() {
        vec![base.clone()]
    } else {
        let file = base.with_extension("jsonl");

        if file.is_file() {
            vec![file]
        } else {
            Vec::new()
        }
    };

    assert!(
        !paths.is_empty() || std::env::var_os(REQUIRE_CORPUS_VAR).is_none(),
        "{REQUIRE_CORPUS_VAR} is set but no corpus was found at {}",
        base.display()
    );

    if paths.is_empty() {
        eprintln!("No local corpus at {}; skipping", base.display());
    }

    paths
        .into_iter()
        .map(|path| {
            let contents = std::fs::read_to_string(&path).unwrap_or_else(|error| {
                panic!("Unreadable corpus file {}: {error}", path.display())
            });

            (path.display().to_string(), contents)
        })
        .collect()
}

/// Recursively drops object fields whose value is `null`.
fn strip_nulls(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(fields) => {
            fields.retain(|_, field| !field.is_null());

            for field in fields.values_mut() {
                strip_nulls(field);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                strip_nulls(item);
            }
        }
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => {}
    }
}
