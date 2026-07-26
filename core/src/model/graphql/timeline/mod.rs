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

/// A timeline response instruction.
///
/// This is the outermost layer of every timeline response: a list of instructions that add,
/// replace, or pin the entries that carry tweets and users. The type parameters are the tweet
/// result and user representations, as for [`ModuleItem`].
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Instruction<'a, T, U> {
    #[serde(rename = "TimelineClearCache")]
    ClearCache,
    #[serde(rename = "TimelineTerminateTimeline")]
    TerminateTimeline {
        direction: crate::model::graphql::properties::TimelineDirection,
    },
    #[serde(rename = "TimelineAddEntries")]
    AddEntries {
        #[serde(borrow)]
        entries: Vec<Entry<'a, T, U>>,
    },
    #[serde(rename = "TimelinePinEntry")]
    PinEntry {
        #[serde(borrow)]
        entry: Entry<'a, T, U>,
    },
    #[serde(rename = "TimelineReplaceEntry")]
    ReplaceEntry {
        entry_id_to_replace: EntryId<'a>,
        #[serde(borrow)]
        entry: Entry<'a, T, U>,
    },
    #[serde(rename = "TimelineAddToModule")]
    AddToModule {
        #[serde(rename = "moduleEntryId")]
        module_entry_id: EntryId<'a>,
        #[serde(rename = "moduleItems", borrow)]
        module_items: Vec<ModuleItem<'a, T, U>>,
        prepend: Option<bool>,
    },
    #[serde(rename = "TimelineShowCover")]
    ShowCover,
    #[serde(rename = "TimelineShowAlert")]
    ShowAlert,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct EntryId<'a>(#[serde(borrow)] pub Cow<'a, str>);

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Entry<'a, T, U> {
    #[serde(rename = "entryId")]
    pub entry_id: EntryId<'a>,
    #[serde(rename = "sortIndex", with = "serde_field_attributes::integer_str")]
    pub sort_index: u64,
    #[serde(borrow)]
    pub content: EntryContent<'a, T, U>,
}

// Boxing the module and item payloads would complicate destructuring for little benefit, since
// cursor entries are rare relative to content entries.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "entryType", deny_unknown_fields)]
pub enum EntryContent<'a, T, U> {
    #[serde(rename = "TimelineTimelineCursor")]
    Cursor(#[serde(borrow)] item::Cursor<'a>),
    #[serde(rename = "TimelineTimelineModule")]
    Module {
        #[serde(rename = "__typename")]
        typename: &'a str,
        #[serde(borrow)]
        items: Vec<ModuleItem<'a, T, U>>,
        metadata: Option<serde_json::Value>,
        #[serde(rename = "displayType")]
        display_type: crate::model::graphql::properties::display::ModuleDisplayType,
        header: Option<EntryContentHeader<'a>>,
        footer: Option<EntryContentFooter<'a>>,
        #[serde(rename = "clientEventInfo")]
        client_event_info: Option<client::event::ClientEventInfo<'a>>,
        #[serde(rename = "feedbackInfo")]
        feedback_info: Option<client::feedback::FeedbackInfo<'a>>,
    },
    #[serde(rename = "TimelineTimelineItem")]
    Item {
        #[serde(rename = "__typename")]
        typename: &'a str,
        #[serde(rename = "itemContent")]
        item_content: ItemContent<'a, T, U>,
        #[serde(rename = "clientEventInfo")]
        client_event_info: Option<client::event::ClientEventInfo<'a>>,
        #[serde(rename = "feedbackInfo")]
        feedback_info: Option<client::feedback::FeedbackInfo<'a>>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EntryContentHeader<'a> {
    #[serde(rename = "displayType")]
    pub display_type: EntryContentHeaderDisplayType,
    pub text: &'a str,
    #[serde(rename = "socialContext")]
    pub social_context: Option<context::SocialContext<'a>>,
    #[serde(rename = "landingUrl")]
    pub landing_url: Option<crate::model::url::Url<'a>>,
    pub icon: Option<EntryContentHeaderIcon>,
    pub sticky: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EntryContentFooter<'a> {
    #[serde(rename = "displayType")]
    pub display_type: EntryContentFooterDisplayType,
    pub text: &'a str,
    #[serde(rename = "landingUrl")]
    pub landing_url: Option<crate::model::url::Url<'a>>,
    pub url: Option<&'a str>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum EntryContentHeaderDisplayType {
    Classic,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum EntryContentHeaderIcon {
    TopicFilled,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum EntryContentFooterDisplayType {
    Classic,
}

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
        #[serde(borrow)]
        title_text: Cow<'a, str>,
        #[serde(borrow)]
        detail_text: Option<Cow<'a, str>>,
        #[serde(borrow)]
        destination_url: Cow<'a, str>,
        pivot_display_type: PivotDisplayType,
        image: Image<'a>,
        detail_text_image: Option<Image<'a>>,
    },
    #[serde(rename = "TimelineLabel")]
    Label {
        #[serde(rename = "__typename")]
        typename: &'a str,
        #[serde(borrow)]
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
        #[serde(borrow)]
        name: Cow<'a, str>,
        #[serde(borrow)]
        description: Option<Cow<'a, str>>,
        social_context: Option<context::SocialContext<'a>>,
        is_ai_trend: Option<bool>,
        trend_url: crate::model::url::Url<'a>,
        trend_metadata: TrendMetadata<'a>,
        grouped_trends: Option<Vec<trends::Trend<'a>>>,
        #[serde(borrow)]
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
