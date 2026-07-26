//! Models for the affiliation label fields of user objects, which identify automated accounts
//! and business affiliates.
use crate::model::graphql::{ResultWrapper, unavailable::UserUnavailableReason};
use regex::Regex;
use serde::de::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::sync::LazyLock;

static AFFILIATE_URL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^https://twitter.com/(\w+)$").expect("valid regex"));

/// An interpreted affiliation label.
///
/// The type parameter is the representation of the automating user: [`AutomatingUser`] for
/// `affiliates_highlighted_label` fields, whose automated labels identify the automating user in
/// their long description, and `()` for `identity_profile_labels_highlighted_label` fields,
/// whose automated labels do not.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Affiliation<'a, A> {
    Business { screen_name: Cow<'a, str> },
    Automated { user: A },
}

/// The automating user identified by the long description of an automated label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AutomatingUser<'a> {
    Available {
        /// Missing if the label's mention results are empty.
        id: Option<u64>,
        screen_name: Cow<'a, str>,
    },
    Unavailable {
        screen_name: Cow<'a, str>,
        reason: UserUnavailableReason,
    },
}

impl AutomatingUser<'_> {
    #[must_use]
    pub fn screen_name(&self) -> &str {
        match self {
            Self::Available { screen_name, .. } | Self::Unavailable { screen_name, .. } => {
                screen_name
            }
        }
    }
}

impl<'a> Affiliation<'a, AutomatingUser<'a>> {
    #[must_use]
    pub fn screen_name(&self) -> &str {
        match self {
            Self::Business { screen_name } => screen_name,
            Self::Automated { user } => user.screen_name(),
        }
    }
}

impl<A: bounded_static::IntoBoundedStatic> bounded_static::IntoBoundedStatic
    for Affiliation<'_, A>
{
    type Static = Affiliation<'static, A::Static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Business { screen_name } => Affiliation::Business {
                screen_name: screen_name.into_static(),
            },
            Self::Automated { user } => Affiliation::Automated {
                user: user.into_static(),
            },
        }
    }
}

impl<A: bounded_static::ToBoundedStatic> bounded_static::ToBoundedStatic for Affiliation<'_, A> {
    type Static = Affiliation<'static, A::Static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::Business { screen_name } => Affiliation::Business {
                screen_name: screen_name.to_static(),
            },
            Self::Automated { user } => Affiliation::Automated {
                user: user.to_static(),
            },
        }
    }
}

impl bounded_static::IntoBoundedStatic for AutomatingUser<'_> {
    type Static = AutomatingUser<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Available { id, screen_name } => AutomatingUser::Available {
                id,
                screen_name: screen_name.into_static(),
            },
            Self::Unavailable {
                screen_name,
                reason,
            } => AutomatingUser::Unavailable {
                screen_name: screen_name.into_static(),
                reason,
            },
        }
    }
}

impl bounded_static::ToBoundedStatic for AutomatingUser<'_> {
    type Static = AutomatingUser<'static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::Available { id, screen_name } => AutomatingUser::Available {
                id: *id,
                screen_name: screen_name.to_static(),
            },
            Self::Unavailable {
                screen_name,
                reason,
            } => AutomatingUser::Unavailable {
                screen_name: screen_name.to_static(),
                reason: *reason,
            },
        }
    }
}

