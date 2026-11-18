//! Generator for X's client transaction IDs.
//!
//! Adapted from Sarabjit Dhiman's [Python implementation][python-generator],
//! with additional reference to [an excellent blog post series][generator-blog-1]
//! by [obfio](https://github.com/obfio).
//!
//! [generator-blog-1]: https://antibot.blog/posts/1741552025433
//! [python-generator]: https://github.com/iSarabjitDhiman/XClientTransaction

use std::borrow::Cow;

use jiff::{RoundMode, Timestamp, TimestampRound, Unit};

pub mod client;
pub mod generator;

/// Generate a transaction ID for an endpoint.
///
/// This is a convenience method. Use a `Client` if you're making multiple requests.
///
/// # Errors
///
/// Returns a [`client::Error`] if downloading or parsing the site information fails.
pub async fn generate(endpoint: &Endpoint<'_>) -> Result<TransactionId, client::Error> {
    let client = client::Client::default();

    client.generate(endpoint).await
}

/// A GraphQL endpoint, identified by its operation name and query-ID version.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Endpoint<'a> {
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

impl<'a> Endpoint<'a> {
    /// Creates an endpoint from a name and version, each accepting any string-like value.
    pub fn new<N: Into<Cow<'a, str>>, V: Into<Cow<'a, str>>>(name: N, version: V) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

/// A generated client transaction ID, along with the timestamp it encodes.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransactionId {
    pub value: String,
    /// Should not be needed (and not currently serialized), but may be useful for debugging.
    pub animation_key: Option<String>,
    pub timestamp: Timestamp,
}

impl TransactionId {
    /// Creates a transaction ID from its value and timestamp (truncated to whole seconds).
    // Rounding down to whole seconds cannot leave Timestamp's representable range.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(value: String, timestamp: Timestamp) -> Self {
        Self {
            value,
            animation_key: None,
            timestamp: timestamp
                .round(
                    TimestampRound::new()
                        .smallest(Unit::Second)
                        .mode(RoundMode::Floor),
                )
                .expect("rounding down to seconds stays within the timestamp range"),
        }
    }
}

/// Verification material extracted from the X home page, used to generate transaction IDs.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SiteInfo {
    pub verification_key: Vec<u8>,
    pub indices: Vec<usize>,
    pub frame: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use jiff::Timestamp;

    use super::TransactionId;

    #[test]
    fn transaction_ids_drop_fractional_seconds_before_and_after_the_epoch() {
        for (milliseconds, seconds) in [(-1001, -2), (-1, -1), (0, 0), (1999, 1)] {
            let timestamp = Timestamp::from_millisecond(milliseconds).unwrap();
            let id = TransactionId::new("test".to_owned(), timestamp);
            assert_eq!(id.timestamp, Timestamp::from_second(seconds).unwrap());
        }
    }

    #[test]
    fn transaction_ids_accept_timestamp_limits() {
        for timestamp in [Timestamp::MIN, Timestamp::MAX] {
            let id = TransactionId::new("test".to_owned(), timestamp);
            assert_eq!(id.timestamp.as_second(), timestamp.as_second());
            assert_eq!(id.timestamp.subsec_nanosecond(), 0);
        }
    }
}
