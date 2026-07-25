#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![forbid(unsafe_code)]
//! Parsing and concurrent tracking of X (Twitter) API rate-limit headers.
//!
//! [`RateLimit`] parses the `x-rate-limit-reset` and `x-rate-limit-remaining` headers from a
//! [`reqwest::header::HeaderMap`], a JSON object, or any iterator of name-value pairs, and
//! [`RateLimits`] is a cheaply cloneable concurrent map that tracks the current limit per scope.
use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use std::hash::Hash;
use std::sync::{Arc, atomic::AtomicU64};
use std::time::Duration;

/// Default name of the header carrying the rate-limit reset time (epoch seconds).
pub const DEFAULT_RATE_LIMIT_RESET_HEADER_NAME: &str = "x-rate-limit-reset";
/// Default name of the header carrying the number of remaining requests.
pub const DEFAULT_RATE_LIMIT_REMAINING_HEADER_NAME: &str = "x-rate-limit-remaining";
/// Fallback wait applied when a reset time cannot be converted to a [`Duration`].
pub const DEFAULT_RATE_LIMIT_ERROR_WAIT: Duration = Duration::from_mins(15);

const DEFAULT_RATE_LIMIT_WAIT_BUFFER: TimeDelta = TimeDelta::seconds(10);
const DEFAULT_RATE_LIMITS_MAP_CAPACITY: usize = 16;

/// Errors that can occur while parsing rate-limit headers.
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// A required header was not present. Contains the header name.
    #[error("Missing header: {0}")]
    MissingHeader(String),
    /// A header was present but its value could not be parsed. Contains the header name.
    #[error("Invalid header value: {0}")]
    InvalidHeader(String),
}

/// A rate limit for a single scope: when the window resets and how many requests remain.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RateLimit {
    /// Time at which the current rate-limit window resets.
    pub reset: DateTime<Utc>,
    /// Number of requests remaining in the current window.
    pub remaining: usize,
}

impl RateLimit {
    /// Creates a new rate limit from a reset time and a remaining count.
    #[must_use]
    pub const fn new(reset: DateTime<Utc>, remaining: usize) -> Self {
        Self { reset, remaining }
    }

