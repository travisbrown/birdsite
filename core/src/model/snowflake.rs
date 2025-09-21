use chrono::{DateTime, SubsecRound, TimeZone, Utc};

/// Convert a Snowflake ID to a time.
#[must_use]
pub fn snowflake_to_date_time(id: u64) -> Option<DateTime<Utc>> {
    if is_snowflake(id) {
        known_snowflake_to_date_time(id)
    } else {
        None
    }
}

const FIRST_SNOWFLAKE: u64 = 250_000_000_000_000;

const fn is_snowflake(id: u64) -> bool {
    id >= FIRST_SNOWFLAKE
}

/// Convert a Snowflake ID to a time.
///
/// Does not confirm whether the ID is in the Snowflake range. An empty result indicates that the
/// integer is out of range.
fn known_snowflake_to_date_time(id: u64) -> Option<DateTime<Utc>> {
    let id = i64::try_from(id).ok()?;
    let timestamp_millis = (id >> 22) + 1_288_834_974_657;

    let date_time = Utc.timestamp_millis_opt(timestamp_millis).single()?;

    Some(date_time.trunc_subsecs(0))
}

#[cfg(test)]
mod test {
    #[test]
    fn pre_snowflake_is_none() {
        assert_eq!(super::snowflake_to_date_time(44196397), None);
    }

    #[test]
    fn snowflake_matches_created_at() {
        assert_eq!(
            super::snowflake_to_date_time(1_016_908_862_136_332_288),
            Some(chrono::DateTime::from_naive_utc_and_offset(
                chrono::NaiveDate::from_ymd_opt(2018, 7, 11)
                    .unwrap()
                    .and_hms_opt(4, 55, 40)
                    .unwrap(),
                chrono::Utc
            ))
        );
    }
}
