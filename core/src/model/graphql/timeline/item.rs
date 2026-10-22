use std::borrow::Cow;

use archivindex_serde::borrowable_str;

use crate::model::graphql::ResultWrapper;
use crate::model::graphql::ads::{PrerollMetadata, PromotedMetadata};
use crate::model::graphql::properties::display::{
    DisplayTreatment, TweetDisplayType, UserDisplayType,
};
use crate::model::graphql::properties::{ConversationAnnotation, CursorType};
use crate::model::graphql::timeline::context::{ForwardPivot, SocialContext};
use crate::model::graphql::user::UserResult;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cursor<'a> {
    /// Present on timeline entry cursors but not on item content cursors.
    #[serde(rename = "__typename", default, with = "borrowable_str::option")]
    pub typename: Option<Cow<'a, str>>,
    #[serde(rename = "cursorType")]
    pub cursor_type: CursorType,
    #[serde(borrow)]
    pub value: Cow<'a, str>,
    #[serde(rename = "stopOnEmptyResponse", default)]
    pub stop_on_empty_response: Option<bool>,
    #[serde(rename = "displayTreatment")]
    pub display_treatment: Option<DisplayTreatment<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a, U> {
    #[serde(borrow)]
    pub user_results: ResultWrapper<UserResult<'a, U>>,
    #[serde(rename = "userDisplayType")]
    pub user_display_type: UserDisplayType,
    #[serde(rename = "socialContext")]
    pub social_context: Option<SocialContext<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tweet<'a, T, U> {
    pub tweet_results: ResultWrapper<T>,
    #[serde(rename = "tweetDisplayType")]
    pub tweet_display_type: TweetDisplayType,
    #[serde(rename = "hasModeratedReplies")]
    pub has_moderated_replies: Option<bool>,
    pub conversation_annotation: Option<ConversationAnnotation>,
    #[serde(rename = "socialContext")]
    pub social_context: Option<SocialContext<'a>>,
    #[serde(rename = "promotedMetadata", borrow)]
    pub promoted_metadata: Option<PromotedMetadata<'a, U>>,
    #[serde(rename = "prerollMetadata")]
    pub preroll_metadata: Option<PrerollMetadata<'a>>,
    pub highlights: Option<TweetHighlights>,
    #[serde(rename = "forwardPivot")]
    pub forward_pivot: Option<ForwardPivot<'a>>,
    #[serde(rename = "tweetContext")]
    pub tweet_context: Option<TweetContext<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetHighlights {
    #[serde(rename = "textHighlights")]
    pub text_highlights: Vec<TextHighlight>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextHighlight {
    #[serde(rename = "startIndex")]
    pub start_index: usize,
    #[serde(rename = "endIndex")]
    pub end_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetContext<'a> {
    /// Should always be `"TimelineTweetContext"`.
    #[serde(rename = "type", borrow)]
    pub tweet_context_type: Cow<'a, str>,
    #[serde(rename = "tweetContext")]
    pub tweet_context: crate::model::graphql::timeline::context::TweetContext<'a>,
}
