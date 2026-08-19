#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
use archivindex_wbm::digest::Sha1Digest;
use archivindex_wbm_json::{context::Context, format::Format};
use archivindex_wbm_json_processing::io::write::SnapshotWriter;
use cli_helpers::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

mod db;

/// Author whose tweets the `wxj extract` command selects (the Grok account).
const GROK_USER_ID: u64 = 1_720_665_183_188_922_368;

/// Default closing whitespace for Twitter JSON snapshots served by the Wayback Machine.
///
/// Twitter tweet snapshots almost always end with `\r\r\n` after the closing brace; this is the
/// default the [`Context`] uses to strip trailing whitespace from stored content and to reconstruct
/// the exact bytes when confirming a snapshot's digest.
const TWITTER_CLOSING_WHITESPACE: [char; 3] = ['\r', '\r', '\n'];

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::TweetsDb { db, command } => match command {
            TweetsDbCommand::AddPairs => {
                let db = db::tweets::Database::open(db)?;
                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(std::io::stdin());

                for result in reader.deserialize::<IdPair>() {
                    let IdPair { user_id, tweet_id } = result?;

                    db.insert(user_id, tweet_id)?;
                }
            }

            TweetsDbCommand::Lookup { last } => {
                let db = db::tweets::Database::open(db)?;
                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(std::io::stdin());

                for result in reader.deserialize::<u64>() {
                    let user_id = result?;

                    if last {
                        if let Some(tweet_id) = db
                            .lookup_live(user_id)
                            .max_by_key(|result| result.as_ref().copied().unwrap_or(u64::MAX))
                        {
                            println!("{},{}", user_id, tweet_id?);
                        } else {
                            println!("{user_id},");
                        }
                    } else {
                        for tweet_id in db.lookup_live(user_id) {
                            println!("{},{}", user_id, tweet_id?);
                        }
                    }
                }
            }
        },
        Command::Wxj { command } => match command {
            WxjCommand::Validate { input, flat } => {
                let reader = BufReader::new(File::open(input)?);
                let decoder = BufReader::new(zstd::Decoder::with_buffer(reader)?);

                for (i, result) in decoder.lines().enumerate() {
                    if i % 1_000_000 == 0 {
                        log::info!("Done: {i}");
                    }

                    let line = result?;

                    if flat {
                        serde_json::from_str::<
                            WxjWrapper<birdsite::model::wxj::flat::TweetSnapshot<'_>>,
                        >(&line)
                        .map_err(|error| Error::Wxj {
                            error,
                            line_number: i + 1,
                        })?;
                    } else {
                        serde_json::from_str::<
                            WxjWrapper<birdsite::model::wxj::data::TweetSnapshot<'_>>,
                        >(&line)
                        .map_err(|error| Error::Wxj {
                            error,
                            line_number: i + 1,
                        })?;
                    }
                }
            }
            WxjCommand::Extract { input } => extract_tweets(input, GROK_USER_ID)?,
        },
        Command::Compact {
            input,
            output,
            level,
        } => compact_snapshots(&input, &output, level)?,
        Command::GittRaw { input } => print_gitt_lines(&input)?,
        Command::Gitt { input } => print_gitt_csv(&input)?,
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI argument reading error")]
    Args(#[from] cli_helpers::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("WXJ error")]
    Wxj {
        error: serde_json::Error,
        line_number: usize,
    },
    #[error("TweetsDB error")]
    TweetsDb(#[from] db::tweets::Error),
}

#[derive(Debug, Parser)]
#[clap(name = "birdsite-hacks", version, author)]
struct Opts {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    TweetsDb {
        #[clap(long)]
        db: PathBuf,
        #[clap(subcommand)]
        command: TweetsDbCommand,
    },
    Wxj {
        #[clap(subcommand)]
        command: WxjCommand,
    },
    /// Compact a directory of digest-named Twitter JSON snapshots into a Zstandard NDJSON file.
    Compact {
        /// Directory whose files are Twitter JSON snapshots named by their Base32 SHA-1 digest.
        #[clap(long)]
        input: PathBuf,
        /// Output path for the compact (Zstandard-compressed NDJSON) file.
        #[clap(long)]
        output: PathBuf,
        /// Zstandard compression level.
        #[clap(long, default_value_t = 14)]
        level: u16,
    },
    /// Print WXJ lines containing a tweet that asks Grok whether something is true.
    GittRaw {
        /// Path to a Zstandard-compressed WXJ JSONL file.
        #[clap(long)]
        input: PathBuf,
    },
    /// Print CSV metadata for tweets that ask Grok whether something is true.
    Gitt {
        /// Path to a Zstandard-compressed WXJ JSONL file.
        #[clap(long)]
        input: PathBuf,
    },
}

#[derive(Debug, Parser)]
enum TweetsDbCommand {
    AddPairs,
    Lookup {
        #[clap(long)]
        last: bool,
    },
}

#[derive(Debug, Parser)]
enum WxjCommand {
    Validate {
        #[clap(long)]
        input: PathBuf,
        #[clap(long)]
        flat: bool,
    },
    Extract {
        #[clap(long)]
        input: PathBuf,
    },
}

/// Returns whether any single tweet text contains both GITT phrases, ignoring ASCII case.
fn tweets_match_gitt<'a>(texts: impl IntoIterator<Item = &'a str>) -> bool {
    texts.into_iter().any(|text| {
        let text = text.to_ascii_lowercase();
        text.contains("@grok") && text.contains("is this true")
    })
}

