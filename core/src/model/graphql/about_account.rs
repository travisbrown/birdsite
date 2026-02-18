use crate::model::graphql::unavailable::UserUnavailableReason;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserResult<'a> {
    Available(User<'a>),
    Unavailable {
        screen_name: Cow<'a, str>,
        reason: UserUnavailableReason,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User<'a> {
    pub screen_name: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
}
