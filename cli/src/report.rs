//! CSV reports over compact snapshot files.
//!
//! `tweet_ids` prints a `user_id,tweet_id` row for every tweet carried in each snapshot's content
//! (including any retweeted, replied-to, or quoted tweets the snapshot carries);
//! `user_observations` prints a row per observed user with the capture timestamps (Unix epoch
//! seconds) at which the user was seen.

use archivindex_wbm_json::exact::ExactSnapshot;
use archivindex_wbm_json_processing::io::read::SnapshotReader;
use birdsite::model::wxj::{self, TweetSnapshot, metadata::tweet::TweetMetadata};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;

/// Report error.
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
    #[error("wxj/data format error on line {line_number}")]
    Format {
        /// One-based line number.
        line_number: usize,
        #[source]
        error: wxj::data::FormatError,
    },
}

/// Summary of a report operation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Summary {
    /// Number of lines read from the input.
    pub read_count: u64,
    /// Number of CSV rows written.
    pub written_count: u64,
}

/// Deserialize snapshot content with the wxj schema selected by `flat`.
fn parse_content(content: &str, flat: bool) -> Result<TweetSnapshot<'_>, serde_json::Error> {
    if flat {
        serde_json::from_str::<wxj::flat::TweetSnapshot<'_>>(content).map(TweetSnapshot::Flat)
    } else {
        serde_json::from_str::<wxj::data::TweetSnapshot<'_>>(content).map(TweetSnapshot::Data)
    }
}

/// The users carried in a snapshot's content, as (ID, screen name) pairs.
///
/// For the data format these are the users of `includes.users` (derived users, which have no
/// screen name, are skipped); for the flat format they are the authors of the tweet and of any
/// quoted or retweeted tweets it carries.
fn content_users<'s>(content: &'s TweetSnapshot<'_>) -> Vec<(u64, &'s str)> {
    match content {
        TweetSnapshot::Data(snapshot) => snapshot
            .includes
            .users()
            .map(|user| (user.id, user.username.as_ref()))
            .collect(),
        TweetSnapshot::Flat(snapshot) => snapshot
            .users()
            .into_iter()
            .map(|user| (user.id, user.screen_name.as_ref()))
            .collect(),
    }
}

/// Write a `user_id,tweet_id` row for every tweet carried in each snapshot's content.
///
/// This is the core of [`tweet_ids`], separated from file I/O so it can be tested directly.
fn write_tweet_ids<'a, I, W>(results: I, flat: bool, writer: &mut W) -> Result<Summary, Error>
where
    I: IntoIterator<Item = Result<ExactSnapshot<'a>, archivindex_wbm_json::Error>>,
    W: Write,
{
    let mut summary = Summary::default();

    for (index, result) in results.into_iter().enumerate() {
        let line_number = index + 1;
        summary.read_count += 1;

        let snapshot = result.map_err(|error| Error::Snapshot { line_number, error })?;
        let content = parse_content(snapshot.content.as_str(), flat)
            .map_err(|error| Error::Json { line_number, error })?;
        let tweets = TweetMetadata::from_tweet_snapshot(&content)
            .map_err(|error| Error::Format { line_number, error })?;

        for tweet in tweets {
            writeln!(writer, "{},{}", tweet.user.id, tweet.id)?;
            summary.written_count += 1;
        }
    }

    Ok(summary)
}

