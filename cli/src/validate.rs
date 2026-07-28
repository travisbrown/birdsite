//! Validate a compact snapshot file: digests, digest order, metadata, and wxj schemas.
//!
//! Each line's stored digest is verified against its content (via
//! [`Context::verify`](archivindex_wbm_json::context::Context::verify)), the digests are
//! required to be in strictly ascending SHA-1 byte order (no duplicates), a line with a URL is
//! required to also have a timestamp (lines with no timestamp at all are tallied but allowed), and
//! each line's content is deserialized with the `birdsite` wxj model types (which reject unknown
//! fields), so validation checks the full schema rather than the presence of a few fields.

use archivindex_wbm_json::{context::Context, exact::ExactSnapshot};
use archivindex_wbm_json_processing::io::read::SnapshotReader;
use birdsite::model::wxj;
use sha1::Sha1;
use std::path::Path;

/// Validation error.
///
/// Only I/O failures abort validation; per-line problems (unparseable lines, digest mismatches,
/// out-of-order digests, invalid metadata, and schema mismatches) are recorded in the summary
/// instead.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}

/// Per-file validation results.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ValidationSummary {
    /// Number of lines read.
    pub line_count: usize,
    /// Number of lines that passed every check.
    pub valid_count: usize,
    /// Number of lines with no timestamp (with or without a URL).
    pub missing_metadata_count: usize,
    /// Number of lines with invalid metadata (a URL but no timestamp).
    pub invalid_metadata_count: usize,
    /// Lines that could not be parsed or whose content did not match the schema.
    pub schema_errors: Vec<LineError>,
    /// Lines whose stored digest did not match their content (or whose format was unsupported).
    pub digest_errors: Vec<LineError>,
    /// Lines whose digest was not strictly greater than every earlier digest (by SHA-1 bytes).
    pub order_errors: Vec<LineError>,
}

impl ValidationSummary {
    /// Whether every line parsed, matched its digest, was in ascending digest order, had valid
    /// metadata, and matched the schema.
    ///
    /// Missing metadata (no timestamp and no URL) does not count against success; it is only
    /// tallied in [`missing_metadata_count`](Self::missing_metadata_count).
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.invalid_metadata_count == 0
            && self.schema_errors.is_empty()
            && self.digest_errors.is_empty()
            && self.order_errors.is_empty()
    }
}

/// A problem found on a single line.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LineError {
    /// One-based line number.
    pub line_number: usize,
    /// The snapshot's stored digest (absent if the line itself could not be parsed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// A description of the problem.
    pub error: String,
}

/// Deserialize snapshot content with the wxj schema selected by `flat`, returning any error.
fn schema_error(content: &str, flat: bool) -> Option<serde_json::Error> {
    if flat {
        serde_json::from_str::<wxj::flat::TweetSnapshot<'_>>(content).err()
    } else {
        serde_json::from_str::<wxj::data::TweetSnapshot<'_>>(content).err()
    }
}

/// Validate parsed snapshot lines: digest correctness, ascending digest order, metadata, and the
/// schema.
///
/// This is the core of [`validate`], separated from file I/O so it can be tested directly.
fn validate_results<
    'a,
    I: IntoIterator<Item = Result<ExactSnapshot<'a>, archivindex_wbm_json::Error>>,
