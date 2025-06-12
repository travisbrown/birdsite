use crate::model::KeyValuePair;
use std::borrow::Cow;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DisclosureType {
    Issue,
    NoDisclosure,
    Political,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotedMetadata<'a, U> {
    pub advertiser_results: AdvertiserResults<U>,
    #[serde(rename = "adMetadataContainer")]
    pub ad_metadata_container: Option<AdMetadataContainer<'a>>,
    #[serde(rename = "disclosureType")]
    pub disclosure_type: DisclosureType,
    #[serde(rename = "experimentValues")]
    pub experiment_values: Option<Vec<KeyValuePair<'a>>>,
    #[serde(rename = "impressionId")]
    pub impression_id: &'a str,
    #[serde(rename = "impressionString")]
    pub impression_string: &'a str,
    #[serde(rename = "promotedTrendName")]
    pub promoted_trend_name: Option<Cow<'a, str>>,
    #[serde(rename = "promotedTrendQueryTerm")]
    pub promoted_trend_query_term: Option<Cow<'a, str>>,
    #[serde(rename = "promotedTrendDescription")]
    pub promoted_trend_description: Option<Cow<'a, str>>,
    #[serde(rename = "promotedTrend")]
    pub promoted_trend: Option<PromotedTrend>,
    #[serde(rename = "clickTrackingInfo")]
    pub click_tracking_info: Option<ClickTrackingInfo<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdvertiserResults<U> {
    pub result: Option<U>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdMetadataContainer<'a> {
    #[serde(rename = "renderLegacyWebsiteCard")]
    pub render_legacy_website_card: Option<bool>,
    #[serde(rename = "isQuickPromote")]
    pub is_quick_promote: Option<bool>,
    pub remove_promoted_attribution_for_preroll: Option<bool>,
    #[serde(rename = "unifiedCardOverride")]
    pub unified_card_override: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromotedTrend {
    #[serde(with = "crate::model::attributes::integer_str")]
    pub rest_id: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DynamicPrerollType {
    Amplify,
    Marketplace,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrerollMetadata<'a> {
    pub preroll: Preroll<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Preroll<'a> {
    #[serde(rename = "prerollId", with = "crate::model::attributes::integer_str")]
    pub preroll_id: u64,
    #[serde(rename = "dynamicPrerollType")]
    pub dynamic_preroll_type: DynamicPrerollType,
    #[serde(rename = "mediaInfo")]
    pub media_info: MediaInfo<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MediaInfo<'a> {
    #[serde(rename = "advertiserName")]
    pub advertiser_name: Cow<'a, str>,
    #[serde(rename = "advertiserProfileImageUrl")]
    pub advertiser_profile_image_url: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClickTrackingInfo<'a> {
    #[serde(rename = "urlParams", borrow)]
    pub url_params: Vec<KeyValuePair<'a>>,
    #[serde(rename = "urlOverride")]
    pub url_override: Option<&'a str>,
    #[serde(rename = "urlOverrideType")]
    pub url_override_type: Option<UrlOverrideType>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UrlOverrideType {
    Dcm,
}
