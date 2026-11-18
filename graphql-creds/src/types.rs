use rusqlite::types::FromSqlError;

pub struct Timestamp(jiff::Timestamp);

impl From<jiff::Timestamp> for Timestamp {
    fn from(value: jiff::Timestamp) -> Self {
        Self(value)
    }
}

impl From<Timestamp> for jiff::Timestamp {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl rusqlite::types::FromSql for Timestamp {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let timestamp_s = value.as_i64()?;

        let timestamp = jiff::Timestamp::from_second(timestamp_s)
            .map_err(|_| FromSqlError::OutOfRange(timestamp_s))?;

        Ok(Self(timestamp))
    }
}

impl rusqlite::types::ToSql for Timestamp {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let seconds = self.0.as_second() - i64::from(self.0.subsec_nanosecond() < 0);
        Ok(seconds.into())
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::Timestamp;

    #[test]
    fn sqlite_round_trips_epoch_seconds() {
        let connection = Connection::open_in_memory().unwrap();
        for seconds in [
            -1,
            0,
            1,
            jiff::Timestamp::MIN.as_second(),
            jiff::Timestamp::MAX.as_second(),
        ] {
            let timestamp = jiff::Timestamp::from_second(seconds).unwrap();
            let result: Timestamp = connection
                .query_row("SELECT ?1", [Timestamp::from(timestamp)], |row| row.get(0))
                .unwrap();
            assert_eq!(jiff::Timestamp::from(result), timestamp);
        }
    }

    #[test]
    fn sqlite_drops_fractional_seconds_before_the_epoch() {
        let connection = Connection::open_in_memory().unwrap();
        let timestamp = Timestamp::from(jiff::Timestamp::from_millisecond(-1).unwrap());
        let seconds: i64 = connection
            .query_row("SELECT ?1", [timestamp], |row| row.get(0))
            .unwrap();
        assert_eq!(seconds, -1);
    }

    #[test]
    fn sqlite_rejects_out_of_range_epoch_seconds() {
        let connection = Connection::open_in_memory().unwrap();
        for seconds in [i64::MIN, i64::MAX] {
            let result =
                connection.query_row("SELECT ?1", [seconds], |row| row.get::<_, Timestamp>(0));
            assert!(
                matches!(result, Err(rusqlite::Error::IntegralValueOutOfRange(0, value)) if value == seconds)
            );
        }
    }
}
