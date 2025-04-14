//! This data format appears for tweets in the Wayback Machine from at least 2023 into 2025 (TODO: find previous start date).

use crate::model::{
    EditControls, PlaceMetadata, TweetPublicMetrics, UserPublicMetrics,
    country::Country,
    entities::{TweetEntities, UserEntities},
    id_str, id_str_array_optional, id_str_optional,
    lang::Lang,
    media::{MediaType, MediaVariant},
};
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FormatError {
    #[error("Multiple referenced IDs")]
    MultipleReferencedIds(Vec<u64>),
    #[error("Missing referenced tweet")]
    MissingReferencedTweet(u64),
    #[error("Missing user")]
    MissingUser(u64),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    pub data: TweetData<'a>,
    #[serde(borrow)]
    pub includes: TweetIncludes<'a>,
    pub errors: Option<Vec<TweetErrors<'a>>>,
}

impl<'a> Tweet<'a> {
    pub fn user(&self) -> Result<User<'a>, FormatError> {
        self.lookup_user(self.data.author_id)
            .ok_or(FormatError::MissingUser(self.data.author_id))
    }

    pub fn lookup_user(&self, id: u64) -> Option<User<'a>> {
        self.includes
            .users
            .iter()
            .find(|user| user.id == id)
            .cloned()
    }

    pub fn lookup_tweet(&self, id: u64) -> Option<TweetData<'a>> {
        self.includes
            .tweets
            .as_ref()
            .and_then(|tweets| tweets.iter().find(|tweet| tweet.id == id))
            .cloned()
    }

    pub fn retweeted(&self) -> Result<Option<TweetData<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::Retweeted)
    }

    pub fn replied_to(&self) -> Result<Option<TweetData<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::RepliedTo)
    }

    pub fn quoted(&self) -> Result<Option<TweetData<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::Quoted)
    }

    /// Find referenced tweet.
    pub fn referenced_tweet(
        &self,
        reference_type: ReferenceType,
    ) -> Result<Option<TweetData<'a>>, FormatError> {
        self.data
            .referenced_tweet_id(reference_type)?
            .map(|id| {
                self.lookup_tweet(id)
                    .ok_or(FormatError::MissingReferencedTweet(id))
            })
            .map_or(Ok(None), |v| v.map(Some))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetData<'a> {
    pub article: Option<Article<'a>>,
    pub attachments: Option<Attachments<'a>>,
    #[serde(with = "id_str")]
    pub id: u64,
    #[serde(with = "id_str")]
    pub author_id: u64,
    pub context_annotations: Option<Vec<ContextAnnotation<'a>>>,
    #[serde(with = "id_str")]
    pub conversation_id: u64,
    pub created_at: DateTime<Utc>,
    pub edit_controls: Option<EditControls>,
    #[serde(with = "id_str_array_optional")]
    #[serde(default)]
    pub edit_history_tweet_ids: Option<Vec<u64>>,
    pub lang: Lang,
    pub entities: Option<TweetEntities<'a>>,
    #[serde(borrow)]
    pub geo: Option<Geo<'a>>,
    pub note_tweet: Option<NoteTweet<'a>>,
    pub possibly_sensitive: bool,
    pub public_metrics: TweetPublicMetrics,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    pub reply_settings: ReplySettings,
    pub text: Cow<'a, str>,
    #[serde(with = "id_str_optional")]
    #[serde(default)]
    pub in_reply_to_user_id: Option<u64>,
    pub source: Option<TweetSource>,
    pub withheld: Option<Withheld>,
}

