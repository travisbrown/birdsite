#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetUnavailableReason {
    Blocked,
    BounceDeleted,
    Bounced,
    Deactivated,
    Deleted,
    ExclusiveTweet,
    HiddenByModerator,
    /// Indicates that the user account has blocked the viewer or is protected.
    Limited,
    /// Indicates either that the user account has been deactivated or the tweet deleted (or both).
    Missing,
    Protected,
    RuleViolation,
    Suspended,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UserUnavailableReason {
    Deactivated,
    NoReason,
    Offboarded,
    Protected,
    Suspended,
}