/// Interpret a business label URL as a profile link.
fn business_from_url<A, E: serde::de::Error>(
    url: crate::model::url::Url<'_>,
) -> Result<Affiliation<'_, A>, E> {
    if AFFILIATE_URL_PATTERN.is_match(&url.url) {
        Ok(Affiliation::Business {
            screen_name: match url.url {
                Cow::Borrowed(url) => Cow::Borrowed(&url[20..]),
                Cow::Owned(url) => Cow::Owned(url[20..].to_string()),
            },
        })
    } else {
        Err(E::invalid_value(
            serde::de::Unexpected::Str(&url.url),
            &"profile URL",
        ))
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for Affiliation<'a, AutomatingUser<'a>> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let label = internal::Affiliate::deserialize(deserializer)?;

        match label.label {
            internal::AffiliateLabel::AutomatedLabel(internal::AutomatedLabel {
                long_description:
                    internal::AutomatedLabelLongDescription {
                        entities:
                            internal::AutomatedLabelEntities((internal::AutomatedLabelEntity {
                                reference:
                                    internal::AutomatedLabelRef {
                                        screen_name,
                                        mention_results: ResultWrapper { result },
                                        ..
                                    },
                                ..
                            },)),
                        ..
                    },
                ..
            }) => match result {
                Some(internal::AutomatedLabelMentionResult::User { rest_id, .. }) => {
                    Ok(Self::Automated {
                        user: AutomatingUser::Available {
                            id: Some(rest_id),
                            screen_name,
                        },
                    })
                }
                Some(internal::AutomatedLabelMentionResult::UserUnavailable { reason, .. }) => {
                    Ok(Self::Automated {
                        user: AutomatingUser::Unavailable {
                            screen_name,
                            reason,
                        },
                    })
                }
                None => Ok(Self::Automated {
                    user: AutomatingUser::Available {
                        id: None,
                        screen_name,
                    },
                }),
            },
            internal::AffiliateLabel::BusinessLabel(business_label) => {
                business_from_url(business_label.url)
            }
        }
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for Affiliation<'a, ()> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let label = internal::IdentityAffiliate::deserialize(deserializer)?;

        match label.label {
            internal::IdentityAffiliateLabel::AutomatedLabel { .. } => {
                Ok(Self::Automated { user: () })
            }
            internal::IdentityAffiliateLabel::BusinessLabel(business_label) => {
                business_from_url(business_label.url)
            }
        }
    }
}