>(
    results: I,
    context: &Context,
    flat: bool,
) -> Result<ValidationSummary, Error> {
    let mut summary = ValidationSummary::default();
    // The hasher is reused across lines; `Context::verify` resets it after each use.
    let mut hasher = Sha1::default();
    // The running maximum digest. Comparing each line against the maximum (rather than only the
    // immediately preceding line) also catches a duplicate of any earlier digest that follows an
    // out-of-order line.
    let mut max_digest = None;

    for (index, result) in results.into_iter().enumerate() {
        let line_number = index + 1;
        summary.line_count += 1;

        match result {
            Ok(snapshot) => {
                let mut valid = true;

                // A URL is only meaningful alongside a timestamp, so a line with a URL but no
                // timestamp is invalid; a line with neither is merely missing metadata.
                if snapshot.timestamp.is_none() {
                    summary.missing_metadata_count += 1;

                    if snapshot.url.is_some() {
                        summary.invalid_metadata_count += 1;
                        valid = false;
                    }
                }

                match max_digest {
                    Some(max) if snapshot.digest <= max => {
                        summary.order_errors.push(LineError {
                            line_number,
                            digest: Some(snapshot.digest.to_string()),
                            error: format!("digest is not greater than earlier digest {max}"),
                        });
                        valid = false;
                    }
                    _ => max_digest = Some(snapshot.digest),
                }

                if let Err(error) = context.verify(&snapshot, &mut hasher) {
                    summary.digest_errors.push(LineError {
                        line_number,
                        digest: Some(snapshot.digest.to_string()),
                        error: error.to_string(),
                    });
                    valid = false;
                }

                if let Some(error) = schema_error(snapshot.content.as_str(), flat) {
                    summary.schema_errors.push(LineError {
                        line_number,
                        digest: Some(snapshot.digest.to_string()),
                        error: error.to_string(),
                    });
                    valid = false;
                }

                if valid {
                    summary.valid_count += 1;
                }
            }
            // Read failures are fatal (the `?` propagates them out of the loop); a line that fails
            // to parse as a compact snapshot record is recorded like a schema mismatch.
            Err(archivindex_wbm_json::Error::Io(error)) => Err(error)?,
            Err(error) => summary.schema_errors.push(LineError {
                line_number,
                digest: None,
                error: error.to_string(),
            }),
        }
    }

    Ok(summary)
}

