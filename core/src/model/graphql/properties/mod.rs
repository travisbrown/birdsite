use std::borrow::Cow;

pub mod display;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TimelineDirection {
    Top,
    Bottom,
    TopAndBottom,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CursorType {
    Bottom,
    ShowMore,
    ShowMoreThreads,
    ShowMoreThreadsPrompt,
    Top,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ConversationAnnotationType {
    Political,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationAnnotation {
    pub conversation_annotation_type: ConversationAnnotationType,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TombstoneInfo<'a> {
    text: Cow<'a, str>,
    #[serde(rename = "richText", borrow)]
    rich_text: crate::model::graphql::text::Text<'a>,
}
