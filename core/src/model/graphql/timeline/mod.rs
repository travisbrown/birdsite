use crate::model::graphql::{
    ResultWrapper,
    ads::PromotedMetadata,
    image::{Image, OriginalImage},
    properties::{
        TombstoneInfo,
        display::{LabelDisplayType, PivotDisplayType, TombstoneDisplayType},
    },
    trends::TrendMetadata,
};
use std::borrow::Cow;

pub mod client;
pub mod context;
pub mod item;
pub mod trends;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleItem<'a, T, U> {
    #[serde(rename = "entryId")]
    pub entry_id: &'a str,
    pub dispensable: Option<bool>,
    pub item: Item<'a, T, U>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Item<'a, T, U> {
    #[serde(rename = "itemContent", borrow)]
    pub item_content: ItemContent<'a, T, U>,
    #[serde(rename = "clientEventInfo")]
    pub client_event_info: Option<client::event::ClientEventInfo<'a>>,
    #[serde(rename = "feedbackInfo")]
    pub feedback_info: Option<client::feedback::FeedbackInfo<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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
    Label {
        #[serde(rename = "__typename")]
        typename: &'a str,
        text: Cow<'a, str>,
        display_type: Option<LabelDisplayType>,
        url: Option<crate::model::url::Url<'a>>,
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
        social_context: Option<context::SocialContext<'a>>,
        is_ai_trend: Option<bool>,
        trend_url: crate::model::graphql::trends::TrendUrl<'a>,
        trend_metadata: TrendMetadata<'a>,
        grouped_trends: Option<Vec<trends::Trend<'a>>>,
        rank: Option<Cow<'a, str>>,
        thumbnail_image: Option<OriginalImage<'a>>,
        images: Option<Vec<trends::TrendImage<'a>>>,
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