/// Extracts the tweet ID from the first `/status/<digits>` path segment in a URL.
fn tweet_id_from_url(url: &str) -> Option<u64> {
    let (_, suffix) = url.split_once("/status/")?;
    let end = suffix
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(suffix.len());

    suffix.get(..end)?.parse().ok()
}

const fn reference_type_name(
    reference_type: birdsite::model::wxj::data::ReferenceType,
) -> &'static str {
    use birdsite::model::wxj::data::ReferenceType;

    match reference_type {
        ReferenceType::Retweeted => "retweeted",
        ReferenceType::RepliedTo => "replied_to",
        ReferenceType::Quoted => "quoted",
    }
}

fn url_tweet_id(url: &birdsite::model::wxj::data::entity::Url<'_>) -> Option<u64> {
    [
        url.expanded_url.as_deref(),
        url.unwound_url.as_deref(),
        url.display_url.as_deref(),
        Some(url.url.as_ref()),
    ]
    .into_iter()
    .flatten()
    .find_map(tweet_id_from_url)
}

/// Prints one CSV row for each matching tweet in a Zstandard-compressed WXJ data-model JSONL file.
fn print_gitt_csv(input: &Path) -> Result<(), Error> {
    let reader = BufReader::new(File::open(input)?);
    let decoder = BufReader::new(zstd::Decoder::with_buffer(reader)?);
    let stdout = std::io::stdout();
    let mut output = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(stdout.lock());

    for (i, result) in decoder.lines().enumerate() {
        let line = result?;
        let snapshot = serde_json::from_str::<
            WxjWrapper<birdsite::model::wxj::data::TweetSnapshot<'_>>,
        >(&line)
        .map_err(|error| Error::Wxj {
            error,
            line_number: i + 1,
        })?;

        let mut seen_tweet_ids = std::collections::HashSet::new();
        let tweets = std::iter::once(&snapshot.content.data)
            .chain(snapshot.content.includes.tweets.iter().flatten())
            .filter(|tweet| seen_tweet_ids.insert(tweet.id));

        for tweet in tweets.filter(|tweet| tweets_match_gitt([tweet.text.as_ref()])) {
            let referenced_tweets = tweet.referenced_tweets.as_deref().unwrap_or_default();
            if referenced_tweets.len() > 1 {
                log::warn!(
                    "Tweet {} has {} referenced tweets; using the first",
                    tweet.id,
                    referenced_tweets.len()
                );
            }
            let reference = referenced_tweets.first();

            let mut url_tweet_ids = tweet
                .entities
                .as_ref()
                .and_then(|entities| entities.urls.as_ref())
                .into_iter()
                .flatten()
                .filter_map(url_tweet_id);
            let url_tweet_id = url_tweet_ids.next();
            let additional_url_tweet_ids = url_tweet_ids.count();
            if additional_url_tweet_ids > 0 {
                log::warn!(
                    "Tweet {} has {} tweet IDs in entities.urls; using the first",
                    tweet.id,
                    additional_url_tweet_ids + 1
                );
            }

            output.serialize((
                tweet.id,
                tweet.author_id,
                snapshot
                    .content
                    .lookup_user(tweet.author_id)
                    .map(|user| user.username.as_ref()),
                reference.map(|reference| reference.id),
                reference.map(|reference| reference_type_name(reference.reference_type)),
                url_tweet_id,
            ))?;
        }
    }

    output.flush()?;

    Ok(())
}