mod internal {
    use crate::model::graphql::ResultWrapper;
    use crate::model::url::Url;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Affiliate<'a> {
        #[serde(borrow)]
        pub label: AffiliateLabel<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(tag = "userLabelType", deny_unknown_fields)]
    pub(super) enum AffiliateLabel<'a> {
        #[serde(borrow)]
        BusinessLabel(BusinessLabel<'a>),
        AutomatedLabel(AutomatedLabel<'a>),
    }

    /// The label structure of `identity_profile_labels_highlighted_label` fields, whose
    /// automated labels have no `longDescription`.
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct IdentityAffiliate<'a> {
        #[serde(borrow)]
        pub label: IdentityAffiliateLabel<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(tag = "userLabelType", deny_unknown_fields)]
    pub(super) enum IdentityAffiliateLabel<'a> {
        #[serde(borrow)]
        BusinessLabel(BusinessLabel<'a>),
        AutomatedLabel {
            #[serde(rename = "badge", borrow)]
            _badge: Badge<'a>,
            /// This will generally be `"Automated"`, but may be localized.
            #[serde(rename = "description", borrow)]
            _description: Cow<'a, str>,
        },
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct BusinessLabel<'a> {
        #[serde(borrow)]
        pub url: Url<'a>,
        #[serde(rename = "badge")]
        pub _badge: Badge<'a>,
        #[serde(rename = "description", borrow)]
        pub _description: Cow<'a, str>,
        #[serde(rename = "userLabelDisplayType")]
        pub _user_label_display_type: BusinessLabelDisplayType,
        // TODO: Confirm that this never contains useful information.
        #[serde(rename = "auxiliary_user_labels")]
        pub _auxiliary_user_labels: Option<serde::de::IgnoredAny>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct AutomatedLabel<'a> {
        #[serde(rename = "badge", borrow)]
        pub _badge: Badge<'a>,
        /// This will generally be `"Automated"`, but may be localized.
        #[serde(rename = "description", borrow)]
        pub _description: Cow<'a, str>,
        #[serde(rename = "longDescription", borrow)]
        pub long_description: AutomatedLabelLongDescription<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Badge<'a> {
        #[serde(rename = "url", borrow)]
        pub _url: Cow<'a, str>,
    }

    #[derive(serde::Deserialize)]
    pub(super) enum BusinessLabelDisplayType {
        Badge,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct AutomatedLabelLongDescription<'a> {
        #[serde(rename = "text")]
        pub _text: Cow<'a, str>,
        #[serde(borrow)]
        pub entities: AutomatedLabelEntities<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct AutomatedLabelEntities<'a>(#[serde(borrow)] pub (AutomatedLabelEntity<'a>,));

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct AutomatedLabelEntity<'a> {
        #[serde(rename = "fromIndex")]
        pub _from_index: usize,
        #[serde(rename = "toIndex")]
        pub _to_index: usize,
        #[serde(rename = "ref", borrow)]
        pub reference: AutomatedLabelRef<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct AutomatedLabelRef<'a> {
        #[serde(rename = "type")]
        pub _automated_label_ref_type: AutomatedLabelRefType,
        #[serde(borrow)]
        pub screen_name: Cow<'a, str>,
        pub mention_results: ResultWrapper<AutomatedLabelMentionResult<'a>>,
    }

    #[derive(serde::Deserialize)]
    pub(super) enum AutomatedLabelRefType {
        TimelineRichTextMention,
    }

    #[derive(serde::Deserialize)]
    #[serde(tag = "__typename", deny_unknown_fields)]
    pub(super) enum AutomatedLabelMentionResult<'a> {
        User {
            #[serde(with = "crate::model::attributes::id")]
            rest_id: u64,
            #[serde(rename = "legacy", alias = "core", borrow)]
            _legacy: Option<crate::model::graphql::user::ScreenNameUserResultLegacy<'a>>,
        },
        UserUnavailable {
            reason: crate::model::graphql::unavailable::UserUnavailableReason,
            #[serde(rename = "message")]
            _message: Option<Cow<'a, str>>,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::graphql::EmptyOr;

    const AUTOMATED: &str = r#"{"label":{"badge":{"url":"https://pbs.twimg.com/semantic_core_img/1428827730364096519/4ZXpTBhS?format=png&name=orig"},"description":"Automated","longDescription":{"text":"Automated by @2jacksArt","entities":[{"fromIndex":13,"toIndex":23,"ref":{"type":"TimelineRichTextMention","screen_name":"2jacksArt","mention_results":{"result":{"__typename":"User","legacy":{"screen_name":"2jacksArt"},"rest_id":"3313940136"}}}}]},"userLabelType":"AutomatedLabel"}}"#;
    const AUTOMATED_UNAVAILABLE: &str = r#"{"label":{"badge":{"url":"https://pbs.twimg.com/semantic_core_img/1428827730364096519/4ZXpTBhS?format=png&name=orig"},"description":"Automated","longDescription":{"text":"Automated by @AldlymyJwd","entities":[{"fromIndex":13,"toIndex":24,"ref":{"type":"TimelineRichTextMention","screen_name":"AldlymyJwd","mention_results":{"result":{"__typename":"UserUnavailable","reason":"Suspended","message":"User is suspended"}}}}]},"userLabelType":"AutomatedLabel"}}"#;
    const BUSINESS: &str = r#"{"label":{"url":{"url":"https://twitter.com/Not_the_Bee","urlType":"DeepLink"},"badge":{"url":"https://pbs.twimg.com/profile_images/1299007907623825411/Vo7e-cEQ_bigger.jpg"},"description":"Not the Bee","userLabelType":"BusinessLabel","userLabelDisplayType":"Badge"}}"#;

    // Observed in `identity_profile_labels_highlighted_label` fields (September 2025), which
    // share the label structure but omit `longDescription` on automated labels.
    const IDENTITY_AUTOMATED: &str = r#"{"label":{"badge":{"url":"https://pbs.twimg.com/semantic_core_img/1428827730364096519/4ZXpTBhS?format=png&name=orig"},"description":"Automated","userLabelType":"AutomatedLabel"}}"#;
    const IDENTITY_BUSINESS: &str = r#"{"label":{"url":{"url":"https://twitter.com/uncensoreddotai","urlType":"DeepLink"},"badge":{"url":"https://pbs.twimg.com/profile_images/1928141117062545408/ZC5izKZr_bigger.jpg"},"description":"Uncensored.AI","userLabelType":"BusinessLabel","userLabelDisplayType":"Badge"}}"#;

    #[test]
    fn deserialize_automated_label() {
        let deserialized: Affiliation<'_, AutomatingUser<'_>> =
            serde_json::from_str(AUTOMATED).unwrap();
        let expected = Affiliation::Automated {
            user: AutomatingUser::Available {
                id: Some(3_313_940_136),
                screen_name: "2jacksArt".into(),
            },
        };

        assert!(matches!(
            &deserialized,
            Affiliation::Automated {
                user: AutomatingUser::Available {
                    screen_name: Cow::Borrowed(_),
                    ..
                }
            }
        ));
        assert_eq!(deserialized, expected);
        assert_eq!(deserialized.screen_name(), "2jacksArt");
    }

    #[test]
    fn deserialize_automated_unavailable_label() {
        let deserialized: Affiliation<'_, AutomatingUser<'_>> =
            serde_json::from_str(AUTOMATED_UNAVAILABLE).unwrap();
        let expected = Affiliation::Automated {
            user: AutomatingUser::Unavailable {
                screen_name: "AldlymyJwd".into(),
                reason: crate::model::graphql::unavailable::UserUnavailableReason::Suspended,
            },
        };

        assert!(matches!(
            &deserialized,
            Affiliation::Automated {
                user: AutomatingUser::Unavailable {
                    screen_name: Cow::Borrowed(_),
                    ..
                }
            }
        ));
        assert_eq!(deserialized, expected);
        assert_eq!(deserialized.screen_name(), "AldlymyJwd");
    }

    #[test]
    fn deserialize_business_label() {
        let deserialized: Affiliation<'_, AutomatingUser<'_>> =
            serde_json::from_str(BUSINESS).unwrap();
        let expected = Affiliation::Business {
            screen_name: "Not_the_Bee".into(),
        };

        assert_eq!(deserialized, expected);
        assert_eq!(deserialized.screen_name(), "Not_the_Bee");
    }

    #[test]
    fn deserialize_identity_labels() {
        let deserialized: Affiliation<'_, ()> = serde_json::from_str(IDENTITY_AUTOMATED).unwrap();
        assert_eq!(deserialized, Affiliation::Automated { user: () });

        let deserialized: Affiliation<'_, ()> = serde_json::from_str(IDENTITY_BUSINESS).unwrap();
        assert_eq!(
            deserialized,
            Affiliation::Business {
                screen_name: "uncensoreddotai".into(),
            }
        );

        // Identity automated labels have no long description, so they cannot identify the
        // automating user.
        assert!(
            serde_json::from_str::<Affiliation<'_, AutomatingUser<'_>>>(IDENTITY_AUTOMATED)
                .is_err()
        );

        let deserialized: EmptyOr<Affiliation<'_, ()>> = serde_json::from_str("{}").unwrap();
        assert_eq!(deserialized, EmptyOr::Empty {});
    }

    #[test]
    fn deserialize_empty_result() {
        let deserialized: EmptyOr<Affiliation<'_, AutomatingUser<'_>>> =
            serde_json::from_str("{}").unwrap();

        assert_eq!(deserialized, EmptyOr::Empty {});
    }

    #[test]
    fn failed_deserialize() {
        let bad_json = r#"{"other":[]}"#;
        let deserialized =
            serde_json::from_str::<EmptyOr<Affiliation<'_, AutomatingUser<'_>>>>(bad_json);

        assert!(deserialized.is_err());
    }
}
