use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AboutProfile<'a> {
    pub account_based_in: Location,
    pub location_accurate: bool,
    pub learn_more_url: LearnMoreUrl,
    pub affiliate_username: Cow<'a, str>,
    pub source: Source,
    pub username_changes: UsernameChanges,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]

pub enum Location {
    #[serde(rename = "United States")]
    UnitedStates,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum LearnMoreUrl {
    #[serde(
        rename = "https://help.twitter.com/managing-your-account/about-twitter-verified-accounts"
    )]
    AboutTwitterVerifiedAccount,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]

pub enum Source {
    #[serde(rename = "United States Android App")]
    UnitedStatesAndroidApp,
    #[serde(rename = "United States App Store")]
    UnitedStatesAppStore,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UsernameChanges {
    #[serde(with = "serde_field_attributes::integer_str")]
    pub count: usize,
    #[serde(
        rename = "last_change_at_msec",
        with = "serde_field_attributes::optional_timestamp_millis_str"
    )]
    pub last_changed_at: Option<DateTime<Utc>>,
}
