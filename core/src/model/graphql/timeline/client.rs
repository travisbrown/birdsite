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

pub mod event {
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
        pub conversation_details: Option<ConversationDetails>,
        #[serde(rename = "timelinesDetails", borrow)]
        pub timelines_details: Option<TimelinesDetails<'a>>,
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
        pub action: super::Action,
        #[serde(borrow)]
        pub component: super::Component<'a>,
        pub element: super::Element<'a>,
    }
}
