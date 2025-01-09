use birdsite::model::wbm::{TweetSnapshot, data, v1};
use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

#[test]
fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(dir) = std::env::var("TEST_JSON_DIR") {
        deserialize_dir(dir)
    } else {
        Ok(())
    }
}

fn deserialize_dir<P: AsRef<Path>>(dir: P) -> Result<(), Box<dyn std::error::Error>> {
    let skip: Option<&str> = None;

    let mut paths = std::fs::read_dir(dir)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;

    paths.sort();

    let mut count = 0;

    for path in paths {
        if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
            if skip
                .map(|skip| &file_name[0..skip.len()] >= skip)
                .unwrap_or(true)
            {
                if let Ok(contents) = read_contents(&path) {
                    // In some cases we might have HTML content mixed in with the JSON files.
                    if !contents.trim().starts_with("<!DOCTYPE html") {
                        match deserialize_str(&contents) {
                            Ok(_) => {}
                            Err(error) => {
                                eprintln!("Error at {:?}: {:?}", path, error);
                            }
                        }

                        count += 1;
                    }
                }
            }
        }
    }

    eprintln!("Files deserialized: {}", count);

    Ok(())
}

fn read_contents<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    if path.as_ref().extension() == Some(OsStr::new("gz")) {
        std::io::read_to_string(GzDecoder::new(File::open(path)?))
    } else {
        std::io::read_to_string(File::open(path)?)
    }
}

fn deserialize_str(content: &str) -> Result<TweetSnapshot, serde_json::Error> {
    serde_json::from_str::<TweetSnapshot>(content)
        .or_else(|_| serde_json::from_str::<v1::Tweet>(content).map(TweetSnapshot::V1))
        .or_else(|error| {
            // This is a hack that allows us to show better error messages.
            if error.to_string().starts_with("unknown field `data`") {
                serde_json::from_str::<data::Tweet>(content).map(TweetSnapshot::Data)
            } else {
                Err(error)
            }
        })
}
