//! Extract the lines of a compact snapshot file whose tweets mention a given user.
//!
//! Each input line is a compact snapshot (as written by pack and enhance) whose content is a
//! wxj/data tweet object; a line matches when the content's `includes.users` array contains a user
//! with the requested ID. Matching lines are copied to the output verbatim, in input order.

use archivindex_wbm_json::exact::ExactSnapshot;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// Extraction error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("snapshot parsing error on line {line_number}")]
    Snapshot {
        /// One-based line number.
        line_number: usize,
        #[source]
        error: archivindex_wbm_json::Error,
    },
    #[error("content JSON parsing error on line {line_number}")]
    Json {
        /// One-based line number.
        line_number: usize,
        #[source]
        error: serde_json::Error,
    },
}

/// Summary of an extract operation.
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Serialize)]
pub struct Summary {
    /// Number of lines read from the input.
    pub read_count: u64,
    /// Number of lines mentioning the user, copied to the output.
    pub matched_count: u64,
}

/// The minimal projection of a snapshot's wxj/data content needed for the filter: the IDs of the
/// users in `includes.users`. Everything else in the content is ignored (and not validated).
#[derive(serde::Deserialize)]
struct Content<'a> {
    #[serde(borrow)]
    includes: Option<Includes<'a>>,
}

#[derive(serde::Deserialize)]
struct Includes<'a> {
    #[serde(borrow, default)]
    users: Vec<UserRef<'a>>,
}

#[derive(serde::Deserialize)]
struct UserRef<'a> {
    // Twitter encodes user IDs as decimal strings in this format, so the ID is kept as the raw
    // string and matched by string equality (no per-user integer parsing).
    #[serde(borrow)]
    id: Cow<'a, str>,
}

/// Copy the lines whose content's `includes.users` array contains a user with ID `user_id` to
/// `writer`.
///
/// This is the core of [`extract`], separated from file I/O and compression so it can be tested
/// directly.
fn extract_lines<I: IntoIterator<Item = std::io::Result<String>>, W: Write>(
    lines: I,
    user_id: u64,
    writer: &mut W,
) -> Result<Summary, Error> {
    // Rendering the target ID once makes each comparison against the stored decimal strings a
    // plain string equality.
    let user_id = user_id.to_string();
    let mut summary = Summary::default();

    for (index, result) in lines.into_iter().enumerate() {
        let line = result?;
        let line_number = index + 1;
        summary.read_count += 1;

        let snapshot =
            ExactSnapshot::parse(&line).map_err(|error| Error::Snapshot { line_number, error })?;

        let content: Content<'_> = serde_json::from_str(snapshot.content.as_str())
            .map_err(|error| Error::Json { line_number, error })?;

        if content
            .includes
            .is_some_and(|includes| includes.users.iter().any(|user| user.id == user_id))
        {
            writeln!(writer, "{line}")?;
            summary.matched_count += 1;
        }
    }

    Ok(summary)
}

/// Extract the lines of a zstd-compressed compact snapshot file whose content mentions `user_id`
/// in its `includes.users` array, copying them verbatim to a new zstd-compressed snapshot file.
///
/// # Arguments
///
/// * `input` - Path to a zstd-compressed compact snapshot file of wxj/data tweet content
/// * `user_id` - The Twitter user ID to select
/// * `output` - Output path for the matching lines (must not already exist)
/// * `compression_level` - Zstandard compression level (e.g. 14)
///
/// # Errors
///
/// Returns [`Error::Io`] if file I/O fails, [`Error::Snapshot`] if a line is not a compact
/// snapshot, or [`Error::Json`] if a snapshot's content is not valid JSON.
pub fn extract(
    input: &Path,
    user_id: u64,
    output: &Path,
    compression_level: u16,
) -> Result<Summary, Error> {
    let reader = BufReader::new(zstd::Decoder::new(File::open(input)?)?);
    // `create_new` refuses to overwrite an existing output file.
    let mut writer = zstd::Encoder::new(
        BufWriter::new(File::create_new(output)?),
        i32::from(compression_level),
    )?;

    let summary = extract_lines(reader.lines(), user_id, &mut writer)?;

    // Finishing the encoder writes the final zstd frame into the buffer, which then needs its own
    // flush to reach the file.
    writer.finish()?.flush()?;

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: u64 = 12;
    /// Any well-formed Base32 SHA-1 digest: `ExactSnapshot::parse` does not check it against the
    /// content.
    const DIGEST: &str = "2PA6SG6HGUHVQD24ZVYHXESJ2I3WIY53";

    /// A compact snapshot line whose content's `includes.users` array contains the given IDs.
    fn line(user_ids: &[u64]) -> String {
        let users: Vec<String> = user_ids
            .iter()
            .map(|id| format!(r#"{{"id":"{id}"}}"#))
            .collect();
        format!(
            r#"{{"digest":"{DIGEST}","content":{{"data":{{}},"includes":{{"users":[{}]}}}}}}"#,
            users.join(",")
        )
    }

    fn run(input: &[String], user_id: u64) -> (Summary, String) {
        let mut output = Vec::new();
        let summary = extract_lines(input.iter().cloned().map(Ok), user_id, &mut output)
            .expect("extract succeeds");

        (summary, String::from_utf8(output).expect("UTF-8 output"))
    }

    #[test]
    fn extract_lines_copies_matching_lines_verbatim() {
        let input = [line(&[12, 34]), line(&[34]), line(&[12])];

        let (summary, output) = run(&input, TARGET);

        assert_eq!(summary.read_count, 3);
        assert_eq!(summary.matched_count, 2);
        assert_eq!(output, format!("{}\n{}\n", input[0], input[2]));
    }

    #[test]
    fn extract_lines_skips_content_without_includes() {
        let input = [
            format!(r#"{{"digest":"{DIGEST}","content":{{"created_at":"x"}}}}"#),
            line(&[12]),
        ];

        let (summary, output) = run(&input, TARGET);

        assert_eq!(summary.matched_count, 1);
        assert_eq!(output, format!("{}\n", input[1]));
    }

    #[test]
    fn extract_lines_does_not_match_id_prefixes() {
        let (summary, output) = run(&[line(&[123]), line(&[1])], TARGET);

        assert_eq!(summary.read_count, 2);
        assert_eq!(summary.matched_count, 0);
        assert!(output.is_empty());
    }

    #[test]
    fn extract_lines_reports_snapshot_error_with_line_number() {
        let input = [line(&[12]), "not a snapshot".to_string()];
        let mut output = Vec::new();

        let result = extract_lines(input.iter().cloned().map(Ok), TARGET, &mut output);

        assert!(matches!(
            result,
            Err(Error::Snapshot { line_number: 2, .. })
        ));
    }
}