/// Write a row per observed user with the capture timestamps at which the user was seen.
///
/// This is the core of [`user_observations`], separated from file I/O so it can be tested
/// directly.
fn write_user_observations<'a, I, W>(
    results: I,
    flat: bool,
    range_only: bool,
    writer: &mut W,
) -> Result<Summary, Error>
where
    I: IntoIterator<Item = Result<ExactSnapshot<'a>, archivindex_wbm_json::Error>>,
    W: Write,
{
    let mut summary = Summary::default();
    // Keyed by (ID, screen name) so a renamed user gets one row per name; the map orders the
    // output rows and each set dedups and orders one user's timestamps as they are inserted. In
    // range-only mode each set is capped at its earliest and latest values (see the insert below).
    let mut observations = BTreeMap::<(u64, String), BTreeSet<i64>>::new();

    for (index, result) in results.into_iter().enumerate() {
        let line_number = index + 1;
        summary.read_count += 1;

        let snapshot = result.map_err(|error| Error::Snapshot { line_number, error })?;

        // A snapshot with no capture timestamp contributes no observations, so its content is
        // never parsed.
        if let Some(timestamp) = snapshot.timestamp {
            let seconds = i64::from(timestamp);
            let content = parse_content(snapshot.content.as_str(), flat)
                .map_err(|error| Error::Json { line_number, error })?;

            for (id, screen_name) in content_users(&content) {
                let timestamps = observations
                    .entry((id, screen_name.to_string()))
                    .or_default();
                timestamps.insert(seconds);

                // In range-only mode only the earliest and latest observations are ever reported,
                // so the set is capped at those two: after each insert into a full {min, max} set
                // the lone interior value is dropped, bounding memory to two entries per user
                // regardless of how many times the user is observed.
                if range_only && timestamps.len() > 2 {
                    let interior = *timestamps
                        .iter()
                        .nth(1)
                        .expect("a set of more than two values has an interior element");
                    timestamps.remove(&interior);
                }
            }
        }
    }

    for ((id, screen_name), timestamps) in observations {
        write!(writer, "{id},{screen_name}")?;

        if range_only {
            // Each set is non-empty by construction. A user with a single observation gets the
            // same value in both range columns, keeping the row shape fixed.
            if let (Some(first), Some(last)) = (timestamps.first(), timestamps.last()) {
                write!(writer, ",{first},{last}")?;
            }
        } else {
            for timestamp in &timestamps {
                write!(writer, ",{timestamp}")?;
            }
        }

        writeln!(writer)?;
        summary.written_count += 1;
    }

    Ok(summary)
}

/// Write a `user_id,tweet_id` CSV row to `writer` for every tweet carried in the content of a
/// zstd-compressed compact snapshot file, in input order.
///
/// # Arguments
///
/// * `input` - Path to a zstd-compressed compact snapshot file
/// * `flat` - Parse content with the wxj/flat schema instead of wxj/data
/// * `writer` - Destination for the CSV rows
///
/// # Errors
///
/// Returns [`Error::Io`] if file I/O fails, [`Error::Snapshot`] if a line is not a compact
/// snapshot, [`Error::Json`] if a snapshot's content does not match the selected schema, or
/// [`Error::Format`] if a wxj/data tweet's referenced-tweet fields are invalid.
pub fn tweet_ids<P: AsRef<Path>, W: Write>(
    input: &P,
    flat: bool,
    writer: &mut W,
) -> Result<Summary, Error> {
    write_tweet_ids(SnapshotReader::open(input)?, flat, writer)
}