/// Validate a zstd-compressed compact snapshot file.
///
/// Each line is checked four ways: its stored digest must match its content under `context`, the
/// digests must be strictly ascending by SHA-1 bytes (which also forbids duplicates), a line with
/// a URL must also have a timestamp (lines with no timestamp at all are only counted), and its
/// content must match a wxj schema.
///
/// # Arguments
///
/// * `input` - Path to a zstd-compressed compact snapshot NDJSON file
/// * `context` - The site context used to verify digests
/// * `flat` - Validate content against the wxj/flat schema instead of wxj/data
///
/// # Errors
///
/// Returns `Error::Io` if the file cannot be opened or read; per-line problems are recorded in the
/// summary rather than returned as errors.
pub fn validate<P: AsRef<Path>>(
    input: &P,
    context: &Context,
    flat: bool,
) -> Result<ValidationSummary, Error> {
    validate_results(SnapshotReader::open(input)?, context, flat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use archivindex_wbm_json::format::Format;

    // Example files are named by the Base32-encoded SHA-1 digest of their bytes, so snapshots
    // built from them with `Context::unprocessed_snapshot` carry correct digests.
    const EXAMPLE_A: &[u8] =
        include_bytes!("../../examples/wxj/other/2PA6SG6HGUHVQD24ZVYHXESJ2I3WIY53");
    const EXAMPLE_B: &[u8] =
        include_bytes!("../../examples/wxj/other/2WRH57KC2V3N4RK3QAH3ZXPDVXJKPDPD");

    const DATA_EXAMPLE: &str =
        include_str!("../../examples/wxj/other/2PA6SG6HGUHVQD24ZVYHXESJ2I3WIY53");

    fn example_snapshot<'a>(bytes: &'a [u8], context: &Context) -> ExactSnapshot<'a> {
        context
            .unprocessed_snapshot(&Format::Utf8, bytes)
            .expect("valid example snapshot")
    }

    /// The two example snapshots in ascending digest order.
    fn ordered_examples(context: &Context) -> (ExactSnapshot<'static>, ExactSnapshot<'static>) {
        let a = example_snapshot(EXAMPLE_A, context);
        let b = example_snapshot(EXAMPLE_B, context);

        if a.digest < b.digest { (a, b) } else { (b, a) }
    }

    #[test]
    fn schema_error_accepts_valid_data_snapshot() {
        assert!(schema_error(DATA_EXAMPLE, false).is_none());
    }

    #[test]
    fn schema_error_rejects_data_snapshot_as_flat() {
        assert!(schema_error(DATA_EXAMPLE, true).is_some());
    }

    #[test]
    fn schema_error_rejects_empty_object() {
        assert!(schema_error("{}", false).is_some());
        assert!(schema_error("{}", true).is_some());
    }

    #[test]
    fn validate_results_accepts_ordered_valid_snapshots() {
        let context = Context::default();
        let (first, second) = ordered_examples(&context);

        let summary = validate_results([Ok(first), Ok(second)], &context, false)
            .expect("validation should not fail on I/O");

        assert!(summary.digest_errors.is_empty());
        assert!(summary.order_errors.is_empty());
        assert_eq!(summary.line_count, 2);
        // Unprocessed snapshots carry no timestamp, so both lines are missing metadata (which
        // does not make them invalid).
        assert_eq!(summary.missing_metadata_count, 2);
        assert_eq!(summary.invalid_metadata_count, 0);
    }

    #[test]
    fn validate_results_rejects_url_without_timestamp() {
        let context = Context::default();
        let mut snapshot = example_snapshot(EXAMPLE_A, &context);
        snapshot.url = Some("https://twitter.com/example/status/1".into());

        let summary = validate_results([Ok(snapshot)], &context, false)
            .expect("validation should not fail on I/O");

        assert_eq!(summary.invalid_metadata_count, 1);
        assert_eq!(summary.missing_metadata_count, 1);
        assert_eq!(summary.valid_count, 0);
        assert!(!summary.is_successful());
    }

    #[test]
    fn validate_results_accepts_url_with_timestamp() {
        let context = Context::default();
        let mut snapshot = example_snapshot(EXAMPLE_A, &context);
        snapshot.url = Some("https://twitter.com/example/status/1".into());
        snapshot.timestamp = Some("20230101000000".parse().expect("valid timestamp"));

        let summary = validate_results([Ok(snapshot)], &context, false)
            .expect("validation should not fail on I/O");

        assert_eq!(summary.invalid_metadata_count, 0);
        assert_eq!(summary.missing_metadata_count, 0);
        assert_eq!(summary.valid_count, 1);
    }

    #[test]
    fn validate_results_rejects_incorrect_digest() {
        let context = Context::default();
        let mut snapshot = example_snapshot(EXAMPLE_A, &context);
        snapshot.digest.0[0] ^= 0xff;

        let summary = validate_results([Ok(snapshot)], &context, false)
            .expect("validation should not fail on I/O");

        assert_eq!(summary.digest_errors.len(), 1);
        assert_eq!(summary.valid_count, 0);
    }

    #[test]
    fn validate_results_rejects_descending_digests() {
        let context = Context::default();
        let (first, second) = ordered_examples(&context);

        let summary = validate_results([Ok(second), Ok(first)], &context, false)
            .expect("validation should not fail on I/O");

        assert_eq!(summary.order_errors.len(), 1);
        assert_eq!(summary.order_errors[0].line_number, 2);
    }

    #[test]
    fn validate_results_rejects_duplicate_digests() {
        let context = Context::default();
        let snapshot = example_snapshot(EXAMPLE_A, &context);

        let summary = validate_results([Ok(snapshot.clone()), Ok(snapshot)], &context, false)
            .expect("validation should not fail on I/O");

        assert_eq!(summary.order_errors.len(), 1);
        assert_eq!(summary.order_errors[0].line_number, 2);
    }

    #[test]
    fn validate_results_records_unparseable_line() {
        let context = Context::default();

        let summary = validate_results(
            [Err(archivindex_wbm_json::Error::InvalidLine)],
            &context,
            false,
        )
        .expect("validation should not fail on I/O");

        assert_eq!(summary.schema_errors.len(), 1);
        assert!(summary.schema_errors[0].digest.is_none());
        assert_eq!(summary.valid_count, 0);
    }
}
