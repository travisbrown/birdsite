#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
use cli_helpers::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod db;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
        },
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
}

#[derive(serde::Deserialize)]
struct IdPair {
    user_id: u64,
    tweet_id: u64,
}

#[derive(serde::Deserialize)]
struct WxjWrapper<S> {
    content: S,
}
