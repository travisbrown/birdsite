//! Parses real archived GraphQL exchanges kept outside version control.
//!
//! The files under `tests/data/local/` (gitignored, so the test is a no-op on a fresh checkout)
//! are single lines from the archived exchange corpus, each named
//! `<request-name>-<timestamp_ms>.json` after its content. They were selected to maximize
//! coverage: every request name observed in the corpus, requests with and without a `version`
//! field, every `errors`/`data` layout the parser distinguishes (errors before data, errors after
//! data, no errors, and no data), and every observed error shape, kind, and source.
use birdsite_graphql::archive::Exchange;

/// Passes the raw variables JSON through, as the exchange-shape tests are request-agnostic.
#[derive(Debug)]
struct Variables(serde_json::Value);

impl<'a> birdsite_graphql::archive::request::Variables<'a> for Variables {
    fn parse_with_name<'de: 'a, A: serde::de::MapAccess<'de>>(
        _name: birdsite_graphql::request::name::RequestName,
        map: &mut A,
    ) -> Result<Self, A::Error>
    where
        Self: Sized,
    {
        Ok(Self(map.next_value()?))
    }
}

/// Passes the raw data JSON through, as the exchange-shape tests are request-agnostic.
#[derive(Debug)]
struct Body(serde_json::Value);

impl<'a> birdsite_graphql::archive::response::ParseWithVariables<'a, Variables> for Body {
    fn parse(
        input: &'a str,
        _variables: &Variables,
    ) -> Result<Self, birdsite_graphql::archive::response::Error>
    where
        Self: Sized + 'a,
    {
        Ok(Self(serde_json::from_str(input)?))
    }
}

#[test]
fn parse_local_exchange_examples() {
    // Kept out of version control, so the test is a no-op when the directory is absent rather
    // than a hard failure.
    let Ok(entries) = std::fs::read_dir("tests/data/local") else {
        return;
    };

    let mut checked = 0;
    for entry in entries {
        let path = entry.expect("Invalid directory entry").path();
        let contents = std::fs::read_to_string(&path).expect("Invalid example file");
        let contents = contents.trim_end();

        let exchange: Exchange<'_, Variables, Body> =
            birdsite_graphql::archive::parse::parse_exchange(
                contents,
                1,
                &birdsite_graphql::request::filter::exclude_filter([]),
            )
            .unwrap_or_else(|error| panic!("{}: parse failure: {error}", path.display()))
            .unwrap_or_else(|name| panic!("{}: unexpectedly filtered ({name:?})", path.display()));

        assert!(
            exchange.request.variables.0.is_object(),
            "{}: request variables are not an object",
            path.display()
        );
        if let Some(body) = &exchange.data {
            assert!(
                body.0.is_object(),
                "{}: response data is not an object",
                path.display()
            );
        }

        // The file is named `<request-name>-<timestamp_ms>.json` after its content; check that the
        // parsed request agrees, which guards against misfiled examples.
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("Invalid file name");
        assert_eq!(
            stem,
            format!(
                "{}-{}",
                exchange.request.name,
                exchange.request.timestamp.timestamp_millis()
            ),
            "{}: file name does not match content",
            path.display()
        );

        checked += 1;
    }

    assert!(checked > 0, "Example directory present but empty");
}
