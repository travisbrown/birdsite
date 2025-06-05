use std::borrow::Cow;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SocialContextType {
    TimelineGeneralContext,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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
    pub landing_url: Option<crate::model::url::Url<'a>>,
    #[serde(rename = "contextImageUrls")]
    pub context_image_urls: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ForwardPivot<'a> {
    #[serde(borrow)]
    pub text: crate::model::graphql::text::Text<'a>,
    #[serde(rename = "landingUrl")]
    pub landing_url: crate::model::url::Url<'a>,
    #[serde(rename = "displayType")]
    pub display_type: ForwardPivotDisplayType,
    #[serde(rename = "engagementNudge")]
    pub engagement_nudge: bool,
    #[serde(rename = "iconImageVariant")]
    pub icon_image_variant: crate::model::graphql::image::Image<'a>,
    #[serde(rename = "softInterventionDisplayType")]
    pub soft_intervention_display_type: SoftInterventionDisplayType,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ForwardPivotDisplayType {
    SoftIntervention,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SoftInterventionDisplayType {
    StayInformed,
}
