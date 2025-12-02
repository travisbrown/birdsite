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
    'a,
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

    // The request object does not include the final closing brace.
    let request_json_str = &input[request_start..input.len() - 1];

    let request: super::request::Request<'_, V> = serde_json::from_str(request_json_str)
        .map_err(|error| Error::RequestJson { error, line_number })?;

    if filter.include(request.name) {
        // We drop the initial opening brace, the comma before the `request` object, and `request` itself.
        let errors_and_data_json_str = &input[1..request_start - 11];

        let ((data_start, data_end), errors_json_range) =
            if errors_and_data_json_str.starts_with("\"errors\"") {
                // Generally if there is an `errors` field, it precedes `data`.
                let errors_end = find_errors_closing_bracket(errors_and_data_json_str.as_bytes())
                    .ok_or(Error::InvalidErrors)?;

                (
                    (errors_end + 9, errors_and_data_json_str.len()),
                    Some((9, errors_end + 1)),
                )
            } else if errors_and_data_json_str.chars().next_back() == Some(']') {
                // In some recent (1 December 2025) cases, the `errors` field may appear after the `data` field.
                // The `errors` field will always be an array, and `data` will be an object.
                // We do not currently handle the possibility that `errors` fields appear both before and after `data`.
                let errors_start = find_errors_opening_bracket(errors_and_data_json_str.as_bytes())
                    .ok_or(Error::InvalidErrors)?;

                (
                    (7, errors_start - 10),
                    Some((errors_start, errors_and_data_json_str.len())),
                )
            } else {
                ((7, errors_and_data_json_str.len()), None)
            };

        let errors = errors_json_range
            .map(|(errors_start, errors_end)| {
                let errors_json_str = &errors_and_data_json_str[errors_start..errors_end];

                serde_json::from_str::<Vec<crate::response::error::Error>>(errors_json_str)
                    .map_err(|error| Error::ErrorsJson { error, line_number })
            })
            .map_or(Ok(None), |errors| errors.map(Some))?
            .unwrap_or_default();

        let data = if data_start < data_end {
            let data_json_str = &errors_and_data_json_str[data_start..data_end];

            Some(
                super::response::ParseWithVariables::parse(data_json_str, &request.variables)
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
    let mut current = input_bytes.len() - 3;
    let mut brace_depth = 1;
    let mut found = false;

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

fn find_errors_opening_bracket(input_bytes: &[u8]) -> Option<usize> {
    let mut current = input_bytes.len() - 1;
    let mut bracket_depth = 1;
    let mut found = false;

    current -= 1;

    while current > 0 {
        match input_bytes[current] {
            b'[' => {
                if bracket_depth == 1 {
                    found = true;
                    break;
                }
                bracket_depth -= 1;
            }
            b']' => {
                bracket_depth += 1;
            }
            _ => {}
        }

        current -= 1;
    }

    if found { Some(current) } else { None }
}

#[cfg(test)]
mod tests {
    use crate::archive::Exchange;

    #[derive(Debug)]
    struct Variables(serde_json::Value);

    impl<'a> crate::archive::request::Variables<'a> for Variables {
        fn parse_with_name<'de: 'a, A: serde::de::MapAccess<'de>>(
            _name: crate::request::name::RequestName,
            map: &mut A,
        ) -> Result<Self, A::Error>
        where
            Self: Sized,
        {
            Ok(Self(map.next_value()?))
        }
    }

    #[derive(Debug)]
    struct Body(serde_json::Value);

    impl<'a> crate::archive::response::ParseWithVariables<'a, Variables> for Body {
        fn parse(
            input: &'a str,
            _variables: &Variables,
        ) -> Result<Self, crate::archive::response::Error>
        where
            Self: Sized + 'a,
        {
            Ok(Self(serde_json::from_str(input)?))
        }
    }

    #[test]
    fn parse_exchange_with_timeout_errors() {
        // We remove the trailing line break from the file contents.
        let json =
            include_str!("../../../examples/graphql/errors-timeout-1764462528033.json").trim_end();

        let exchange: Exchange<'_, Variables, Body> =
            super::parse_exchange(json, 1, &crate::request::filter::exclude_filter([]))
                .unwrap()
                .unwrap();

        assert!(exchange.request.variables.0.is_object());
        assert_eq!(exchange.errors.len(), 3);

        let data_field = exchange.data.unwrap();
        let data_object = data_field.0.as_object().unwrap();

        assert_eq!(data_object.keys().collect::<Vec<_>>(), vec!["user"]);
    }

    #[test]
    fn parse_exchange_with_authorization_errors() {
        // We remove the trailing line break from the file contents.
        let json =
            include_str!("../../../examples/graphql/errors-authorization-1764497106517.json")
                .trim_end();

        let exchange: Exchange<'_, Variables, Body> =
            super::parse_exchange(json, 1, &crate::request::filter::exclude_filter([]))
                .unwrap()
                .unwrap();

        assert!(exchange.request.variables.0.is_object());
        assert_eq!(exchange.errors.len(), 24);

        let data_field = exchange.data.unwrap();
        let data_object = data_field.0.as_object().unwrap();

        assert_eq!(data_object.keys().collect::<Vec<_>>(), vec!["tweetResult"]);
    }

    #[test]
    fn parse_exchange_without_errors() {
        // We remove the trailing line break from the file contents.
        let json =
            include_str!("../../../examples/graphql/errors-none-1764460993001.json").trim_end();

        let exchange: Exchange<'_, Variables, Body> =
            super::parse_exchange(json, 1, &crate::request::filter::exclude_filter([]))
                .unwrap()
                .unwrap();

        assert!(exchange.request.variables.0.is_object());
        assert_eq!(exchange.errors.len(), 0);

        let data_field = exchange.data.unwrap();
        let data_object = data_field.0.as_object().unwrap();

        assert_eq!(
            data_object.keys().collect::<Vec<_>>(),
            vec!["user_result_by_screen_name"]
        );
    }

    #[test]
    fn parse_exchange_with_missing_data() {
        // We remove the trailing line break from the file contents.
        let json = include_str!("../../../examples/graphql/errors-data-missing-1738068567896.json")
            .trim_end();

        let exchange: Exchange<'_, Variables, Body> =
            super::parse_exchange(json, 1, &crate::request::filter::exclude_filter([]))
                .unwrap()
                .unwrap();

        assert!(exchange.request.variables.0.is_object());
        assert_eq!(exchange.errors.len(), 1);

        assert!(exchange.data.is_none());
    }
}
