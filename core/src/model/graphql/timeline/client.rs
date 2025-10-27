#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum Action {
    #[serde(rename = "click")]
    Click,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Component<'a> {
    #[serde(rename = "suggest_who_to_follow")]
    SuggestWhoToFollow,
    #[serde(rename = "trends")]
    Trends,
    #[serde(rename = "tweet")]
    Tweet,
    #[serde(rename = "unified_events")]
    UnifiedEvents,
    Other(&'a str),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Element<'a> {
    #[serde(rename = "feedback")]
    Feedback,
    #[serde(rename = "trend")]
    Trend,
    Other(&'a str),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationDetails {
    #[serde(rename = "conversationSection")]
    pub conversation_section: ConversationSection,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ConversationSection {
    AbusiveQuality,
    HighQuality,
    LowQuality,
    RelatedTweet,
}

pub mod event {
    use serde_field_attributes::{integer_str, optional_integer_str};
    use std::borrow::Cow;

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct ClientEventInfo<'a> {
        #[serde(borrow)]
        pub component: Option<super::Component<'a>>,
        pub element: Option<super::Element<'a>>,
        pub details: Option<Details<'a>>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Details<'a> {
        #[serde(rename = "conversationDetails")]
        pub conversation_details: Option<super::ConversationDetails>,
        #[serde(rename = "timelinesDetails", borrow)]
        pub timelines_details: Option<TimelinesDetails<'a>>,
        #[serde(rename = "guideDetails")]
        pub guide_details: Option<GuideDetails<'a>>,
        pub ai_trend_details: Option<AiTrendDetails>,
        // TODO: Fill this is.
        pub live_event_details: Option<serde_json::Value>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TimelinesDetails<'a> {
        #[serde(rename = "injectionType")]
        pub injection_type: Option<InjectionType<'a>>,
        #[serde(rename = "controllerData")]
        pub controller_data: Option<&'a str>,
        #[serde(rename = "sourceData")]
        pub source_data: Option<&'a str>,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(untagged)]
    pub enum InjectionType<'a> {
        CommunityTweet,
        ForYouLocal,
        ForYouPopularGeo,
        ForYouPromoted,
        ForYouSimclusters,
        ForYouTrends,
        OrganicListTweet,
        RankedOrganicTweet,
        WhoToFollow,
        WhoToSubscribe,
        Other(&'a str),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct GuideDetails<'a> {
        pub identifier: &'a str,
        pub token: Option<&'a str>,
        #[serde(rename = "transparentGuideDetails")]
        pub transparent_guide_details: Option<TransparentGuideDetails<'a>>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(tag = "type", deny_unknown_fields)]
    pub enum TransparentGuideDetails<'a> {
        TimelineEventUrtMetadata {
            #[serde(rename = "impressionId")]
            impression_id: &'a str,
            position: usize,
            #[serde(rename = "sourceId", with = "optional_integer_str", default)]
            source_id: Option<u64>,
            #[serde(rename = "eventId", with = "integer_str")]
            event_id: u64,
        },
        TimelineTrendUrtMetadata {
            /// This can be negative (for example, `-8530644708937920429`).
            #[serde(rename = "impressionId", with = "integer_str")]
            impression_id: i64,
            #[serde(rename = "impressionToken")]
            impression_token: &'a str,
            position: usize,
            #[serde(rename = "trendName")]
            trend_name: &'a str,
            #[serde(rename = "relatedTerms")]
            related_terms: Option<Cow<'a, [Cow<'a, str>]>>,
            #[serde(rename = "clusterId", with = "optional_integer_str", default)]
            cluster_id: Option<u64>,
        },
        TimelineSemanticCoreInterest {
            #[serde(rename = "domainId", with = "integer_str")]
            domain_id: u64,
            #[serde(rename = "entityId", with = "integer_str")]
            entity_id: u64,
        },
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct AiTrendDetails {
        #[serde(with = "integer_str")]
        pub trend_id: u64,
    }
}

pub mod feedback {
    use std::borrow::Cow;

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct FeedbackInfo<'a> {
        #[serde(rename = "clientEventInfo", borrow)]
        pub client_event_info: Option<ClientEventInfo<'a>>,
        #[serde(rename = "feedbackKeys")]
        pub feedback_keys: Cow<'a, [&'a str]>,
        #[serde(rename = "feedbackMetadata")]
        pub feedback_metadata: Option<&'a str>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct ClientEventInfo<'a> {
        pub action: Option<super::Action>,
        #[serde(borrow)]
        pub component: Option<super::Component<'a>>,
        pub element: Option<super::Element<'a>>,
        pub details: Option<Details>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Details {
        #[serde(rename = "conversationDetails")]
        pub conversation_details: Option<super::ConversationDetails>,
    }
}
