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
}
