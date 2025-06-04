use crate::model::graphql::{
    ResultWrapper,
    ads::PromotedMetadata,
    image::{Image, OriginalImage},
    properties::{
        TombstoneInfo,
        context::SocialContext,
        display::{LabelDisplayType, PivotDisplayType, TombstoneDisplayType},
    },
    trends::TrendMetadata,
};
use std::borrow::Cow;

pub mod item;

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "itemType", deny_unknown_fields)]
pub enum ItemContent<'a, T, U> {
    #[serde(rename = "TimelineTimelineCursor")]
    Cursor {
        #[serde(rename = "__typename")]
        typename: &'a str,
        #[serde(flatten)]
        cursor: item::Cursor<'a>,
    },
    #[serde(rename = "TimelineUser")]
    User {
        #[serde(rename = "__typename")]
        typename: &'a str,
        #[serde(flatten)]
        user: item::User<'a, U>,
    },
    #[serde(rename = "TimelineTweet")]
    Tweet {
        #[serde(rename = "__typename")]
        typename: &'a str,
        #[serde(flatten)]
        tweet: item::Tweet<'a, T, U>,
    },
    #[serde(rename = "TimelineTombstone")]
    Tombstone {
        #[serde(rename = "__typename")]
        typename: &'a str,
        #[serde(rename = "tombstoneInfo")]
        tombstone_info: TombstoneInfo<'a>,
        #[serde(rename = "tombstoneDisplayType")]
        tombstone_display_type: Option<TombstoneDisplayType>,
    },
    #[serde(rename = "TimelineCommunity")]
    Community {
        #[serde(rename = "__typename")]
        typename: &'a str,
        community_results: ResultWrapper<crate::model::graphql::community::CommunityResult<'a, U>>,
    },
    #[serde(rename = "TimelinePivot")]
    Pivot {
        #[serde(rename = "__typename")]
        typename: &'a str,
        title_text: Cow<'a, str>,
        detail_text: Option<Cow<'a, str>>,
        destination_url: Cow<'a, str>,
        pivot_display_type: PivotDisplayType,
        image: Image<'a>,
        detail_text_image: Option<Image<'a>>,
    },
    #[serde(rename = "TimelineLabel")]
    // TODO: Add URL
    Label {
        #[serde(rename = "__typename")]
        typename: &'a str,
        text: Cow<'a, str>,
        display_type: Option<LabelDisplayType>,
        url: Option<crate::model::entity::Url<'a>>,
    },
    #[serde(rename = "TimelineEventSummary")]
    // TODO: Support access for possible media tweets
    EventSummary,
    #[serde(rename = "TimelineTrend")]
    Trend {
        #[serde(rename = "__typename")]
        typename: &'a str,
        name: Cow<'a, str>,
        description: Option<Cow<'a, str>>,
        social_context: Option<SocialContext<'a>>,
        is_ai_trend: Option<bool>,
        trend_url: crate::model::entity::Url<'a>,
        trend_metadata: TrendMetadata<'a>,
        // TODO: Consider extracting these.
        grouped_trends: Option<serde::de::IgnoredAny>,
        rank: Option<Cow<'a, str>>,
        thumbnail_image: Option<OriginalImage<'a>>,
        /// TODO: The elements here are both string and objects, need to decide how to handle this.
        images: Option<Vec<serde::de::IgnoredAny>>,
        promoted_metadata: Option<PromotedMetadata<'a, U>>,
        associated_cards: Option<Vec<()>>,
    },
    #[serde(rename = "TimelinePrompt")]
    Prompt,
    #[serde(rename = "TimelineTopicFollowPrompt")]
    TopicFollowPrompt,
    #[serde(rename = "TimelineMessagePrompt")]
    MessagePrompt,
    #[serde(rename = "TimelineSpelling")]
    Spelling,
    #[serde(rename = "TimelineTwitterList")]
    // TODO: Support access for user results
    TwitterList,
    #[serde(rename = "TimelineScoreEventCard")]
    // TODO: Support access for user results
    ScoreEventCard,
    #[serde(rename = "TimelineRecruitingOrganization")]
    // TODO: Support access for user results
    RecruitingOrganization,
    #[serde(rename = "TimelineTile")]
    // TODO: Support access for user results
    Tile,
    #[serde(rename = "TimelineFrame")]
    Frame,
}
