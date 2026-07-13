//! CLI for processing Wayback Machine Twitter snapshot data.
//!
//! Packs digest-named tweet files into compact zstd NDJSON, enhances compact files with CDX
//! metadata from a capture metadata database, validates compact files against wxj schemas,
//! extracts the lines of a compact file whose tweets mention a given user, and prints per-tweet
//! and per-user CSV reports over compact files. The
//! Twitter-specific pieces (the default closing whitespace and the CEL query that infers a
//! tweet's canonical URL) live in the bundled `twitter.toml` context configuration; the
//! operations themselves come from `archivindex-wbm-json`.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
use archivindex_wbm_json::context::Context;
use cli_helpers::prelude::*;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

mod extract;
mod report;
mod validate;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::Pack {
            data,
            invalid_db,
            output,
            level,
        } => {
            // Tweet snapshots are plain UTF-8 text; there is no non-default format to detect.
            let summary = archivindex_wbm_json::process::pack::pack(
                &data,
                invalid_db.as_deref(),
                &output,
                level,
                &twitter_context(),
                |_bytes| None,
            )?;

            log::info!(
                "Packed: {} written ({} with an expected digest), {} skipped",
                summary.written_count,
                summary.expected_digest_count,
                summary.skipped_count
            );

            println!("{}", serde_json::json!(summary));
        }
        Command::Enhance {
            input,
            metadata_db,
            invalid_db,
            output,
            level,
            batch_size,
        } => {
            let metadata = archivindex_wbm_cdx_index::metadata::MetadataDb::open(&metadata_db)?;
            let summary = archivindex_wbm_json::process::enhance::enhance(
                &input,
                &invalid_db,
                &output,
                level,
                batch_size,
                &twitter_context(),
                |digests| metadata.multi_get(digests),
            )?;

            log::info!(
                "Enhanced: {} read, {} enhanced, {} already enhanced, {} unmatched",
                summary.read_count,
                summary.enhanced_count,
                summary.already_enhanced_count,
                summary.unmatched_count
            );

            println!("{}", serde_json::json!(summary));
        }
        Command::Validate { input, flat } => {
            let summary = validate::validate(&input, &twitter_context(), flat)?;

            log::info!(
                "Validated {} lines: {} valid, {} missing metadata, {} invalid metadata, \
                 {} schema errors, {} digest errors, {} order errors",
                summary.line_count,
                summary.valid_count,
                summary.missing_metadata_count,
                summary.invalid_metadata_count,
                summary.schema_errors.len(),
                summary.digest_errors.len(),
                summary.order_errors.len()
            );

            if !summary.is_successful() {
                log::warn!("Validation found problems (see the summary for details)");
            }

            println!("{}", serde_json::json!(summary));
        }
        Command::Extract {
            input,
            user_id,
            output,
            level,
        } => {
            let summary = extract::extract(&input, user_id, &output, level)?;

            log::info!(
                "Extracted: {} of {} lines mentioning user {user_id}",
                summary.matched_count,
                summary.read_count
            );

            println!("{}", serde_json::json!(summary));
        }
        Command::TweetIds { input, flat } => {
            run_report("tweet IDs", |writer| {
                report::tweet_ids(&input, flat, writer)
            })?;
        }
        Command::UserObservations {
            input,
            flat,
            range_only,
        } => {
            run_report("user observation rows", |writer| {
                report::user_observations(&input, flat, range_only, writer)
            })?;
        }
    }

    Ok(())
}

/// Run a report command, writing its CSV rows to standard output and logging its counts (with the
/// rows described by `row_description`).
fn run_report<F>(row_description: &str, run: F) -> Result<(), Error>
where
    F: FnOnce(
        &mut BufWriter<std::io::StdoutLock<'static>>,
    ) -> Result<report::Summary, report::Error>,
{
    let mut writer = BufWriter::new(std::io::stdout().lock());
    let summary = run(&mut writer)?;
    writer.flush()?;

    log::info!(
        "Printed {} {row_description} from {} lines",
        summary.written_count,
        summary.read_count
    );

    Ok(())
}

/// Builds the Twitter snapshot context from the bundled configuration.
///
/// # Panics
///
/// Panics if the bundled configuration is invalid (a bug).
fn twitter_context() -> Context {
    let config: archivindex_wbm_json::context::ContextConfig =
        toml::from_str(include_str!("twitter.toml")).expect("valid Twitter context configuration");
    Context::from_config(config).expect("valid Twitter context URL query")
}

