//! Validate a compact snapshot file against wxj schemas.
//!
//! Each line's content is deserialized with the `birdsite` wxj model types (which reject unknown
//! fields), so validation checks the full schema rather than the presence of a few fields.

use archivindex_wbm_json::io::read::SnapshotReader;
use birdsite::model::wxj;
use std::path::Path;

/// Validation error.
///
/// Only I/O failures abort validation; per-line problems (unparseable lines and schema mismatches)
/// are recorded in the summary instead.
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
    /// Number of lines whose content matched the schema.
    pub valid_count: usize,
    /// Lines that could not be parsed or whose content did not match the schema.
    pub schema_errors: Vec<SchemaError>,
}

impl ValidationSummary {
    /// Whether every line parsed and its content matched the schema.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.schema_errors.is_empty()
    }
}

/// A line that could not be parsed or whose content did not match the schema.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SchemaError {
    /// One-based line number.
    pub line_number: usize,
    /// The snapshot's digest (absent if the line itself could not be parsed as a snapshot record).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The parsing or deserialization error message.
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

/// Validate a zstd-compressed compact snapshot file against a wxj schema.
///
/// # Arguments
///
/// * `input` - Path to a zstd-compressed compact snapshot NDJSON file
/// * `flat` - Validate content against the wxj/flat schema instead of wxj/data
///
/// # Errors
///
/// Returns `Error::Io` if the file cannot be opened or read; per-line problems are recorded in the
/// summary rather than returned as errors.
pub fn validate<P: AsRef<Path>>(input: &P, flat: bool) -> Result<ValidationSummary, Error> {
    let mut summary = ValidationSummary::default();

    for (index, result) in SnapshotReader::open(input)?.enumerate() {
        summary.line_count += 1;

        match result {
            Ok(snapshot) => match schema_error(snapshot.content.as_str(), flat) {
                None => summary.valid_count += 1,
                Some(error) => summary.schema_errors.push(SchemaError {
                    line_number: index + 1,
                    digest: Some(snapshot.digest.to_string()),
                    error: error.to_string(),
                }),
            },
            // Read failures are fatal (the `?` propagates them out of the loop); a line that fails
            // to parse as a compact snapshot record is recorded like a schema mismatch.
            Err(archivindex_wbm_json::Error::Io(error)) => Err(error)?,
            Err(error) => summary.schema_errors.push(SchemaError {
                line_number: index + 1,
                digest: None,
                error: error.to_string(),
            }),
        }
    }

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::schema_error;

    const DATA_EXAMPLE: &str =
        include_str!("../../examples/wxj/other/2PA6SG6HGUHVQD24ZVYHXESJ2I3WIY53");

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
}
