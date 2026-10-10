//! Extract selected tweet results from archived `TweetResultsByRestIds` GraphQL exchanges.

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use birdsite_graphql::archive::Exchange;
use birdsite_graphql::archive::parse::parse_exchange;
use birdsite_graphql::request::filter::include_filter;
use birdsite_graphql::request::name::RequestName;
use cli_helpers::prelude::log;
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Invalid status ID on stdin line {line_number}")]
    StatusId {
        #[source]
        error: std::num::ParseIntError,
        line_number: usize,
    },
    #[error("GraphQL archive parsing error")]
    Graphql(#[from] birdsite_graphql::archive::parse::Error),
    #[error("Result JSON writing error")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug)]
struct Variables {
    tweet_ids: Vec<u64>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TweetResultsVariables {
    tweet_ids: Vec<u64>,
}

impl<'a> birdsite_graphql::archive::request::Variables<'a> for Variables {
    fn parse_with_name<'de: 'a, A: serde::de::MapAccess<'de>>(
        name: RequestName,
        map: &mut A,
    ) -> Result<Self, A::Error> {
        if name == RequestName::TweetResultsByRestIds {
            let variables = map.next_value::<TweetResultsVariables>()?;

            Ok(Self {
                tweet_ids: variables.tweet_ids,
            })
        } else {
            map.next_value::<serde::de::IgnoredAny>()?;

            Ok(Self {
                tweet_ids: Vec::new(),
            })
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct TweetResults {
    #[serde(rename = "tweetResult")]
    results: Vec<ResultElement>,
}

#[derive(Debug, serde::Deserialize)]
struct ResultElement {
    result: Option<Value>,
}

impl<'a> birdsite_graphql::archive::response::ParseWithVariables<'a, Variables> for TweetResults {
    fn parse(
        input: &'a str,
        variables: &Variables,
    ) -> Result<Self, birdsite_graphql::archive::response::Error> {
        let results: Self = serde_json::from_str(input)?;
        let expected = variables.tweet_ids.len();
        let returned = results.results.len();

        if returned == expected {
            Ok(results)
        } else {
            Err(
                birdsite_graphql::archive::response::Error::InvalidResultLength {
                    expected,
                    returned,
                },
            )
        }
    }
}

/// Returns the ID for an actual tweet result, excluding unavailable, null, and tombstone results.
fn result_tweet_id(result: &Value) -> Option<u64> {
    let object = result.as_object()?;

    match object.get("__typename")?.as_str()? {
        "Tweet" => object.get("rest_id")?.as_str()?.parse().ok(),
        "TweetWithVisibilityResults" => object.get("tweet")?.get("rest_id")?.as_str()?.parse().ok(),
        _ => None,
    }
}

fn read_status_ids<R: BufRead>(reader: R) -> Result<BTreeSet<u64>, Error> {
    reader
        .lines()
        .enumerate()
        .map(|(index, result)| {
            let line = result?;

            line.trim().parse().map_err(|error| Error::StatusId {
                error,
                line_number: index + 1,
            })
        })
        .collect()
}

fn extract<R: BufRead, W: Write>(
    reader: R,
    output: &mut W,
    requested_ids: &mut BTreeSet<u64>,
) -> Result<(), Error> {
    let filter = include_filter([RequestName::TweetResultsByRestIds]);

    for (index, result) in reader.lines().enumerate() {
        if index % 1_000_000 == 0 {
            log::info!("Done: {index}");
        }

        let line = result?;
        let parsed: Result<Result<Exchange<'_, Variables, TweetResults>, RequestName>, _> =
            parse_exchange(&line, index + 1, &filter);

        let Ok(exchange) = parsed? else {
            continue;
        };
        let Some(data) = exchange.data else {
            continue;
        };
        let timestamp_ms = exchange.request.timestamp.timestamp_millis().to_string();

        for (requested_id, element) in exchange
            .request
            .variables
            .tweet_ids
            .into_iter()
            .zip(data.results)
        {
            let Some(mut result) = element.result else {
                continue;
            };

            if result_tweet_id(&result) == Some(requested_id) && requested_ids.remove(&requested_id)
            {
                result
                    .as_object_mut()
                    .expect("tweet results are JSON objects")
                    .insert(
                        "timestamp_ms".to_owned(),
                        Value::String(timestamp_ms.clone()),
                    );
                serde_json::to_writer(&mut *output, &result)?;
                writeln!(output)?;
            }
        }
    }

    Ok(())
}

fn input_paths(input: &Path) -> Result<Vec<PathBuf>, Error> {
    if !input.is_dir() {
        return Ok(vec![input.to_path_buf()]);
    }

    let mut paths = Vec::new();
    for result in std::fs::read_dir(input)? {
        let entry = result?;
        if entry.file_type()?.is_file() {
            paths.push(entry.path());
        }
    }

    paths.sort_by(|left, right| right.file_name().cmp(&left.file_name()));

    Ok(paths)
}

pub fn run(input: &Path, remaining: &Path) -> Result<(), Error> {
    let stdin = std::io::stdin();
    let mut requested_ids = read_status_ids(stdin.lock())?;

    let stdout = std::io::stdout();
    let mut output = BufWriter::new(stdout.lock());

    for path in input_paths(input)? {
        log::info!("Processing {}", path.display());

        let input = BufReader::new(File::open(path)?);
        let decoder = BufReader::new(zstd::Decoder::with_buffer(input)?);

        extract(decoder, &mut output, &mut requested_ids)?;
    }
    output.flush()?;

    let mut remaining_output = BufWriter::new(File::create(remaining)?);
    for tweet_id in requested_ids {
        writeln!(remaining_output, "{tweet_id}")?;
    }
    remaining_output.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs::File;
    use std::io::Cursor;

    use super::{extract, input_paths, read_status_ids};

    const TWEET: &str = r#"{"data":{"tweetResult":[{"result":{"__typename":"Tweet","rest_id":"42","text":"found"}},{"result":{"__typename":"TweetUnavailable","reason":"Deleted"}},{"result":{"__typename":"TweetWithVisibilityResults","tweet":{"rest_id":"8","text":"limited"}}},{},{"result":null}]},"request":{"name":"TweetResultsByRestIds","timestamp_ms":"1782885597308","variables":{"tweetIds":[42,7,8,9,10]}}}"#;

    #[test]
    fn status_ids_are_deduplicated_and_sorted() {
        let ids = read_status_ids(Cursor::new("9\n2\n9\n")).unwrap();

        assert_eq!(ids.into_iter().collect::<Vec<_>>(), vec![2, 9]);
    }

    #[test]
    fn extracts_each_available_tweet_once_and_keeps_unavailable_ids() {
        let input = format!("{TWEET}\n{TWEET}\n");
        let mut output = Vec::new();
        let mut requested_ids = BTreeSet::from([7, 8, 9, 10, 42, 99]);

        extract(Cursor::new(input), &mut output, &mut requested_ids).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                "{\"__typename\":\"Tweet\",\"rest_id\":\"42\",\"text\":\"found\",\"timestamp_ms\":\"1782885597308\"}\n",
                "{\"__typename\":\"TweetWithVisibilityResults\",\"tweet\":{\"rest_id\":\"8\",\"text\":\"limited\"},\"timestamp_ms\":\"1782885597308\"}\n"
            )
        );
        assert_eq!(requested_ids, BTreeSet::from([7, 9, 10, 99]));
    }

    #[test]
    fn directory_inputs_are_regular_files_in_reverse_name_order() {
        let directory = tempfile::tempdir().unwrap();
        File::create(directory.path().join("a.ndjson.zst")).unwrap();
        File::create(directory.path().join("c.ndjson.zst")).unwrap();
        File::create(directory.path().join("b.ndjson.zst")).unwrap();
        std::fs::create_dir(directory.path().join("z-directory")).unwrap();

        let names = input_paths(directory.path())
            .unwrap()
            .into_iter()
            .map(|path| path.file_name().unwrap().to_owned())
            .collect::<Vec<_>>();

        assert_eq!(names, ["c.ndjson.zst", "b.ndjson.zst", "a.ndjson.zst"]);
    }
}