/// Prints matching lines from a Zstandard-compressed WXJ data-model JSONL file.
fn print_gitt_lines(input: &Path) -> Result<(), Error> {
    let reader = BufReader::new(File::open(input)?);
    let decoder = BufReader::new(zstd::Decoder::with_buffer(reader)?);
    let stdout = std::io::stdout();
    let mut output = stdout.lock();

    for (i, result) in decoder.lines().enumerate() {
        let line = result?;
        let snapshot = serde_json::from_str::<
            WxjWrapper<birdsite::model::wxj::data::TweetSnapshot<'_>>,
        >(&line)
        .map_err(|error| Error::Wxj {
            error,
            line_number: i + 1,
        })?;

        let texts = std::iter::once(snapshot.content.data.text.as_ref()).chain(
            snapshot
                .content
                .includes
                .tweets
                .iter()
                .flatten()
                .map(|tweet| tweet.text.as_ref()),
        );

        if tweets_match_gitt(texts) {
            writeln!(output, "{line}")?;
        }
    }

    Ok(())
}

/// Prints one CSV row for each tweet by `author_id` found in the directory of WXJ snapshot files at
/// `input`.
fn extract_tweets(input: PathBuf, author_id: u64) -> Result<(), Error> {
    let mut paths = std::fs::read_dir(input)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;

    paths.sort();

    log::info!("Loaded {} paths", paths.len());

    for path in &paths {
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                if let Ok(snapshot) =
                    serde_json::from_str::<birdsite::model::wxj::data::TweetSnapshot<'_>>(&contents)
                {
                    for tweet in snapshot.includes.tweets.iter().flatten() {
                        if tweet.author_id == author_id {
                            let user = snapshot.lookup_user(author_id);

                            let in_reply_to_user_id = tweet.in_reply_to_user_id;

                            let in_reply_to_info = in_reply_to_user_id.and_then(|user_id| {
                                snapshot
                                    .lookup_user(user_id)
                                    .map(|user| (user.username.clone(), user.verified))
                            });

                            println!(
                                "{},{},{},{},{},{},{}",
                                tweet.author_id,
                                tweet.id,
                                tweet.created_at,
                                in_reply_to_user_id
                                    .map(|user_id| user_id.to_string())
                                    .unwrap_or_default(),
                                in_reply_to_info
                                    .as_ref()
                                    .map(|(screen_name, _)| screen_name.to_string())
                                    .unwrap_or_default(),
                                in_reply_to_info
                                    .as_ref()
                                    .map(|(_, verified)| verified.to_string())
                                    .unwrap_or_default(),
                                user.and_then(|user| user.public_metrics.tweet_count)
                                    .map(|tweet_count| tweet_count.to_string())
                                    .unwrap_or_default()
                            );
                        }
                    }
                }
            }
            Err(error) => {
                log::error!("{}: {}", path.as_os_str().to_string_lossy(), error);
            }
        }
    }

    Ok(())
}

