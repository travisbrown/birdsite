use bounded_static_derive_more::ToStatic;
use serde_field_attributes::integer_str;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum VerifiedType {
    Business,
    Government,
    User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TranslatorType {
    None,
    Regular,
    Badged,
    Moderator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProfileImageShape {
    Circle,
    Square,
    Hexagon,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ParodyCommentaryFanLabel {
    None,
    Parody,
    Commentary,
    Fan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct HighlightsInfo {
    pub can_highlight_tweets: bool,
    #[serde(with = "integer_str")]
    pub highlighted_tweets: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BusinessAccount {
    pub affiliates_count: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProfileInterstitialType {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "fake_account")]
    FakeAccount,
    #[serde(rename = "offensive_profile_content")]
    OffensiveProfileContent,
    #[serde(rename = "sensitive_media")]
    SensitiveMedia,
    #[serde(rename = "timeout")]
    Timeout,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TipjarSettings<'a> {
    pub is_enabled: Option<bool>,
    pub bandcamp_handle: Option<Cow<'a, str>>,
    pub bitcoin_handle: Option<Cow<'a, str>>,
    pub cash_app_handle: Option<Cow<'a, str>>,
    pub ethereum_handle: Option<Cow<'a, str>>,
    pub gofundme_handle: Option<Cow<'a, str>>,
    pub patreon_handle: Option<Cow<'a, str>>,
    pub pay_pal_handle: Option<Cow<'a, str>>,
    pub venmo_handle: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic)]
pub struct Professional<'a> {
    pub id: u64,
    pub professional_type: ProfessionalType,
    pub category: Option<ProfessionalCategory<'a>>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for Professional<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal_professional = internal::Professional::deserialize(deserializer)?;

        let category = internal_professional.category.map_or_else(
            || Ok(None),
            |mut category| {
                // Have seen at least one non-error case where the category is repeated.
                // In the non-exceptional case there's only one, so this is cheap.
                category.dedup();

                let first_category = category.pop();

                if category.is_empty() {
                    Ok(first_category)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Seq,
                        &"single unique category",
                    ))
                }
            },
        )?;

        Ok(Self {
            id: internal_professional.rest_id,
            professional_type: internal_professional.professional_type,
            category,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProfessionalType {
    Business,
    Creator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProfessionalCategoryIconName {
    IconBriefcaseStroke,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfessionalCategory<'a> {
    pub id: u64,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub icon_name: ProfessionalCategoryIconName,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum Visibility {
    Public,
    Following,
    Followers,
    MutualFollow,
    #[serde(rename = "Self")]
    SelfVisible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Birthdate {
    pub year: Option<usize>,
    pub month: Option<usize>,
    pub day: Option<usize>,
    pub visibility: Visibility,
    pub year_visibility: Visibility,
}

mod internal {
    use serde_field_attributes::integer_str;

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Professional<'a> {
        #[serde(with = "integer_str")]
        pub rest_id: u64,
        pub professional_type: super::ProfessionalType,
        #[serde(borrow)]
        pub category: Option<Vec<super::ProfessionalCategory<'a>>>,
    }
}
