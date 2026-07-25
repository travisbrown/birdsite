use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc, serde::ts_milliseconds};
use std::borrow::Cow;

/// Current helpfulness status of a Community Note.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Status {
    #[serde(rename = "NEEDS_MORE_RATINGS")]
    NeedsMoreRatings,
    #[serde(rename = "CURRENTLY_RATED_NOT_HELPFUL")]
    NotHelpful,
    #[serde(rename = "CURRENTLY_RATED_HELPFUL")]
    Helpful,
}

impl Status {
    /// Returns whether the note is rated helpful, or `None` if it still needs more ratings.
    #[must_use]
    pub const fn is_helpful(self) -> Option<bool> {
        match self {
            Self::NeedsMoreRatings => None,
            Self::NotHelpful => Some(false),
            Self::Helpful => Some(true),
        }
    }
}

/// A note author's classification of the tweet the note is attached to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Classification {
    #[serde(rename = "NOT_MISLEADING")]
    NotMisleading,
    #[serde(rename = "MISINFORMED_OR_POTENTIALLY_MISLEADING")]
    Misleading,
    #[serde(rename = "")]
    Empty,
}

/// An entry in the note status history data set, recording a note's current status.
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
pub struct NoteStatusHistoryEntry<'a> {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    #[serde(rename = "noteAuthorParticipantId", borrow)]
    pub participant_id: Cow<'a, str>,
    #[serde(rename = "createdAtMillis", with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "currentStatus")]
    pub current_status: Status,
}

/// An entry in the notes data set, describing a single Community Note.
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
pub struct NoteEntry<'a> {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    #[serde(rename = "noteAuthorParticipantId", borrow)]
    pub participant_id: Cow<'a, str>,
    #[serde(rename = "createdAtMillis", with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    /// ID of the tweet the note is attached to. The source data uses `-1` for notes that are not
    /// associated with a tweet; that sentinel is represented here as `None`.
    #[serde(rename = "tweetId", with = "optional_tweet_id")]
    pub tweet_id: Option<u64>,
    #[serde(rename = "classification")]
    pub classification: Classification,
}

/// Serde conversion for the `tweetId` field, mapping the `-1` sentinel to `None` and any
/// non-negative value to `Some`. Any other negative value is rejected.
mod optional_tweet_id {
    /// Deserializes a signed tweet ID, mapping the `-1` sentinel to `None` and any other
    /// non-negative value to `Some`. Any other negative value is rejected as invalid rather
    /// than silently discarded.
    pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        let raw = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        if raw == -1 {
            Ok(None)
        } else {
            u64::try_from(raw).map(Some).map_err(|_| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Signed(raw),
                    &"a non-negative id or the -1 sentinel",
                )
            })
        }
    }

    /// Serializes `None` back to the `-1` sentinel and `Some` to the underlying value.
    // serde's `with` module requires the `&Option<_>` receiver shape.
    #[allow(clippy::ref_option)]
    pub(super) fn serialize<S: serde::Serializer>(
        value: &Option<u64>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(id) => serializer.serialize_u64(*id),
            None => serializer.serialize_i64(-1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrows_participant_id() {
        // Regression: without `#[serde(borrow)]`, serde's blanket `Cow` impl always produces
        // `Cow::Owned`, heap-allocating every participant id.
        let json = concat!(
            r#"{"noteId":1,"noteAuthorParticipantId":"4874E27F86D8AC0AC3AB85FF754C25BE","#,
            r#""createdAtMillis":1245946721000,"currentStatus":"CURRENTLY_RATED_HELPFUL"}"#
        );

        let entry = serde_json::from_str::<NoteStatusHistoryEntry<'_>>(json).unwrap();

        assert!(matches!(entry.participant_id, Cow::Borrowed(_)));
    }

    #[test]
    fn tweet_id_sentinel_and_values() {
        fn parse(raw: &str) -> Result<Option<u64>, serde_json::Error> {
            super::optional_tweet_id::deserialize(&mut serde_json::Deserializer::from_str(raw))
        }

        // The `-1` sentinel maps to `None`, non-negative ids parse, but any other negative value
        // is rejected rather than silently discarded.
        assert_eq!(parse("-1").unwrap(), None);
        assert_eq!(parse("0").unwrap(), Some(0));
        assert_eq!(parse("123").unwrap(), Some(123));
        assert!(parse("-2").is_err());
    }
}