/// Compacts a directory of digest-named Twitter JSON snapshot files into a single Zstandard NDJSON
/// file at `output`.
///
/// Each file in `input` is read and kept only if it clears two checks:
///
/// 1. Its contents hash to the Base32 SHA-1 digest it is named by (computed under the default
///    Twitter closing whitespace), and
/// 2. Its contents parse as the WXJ data-model tweet snapshot
///    ([`birdsite::model::wxj::data::TweetSnapshot`]).
///
/// Qualifying snapshots are serialized as canonical NDJSON (one per line) into the Zstandard stream
/// via [`SnapshotWriter`] at the given `level`. Files are processed in digest-sorted order so the
/// output is sorted and the writer's consecutive-digest deduplication removes duplicates.
///
/// # Errors
///
/// Returns [`Error::Io`] if the directory cannot be read or if writing the output fails.
fn compact_snapshots(input: &PathBuf, output: &PathBuf, level: u16) -> Result<(), Error> {
    let context =
        Context::from_static(&TWITTER_CLOSING_WHITESPACE).expect("valid closing whitespace");

    // Keep only files whose name is a valid Base32 SHA-1 digest, pairing each with its decoded
    // digest. Anything not named by a digest cannot "match the digest", so it is dropped here.
    let mut files = std::fs::read_dir(input)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter_map(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .and_then(|name| name.parse::<Sha1Digest>().ok())
                .map(|digest| (digest, path))
        })
        .collect::<Vec<_>>();

    // Sort by the decoded digest, whose `Ord` compares the raw `[u8; 20]` (binary SHA-1 order).
    // Base32's ASCII order differs from binary order, so sorting the encoded filenames would not
    // match the digest order the rest of the ecosystem uses. Sorting here yields a digest-sorted
    // output and lets the writer skip consecutive duplicate digests.
    files.sort_by_key(|(digest, _)| *digest);

    log::info!("Loaded {} paths", files.len());

    // The writer owns the context; `create_new` refuses to overwrite an existing output file.
    let mut writer = SnapshotWriter::create(output, level, context)?;

    let mut written: u64 = 0;

    for (expected_digest, path) in &files {
        let bytes = std::fs::read(path)?;

        // Building an unprocessed snapshot computes the digest over the raw bytes and strips the
        // default Twitter closing whitespace (`\r\r\n`) into the content field.
        let snapshot = match writer.context().unprocessed_snapshot(&Format::Utf8, &bytes) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                log::warn!("{}: {}", path.as_os_str().to_string_lossy(), error);
                continue;
            }
        };

        // Check 1: the contents must hash to the digest the file is named by.
        if snapshot.digest != *expected_digest {
            continue;
        }

        // Check 2: the content must deserialize as a WXJ data-model tweet snapshot.
        if let Err(error) = serde_json::from_str::<birdsite::model::wxj::data::TweetSnapshot<'_>>(
            snapshot.content.as_str(),
        ) {
            log::info!("{expected_digest}: not a valid tweet snapshot: {error}");
            continue;
        }

        if writer.write_snapshot(&snapshot)? {
            written += 1;
        }
    }

    writer.finish()?;

    log::info!(
        "Wrote {written} snapshots to {}",
        output.as_os_str().to_string_lossy()
    );

    Ok(())
}

#[derive(serde::Deserialize)]
struct IdPair {
    user_id: u64,
    tweet_id: u64,
}

#[derive(serde::Deserialize)]
struct WxjWrapper<S> {
    // Never read: deserializing it is the validation.
    #[allow(dead_code)]
    content: S,
}

#[cfg(test)]
mod tests {
    use super::{tweet_id_from_url, tweets_match_gitt};

    #[test]
    fn gitt_match_is_case_insensitive() {
        assert!(tweets_match_gitt(["Hey @GrOk, IS THIS TRUE?"]));
    }

    #[test]
    fn gitt_match_requires_both_phrases_in_one_tweet() {
        assert!(!tweets_match_gitt(["@grok", "is this true?"]));
    }

    #[test]
    fn gitt_match_checks_every_tweet() {
        assert!(tweets_match_gitt([
            "unrelated",
            "@grok could you tell me: is this true?",
        ]));
    }

    #[test]
    fn extracts_tweet_id_from_url_with_trailing_path() {
        assert_eq!(
            tweet_id_from_url("https://x.com/HieroBorschtEsq/status/2008995132025258106/photo/1"),
            Some(2_008_995_132_025_258_106)
        );
    }

    #[test]
    fn url_without_status_id_does_not_match() {
        assert_eq!(tweet_id_from_url("https://x.com/user/status/nope"), None);
        assert_eq!(tweet_id_from_url("https://example.com/123"), None);
    }
}
