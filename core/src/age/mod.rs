use chrono::{DateTime, SubsecRound, TimeZone, Utc};

/// Convert a Snowflake ID to a time.
pub fn snowflake_to_date_time(id: u64) -> Option<DateTime<Utc>> {
    if is_snowflake(id) {
        known_snowflake_to_date_time(id)
    } else {
        None
    }
}

pub enum IdDateTime {
    Exact(DateTime<Utc>),
    Estimated(DateTime<Utc>),
    Unknown,
}

impl IdDateTime {
    pub fn from_user_id(id: u64) -> Self {
        if is_snowflake(id) {
            known_snowflake_to_date_time(id).map_or_else(|| Self::Unknown, Self::Exact)
        } else {
            Self::Unknown
        }
    }
}

const FIRST_SNOWFLAKE: u64 = 250000000000000;

fn is_snowflake(id: u64) -> bool {
    id >= FIRST_SNOWFLAKE
}

/// Convert a Snowflake ID to a time.
///
/// Does not confirm whether the ID is in the Snowflake range. An empty result indicates that the
/// integer is out of range.
fn known_snowflake_to_date_time(id: u64) -> Option<DateTime<Utc>> {
    let id = i64::try_from(id).ok()?;
    let timestamp_millis = (id >> 22) + 1288834974657;

    let date_time = Utc.timestamp_millis_opt(timestamp_millis).single()?;

    Some(date_time.trunc_subsecs(0))
}
