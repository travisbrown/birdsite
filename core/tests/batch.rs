use birdsite::model::wbm::{data, v1, TweetSnapshot};
use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

#[test]
fn test_batch() -> Result<(), Box<dyn std::error::Error>> {
    //let source_dir = "/mnt/fast/tmp/wbm/unfiltdtruth-data/data/";
    //let source_dir = "/mnt/fast/tmp/wbm/OthmanOnX-data/data/";
    //let source_dir = "/mnt/fast/tmp/wbm/Gavin_Kliger-data/data/";
    //let source_dir = "/home/travis/projects/cancel-culture/tmp/store/data/";
    //let source_dir = "/home/travis/projects/archive/wbm-01/data/";
    let source_dir = "/mnt/sde1/data/twitter/wbm/data/data/";
    //let skip = Some("ZC");
    //let skip = Some("A2");
    let skip: Option<&str> = None;

    let mut paths = std::fs::read_dir(source_dir)?
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
                //println!("{:?}", path);
                if let Ok(contents) = read_contents(&path) {
                    if !contents.trim().starts_with("<!DOCTYPE html") {
                        match parse(&contents) {
                            Ok(_) => {}
                            Err(error) => {
                                println!("{:?}", path);
                                Err(error)?;
                            }
                        }

                        count += 1;

                        //println!("{}", tweet.id());
                    } else {
                        //println!("Skip HTML");
                    }
                } else {
                    //println!("Skip non-text");
                }
            }
        }
    }

    println!("{} done", count);

    Ok(())
}

fn read_contents<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    if path.as_ref().extension() == Some(OsStr::new("gz")) {
        std::io::read_to_string(GzDecoder::new(File::open(path)?))
    } else {
        std::io::read_to_string(File::open(path)?)
    }
}

fn parse(content: &str) -> Result<TweetSnapshot, serde_json::Error> {
    serde_json::from_str::<TweetSnapshot>(content)
        .or_else(|_| serde_json::from_str::<v1::Tweet>(content).map(TweetSnapshot::V1))
        .or_else(|error| {
            if error.to_string().starts_with("unknown field `data`") {
                serde_json::from_str::<data::Tweet>(content).map(TweetSnapshot::Data)
            } else {
                Err(error)
            }
        })
}
