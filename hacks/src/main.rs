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
        Command::GraphQl { input, command } => {
            use birdsite_graphql::request::{name::RequestName, variables::Variables};

            let filter = [
                RequestName::AboutAccountQuery,
                RequestName::BirdwatchFetchOneNote,
                RequestName::BirdwatchFetchPublicData,
                RequestName::MembersSliceTimelineQuery,
                RequestName::TweetResultsByRestIds,
                RequestName::UserByRestId,
            ];

            match command {
                GraphQlCommand::Extract => {
                    use birdsite_graphql::response::data::Data;

                    let exchanges = birdsite_graphql::archive::io::parse_exchanges_zst::<
                        Variables,
                        birdsite_graphql::response::data::Data,
                        _,
                        _,
                    >(input, filter)?;

                    for result in exchanges {
                        match result? {
                            Ok(exchange) => match exchange.data {
                                Some(data) => match data {
                                    Data::AboutAccountQuery(user_result) => {
                                        log::warn!("AboutAccountQuery: {user_result:?}");
                                    }
                                    Data::BirdwatchFetchOneNote(note) => {
                                        log::warn!("BirdwatchFetchOneNote: {note:?}");
                                    }
                                    Data::BirdwatchFetchPublicData(bundle) => {
                                        log::warn!("BirdwatchFetchPublicData: {bundle:?}");
                                    }
                                    Data::MembersSliceTimelineQuery(users) => {
                                        for user in users {
                                            log::warn!("MembersSliceTimelineQuery: {user:?}");
                                        }
                                    }
                                    Data::TweetResultsByRestIds(tweets) => {
                                        for tweet in tweets {
                                            log::warn!("TweetResultsByRestIds: {tweet:?}");
                                        }
                                    }
                                    Data::UserResultByRestId(user_result) => {
                                        log::warn!("UserResultByRestId: {user_result:?}");
                                    }
                                },
                                _ => {}
                            },
                            Err(skipped) => {
                                log::info!("Skipped: {skipped}");
                            }
                        }
                    }
                }
            }
        }
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
            WxjCommand::Extract { input } => {
                let mut paths = std::fs::read_dir(input)?
                    .map(|entry| entry.map(|entry| entry.path()))
                    .collect::<Result<Vec<_>, _>>()?;

                paths.sort();

                log::info!("Loaded {} paths", paths.len());

                for (_i, path) in paths.iter().enumerate() {
                    match std::fs::read_to_string(path) {
                        Ok(contents) => {
                            if let Ok(snapshot) = serde_json::from_str::<
                                birdsite::model::wxj::data::TweetSnapshot<'_>,
                            >(&contents)
                            {
                                //let snapshot = wrapper.content;

                                for tweet in snapshot.includes.tweets.clone().unwrap_or_default() {
                                    /*if tweet.text.to_lowercase().contains("@grok") {*/
                                    if tweet.author_id == 1720665183188922368 {
                                        let user = snapshot.lookup_user(1720665183188922368);

                                        let in_reply_to_user_id = tweet.in_reply_to_user_id;

                                        let in_reply_to_info =
                                            in_reply_to_user_id.and_then(|user_id| {
                                                snapshot.lookup_user(user_id).map(|user| {
                                                    (user.username.to_string(), user.verified)
                                                })
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
                                                .map(|(screen_name, _)| screen_name.clone())
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
    #[error("GraphQL error")]
    GraphQl(#[from] birdsite_graphql::archive::io::Error),
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
    GraphQl {
        #[clap(long)]
        input: PathBuf,
        #[clap(subcommand)]
        command: GraphQlCommand,
    },
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
enum GraphQlCommand {
    Extract,
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

#[derive(serde::Deserialize)]
struct IdPair {
    user_id: u64,
    tweet_id: u64,
}

#[derive(serde::Deserialize)]
struct WxjWrapper<S> {
    content: S,
}