/// Top-level application error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CLI argument reading error")]
    Args(#[from] cli_helpers::Error),
    #[error("pack error")]
    Pack(#[from] archivindex_wbm_json::process::pack::Error),
    #[error("enhance error")]
    Enhance(
        #[from]
        archivindex_wbm_json::process::enhance::Error<archivindex_wbm_cdx_index::metadata::Error>,
    ),
    #[error("validation error")]
    Validate(#[from] validate::Error),
    #[error("extract error")]
    Extract(#[from] extract::Error),
    #[error("report error")]
    Report(#[from] report::Error),
    #[error("capture metadata database error")]
    Metadata(#[from] archivindex_wbm_cdx_index::metadata::Error),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Parser)]
#[clap(name = "birdsite-cli", version, author)]
struct Opts {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    /// Pack a data directory of digest-named tweet files into a compact zstd NDJSON file, without
    /// CDX metadata (only the digest, the expected digest from the invalid-digest log, and the
    /// content).
    Pack {
        /// Directories of raw data files, each named by the SHA-1 digest of its bytes.
        #[clap(long)]
        data: Vec<PathBuf>,
        #[allow(clippy::doc_markdown)]
        /// Path to the SQLite database of known-invalid digests (omit to attach no expected
        /// digests).
        #[clap(long)]
        invalid_db: Option<PathBuf>,
        /// Output path for the packed zstd NDJSON file (must not already exist).
        #[clap(long)]
        output: PathBuf,
        /// Zstandard compression level.
        #[clap(long, default_value = "14")]
        level: u16,
    },
    /// Enhance a compact snapshot file with CDX metadata (timestamp, and a URL when the content
    /// does not infer it) from a capture metadata database, retrying under the expected digest
    /// from the invalid-digest log where the content digest has no captures.
    Enhance {
        /// Path to a zstd-compressed compact snapshot file.
        #[clap(long)]
        input: PathBuf,
        #[allow(clippy::doc_markdown)]
        /// Path to the capture metadata RocksDB database.
        #[clap(long)]
        metadata_db: PathBuf,
        #[allow(clippy::doc_markdown)]
        /// Path to the SQLite database of known-invalid digests.
        #[clap(long)]
        invalid_db: PathBuf,
        /// Output path for the enhanced zstd NDJSON file (must not already exist).
        #[clap(long)]
        output: PathBuf,
        /// Zstandard compression level.
        #[clap(long, default_value = "14")]
        level: u16,
        /// Number of snapshots buffered per capture lookup batch.
        #[clap(long, default_value = "1024")]
        batch_size: std::num::NonZeroUsize,
    },
    /// Validate a compact snapshot file against wxj schemas.
    Validate {
        /// Path to a zstd-compressed compact snapshot file.
        #[clap(long)]
        input: PathBuf,
        /// Validate against wxj/flat schema instead of wxj/data.
        #[clap(long)]
        flat: bool,
    },
    /// Extract the lines of a compact snapshot file whose content's includes.users array contains
    /// a user with the given ID, copying them verbatim to a new file.
    Extract {
        /// Path to a zstd-compressed compact snapshot file of wxj/data tweet content.
        #[clap(long)]
        input: PathBuf,
        /// Twitter user ID to select.
        #[clap(long)]
        user_id: u64,
        /// Output path for the matching lines (zstd-compressed NDJSON, must not already exist).
        #[clap(long)]
        output: PathBuf,
        /// Zstandard compression level.
        #[clap(long, default_value = "14")]
        level: u16,
    },
    /// Print a `user_id,tweet_id` CSV row to standard output for every tweet carried in a compact
    /// snapshot file's content (including any retweeted, replied-to, or quoted tweets a snapshot
    /// carries).
    TweetIds {
        /// Path to a zstd-compressed compact snapshot file.
        #[clap(long)]
        input: PathBuf,
        /// Parse content with the wxj/flat schema instead of wxj/data.
        #[clap(long)]
        flat: bool,
    },
    /// Print a CSV row to standard output for every user observed in a compact snapshot file: the
    /// user's ID and screen name followed by the deduplicated capture timestamps (Unix epoch
    /// seconds) of the snapshots whose content carries the user.
    UserObservations {
        /// Path to a zstd-compressed compact snapshot file.
        #[clap(long)]
        input: PathBuf,
        /// Parse content with the wxj/flat schema instead of wxj/data.
        #[clap(long)]
        flat: bool,
        /// Print only the first and last observation timestamps.
        #[clap(long)]
        range_only: bool,
    },
}
