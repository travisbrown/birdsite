use crate::model::wxj::{TweetSnapshot, data, flat};
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserMetadata {
    pub id: u64,
    pub created_at: Option<DateTime<Utc>>,
}

impl UserMetadata {
    #[must_use]
    pub const fn new(id: u64, created_at: Option<DateTime<Utc>>) -> Self {
        Self { id, created_at }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TweetMetadata {
    pub id: u64,
    pub user: UserMetadata,
    pub created_at: DateTime<Utc>,
    pub retweeted_id: Option<u64>,
    pub replied_to_id: Option<u64>,
    pub quoted_id: Option<u64>,
}

impl TweetMetadata {
    #[must_use]
    pub const fn new(
        id: u64,
        user: UserMetadata,
        created_at: DateTime<Utc>,
        retweeted_id: Option<u64>,
        replied_to_id: Option<u64>,
        quoted_id: Option<u64>,
    ) -> Self {
        Self {
            id,
            user,
            created_at,
            retweeted_id,
            replied_to_id,
            quoted_id,
        }
    }

    pub fn from_tweet_snapshot(
        snapshot: &TweetSnapshot<'_>,
    ) -> Result<Vec<Self>, data::FormatError> {
        match snapshot {
            TweetSnapshot::Data(snapshot) => Self::from_data_tweet_snapshot(snapshot),
            TweetSnapshot::Flat(snapshot) => Ok(Self::from_flat_tweet_snapshot(snapshot)),
        }
    }

    #[must_use]
    pub fn from_flat_tweet_snapshot(snapshot: &flat::TweetSnapshot<'_>) -> Vec<Self> {
        let mut results = Vec::with_capacity(1);

        Self::extract_flat_tweet(&mut results, snapshot);

        results
    }

    pub fn from_data_tweet_snapshot(
        snapshot: &data::TweetSnapshot<'_>,
    ) -> Result<Vec<Self>, data::FormatError> {
        let mut tweets = vec![Self::extract_data_tweet(snapshot, &snapshot.data)?];

        if let Some(tweet) = snapshot
            .data
            .retweeted_id()?
            .and_then(|id| snapshot.lookup_tweet(id))
        {
            tweets.extend(Self::extract_data_tweet(snapshot, &tweet));
        }

        if let Some(tweet) = snapshot
            .data
            .replied_to_id()?
            .and_then(|id| snapshot.lookup_tweet(id))
        {
            tweets.extend(Self::extract_data_tweet(snapshot, &tweet));
        }

        if let Some(tweet) = snapshot
            .data
            .quoted_id()?
            .and_then(|id| snapshot.lookup_tweet(id))
        {
            tweets.extend(Self::extract_data_tweet(snapshot, &tweet));
        }

        Ok(tweets)
    }

    fn extract_flat_tweet(results: &mut Vec<Self>, snapshot: &flat::TweetSnapshot<'_>) {
        let user = UserMetadata::new(snapshot.user.id, Some(snapshot.user.created_at));

        if let Some(status) = snapshot.quoted_status.as_ref() {
            Self::extract_flat_tweet(results, status);
        }

        if let Some(status) = snapshot.retweeted_status.as_ref() {
            Self::extract_flat_tweet(results, status);
        }

        results.push(Self::new(
            snapshot.id,
            user,
            snapshot.created_at,
            snapshot.retweeted_status.as_ref().map(|status| status.id),
            snapshot.in_reply_to_status_id,
            snapshot.quoted_status_id,
        ));
    }

    fn extract_data_tweet(
        snapshot: &data::TweetSnapshot<'_>,
        data: &data::Tweet<'_>,
    ) -> Result<Self, data::FormatError> {
        let user = UserMetadata::new(
            data.author_id,
            snapshot
                .lookup_user(data.author_id)
                .map(|user| user.created_at),
        );

        Ok(Self::new(
            data.id,
            user,
            data.created_at,
            data.retweeted_id()?,
            data.replied_to_id()?,
            data.quoted_id()?,
        ))
    }
}
