use chrono::{DateTime, Utc, serde::ts_milliseconds};
use std::borrow::Cow;

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
    pub const fn is_helpful(self) -> Option<bool> {
        match self {
            Self::NeedsMoreRatings => None,
            Self::NotHelpful => Some(false),
            Self::Helpful => Some(true),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Classification {
    #[serde(rename = "NOT_MISLEADING")]
    NotMisleading,
    #[serde(rename = "MISINFORMED_OR_POTENTIALLY_MISLEADING")]
    Misleading,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct NoteStatusHistoryEntry<'a> {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    #[serde(rename = "noteAuthorParticipantId")]
    pub participant_id: Cow<'a, str>,
    #[serde(rename = "createdAtMillis", with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "currentStatus")]
    pub current_status: Status,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct NoteEntry<'a> {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    #[serde(rename = "noteAuthorParticipantId")]
    pub participant_id: Cow<'a, str>,
    #[serde(rename = "createdAtMillis", with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "tweetId")]
    pub tweet_id: u64,
    #[serde(rename = "classification")]
    pub classification: Classification,
}