impl TweetData<'_> {
    pub fn retweeted_id(&self) -> Result<Option<u64>, FormatError> {
        self.referenced_tweet_id(ReferenceType::Retweeted)
    }

    pub fn replied_to_id(&self) -> Result<Option<u64>, FormatError> {
        self.referenced_tweet_id(ReferenceType::RepliedTo)
    }

    pub fn quoted_id(&self) -> Result<Option<u64>, FormatError> {
        self.referenced_tweet_id(ReferenceType::Quoted)
    }

    /// Find referenced tweet.
    pub fn referenced_tweet_id(
        &self,
        reference_type: ReferenceType,
    ) -> Result<Option<u64>, FormatError> {
        self.referenced_tweets
            .as_ref()
            .and_then(|referenced_tweets| {
                let mut ids = referenced_tweets.iter().filter_map(|referenced_tweet| {
                    if referenced_tweet.reference_type == reference_type {
                        Some(referenced_tweet.id)
                    } else {
                        None
                    }
                });

                ids.next().map(|id| match ids.next() {
                    Some(multiple_id) => {
                        let mut bad_ids = vec![id, multiple_id];
                        bad_ids.extend(ids);

                        Err(FormatError::MultipleReferencedIds(bad_ids))
                    }
                    None => Ok(id),
                })
            })
            .map_or(Ok(None), |v| v.map(Some))
    }

    pub fn mention_ids(&self) -> Vec<u64> {
        self.entities
            .as_ref()
            .and_then(|entities| entities.mentions.as_ref())
            .map(|mentions| {
                mentions
                    .iter()
                    .filter_map(|mention| mention.id.and_then(|id| id.0))
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetSource {
    #[serde(rename = "Twitter")]
    Twitter,
    #[serde(rename = "Twitter for iPhone")]
    TwitterForIphone,
    #[serde(rename = "Twitter Web App")]
    TwitterWebApp,
    #[serde(rename = "Twitter Web Client")]
    TwitterWebClient,
    #[serde(rename = "Twitter for iPad")]
    TwitterForIpad,
    #[serde(rename = "Twitter for Android")]
    TwitterForAndroid,
    #[serde(rename = "Twitter for Mac")]
    TwitterForMac,
    #[serde(rename = "Twitter for Advertisers")]
    TwitterForAdvertisers,
    #[serde(rename = "TweetDeck")]
    TweetDeck,
    #[serde(rename = "Tweet From Twetch")]
    TweetFromTwetch,
    #[serde(rename = "Buffer")]
    Buffer,
    #[serde(rename = "SocialFlow")]
    SocialFlow,
    #[serde(rename = "Sprout Social")]
    SproutSocial,
    #[serde(rename = "illuminatibot")]
    Illuminatibot,
    #[serde(rename = "twittbot.net")]
    TwittbotNet,
    #[serde(rename = "drudge_rssfeed")]
    DrudgeRssFeed,
    #[serde(rename = "Hypefury")]
    Hypefury,
    #[serde(rename = "Hootsuite Inc.")]
    HootsuiteInc,
    #[serde(rename = "IFTTT")]
    Ifttt,
    #[serde(rename = "Jetpack.com")]
    JetpackDotCom,
    #[serde(rename = "Twitter Media Studio")]
    TwitterMediaStudio,
    #[serde(rename = "True Anthem")]
    TrueAnthem,
    #[serde(rename = "OneUp App")]
    OneUpApp,
    #[serde(rename = "Echobox")]
    Echobox,
    #[serde(rename = "SocialNewsDesk")]
    SocialNewsDesk,
    #[serde(rename = "The White House")]
    TheWhiteHouse,
    #[serde(rename = "BlackMagic.so")]
    BlackMagicSo,
    #[serde(rename = "Typefully")]
    Typefully,
    #[serde(rename = "Unfollowspy")]
    Unfollowspy,
    #[serde(rename = "TweetDeck Web App")]
    TweetDeckWebApp,
    #[serde(rename = "Tweetbot for iΟS")]
    TweetbotForIos,
    #[serde(rename = "Sprinklr")]
    Sprinklr,
    #[serde(rename = "Sprinklr Publishing")]
    SprinklrPublishing,
    #[serde(rename = "Oldhammer Bot")]
    OldhammerBot,
    //#[serde(rename = "BotFrens.co")]
    //BotFrensCo,
    #[serde(rename = "Twitter Media Studio - LiveCut")]
    TwitterMediaStudioLiveCut,
    #[serde(rename = "Echofon")]
    Echofon,
    #[serde(rename = "Twitter Ads")]
    TwitterAds,
    #[serde(rename = "FS Poster 3")]
    FSPoster3,
    #[serde(rename = "OnlyPultCom")]
    OnlyPultCom,
    #[serde(rename = "healthb0t")]
    Healthb0t,
    #[serde(rename = "rtlnieuws.nl prod web")]
    RtlnieuwsNlProdWeb,
    #[serde(rename = "Agorapulse app")]
    AgorapulseApp,
    #[serde(rename = "M3 Media")]
    M3Media,
    #[serde(rename = "Echo Chamber Buster")]
    EchoChamberBuster,
    #[serde(rename = "TweetCaster for Android")]
    TweetCasterForAndroid,
    #[serde(rename = "SmarterQueue")]
    SmarterQueue,
    #[serde(rename = "tokdrama")]
    Tokdrama,
    #[serde(rename = "Zapier.com")]
    ZapierCom,
    #[serde(rename = "Chatrecruit auto-tweets")]
    ChatrecruitAutoTweets,
    #[serde(rename = "Emplifi")]
    Emplifi,
    #[serde(rename = "Zero Hedge Publisher II")]
    ZeroHedgePublisherIi,
    #[serde(rename = "Rabois Likes")]
    RaboisLikes,
    #[serde(rename = "fllwrs")]
    Fllwrs,
    #[serde(rename = "Adobe Express")]
    AdobeExpress,
    #[serde(rename = "YBRP")]
    Ybrp,
    #[serde(rename = "Cheap Bots, Done Quick!")]
    CheapBotsDoneQuick,
    #[serde(rename = "Circleboom Publish")]
    CircleboomPublish,
    #[serde(rename = "BotFrens.com")]
    BotFrensCom,
    #[serde(rename = "Twitter for  iPhone")]
    TwitterForSpaceIphone,
    #[serde(rename = "Tweetbot for Mac")]
    TweetbotForMac,
    #[serde(rename = "Make (formerly Integromat)")]
    MakeFormerlyIntegromat,
    #[serde(rename = "libertyrecap")]
    Libertyrecap,
    #[serde(rename = "FS-Poster-App")]
    FsPosterApp,
    #[serde(rename = "MyKCM")]
    MyKcm,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextAnnotation<'a> {
    #[serde(borrow)]
    pub domain: ContextDomain<'a>,
    pub entity: ContextEntity<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextDomain<'a> {
    #[serde(with = "id_str")]
    pub id: u64,
    pub name: &'a str,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextEntity<'a> {
    #[serde(with = "id_str")]
    pub id: u64,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetIncludes<'a> {
    pub media: Option<Vec<Media<'a>>>,
    #[serde(borrow)]
    pub users: Vec<User<'a>>,
    pub tweets: Option<Vec<TweetData<'a>>>,
    pub polls: Option<Vec<Poll<'a>>>,
    pub places: Option<Vec<Place<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Poll<'a> {
    #[serde(with = "id_str")]
    pub id: u64,
    pub voting_status: PollVotingStatus,
    pub duration_minutes: usize,
    pub end_datetime: DateTime<Utc>,
    pub options: Vec<PollOption<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PollVotingStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PollOption<'a> {
    pub position: usize,
    pub label: Cow<'a, str>,
    pub votes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Place<'a> {
    #[serde(flatten, borrow)]
    pub metadata: PlaceMetadata<'a>,
    pub geo: PlaceGeo,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceGeo {
    #[serde(rename = "type")]
    pub geo_type: PlaceGeoType,
    pub properties: PlaceGeoProperties,
    // TODO: Fix this.
    pub bbox: Vec<serde_json::Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PlaceGeoType {
    Feature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceGeoProperties {}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Media<'a> {
    Url(MediaUrl<'a>),
    #[serde(borrow)]
    Variants(MediaVariants<'a>),
}

impl<'a> Media<'a> {
    pub fn metadata(&self) -> &MediaMetadata<'a> {
        match self {
            Self::Url(MediaUrl { metadata, .. }) => metadata,
            Self::Variants(MediaVariants { metadata, .. }) => metadata,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaMetadata<'a> {
    pub media_key: &'a str,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub public_metrics: Option<MediaPublicMetrics>,
    pub height: usize,
    pub width: usize,
    pub duration_ms: Option<usize>,
    pub preview_image_url: Option<Cow<'a, str>>,
    pub alt_text: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaUrl<'a> {
    #[serde(flatten, borrow)]
    pub metadata: MediaMetadata<'a>,
    pub url: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaVariants<'a> {
    #[serde(flatten)]
    pub metadata: MediaMetadata<'a>,
    #[serde(borrow)]
    pub variants: Vec<MediaVariant<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaPublicMetrics {
    pub view_count: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoteTweet<'a> {
    #[serde(borrow)]
    pub entities: Option<TweetEntities<'a>>,
    pub text: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub reference_type: ReferenceType,
    #[serde(with = "id_str")]
    pub id: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReferenceType {
    #[serde(rename = "retweeted")]
    Retweeted,
    #[serde(rename = "replied_to")]
    RepliedTo,
    #[serde(rename = "quoted")]
    Quoted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReplySettings {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "following")]
    Following,
    #[serde(rename = "mentionedUsers")]
    MentionedUsers,
    #[serde(rename = "subscribers")]
    Subscribers,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "id_str")]
    pub id: u64,
    pub username: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub description: Cow<'a, str>,
    pub location: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub profile_image_url: Cow<'a, str>,
    #[serde(with = "id_str_optional")]
    #[serde(default)]
    pub pinned_tweet_id: Option<u64>,
    #[serde(borrow)]
    pub entities: Option<UserEntities<'a>>,
    pub verified: bool,
    pub protected: bool,
    pub public_metrics: UserPublicMetrics,
    pub withheld: Option<Withheld>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Withheld {
    pub copyright: Option<bool>,
    pub country_codes: Vec<Country>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Article<'a> {
    pub title: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attachments<'a> {
    // TODO: use a structured representation for these keys (format: "3_1881163280061730816").
    #[serde(borrow)]
    pub media_keys: Option<Vec<&'a str>>,
    #[serde(with = "id_str_array_optional")]
    #[serde(default)]
    pub media_source_tweet_id: Option<Vec<u64>>,
    #[serde(with = "id_str_array_optional")]
    #[serde(default)]
    pub poll_ids: Option<Vec<u64>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Geo<'a> {
    pub place_id: Option<&'a str>,
    // TODO: Fix this.
    pub coordinates: Option<Coordinates>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Coordinates {
    #[serde(rename = "type")]
    pub coordinates_type: Option<CoordinatesType>,
    // TODO: Fix this.
    pub coordinates: Option<Vec<serde_json::Value>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CoordinatesType {
    Point,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetErrors<'a> {
    pub resource_id: &'a str,
    pub parameter: &'a str,
    pub resource_type: TweetErrorResourceType,
    pub section: Option<TweetErrorSection>,
    pub title: &'a str,
    pub value: &'a str,
    pub detail: &'a str,
    #[serde(rename = "type")]
    pub error_type: TweetErrorType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorResourceType {
    #[serde(rename = "tweet")]
    Tweet,
    #[serde(rename = "user")]
    User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorSection {
    #[serde(rename = "includes")]
    Includes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorType {
    #[serde(rename = "https://api.twitter.com/2/problems/not-authorized-for-resource")]
    NotAuthorizedForResource,
    #[serde(rename = "https://api.twitter.com/2/problems/resource-not-found")]
    ResourceNotFound,
}

#[cfg(test)]
mod tests {
    use super::Tweet;

    const TWEET_EXAMPLE_01: &str =
        include_str!("../../../../examples/wbm-data-1879337629495496982.json");
    const TWEET_EXAMPLE_02: &str =
        include_str!("../../../../examples/wbm-data-1637945937258754048.json");
    const TWEET_EXAMPLE_03: &str =
        include_str!("../../../../examples/wbm-data-1881187152257810817.json");
    const TWEET_EXAMPLE_04: &str =
        include_str!("../../../../examples/wbm-data-1857270748944994804.json");
    // TODO: redundant (passes with no changes)
    const TWEET_EXAMPLE_05: &str =
        include_str!("../../../../examples/wbm-data-1859432681898983820.json");
    const TWEET_EXAMPLE_06: &str =
        include_str!("../../../../examples/wbm-data-1865165548880888019.json");
    const TWEET_EXAMPLE_07: &str =
        include_str!("../../../../examples/wbm-data-1875993789892047207.json");
    const TWEET_EXAMPLE_08: &str =
        include_str!("../../../../examples/wbm-data-1813324959801901215.json");
    const TWEET_EXAMPLE_09: &str =
        include_str!("../../../../examples/wbm-data-1847247038661976161.json");
    const TWEET_EXAMPLE_10: &str =
        include_str!("../../../../examples/wbm-data-1857219243219021988.json");
    const TWEET_EXAMPLE_11: &str =
        include_str!("../../../../examples/wbm-data-1875609942314623471.json");
    const TWEET_EXAMPLE_12: &str =
        include_str!("../../../../examples/wbm-data-1848424834167865685.json");
    const TWEET_EXAMPLE_13: &str =
        include_str!("../../../../examples/wbm-data-1847476804564488484.json");
    const TWEET_EXAMPLE_14: &str =
        include_str!("../../../../examples/wbm-data-1801398574380445880.json");
    const TWEET_EXAMPLE_15: &str =
        include_str!("../../../../examples/wbm-data-1829223684940480956.json");
    const TWEET_EXAMPLE_16: &str =
        include_str!("../../../../examples/wbm-data-1847306394623754337.json");
    const TWEET_EXAMPLE_17: &str =
        include_str!("../../../../examples/wbm-data-1840582159381205076.json");
    const TWEET_EXAMPLE_18: &str =
        include_str!("../../../../examples/wbm-data-1798152929783816332.json");
    const TWEET_EXAMPLE_19: &str =
        include_str!("../../../../examples/wbm-data-1820641895174865381.json");
    const TWEET_EXAMPLE_20: &str =
        include_str!("../../../../examples/wbm-data-1845998964283527207.json");
    const TWEET_EXAMPLE_21: &str =
        include_str!("../../../../examples/wbm-data-1804245824467075206.json");
    const TWEET_EXAMPLE_22: &str =
        include_str!("../../../../examples/wbm-data-1821864680228274256.json");
    const TWEET_EXAMPLE_23: &str =
        include_str!("../../../../examples/wbm-data-1808937099589996745.json");
    const TWEET_EXAMPLE_24: &str =
        include_str!("../../../../examples/wbm-data-1814274947960709489.json");
    const TWEET_EXAMPLE_25: &str =
        include_str!("../../../../examples/wbm-data-1795974655615857059.json");
    const TWEET_EXAMPLE_26: &str =
        include_str!("../../../../examples/wbm-data-1604707978090663937.json");
    const TWEET_EXAMPLE_27: &str =
        include_str!("../../../../examples/wbm-data-1848088587947958523.json");
    const TWEET_EXAMPLE_28: &str =
        include_str!("../../../../examples/wbm-data-1796341860165963967.json");
    const TWEET_EXAMPLE_29: &str =
        include_str!("../../../../examples/wbm-data-1809434014178042096.json");
    const TWEET_EXAMPLE_30: &str =
        include_str!("../../../../examples/wbm-data-1833402059414245783.json");
    const TWEET_EXAMPLE_31: &str =
        include_str!("../../../../examples/wbm-data-1796552569180889293.json");
    const TWEET_EXAMPLE_32: &str =
        include_str!("../../../../examples/wbm-data-1850024819284152449.json");
    const TWEET_EXAMPLE_33: &str =
        include_str!("../../../../examples/wbm-data-1631317712193351681.json");
    const TWEET_EXAMPLE_34: &str =
        include_str!("../../../../examples/wbm-data-1820765571362824614.json");
    const TWEET_EXAMPLE_35: &str =
        include_str!("../../../../examples/wbm-data-1827105765011734994.json");
    const TWEET_EXAMPLE_36: &str =
        include_str!("../../../../examples/wbm-data-1852849270564495748.json");
    const TWEET_EXAMPLE_37: &str =
        include_str!("../../../../examples/wbm-data-1849454441830739991.json");
    const TWEET_EXAMPLE_38: &str =
        include_str!("../../../../examples/wbm-data-1869356652480327743.json");
    const TWEET_EXAMPLE_39: &str =
        include_str!("../../../../examples/wbm-data-1869198471598858378.json");

    #[test]
    fn parse_tweet_data_example_01() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_01).unwrap();

        assert_eq!(tweet.data.id, 1879337629495496982);
    }

    #[test]
    fn parse_tweet_data_example_02() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_02).unwrap();

        assert_eq!(tweet.data.id, 1637945937258754048);
    }

    #[test]
    fn parse_tweet_data_example_03() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_03).unwrap();

        assert_eq!(tweet.data.id, 1881187152257810817);
    }

    #[test]
    fn parse_tweet_data_example_04() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_04).unwrap();

        assert_eq!(tweet.data.id, 1857270748944994804);
    }

    #[test]
    fn parse_tweet_data_example_05() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_05).unwrap();

        assert_eq!(tweet.data.id, 1859432681898983820);
    }

    #[test]
    fn parse_tweet_data_example_06() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_06).unwrap();

        assert_eq!(tweet.data.id, 1865165548880888019);
    }

    #[test]
    fn parse_tweet_data_example_07() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_07).unwrap();

        assert_eq!(tweet.data.id, 1875993789892047207);
    }

    #[test]
    fn parse_tweet_data_example_08() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_08).unwrap();

        assert_eq!(tweet.data.id, 1813324959801901215);
    }

    #[test]
    fn parse_tweet_data_example_09() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_09).unwrap();

        assert_eq!(tweet.data.id, 1847247038661976161);
    }

    #[test]
    fn parse_tweet_data_example_10() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_10).unwrap();

        assert_eq!(tweet.data.id, 1857219243219021988);
    }

    #[test]
    fn parse_tweet_data_example_11() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_11).unwrap();

        assert_eq!(tweet.data.id, 1875609942314623471);
    }

    #[test]
    fn parse_tweet_data_example_12() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_12).unwrap();

        assert_eq!(tweet.data.id, 1848424834167865685);
    }

    #[test]
    fn parse_tweet_data_example_13() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_13).unwrap();

        assert_eq!(tweet.data.id, 1847476804564488484);
    }

    #[test]
    fn parse_tweet_data_example_14() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_14).unwrap();

        assert_eq!(tweet.data.id, 1801398574380445880);
    }

    #[test]
    fn parse_tweet_data_example_15() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_15).unwrap();

        assert_eq!(tweet.data.id, 1829223684940480956);
    }

    #[test]
    fn parse_tweet_data_example_16() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_16).unwrap();

        assert_eq!(tweet.data.id, 1847306394623754337);
    }

    #[test]
    fn parse_tweet_data_example_17() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_17).unwrap();

        assert_eq!(tweet.data.id, 1840582159381205076);
    }

    #[test]
    fn parse_tweet_data_example_18() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_18).unwrap();

        assert_eq!(tweet.data.id, 1798152929783816332);
    }

    #[test]
    fn parse_tweet_data_example_19() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_19).unwrap();

        assert_eq!(tweet.data.id, 1820641895174865381);
    }

    #[test]
    fn parse_tweet_data_example_20() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_20).unwrap();

        assert_eq!(tweet.data.id, 1845998964283527207);
    }

    #[test]
    fn parse_tweet_data_example_21() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_21).unwrap();

        assert_eq!(tweet.data.id, 1804245824467075206);
    }

    #[test]
    fn parse_tweet_data_example_22() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_22).unwrap();

        assert_eq!(tweet.data.id, 1821864680228274256);
    }

    #[test]
    fn parse_tweet_data_example_23() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_23).unwrap();

        assert_eq!(tweet.data.id, 1808937099589996745);
    }

    #[test]
    fn parse_tweet_data_example_24() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_24).unwrap();

        assert_eq!(tweet.data.id, 1814274947960709489);
    }

    #[test]
    fn parse_tweet_data_example_25() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_25).unwrap();

        assert_eq!(tweet.data.id, 1795974655615857059);
    }

    #[test]
    fn parse_tweet_data_example_26() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_26).unwrap();

        assert_eq!(tweet.data.id, 1604707978090663937);
    }

    #[test]
    fn parse_tweet_data_example_27() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_27).unwrap();

        assert_eq!(tweet.data.id, 1848088587947958523);
    }

    #[test]
    fn parse_tweet_data_example_28() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_28).unwrap();

        assert_eq!(tweet.data.id, 1796341860165963967);
    }

    #[test]
    fn parse_tweet_data_example_29() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_29).unwrap();

        assert_eq!(tweet.data.id, 1809434014178042096);
    }

    #[test]
    fn parse_tweet_data_example_30() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_30).unwrap();

        assert_eq!(tweet.data.id, 1833402059414245783);
    }

    #[test]
    fn parse_tweet_data_example_31() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_31).unwrap();

        assert_eq!(tweet.data.id, 1796552569180889293);
    }

    #[test]
    fn parse_tweet_data_example_32() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_32).unwrap();

        assert_eq!(tweet.data.id, 1850024819284152449);
    }

    #[test]
    fn parse_tweet_data_example_33() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_33).unwrap();

        assert_eq!(tweet.data.id, 1631317712193351681);
    }

    #[test]
    fn parse_tweet_data_example_34() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_34).unwrap();

        assert_eq!(tweet.data.id, 1820765571362824614);
    }

    #[test]
    fn parse_tweet_data_example_35() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_35).unwrap();

        assert_eq!(tweet.data.id, 1827105765011734994);
    }

    #[test]
    fn parse_tweet_data_example_36() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_36).unwrap();

        assert_eq!(tweet.data.id, 1852849270564495748);
    }

    #[test]
    fn parse_tweet_data_example_37() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_37).unwrap();

        assert_eq!(tweet.data.id, 1849454441830739991);
    }

    #[test]
    fn parse_tweet_data_example_38() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_38).unwrap();

        assert_eq!(tweet.data.id, 1869356652480327743);
    }

    #[test]
    fn parse_tweet_data_example_39() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_39).unwrap();

        assert_eq!(tweet.data.id, 1869198471598858378);
    }
}
