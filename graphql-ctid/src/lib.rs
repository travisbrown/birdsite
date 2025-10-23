//! Generator for X's client transaction IDs.
//!
//! Adapted from Sarabjit Dhiman's [Python implementation][python-generator],
//! with additional reference to [an excellent blog post series][generator-blog-1]
//! by ["obfio"](https://github.com/obfio).
//!
//! [generator-blog-1]: https://antibot.blog/posts/1741552025433
//! [python-generator]: https://github.com/iSarabjitDhiman/XClientTransaction

#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![forbid(unsafe_code)]
use chrono::{DateTime, SubsecRound, Utc};
use std::borrow::Cow;

pub mod client;
pub mod generator;

/// Generate a transaction ID for an endpoint.
///
/// This is a convenience method. Use a `Client` if you're making multiple requests.
pub async fn generate(endpoint: &Endpoint<'_>) -> Result<TransactionId, client::Error> {
    let client = client::Client::default();

    client.generate(endpoint).await
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Endpoint<'a> {
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

impl<'a> Endpoint<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(name: S, version: S) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransactionId {
    pub value: String,
    /// Should not be needed (and not currently serialized), but may be useful for debugging.
    pub animation_key: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl TransactionId {
    pub fn new(value: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            value,
            animation_key: None,
            timestamp: timestamp.trunc_subsecs(0),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SiteInfo {
    pub verification_key: Vec<u8>,
    pub indices: Vec<usize>,
    pub frame: Vec<i32>,
}
