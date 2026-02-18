use crate::model::graphql::{unavailable::TweetUnavailableReason, user::UserResult};
use std::borrow::Cow;

pub mod partial;
pub mod preview;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TweetResult<'a> {
    Full(Tweet<'a>),
    Preview(preview::TweetPreview<'a>),
    Unavailable {
        id: u64,
        /// May be empty in the case where we have a tombstone with no explanation.
        reason: Option<TweetUnavailableReason>,
    },
    Incomplete {
        id: u64,
    },
}

impl TweetResult<'_> {
    #[must_use]
    pub const fn id(&self) -> u64 {
        match self {
            Self::Full(tweet) => tweet.id,
            Self::Preview(tweet) => tweet.id,
            Self::Unavailable { id, .. } => *id,
            Self::Incomplete { id } => *id,
        }
    }
}

impl bounded_static::IntoBoundedStatic for TweetResult<'_> {
    type Static = TweetResult<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Full(tweet) => Self::Static::Full(tweet.into_static()),
            Self::Preview(tweet) => Self::Static::Preview(tweet.into_static()),
            Self::Unavailable { id, reason } => Self::Static::Unavailable { id, reason },
            Self::Incomplete { id } => Self::Static::Incomplete { id },
        }
    }
}

// TODO: Fill this in.
#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct Tweet<'a> {
    pub id: u64,
    pub user: UserResult<'a>,
    pub full_text: Cow<'a, str>,
}
