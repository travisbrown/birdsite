use crate::model::graphql::{unavailable::TweetUnavailableReason, user::UserResult};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TweetResult<'a> {
    Available(Tweet<'a>),
    Unavailable {
        id: u64,
        reason: TweetUnavailableReason,
    },
    Incomplete {
        id: u64,
    },
}

impl<'a> TweetResult<'a> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Available(tweet) => tweet.id,
            Self::Unavailable { id, .. } => *id,
            Self::Incomplete { id } => *id,
        }
    }
}

impl<'a> bounded_static::IntoBoundedStatic for TweetResult<'a> {
    type Static = TweetResult<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Available(tweet) => Self::Static::Available(tweet.into_static()),
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
