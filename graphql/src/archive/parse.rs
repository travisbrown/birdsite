use super::Exchange;
use crate::request::{filter::RequestFilter, name::RequestName};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request JSON decoding")]
    RequestJson {
        error: serde_json::Error,
        line_number: usize,
    },
    #[error("Errors JSON decoding")]
    ErrorsJson {
        error: serde_json::Error,
        line_number: usize,
    },
    #[error("Data JSON decoding")]
    DataJson {
        error: serde_json::Error,
        data_start: usize,
        request_name: RequestName,
        request_timestamp: i64,
    },
    #[error("Invalid request field")]
    InvalidRequest,
    #[error("Invalid errors field")]
    InvalidErrors,
    #[error("Result length does not match request")]
    InvalidResultLength { expected: usize, returned: usize },
}

pub fn parse_exchange<
    'a: 'de,
    'de,
    V: super::request::Variables<'a> + 'a,
    R: super::response::ParseWithVariables<'a, V> + 'a,
    F: RequestFilter,
>(
    input: &'a str,
    line_number: usize,
    filter: &F,
) -> Result<Result<Exchange<'a, V, R>, RequestName>, Error> {
    let input_bytes = input.as_bytes();
    let request_start = find_request_open_brace(input_bytes).ok_or(Error::InvalidRequest)?;
    let request: super::request::Request<'_, V> =
        serde_json::from_str(&input[request_start..input.len() - 1])
            .map_err(|error| Error::RequestJson { error, line_number })?;

    if filter.include(request.name) {
        let (data_start, errors) = if input_bytes[2] == b'e' {
            let errors_end =
                find_errors_closing_bracket(input_bytes).ok_or(Error::InvalidErrors)?;
            let errors =
                serde_json::from_str::<Vec<crate::response::error::Error>>(&input[10..=errors_end])
                    .map_err(|error| Error::ErrorsJson { error, line_number })?;

            (errors_end + 9, errors)
        } else {
            (8, vec![])
        };

        let data = if data_start < request_start - 11 {
            Some(
                super::response::ParseWithVariables::parse(
                    &input[data_start..request_start - 11],
                    &request.variables,
                )
                .map_err(|error| match error {
                    super::response::Error::InvalidResultLength { expected, returned } => {
                        Error::InvalidResultLength { expected, returned }
                    }
                    super::response::Error::Json(error) => Error::DataJson {
                        error,
                        data_start,
                        request_name: request.name,
                        request_timestamp: request.timestamp.timestamp_millis(),
                    },
                })?,
            )
        } else {
            // The case where there is no `data` field at all.
            None
        };

        Ok(Ok(Exchange {
            request,
            data,
            errors,
        }))
    } else {
        Ok(Err(request.name))
    }
}

/// Encodes some assumptions.
fn find_request_open_brace(input_bytes: &[u8]) -> Option<usize> {
    let mut current = input_bytes.len() - 1;
    let mut brace_depth = 1;
    let mut found = false;

    current -= 2;

    while current > 0 {
        match input_bytes[current] {
            b'{' => {
                if brace_depth == 1 {
                    found = true;
                    break;
                }
                brace_depth -= 1;
            }
            b'}' => {
                brace_depth += 1;
            }
            _ => {}
        }

        current -= 1;
    }

    if found { Some(current) } else { None }
}

fn find_errors_closing_bracket(input_bytes: &[u8]) -> Option<usize> {
    let mut current = 13;
    let mut bracket_depth = 1;
    let mut found = false;

    while current < input_bytes.len() {
        match input_bytes[current] {
            b']' => {
                if bracket_depth == 1 {
                    found = true;
                    break;
                }
                bracket_depth -= 1;
            }
            b'[' => {
                bracket_depth += 1;
            }
            _ => {}
        }

        current += 1;
    }

    if found { Some(current) } else { None }
}
