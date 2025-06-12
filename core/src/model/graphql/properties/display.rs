use std::borrow::Cow;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum LabelDisplayType {
    InlineHeader,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ModuleDisplayType {
    Carousel,
    Vertical,
    VerticalConversation,
    VerticalGrid,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum PivotDisplayType {
    Fill,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TombstoneDisplayType {
    Inline,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetDisplayType {
    CondensedTweet,
    EmphasizedPromotedTweet,
    Media,
    MediaGrid,
    SelfThread,
    Tweet,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UserDisplayType {
    SubscribableUser,
    User,
    UserDetailed,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayTreatment<'a> {
    #[serde(rename = "actionText", borrow)]
    pub action_text: Cow<'a, str>,
    #[serde(rename = "labelText", borrow)]
    pub label_text: Option<Cow<'a, str>>,
}
