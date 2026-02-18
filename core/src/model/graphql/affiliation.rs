use crate::model::graphql::unavailable::UserUnavailableReason;
use regex::Regex;
use serde::de::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::sync::LazyLock;

static AFFILIATE_URL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^https://twitter.com/(\w+)$").unwrap());

#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub enum AffiliationResult<'a> {
    Active(Affiliation<'a>),
    /// In some cases there is no `long_description` field, so we do not know the operator's user
    /// ID or screen name.
    Automated,
    Empty,
}

impl<'a> AffiliationResult<'a> {
    pub fn affilation(&self) -> Option<&Affiliation<'a>> {
        match self {
            Self::Active(affiliation) => Some(affiliation),
            Self::Automated | Self::Empty => None,
        }
    }
}

impl<'a> From<AffiliationResult<'a>> for Option<Affiliation<'a>> {
    fn from(value: AffiliationResult<'a>) -> Self {
        match value {
            AffiliationResult::Active(affiliation) => Some(affiliation),
            AffiliationResult::Automated | AffiliationResult::Empty => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct Affiliation<'a> {
    pub screen_name: Cow<'a, str>,
    pub state: AffiliationStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AffiliationStatus {
    Automated {
        id: Option<u64>,
    },
    AutomatedUnavailable {
        reason: Option<UserUnavailableReason>,
    },
    Business,
}

impl<'a> Affiliation<'a> {
    pub fn automated(screen_name: &'a str, id: u64) -> Self {
        Self {
            screen_name: screen_name.into(),
            state: AffiliationStatus::Automated { id: Some(id) },
        }
    }

    pub fn automated_unavailable(
        screen_name: &'a str,
        unavailable_reason: UserUnavailableReason,
    ) -> Self {
        Self {
            screen_name: screen_name.into(),
            state: AffiliationStatus::AutomatedUnavailable {
                reason: Some(unavailable_reason),
            },
        }
    }

    pub fn business(screen_name: &'a str) -> Self {
        Self {
            screen_name: screen_name.into(),
            state: AffiliationStatus::Business,
        }
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for AffiliationResult<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let label = internal::Affiliation::deserialize(deserializer)?;

        match label.label {
            Some(internal::AffiliationLabel::AutomatedLabel(internal::AutomatedLabel {
                long_description:
                    Some(internal::automated::AutomatedLabelLongDescription {
                        entities:
                            internal::automated::AutomatedLabelEntities((
                                internal::automated::AutomatedLabelEntity {
                                    reference:
                                        internal::automated::AutomatedLabelRef {
                                            screen_name,
                                            user_results:
                                                internal::automated::UserResults { result, .. },
                                            ..
                                        },
                                    ..
                                },
                            )),
                        ..
                    }),
                ..
            })) => match result {
                Some(internal::automated::AutomatedLabelUserResult::User { rest_id, .. }) => {
                    Ok(Self::Active(Affiliation {
                        screen_name,
                        state: AffiliationStatus::Automated { id: Some(rest_id) },
                    }))
                }
                Some(internal::automated::AutomatedLabelUserResult::UserUnavailable {
                    reason,
                    ..
                }) => Ok(Self::Active(Affiliation {
                    screen_name,
                    state: AffiliationStatus::AutomatedUnavailable { reason },
                })),
                None => Ok(Self::Active(Affiliation {
                    screen_name,
                    state: AffiliationStatus::Automated { id: None },
                })),
            },
            Some(internal::AffiliationLabel::AutomatedLabel(internal::AutomatedLabel {
                long_description: None,
                ..
            })) => Ok(Self::Automated),
            Some(internal::AffiliationLabel::BusinessLabel(internal::BusinessLabel {
                url,
                ..
            })) => {
                if AFFILIATE_URL_PATTERN.is_match(&url.url) {
                    Ok(Self::Active(Affiliation {
                        screen_name: match url.url {
                            Cow::Borrowed(url) => Cow::Borrowed(&url[20..]),
                            Cow::Owned(url) => Cow::Owned(url[20..].to_string()),
                        },
                        state: AffiliationStatus::Business,
                    }))
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(&url.url),
                        &"profile URL",
                    ))
                }
            }
            None => Ok(Self::Empty),
        }
    }
}

