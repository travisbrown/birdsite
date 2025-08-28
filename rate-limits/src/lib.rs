use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use std::hash::Hash;
use std::sync::{Arc, atomic::AtomicU64};
use std::time::Duration;

pub const DEFAULT_RATE_LIMIT_RESET_HEADER_NAME: &str = "x-rate-limit-reset";
pub const DEFAULT_RATE_LIMIT_REMAINING_HEADER_NAME: &str = "x-rate-limit-remaining";
pub const DEFAULT_RATE_LIMIT_ERROR_WAIT: Duration = Duration::from_secs(15 * 60);

const DEFAULT_RATE_LIMIT_WAIT_BUFFER: TimeDelta = TimeDelta::seconds(10);
const DEFAULT_RATE_LIMITS_MAP_CAPACITY: usize = 16;

pub type RateLimitResult = Result<RateLimit, Error>;

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Missing header")]
    MissingHeader(String),
    #[error("Invalid header value")]
    InvalidHeader(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RateLimit {
    pub reset: DateTime<Utc>,
    pub remaining: usize,
}

impl RateLimit {
    pub fn new(reset: DateTime<Utc>, remaining: usize) -> Self {
        Self { reset, remaining }
    }

    pub fn wait(&self) -> Option<Duration> {
        if self.remaining == 0 {
            let timestamp = Utc::now();
            let wait = self.reset - timestamp + DEFAULT_RATE_LIMIT_WAIT_BUFFER;

            if wait.num_seconds() > 0 {
                // If somehow the time delta is out of range, we use our default error wait.
                Some(wait.to_std().unwrap_or(DEFAULT_RATE_LIMIT_ERROR_WAIT))
            } else {
                None
            }
        } else {
            None
        }
    }

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

    pub fn parse_headers_iter<'a, I: Iterator<Item = (&'a str, &'a str)>>(
        reset_header_name: &str,
        remaining_header_name: &str,
        headers: I,
    ) -> Result<Self, Error> {
        let mut reset = None;
        let mut remaining = None;

        for (key, value) in headers {
            if reset.is_none() && key == reset_header_name {
                reset = Some(
                    value
                        .parse::<i64>()
                        .ok()
                        .and_then(|seconds| Utc.timestamp_opt(seconds, 0).single())
                        .ok_or_else(|| Error::InvalidHeader(reset_header_name.to_string()))?,
                );
            } else if remaining.is_none() && key == remaining_header_name {
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

        match reset {
            Some(reset) => match remaining {
                Some(remaining) => Ok(Self { reset, remaining }),
                None => Err(Error::MissingHeader(remaining_header_name.to_string())),
            },
            None => Err(Error::MissingHeader(reset_header_name.to_string())),
        }
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
    pub fn wait(&self, scope: &S) -> Option<Duration> {
        self.get(scope).and_then(|rate_limit| rate_limit.wait())
    }

    pub fn get(&self, scope: &S) -> Option<RateLimit> {
        rate_limit_from_atomic64(self.underlying.get(scope)?.value())
    }

    pub fn put(&self, scope: S, value: RateLimit) {
        // If we're given a timestamp where the epoch second doesn't fit into a `u32`, we simply do nothing.
        if let Ok(reset_s) = u32::try_from(value.reset.timestamp())
            && let Ok(remaining) = u32::try_from(value.remaining)
        {
            let encoded: u64 = bytemuck::cast([reset_s, remaining]);

            let entry = self.underlying.entry(scope).or_default();
            entry.store(encoded, std::sync::atomic::Ordering::Relaxed);
        }
    }

    pub fn iter(&self) -> RateLimitsIterator<'_, S> {
        RateLimitsIterator {
            underlying: self.underlying.iter(),
        }
    }
}

pub struct RateLimitsIterator<'a, S> {
    underlying: dashmap::iter::Iter<'a, S, AtomicU64>,
}

impl<'a, S: Clone + Eq + Hash> Iterator for RateLimitsIterator<'a, S> {
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
    let reset = Utc.timestamp_opt(values[0] as i64, 0).single()?;
    let remaining = usize::try_from(values[1]).ok()?;

    Some(RateLimit::new(reset, remaining))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{SubsecRound, Utc};
    use std::collections::HashSet;

    #[test]
    fn rate_limits() {
        let scope_a = "A".to_string();
        let scope_b = "BBBBBB".to_string();
        let scope_c = "".to_string();
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
        )
    }
}