/// Write a CSV row to `writer` for every user observed in a zstd-compressed compact snapshot
/// file, in ascending (user ID, screen name) order.
///
/// Each row is the user's ID and screen name followed by the deduplicated, ascending capture
/// timestamps (Unix epoch seconds) of the snapshots whose content carries the user. Snapshots
/// with no capture timestamp are skipped.
///
/// # Arguments
///
/// * `input` - Path to a zstd-compressed compact snapshot file
/// * `flat` - Parse content with the wxj/flat schema instead of wxj/data
/// * `range_only` - Print only the first and last observation timestamps
/// * `writer` - Destination for the CSV rows
///
/// # Errors
///
/// Returns [`Error::Io`] if file I/O fails, [`Error::Snapshot`] if a line is not a compact
/// snapshot, or [`Error::Json`] if a timestamped snapshot's content does not match the selected
/// schema.
pub fn user_observations<P: AsRef<Path>, W: Write>(
    input: &P,
    flat: bool,
    range_only: bool,
    writer: &mut W,
) -> Result<Summary, Error> {
    write_user_observations(SnapshotReader::open(input)?, flat, range_only, writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use archivindex_wbm_json::{context::Context, format::Format};

    /// A wxj/data snapshot whose content is a retweet: the main tweet (2056732294355026173 by
    /// 627778780) plus the retweeted tweet (2056695384135581993 by 2903110717) and both users
    /// (`Wildharv` and `FrLillie`) in its includes.
    const DATA_EXAMPLE: &[u8] =
        include_bytes!("../../examples/wxj/other/2PA6SG6HGUHVQD24ZVYHXESJ2I3WIY53");

    fn example_snapshot(context: &Context) -> ExactSnapshot<'static> {
        context
            .unprocessed_snapshot(&Format::Utf8, DATA_EXAMPLE)
            .expect("valid example snapshot")
    }

    /// The example snapshot with the given capture timestamp attached.
    fn observed_example(context: &Context, timestamp: &str) -> ExactSnapshot<'static> {
        let mut snapshot = example_snapshot(context);
        snapshot.timestamp = Some(timestamp.parse().expect("valid timestamp"));

        snapshot
    }

    #[test]
    fn write_tweet_ids_prints_main_and_referenced_tweets() {
        let context = Context::default();
        let mut output = Vec::new();

        let summary = write_tweet_ids([Ok(example_snapshot(&context))], false, &mut output)
            .expect("tweet IDs succeed");

        assert_eq!(
            summary,
            Summary {
                read_count: 1,
                written_count: 2
            }
        );
        assert_eq!(
            String::from_utf8(output).expect("UTF-8 output"),
            "627778780,2056732294355026173\n2903110717,2056695384135581993\n"
        );
    }

    #[test]
    fn write_tweet_ids_reports_snapshot_error_with_line_number() {
        let context = Context::default();
        let mut output = Vec::new();

        let result = write_tweet_ids(
            [
                Ok(example_snapshot(&context)),
                Err(archivindex_wbm_json::Error::InvalidLine),
            ],
            false,
            &mut output,
        );

        assert!(matches!(
            result,
            Err(Error::Snapshot { line_number: 2, .. })
        ));
    }

    #[test]
    fn write_tweet_ids_reports_schema_mismatch_as_json_error() {
        let context = Context::default();
        let mut output = Vec::new();

        // The example content is wxj/data, so parsing it as wxj/flat fails.
        let result = write_tweet_ids([Ok(example_snapshot(&context))], true, &mut output);

        assert!(matches!(result, Err(Error::Json { line_number: 1, .. })));
    }

    #[test]
    fn write_user_observations_dedups_and_sorts_timestamps() {
        let context = Context::default();
        let mut output = Vec::new();

        // Two distinct capture timestamps (given newest first) plus a duplicate of the newer one.
        let results = [
            Ok(observed_example(&context, "20230101000001")),
            Ok(observed_example(&context, "20230101000000")),
            Ok(observed_example(&context, "20230101000001")),
        ];

        let summary = write_user_observations(results, false, false, &mut output)
            .expect("user observations succeed");

        assert_eq!(
            summary,
            Summary {
                read_count: 3,
                written_count: 2
            }
        );
        assert_eq!(
            String::from_utf8(output).expect("UTF-8 output"),
            "627778780,Wildharv,1672531200,1672531201\n\
             2903110717,FrLillie,1672531200,1672531201\n"
        );
    }

    #[test]
    fn write_user_observations_range_only_prints_first_and_last() {
        let context = Context::default();
        let mut output = Vec::new();

        let results = [
            Ok(observed_example(&context, "20230101000000")),
            Ok(observed_example(&context, "20230101000001")),
            Ok(observed_example(&context, "20230101000002")),
        ];

        let summary = write_user_observations(results, false, true, &mut output)
            .expect("user observations succeed");

        assert_eq!(summary.written_count, 2);
        assert_eq!(
            String::from_utf8(output).expect("UTF-8 output"),
            "627778780,Wildharv,1672531200,1672531202\n\
             2903110717,FrLillie,1672531200,1672531202\n"
        );
    }

    #[test]
    fn write_user_observations_range_only_keeps_extremes_under_reordering() {
        let context = Context::default();
        let mut output = Vec::new();

        // Timestamps arrive out of order and include values that become the new minimum and
        // maximum only after interior values have already been dropped by the two-value cap, so
        // this exercises that capping still preserves the true earliest and latest.
        let results = [
            Ok(observed_example(&context, "20230101000005")),
            Ok(observed_example(&context, "20230101000003")),
            Ok(observed_example(&context, "20230101000009")),
            Ok(observed_example(&context, "20230101000001")),
            Ok(observed_example(&context, "20230101000007")),
        ];

        let summary = write_user_observations(results, false, true, &mut output)
            .expect("user observations succeed");

        assert_eq!(summary.written_count, 2);
        assert_eq!(
            String::from_utf8(output).expect("UTF-8 output"),
            "627778780,Wildharv,1672531201,1672531209\n\
             2903110717,FrLillie,1672531201,1672531209\n"
        );
    }

    #[test]
    fn write_user_observations_skips_snapshots_without_timestamps() {
        let context = Context::default();
        let mut output = Vec::new();

        let summary =
            write_user_observations([Ok(example_snapshot(&context))], false, false, &mut output)
                .expect("user observations succeed");

        assert_eq!(
            summary,
            Summary {
                read_count: 1,
                written_count: 0
            }
        );
        assert!(output.is_empty());
    }
}
