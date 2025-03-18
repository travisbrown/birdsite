use birdsite::model::wbm::data::FormatError;
use birdsite_db::{Snapshot, TweetMetadata, UserMetadata};
use cli_helpers::prelude::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
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
            let db = birdsite_db::Database::open(db)?;

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
                                    db.insert_snapshot(parse_snapshot(&tweet)?)?;
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
    #[error("Database error")]
    Db(#[from] birdsite_db::Error),
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
}

fn parse_snapshot(snapshot: &birdsite::model::wbm::data::Tweet) -> Result<Snapshot, FormatError> {
    let user = snapshot.user()?;
    let tweet = TweetMetadata::new(
        snapshot.data.id,
        UserMetadata::new(user.id, user.created_at),
        snapshot.data.created_at,
    );

    let retweeted = snapshot
        .retweeted()?
        .map(|tweet| {
            snapshot
                .lookup_user(tweet.author_id)
                .ok_or(FormatError::MissingUser(tweet.author_id))
                .map(|user| {
                    TweetMetadata::new(
                        tweet.id,
                        UserMetadata::new(user.id, user.created_at),
                        tweet.created_at,
                    )
                })
        })
        .map_or(Ok(None), |v| v.map(Some))?;

    let replied_to = snapshot
        .replied_to()?
        .map(|tweet| {
            snapshot
                .lookup_user(tweet.author_id)
                .ok_or(FormatError::MissingUser(tweet.author_id))
                .map(|user| {
                    TweetMetadata::new(
                        tweet.id,
                        UserMetadata::new(user.id, user.created_at),
                        tweet.created_at,
                    )
                })
        })
        .map_or(Ok(None), |v| v.map(Some))?;

    let quoted = snapshot
        .quoted()?
        .map(|tweet| {
            snapshot
                .lookup_user(tweet.author_id)
                .ok_or(FormatError::MissingUser(tweet.author_id))
                .map(|user| {
                    TweetMetadata::new(
                        tweet.id,
                        UserMetadata::new(user.id, user.created_at),
                        tweet.created_at,
                    )
                })
        })
        .map_or(Ok(None), |v| v.map(Some))?;

    let mentions = todo![];

    Ok(Snapshot::new(
        tweet, retweeted, replied_to, quoted, mentions,
    ))
}
