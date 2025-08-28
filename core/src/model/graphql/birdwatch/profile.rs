use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct Profile<'a> {
    pub alias: Option<Cow<'a, str>>,
    pub ratings_count: Option<RatingsCount>,
    pub notes_count: Option<NotesCount>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct RatingsCount {
    pub successful: HelpfulnessCount,
    pub unsuccessful: HelpfulnessCount,
    #[serde(with = "crate::model::attributes::timestamp_msec")]
    pub last_updated_at: DateTime<Utc>,
    pub rated_after_decision: Option<usize>,
    pub awaiting_more_ratings: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum HelpfulnessCount {
    Available {
        helpful_count: usize,
        not_helpful_count: usize,
        total: usize,
    },
    Empty {},
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct NotesCount {
    pub currently_rated_helpful: Option<usize>,
    pub currently_rated_not_helpful: Option<usize>,
    pub awaiting_more_ratings: Option<usize>,
    #[serde(with = "crate::model::attributes::timestamp_msec")]
    pub last_updated_at: DateTime<Utc>,
}