    #[must_use]
    pub fn wait(&self) -> Option<Duration> {
        if self.remaining == 0 {
            let timestamp = Utc::now();
            let wait = self.reset - timestamp + DEFAULT_RATE_LIMIT_WAIT_BUFFER;

            if wait > TimeDelta::zero() {
                // If somehow the time delta is out of range, we use our default error wait.
                Some(wait.to_std().unwrap_or(DEFAULT_RATE_LIMIT_ERROR_WAIT))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Parses a rate limit from a reqwest header map.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingHeader`] if either header is absent, or [`Error::InvalidHeader`] if
    /// a value is not a valid integer (or, for the reset header, not a valid epoch second).
    pub fn parse_headers(
        reset_header_name: &str,
        remaining_header_name: &str,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<Self, Error> {
        let reset_value = headers
            .get(reset_header_name)
            .ok_or_else(|| Error::MissingHeader(reset_header_name.to_string()))?;

        let remaining_value = headers
            .get(remaining_header_name)
            .ok_or_else(|| Error::MissingHeader(remaining_header_name.to_string()))?;

        let reset = reset_value
            .to_str()
            .ok()
            .and_then(|value| {
                value
                    .parse::<i64>()
                    .ok()
                    .and_then(|seconds| Utc.timestamp_opt(seconds, 0).single())
            })
            .ok_or_else(|| Error::InvalidHeader(reset_header_name.to_string()))?;

        let remaining = remaining_value
            .to_str()
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| Error::InvalidHeader(remaining_header_name.to_string()))?;

        Ok(Self { reset, remaining })
    }

    /// Parses a rate limit from headers represented as a JSON object.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`parse_headers`](Self::parse_headers).
    pub fn parse_headers_json(
        reset_header_name: &str,
        remaining_header_name: &str,
        headers: &serde_json::Value,
    ) -> Result<Self, Error> {
        let reset_value = headers
            .get(reset_header_name)
            .ok_or_else(|| Error::MissingHeader(reset_header_name.to_string()))?;

        let remaining_value = headers
            .get(remaining_header_name)
            .ok_or_else(|| Error::MissingHeader(remaining_header_name.to_string()))?;

        let reset = reset_value
            .as_str()
            .and_then(|value| {
                value
                    .parse::<i64>()
                    .ok()
                    .and_then(|seconds| Utc.timestamp_opt(seconds, 0).single())
            })
            .ok_or_else(|| Error::InvalidHeader(reset_header_name.to_string()))?;

        let remaining = remaining_value
            .as_str()
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| Error::InvalidHeader(remaining_header_name.to_string()))?;

        Ok(Self { reset, remaining })
    }

    /// Parses a rate limit from an iterator of header name-value pairs.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`parse_headers`](Self::parse_headers).
    pub fn parse_headers_iter<'a, I: Iterator<Item = (&'a str, &'a str)>>(
        reset_header_name: &str,
        remaining_header_name: &str,
        headers: I,
    ) -> Result<Self, Error> {
        let mut reset = None;
        let mut remaining = None;

        // Header names are case-insensitive, matching the `HeaderMap`-based `parse_headers`.
        for (key, value) in headers {
            if reset.is_none() && key.eq_ignore_ascii_case(reset_header_name) {
                reset = Some(
                    value
                        .parse::<i64>()
                        .ok()
                        .and_then(|seconds| Utc.timestamp_opt(seconds, 0).single())
                        .ok_or_else(|| Error::InvalidHeader(reset_header_name.to_string()))?,
                );
            } else if remaining.is_none() && key.eq_ignore_ascii_case(remaining_header_name) {
                remaining = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| Error::InvalidHeader(remaining_header_name.to_string()))?,
                );
            }

            if reset.is_some() && remaining.is_some() {
                break;
            }
        }

        reset.map_or_else(
            || Err(Error::MissingHeader(reset_header_name.to_string())),
            |reset| {
                remaining.map_or_else(
                    || Err(Error::MissingHeader(remaining_header_name.to_string())),
                    |remaining| Ok(Self { reset, remaining }),
                )
            },
        )
    }
}

impl TryFrom<&reqwest::header::HeaderMap> for RateLimit {
    type Error = Error;

    fn try_from(value: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
        Self::parse_headers(
            DEFAULT_RATE_LIMIT_RESET_HEADER_NAME,
            DEFAULT_RATE_LIMIT_REMAINING_HEADER_NAME,
            value,
        )
    }
}

impl TryFrom<&serde_json::Value> for RateLimit {
    type Error = Error;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        Self::parse_headers_json(
            DEFAULT_RATE_LIMIT_RESET_HEADER_NAME,
            DEFAULT_RATE_LIMIT_REMAINING_HEADER_NAME,
            value,
        )
    }
}

/// A concurrent map for tracking rate limits for a set of scopes.
#[derive(Clone)]
pub struct RateLimits<S> {
    underlying: Arc<dashmap::DashMap<S, AtomicU64>>,
}

impl<S: Eq + Hash> Default for RateLimits<S> {
    fn default() -> Self {
        Self {
            underlying: Arc::new(dashmap::DashMap::with_capacity(
                DEFAULT_RATE_LIMITS_MAP_CAPACITY,
            )),
        }
    }
}

impl<S: Eq + Hash> RateLimits<S> {
    /// Returns how long to wait for the given scope, if it is currently rate-limited.
    pub fn wait(&self, scope: &S) -> Option<Duration> {
        self.get(scope).and_then(|rate_limit| rate_limit.wait())
    }

    /// Returns the most recently recorded rate limit for the given scope, if any.
    pub fn get(&self, scope: &S) -> Option<RateLimit> {
        rate_limit_from_atomic64(self.underlying.get(scope)?.value())
    }

    /// Records the rate limit for the given scope.
    ///
    /// If the reset time's epoch second does not fit into a `u32` (before 1970 or after 2106), or
    /// the remaining count exceeds `u32::MAX`, the value cannot be encoded and this is a no-op.
    pub fn put(&self, scope: S, value: RateLimit) {
        if let Ok(reset_s) = u32::try_from(value.reset.timestamp())
            && let Ok(remaining) = u32::try_from(value.remaining)
        {
            let encoded: u64 = bytemuck::cast([reset_s, remaining]);

            // Insert the fully-encoded value atomically so a concurrent reader never observes a
            // transient zeroed (`or_default`) entry for a scope that was just recorded.
            self.underlying
                .entry(scope)
                .and_modify(|current| current.store(encoded, std::sync::atomic::Ordering::Relaxed))
                .or_insert_with(|| AtomicU64::new(encoded));
        }
    }
}

impl<S: Clone + Eq + Hash> RateLimits<S> {
    /// Returns an iterator over the recorded scopes and their rate limits.
    ///
    /// The iterator holds shard read locks for its lifetime, so calling [`RateLimits::put`] on the
    /// same map from the same thread while iterating may deadlock; collect the iterator first if
    /// you need to mutate concurrently.
    #[must_use]
    pub fn iter(&self) -> RateLimitsIterator<'_, S> {
        RateLimitsIterator {
            underlying: self.underlying.iter(),
        }
    }
}

impl<'a, S: Clone + Eq + Hash> IntoIterator for &'a RateLimits<S> {
    type Item = (S, Option<RateLimit>);
    type IntoIter = RateLimitsIterator<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the scopes and rate limits recorded in a [`RateLimits`] map.
pub struct RateLimitsIterator<'a, S> {
    underlying: dashmap::iter::Iter<'a, S, AtomicU64>,
}

impl<S: Clone + Eq + Hash> Iterator for RateLimitsIterator<'_, S> {
    type Item = (S, Option<RateLimit>);

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().map(|ref_multi| {
            (
                ref_multi.key().clone(),
                rate_limit_from_atomic64(ref_multi.value()),
            )
        })
    }
}

impl<S: Eq + Clone + Hash + std::fmt::Debug> std::fmt::Debug for RateLimits<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().collect::<Vec<_>>().fmt(f)
    }
}

fn rate_limit_from_atomic64(value: &AtomicU64) -> Option<RateLimit> {
    let values: [u32; 2] = bytemuck::cast(value.load(std::sync::atomic::Ordering::Relaxed));

    // These conversions should never fail, since we control the values that go into the map.
    // In the case that one does fail, we simply show that there was no entry for this scope.
    let reset = Utc.timestamp_opt(i64::from(values[0]), 0).single()?;
    let remaining = usize::try_from(values[1]).ok()?;

    Some(RateLimit::new(reset, remaining))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{SubsecRound, Utc};
    use std::collections::HashSet;

    #[test]
    fn parse_headers_iter_ignores_header_name_case() {
        // Regression: header names were compared case-sensitively, unlike the case-insensitive
        // `HeaderMap`-based `parse_headers`.
        let headers = [
            ("X-Rate-Limit-Reset", "1245946721"),
            ("X-Rate-Limit-Remaining", "42"),
        ];

        let rate_limit = RateLimit::parse_headers_iter(
            "x-rate-limit-reset",
            "x-rate-limit-remaining",
            headers.into_iter(),
        )
        .unwrap();

        assert_eq!(rate_limit.remaining, 42);
        assert_eq!(rate_limit.reset.timestamp(), 1_245_946_721);
    }

    #[test]
    fn rate_limits() {
        let scope_a = "A".to_string();
        let scope_b = "BBBBBB".to_string();
        let scope_c = String::new();
        let rate_limits = RateLimits::default();

        assert_eq!(rate_limits.get(&scope_a.as_str()), None);

        let rate_limit_a = RateLimit::new(Utc::now().round_subsecs(0), 123);

        rate_limits.put(scope_a.as_str(), rate_limit_a);
        assert_eq!(rate_limits.get(&scope_a.as_str()), Some(rate_limit_a));

        let rate_limit_b = RateLimit::new(Utc::now().round_subsecs(0), 45678);

        rate_limits.put(scope_b.as_str(), rate_limit_b);
        assert_eq!(rate_limits.get(&scope_a.as_str()), Some(rate_limit_a));
        assert_eq!(rate_limits.get(&scope_b.as_str()), Some(rate_limit_b));

        let cloned = rate_limits.clone();

        let rate_limit_c = RateLimit::new(Utc::now().round_subsecs(0), 0);

        cloned.put(scope_c.as_str(), rate_limit_c);

        assert_eq!(cloned.get(&scope_a.as_str()), Some(rate_limit_a));
        assert_eq!(cloned.get(&scope_b.as_str()), Some(rate_limit_b));
        assert_eq!(cloned.get(&scope_c.as_str()), Some(rate_limit_c));

        assert_eq!(rate_limits.get(&scope_a.as_str()), Some(rate_limit_a));
        assert_eq!(rate_limits.get(&scope_b.as_str()), Some(rate_limit_b));
        assert_eq!(rate_limits.get(&scope_c.as_str()), Some(rate_limit_c));

        let rate_limit_b = RateLimit::new(Utc::now().round_subsecs(0), 987);

        rate_limits.put(scope_b.as_str(), rate_limit_b);

        assert_eq!(cloned.get(&scope_a.as_str()), Some(rate_limit_a));
        assert_eq!(cloned.get(&scope_b.as_str()), Some(rate_limit_b));
        assert_eq!(cloned.get(&scope_c.as_str()), Some(rate_limit_c));

        assert_eq!(rate_limits.get(&scope_a.as_str()), Some(rate_limit_a));
        assert_eq!(rate_limits.get(&scope_b.as_str()), Some(rate_limit_b));
        assert_eq!(rate_limits.get(&scope_c.as_str()), Some(rate_limit_c));

        let values = rate_limits.iter().collect::<HashSet<_>>();

        assert_eq!(
            values,
            vec![
                (scope_a.as_ref(), Some(rate_limit_a)),
                (scope_b.as_ref(), Some(rate_limit_b)),
                (scope_c.as_ref(), Some(rate_limit_c))
            ]
            .into_iter()
            .collect()
        );
    }
}