mod internal {
    use crate::model::url::Url;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Affiliation<'a> {
        #[serde(borrow)]
        pub label: Option<AffiliationLabel<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(tag = "userLabelType", deny_unknown_fields)]
    pub enum AffiliationLabel<'a> {
        #[serde(borrow)]
        BusinessLabel(BusinessLabel<'a>),
        AutomatedLabel(AutomatedLabel<'a>),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    struct Badge<'a> {
        url: Cow<'a, str>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BusinessLabel<'a> {
        pub url: Url<'a>,
        #[serde(rename = "description")]
        _description: Cow<'a, str>,
        #[serde(rename = "badge")]
        _badge: Badge<'a>,
        #[serde(rename = "userLabelDisplayType")]
        _user_label_display_type: business::BusinessLabelDisplayType,
        // TODO: Confirm that this never contains useful information.
        #[serde(rename = "auxiliary_user_labels")]
        _auxiliary_user_labels: Option<serde::de::IgnoredAny>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct AutomatedLabel<'a> {
        #[serde(borrow)]
        //#[serde(rename = "longDescription")]
        pub long_description: Option<automated::AutomatedLabelLongDescription<'a>>,
        /// This will generally be `"Automated"`, but may be localized.
        #[serde(rename = "description")]
        _description: Cow<'a, str>,
        #[serde(rename = "badge")]
        _badge: Badge<'a>,
    }

    mod business {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum BusinessLabelDisplayType {
            Badge,
        }
    }

    pub mod automated {
        use serde_field_attributes::integer_str;
        use std::borrow::Cow;

        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct AutomatedLabelLongDescription<'a> {
            #[serde(rename = "text")]
            _text: Cow<'a, str>,
            #[serde(borrow)]
            pub entities: AutomatedLabelEntities<'a>,
        }

        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct AutomatedLabelEntities<'a>(#[serde(borrow)] pub (AutomatedLabelEntity<'a>,));

        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct AutomatedLabelEntity<'a> {
            //#[serde(rename = "fromIndex")]
            from_index: usize,
            //#[serde(rename = "toIndex")]
            to_index: usize,
            #[serde(rename = "ref", borrow)]
            pub reference: AutomatedLabelRef<'a>,
        }

        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct AutomatedLabelRef<'a> {
            // Different combinations of type fields are used at different times, but they should
            // all have the same value.
            #[serde(rename = "type")]
            pub _type: Option<AutomatedLabelRefType>,
            #[serde(rename = "__typename")]
            pub _typename: Option<AutomatedLabelRefType>,
            #[serde(rename = "__isTimelineReferenceObject")]
            pub _is_timeline_reference_object: Option<AutomatedLabelRefType>,
            #[serde(borrow)]
            pub screen_name: Cow<'a, str>,
            // In older responses we find `mention_results`.
            #[serde(alias = "mention_results")]
            pub user_results: UserResults<'a>,
        }

        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct UserResults<'a> {
            #[serde(borrow)]
            pub result: Option<AutomatedLabelUserResult<'a>>,
            #[serde(rename = "id")]
            _id: Option<Cow<'a, str>>,
        }

        #[derive(serde::Deserialize)]
        pub enum AutomatedLabelRefType {
            TimelineRichTextMention,
        }

        #[derive(serde::Deserialize)]
        #[serde(tag = "__typename", deny_unknown_fields)]
        pub enum AutomatedLabelUserResult<'a> {
            User {
                #[serde(with = "integer_str")]
                rest_id: u64,
                #[serde(rename = "legacy", alias = "core", borrow)]
                _legacy: Option<ScreenNameUserResultLegacy<'a>>,
                // Not present in older responses.
                #[serde(rename = "id")]
                _id: Option<Cow<'a, str>>,
            },
            UserUnavailable {
                reason: Option<crate::model::graphql::unavailable::UserUnavailableReason>,
                #[serde(rename = "message")]
                _message: Option<Cow<'a, str>>,
            },
        }

        #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(deny_unknown_fields)]
        pub struct ScreenNameUserResultLegacy<'a> {
            pub screen_name: &'a str,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::graphql::unavailable::UserUnavailableReason;

    const AUTOMATED: &str = r#"{"label":{"badge":{"url":"https://pbs.twimg.com/semantic_core_img/1428827730364096519/4ZXpTBhS?format=png&name=orig"},"description":"Automated","longDescription":{"text":"Automated by @2jacksArt","entities":[{"fromIndex":13,"toIndex":23,"ref":{"type":"TimelineRichTextMention","screen_name":"2jacksArt","mention_results":{"result":{"__typename":"User","legacy":{"screen_name":"2jacksArt"},"rest_id":"3313940136"}}}}]},"userLabelType":"AutomatedLabel"}}"#;
    const AUTOMATED_UNAVAILABLE: &str = r#"{"label":{"badge":{"url":"https://pbs.twimg.com/semantic_core_img/1428827730364096519/4ZXpTBhS?format=png&name=orig"},"description":"Automated","longDescription":{"text":"Automated by @AldlymyJwd","entities":[{"fromIndex":13,"toIndex":24,"ref":{"type":"TimelineRichTextMention","screen_name":"AldlymyJwd","mention_results":{"result":{"__typename":"UserUnavailable","reason":"Suspended","message":"User is suspended"}}}}]},"userLabelType":"AutomatedLabel"}}"#;
    const BUSINESS: &str = r#"{"label":{"url":{"url":"https://twitter.com/Not_the_Bee","urlType":"DeepLink"},"badge":{"url":"https://pbs.twimg.com/profile_images/1299007907623825411/Vo7e-cEQ_bigger.jpg"},"description":"Not the Bee","userLabelType":"BusinessLabel","userLabelDisplayType":"Badge"}}"#;

    #[test]
    fn deserialize_automated_label() {
        let deserialized: AffiliationResult<'_> = serde_json::from_str(AUTOMATED).unwrap();
        let expected = AffiliationResult::Active(Affiliation::automated("2jacksArt", 3313940136));

        assert!(matches!(
            deserialized.affilation().unwrap().screen_name,
            Cow::Borrowed(_)
        ));
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn deserialize_automated_unavailable_label() {
        let deserialized: AffiliationResult<'_> =
            serde_json::from_str(AUTOMATED_UNAVAILABLE).unwrap();
        let expected = AffiliationResult::Active(Affiliation::automated_unavailable(
            "AldlymyJwd",
            UserUnavailableReason::Suspended,
        ));

        assert!(matches!(
            deserialized.affilation().unwrap().screen_name,
            Cow::Borrowed(_)
        ));
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn deserialize_business_label() {
        let deserialized: AffiliationResult<'_> = serde_json::from_str(BUSINESS).unwrap();
        let expected = AffiliationResult::Active(Affiliation::business("Not_the_Bee"));

        // TODO: figure out why this fails.
        //assert!(matches!(deserialized.screen_name, Cow::Borrowed(_)));
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn deserialize_empty_result() {
        let deserialized: AffiliationResult<'_> = serde_json::from_str("{}").unwrap();
        let expected = AffiliationResult::Empty {};

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn failed_deserialize() {
        let bad_json = r#"{"other":[]}"#;
        let deserialized = serde_json::from_str::<AffiliationResult<'_>>(bad_json);

        assert!(deserialized.is_err());
    }
}
