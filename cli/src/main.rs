use birdsite::model::wbm::data::FormatError;
use birdsite_db::metadata::{TweetMetadata, UserMetadata};
use chrono::{DateTime, Utc};
use cli_helpers::prelude::*;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::KnownTweets { db } => {
            let db = birdsite_db::metadata::Database::open(db)?;

            for result in db.known_tweets() {
                let (id, user_id, timestamp) = result?;
                let timestamp_code = if timestamp.is_stored() { "+" } else { "" };

                println!(
                    "{},{},{},{}",
                    id,
                    user_id,
                    timestamp_code,
                    DateTime::<Utc>::from(timestamp)
                );
            }
        }
        Command::KnownUsers { db } => {
            let db = birdsite_db::metadata::Database::open(db)?;

            for result in db.known_users() {
                let (id, timestamp) = result?;
                let timestamp_code = if timestamp.is_stored() { "+" } else { "" };

                println!(
                    "{},{},{}",
                    id,
                    timestamp_code,
                    DateTime::<Utc>::from(timestamp)
                );
            }
        }
        Command::UnknownTweets { db } => {
            let db = birdsite_db::metadata::Database::open(db)?;

            let known_ids = db
                .known_tweets()
                .map(|result| result.map(|(id, _, _)| id))
                .collect::<Result<BTreeSet<_>, _>>()?;

            let unknown_ids = db
                .seen_tweets()
                .filter(|result| {
                    result
                        .as_ref()
                        .map(|id| !known_ids.contains(&id))
                        .unwrap_or(true)
                })
                .collect::<Result<BTreeSet<_>, _>>()?;

            for id in unknown_ids {
                println!("{}", id);
            }
        }
        Command::UnknownUsers { db } => {
            let db = birdsite_db::metadata::Database::open(db)?;

            let known_ids = db
                .known_users()
                .map(|result| result.map(|(id, _)| id))
                .collect::<Result<BTreeSet<_>, _>>()?;

            let unknown_ids = db
                .seen_users()
                .filter(|result| {
                    result
                        .as_ref()
                        .map(|id| !known_ids.contains(&id))
                        .unwrap_or(true)
                })
                .collect::<Result<BTreeSet<_>, _>>()?;

            for id in unknown_ids {
                println!("{}", id);
            }
        }
        Command::Extract { input } => {
            let mut paths = std::fs::read_dir(input)?
                .map(|entry| entry.map(|entry| entry.path()))
                .collect::<Result<Vec<_>, _>>()?;

            paths.sort();

            for path in paths {
                if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
                    match std::io::read_to_string(std::fs::File::open(&path)?).map_err(Error::from)
                    {
                        Ok(contents) => {
                            match serde_json::from_str::<birdsite::model::wbm::TweetSnapshot>(
                                &contents,
                            ) {
                                Ok(snapshot) => {
                                    for user in snapshot.user_info() {
                                        println!("{},{}", user.id, user.screen_name);
                                    }
                                }
                                Err(error) => {
                                    log::error!("{}: {:?}", file_name, error);
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("{}: {:?}", file_name, error);
                        }
                    }
                }
            }
        }
        Command::Import { input, db } => {
            let db = birdsite_db::metadata::Database::open(db)?;

            let mut paths = std::fs::read_dir(input)?
                .map(|entry| entry.map(|entry| entry.path()))
                .collect::<Result<Vec<_>, _>>()?;

            paths.sort();

            for path in paths {
                if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
                    match std::io::read_to_string(std::fs::File::open(&path)?).map_err(Error::from)
                    {
                        Ok(contents) => {
                            match serde_json::from_str::<birdsite::model::wbm::TweetSnapshot>(
                                &contents,
                            ) {
                                Ok(birdsite::model::wbm::TweetSnapshot::Data(tweet)) => {
                                    match parse_snapshot(&tweet) {
                                        Ok(snapshot) => {
                                            db.insert_snapshot(&snapshot)?;
                                        }
                                        Err(error) => {
                                            log::error!("{}: {:?}", file_name, error);
                                        }
                                    }
                                }
                                Ok(_) => {}
                                Err(error) => {
                                    log::error!("{}: {:?}", file_name, error);
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("{}: {:?}", file_name, error);
                        }
                    }
                }
            }
        }
        Command::LookupUser { db, id } => {
            let db = birdsite_db::metadata::Database::open(db)?;
            log::info!("Loaded");

            let created_at = db.lookup_user(id)?;

            log::info!("Found user");

            let tweet_ids = db
                .lookup_tweets_by_user(id)
                .collect::<Result<Vec<_>, _>>()?;

            log::info!("Found {} tweets", tweet_ids.len());

            let mut retweet_observations = vec![];

            for tweet_id in &tweet_ids {
                if let Some(retweeted_id) = db.lookup_retweet_target(*tweet_id)? {
                    if let Some((user_id, _)) = db.lookup_tweet(retweeted_id)? {
                        retweet_observations.push(user_id);
                    }
                }
            }

            retweet_observations.sort();

            let mut retweets = retweet_observations
                .into_iter()
                .chunk_by(|id| *id)
                .into_iter()
                .map(|(id, seen)| (id, seen.count()))
                .collect::<Vec<_>>();
            retweets.sort_by_key(|(_, len)| std::cmp::Reverse(*len));

            let mut mention_observations = vec![];

            for tweet_id in &tweet_ids {
                for result in db.lookup_mention_targets(*tweet_id) {
                    let user_id = result?;

                    mention_observations.push(user_id);
                }
            }

            mention_observations.sort();

            let mut mentions = mention_observations
                .into_iter()
                .chunk_by(|id| *id)
                .into_iter()
                .map(|(id, seen)| (id, seen.count()))
                .collect::<Vec<_>>();
            mentions.sort_by_key(|(_, len)| std::cmp::Reverse(*len));

            let mut mentioned_observations = vec![];

            for tweet_id in db.lookup_mention_sources(id) {
                if let Some((id, _)) = db.lookup_tweet(tweet_id?)? {
                    mentioned_observations.push(id);
                }
            }

            mentioned_observations.sort();

            let mut mentioned = mentioned_observations
                .into_iter()
                .chunk_by(|id| *id)
                .into_iter()
                .map(|(id, seen)| (id, seen.count()))
                .collect::<Vec<_>>();
            mentioned.sort_by_key(|(_, len)| std::cmp::Reverse(*len));

            let result = UserInfo {
                created_at,
                retweets,
                mentions,
                mentioned,
                tweets: tweet_ids,
            };

            log::info!("Done");

            println!("{}", serde_json::json!(result));
        }
        Command::TweetsDbAdd { db } => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(std::io::stdin());

            let pairs = reader.deserialize::<TweetIdLine>();

            let db = birdsite_db::tweets::Database::open(db)?;

            for result in pairs {
                let TweetIdLine { user_id, tweet_id } = result?;

                db.insert(user_id, tweet_id)?;
            }
        }
        Command::TweetsDbLookup { db, last } => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(std::io::stdin());

            let user_ids = reader.deserialize::<u64>();

            let db = birdsite_db::tweets::Database::open(db)?;

            for result in user_ids {
                let user_id = result?;

                if last {
                    if let Some(tweet_id) = db
                        .lookup_live(user_id)
                        .max_by_key(|result| result.as_ref().copied().unwrap_or(u64::MAX))
                    {
                        println!("{},{}", user_id, tweet_id?);
                    }
                } else {
                    for tweet_id in db.lookup_live(user_id) {
                        println!("{},{}", user_id, tweet_id?);
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI argument reading error")]
    Args(#[from] cli_helpers::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("Metadata database error")]
    MetadataDb(#[from] birdsite_db::metadata::Error),
    #[error("Tweets database error")]
    TweetsDb(#[from] birdsite_db::tweets::Error),
    #[error("Wayback Machine snapshot format error")]
    WbmDataFormat(#[from] birdsite::model::wbm::data::FormatError),
}

#[derive(Debug, Parser)]
#[clap(name = "birdsite", version, author)]
struct Opts {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    KnownTweets {
        #[clap(long)]
        db: PathBuf,
    },
    KnownUsers {
        #[clap(long)]
        db: PathBuf,
    },
    UnknownTweets {
        #[clap(long)]
        db: PathBuf,
    },
    UnknownUsers {
        #[clap(long)]
        db: PathBuf,
    },
    Extract {
        #[clap(long)]
        input: PathBuf,
    },
    Import {
        #[clap(long)]
        input: PathBuf,
        #[clap(long)]
        db: PathBuf,
    },
    LookupUser {
        #[clap(long)]
        db: PathBuf,
        #[clap(long)]
        id: u64,
    },
    TweetsDbAdd {
        #[clap(long)]
        db: PathBuf,
    },
    TweetsDbLookup {
        #[clap(long)]
        db: PathBuf,
        #[clap(long)]
        last: bool,
    },
}

#[derive(serde::Serialize)]
struct UserInfo {
    #[serde(with = "chrono::serde::ts_seconds_option")]
    created_at: Option<DateTime<Utc>>,
    retweets: Vec<(u64, usize)>,
    mentions: Vec<(u64, usize)>,
    mentioned: Vec<(u64, usize)>,
    tweets: Vec<u64>,
}

fn parse_tweet_data(
    snapshot: &birdsite::model::wbm::data::Tweet,
    data: &birdsite::model::wbm::data::TweetData,
) -> Result<TweetMetadata, FormatError> {
    let user = UserMetadata::new(
        data.author_id,
        snapshot
            .lookup_user(data.author_id)
            .map(|user| user.created_at),
    );

    let mentions = data
        .mention_ids()
        .into_iter()
        .map(|user_id| {
            UserMetadata::new(
                user_id,
                snapshot.lookup_user(user_id).map(|user| user.created_at),
            )
        })
        .collect::<Vec<_>>();

    Ok(TweetMetadata::new(
        data.id,
        user,
        data.created_at,
        data.retweeted_id()?,
        data.replied_to_id()?,
        data.quoted_id()?,
        mentions,
    ))
}

fn parse_snapshot(
    snapshot: &birdsite::model::wbm::data::Tweet,
) -> Result<Vec<TweetMetadata>, FormatError> {
    let mut tweets = vec![parse_tweet_data(snapshot, &snapshot.data)?];

    if let Some(tweet) = snapshot
        .data
        .retweeted_id()?
        .and_then(|id| snapshot.lookup_tweet(id))
    {
        tweets.extend(parse_tweet_data(snapshot, &tweet));
    }

    if let Some(tweet) = snapshot
        .data
        .replied_to_id()?
        .and_then(|id| snapshot.lookup_tweet(id))
    {
        tweets.extend(parse_tweet_data(snapshot, &tweet));
    }

    if let Some(tweet) = snapshot
        .data
        .quoted_id()?
        .and_then(|id| snapshot.lookup_tweet(id))
    {
        tweets.extend(parse_tweet_data(snapshot, &tweet));
    }

    Ok(tweets)
}

#[derive(serde::Deserialize)]
struct TweetIdLine {
    user_id: u64,
    tweet_id: u64,
}
