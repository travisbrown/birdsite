use std::borrow::Cow;

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SocialContextType {
    TimelineGeneralContext,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SocialContextContextType {
    Community,
    Conversation,
    Facepile,
    Follow,
    Like,
    Location,
    Pin,
    TextOnly,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SocialContext<'a> {
    #[serde(rename = "type")]
    pub social_context_type: SocialContextType,
    #[serde(rename = "contextType")]
    pub context_type: SocialContextContextType,
    pub text: Cow<'a, str>,
    #[serde(rename = "landingUrl")]
    pub landing_url: Option<crate::model::entity::Url<'a>>,
    #[serde(rename = "contextImageUrls")]
    pub context_image_urls: Option<Vec<Cow<'a, str>>>,
}
