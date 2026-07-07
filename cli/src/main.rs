//! CLI for processing Wayback Machine Twitter snapshot data.
//!
//! Packs digest-named tweet files into compact zstd NDJSON, enhances compact files with CDX
//! metadata from a CDX index database, and validates compact files against wxj schemas. The
//! Twitter-specific pieces (the default closing whitespace and the CEL query that infers a
//! tweet's canonical URL) live in the bundled `twitter.toml` context configuration; the
//! operations themselves come from `archivindex-wbm-json`.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
use archivindex_wbm_json::context::Context;
use cli_helpers::prelude::*;
use std::path::PathBuf;

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
            let context = twitter_context();
            // Tweet snapshots are plain UTF-8 text; there is no non-default format to detect.
            let summary = archivindex_wbm_json::process::pack::pack(
                &data,
                &invalid_db,
                &output,
                level,
                &context,
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
            cdx_index,
            output,
            level,
        } => {
            let context = twitter_context();
            let index = archivindex_wbm_cdx_index::CdxIndex::open(&cdx_index)?;
            let summary = archivindex_wbm_json::process::enhance::enhance(
                &input,
                &output,
                level,
                &context,
                |digest| index.captures_by_digest(digest),
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
            let summary = validate::validate(&input, flat)?;

            log::info!(
                "Validated {} lines: {} valid, {} schema errors",
                summary.line_count,
                summary.valid_count,
                summary.schema_errors.len()
            );

            if !summary.is_successful() {
                log::warn!("Validation found problems (see the summary for details)");
            }

            println!("{}", serde_json::json!(summary));
        }
    }

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
        #[from] archivindex_wbm_json::process::enhance::Error<archivindex_wbm_cdx_index::Error>,
    ),
    #[error("validation error")]
    Validate(#[from] validate::Error),
    #[error("CDX index error")]
    CdxIndex(#[from] archivindex_wbm_cdx_index::Error),
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
        /// Path to the SQLite database of known-invalid digests.
        #[clap(long)]
        invalid_db: PathBuf,
        /// Output path for the packed zstd NDJSON file (must not already exist).
        #[clap(long)]
        output: PathBuf,
        /// Zstandard compression level.
        #[clap(long, default_value = "14")]
        level: u16,
    },
    /// Enhance a compact snapshot file with CDX metadata (timestamp, and a URL when the content
    /// does not infer it) from a CDX index database.
    Enhance {
        /// Path to a zstd-compressed compact snapshot file.
        #[clap(long)]
        input: PathBuf,
        /// Path to the CDX index database.
        #[clap(long)]
        cdx_index: PathBuf,
        /// Output path for the enhanced zstd NDJSON file (must not already exist).
        #[clap(long)]
        output: PathBuf,
        /// Zstandard compression level.
        #[clap(long, default_value = "14")]
        level: u16,
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
}
