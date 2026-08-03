use std::borrow::Cow;
use std::marker::PhantomData;

/// The result payload of a user lookup, dispatched on the `__typename` tag.
///
/// Deserialization is implemented by hand below: the derive cannot combine `deny_unknown_fields`
/// with the flattened `User` payload (serde would match keys against the named variant fields
/// only, rejecting every field of any record), so the manual implementation delegates the
/// remaining fields of `User` records to the payload type and strictly rejects unknown fields of
/// `UserUnavailable` records.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(tag = "__typename")]
pub enum UserResult<'a, U> {
    User {
        #[serde(flatten)]
        user: U,
    },
    UserUnavailable {
        reason: crate::model::graphql::unavailable::UserUnavailableReason,
        /// This fields seems to have been added as mandatory around 16 September 2023.
        message: Option<&'a str>,
        unavailable_message: Option<crate::model::graphql::text::Text<'a>>,
    },
}

impl<'a, 'de: 'a, U: serde::de::Deserialize<'de>> serde::de::Deserialize<'de>
    for UserResult<'a, U>
{
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ResultVisitor<'a, U> {
            _phantom: PhantomData<(&'a (), U)>,
        }

        impl<'a, 'de: 'a, U: serde::de::Deserialize<'de>> serde::de::Visitor<'de> for ResultVisitor<'a, U> {
            type Value = UserResult<'a, U>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("user result object")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                // The `__typename` tag is always the first field in archived records, which lets
                // `User` records hand the rest of the map to the payload type without buffering.
                map.next_key::<TagField>()?
                    .ok_or_else(|| serde::de::Error::missing_field("__typename"))?;

                match map.next_value::<TypeName>()? {
                    TypeName::User => Ok(UserResult::User {
                        user: U::deserialize(serde::de::value::MapAccessDeserializer::new(map))?,
                    }),
                    TypeName::UserUnavailable => {
                        let mut reason = None;
                        let mut message = None;
                        let mut unavailable_message = None;

                        // `UnavailableField` has no catch-all variant, so an unknown field name
                        // fails `next_key` here, preserving the strictness that
                        // `deny_unknown_fields` would otherwise provide.
                        while let Some(field) = map.next_key::<UnavailableField>()? {
                            match field {
                                UnavailableField::Reason => {
                                    set_once(&mut reason, "reason", &mut map)
                                }
                                UnavailableField::Message => {
                                    set_once(&mut message, "message", &mut map)
                                }
                                UnavailableField::UnavailableMessage => set_once(
                                    &mut unavailable_message,
                                    "unavailable_message",
                                    &mut map,
                                ),
                            }?;
                        }

                        Ok(UserResult::UserUnavailable {
                            reason: reason
                                .ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                            message: message.flatten(),
                            unavailable_message: unavailable_message.flatten(),
                        })
                    }
                }
            }
        }

        deserializer.deserialize_map(ResultVisitor {
            _phantom: PhantomData,
        })
    }
}

/// The leading tag field; any other first key fails with an "unknown variant" error.
#[derive(serde::Deserialize)]
enum TagField {
    #[serde(rename = "__typename")]
    Typename,
}

/// The known `__typename` values; anything else fails with an "unknown variant" error.
#[derive(serde::Deserialize)]
enum TypeName {
    User,
    UserUnavailable,
}

/// The known `UserUnavailable` fields; anything else fails with an "unknown variant" error.
#[derive(serde::Deserialize)]
enum UnavailableField {
    #[serde(rename = "reason")]
    Reason,
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "unavailable_message")]
    UnavailableMessage,
}

/// Reads the next map value into `slot`, failing on a repeated field as the derive would.
fn set_once<'de, A: serde::de::MapAccess<'de>, T: serde::de::Deserialize<'de>>(
    slot: &mut Option<T>,
    name: &'static str,
    map: &mut A,
) -> Result<(), A::Error> {
    if slot.replace(map.next_value()?).is_some() {
        Err(serde::de::Error::duplicate_field(name))
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "crate::model::attributes::id_str")]
    pub rest_id: u64,
    #[serde(borrow)]
    pub legacy: Legacy<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//#[serde(deny_unknown_fields)]
pub struct Legacy<'a> {
    #[serde(borrow)]
    pub screen_name: Cow<'a, str>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScreenNameUserResults<'a> {
    #[serde(borrow)]
    pub result: Option<ScreenNameUserResult<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "__typename", deny_unknown_fields)]
pub enum ScreenNameUserResult<'a> {
    User {
        /// Seems to have switched to `core` around 22 May 2025.
        #[serde(alias = "core", borrow)]
        legacy: ScreenNameUserResultLegacy<'a>,
    },
    UserUnavailable {},
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScreenNameUserResultLegacy<'a> {
    pub screen_name: &'a str,
}

#[cfg(test)]
mod tests {
    use super::{User, UserResult};

    /// A record with the fields the payload type does not model, which must flow into the
    /// flattened payload instead of being rejected at the enum level.
    const FULL_USER: &str = concat!(
        r#"{"__typename":"User","id":"VXNlcjoxMg==","rest_id":"12","#,
        r#""affiliates_highlighted_label":{},"is_blue_verified":true,"#,
        r#""legacy":{"screen_name":"example","name":"Example"}}"#
    );

    #[test]
    fn deserialize_user_with_unmodeled_fields() {
        let result = serde_json::from_str::<UserResult<'_, User<'_>>>(FULL_USER).unwrap();

        assert!(matches!(result, UserResult::User { user } if user.rest_id == 12));
    }

    #[test]
    fn deserialize_unavailable() {
        let json = concat!(
            r#"{"__typename":"UserUnavailable","reason":"Suspended","message":"User is suspended","#,
            r#""unavailable_message":{"text":"Account suspended","rtl":false,"entities":[]}}"#
        );

        let result = serde_json::from_str::<UserResult<'_, User<'_>>>(json).unwrap();

        assert!(matches!(
            result,
            UserResult::UserUnavailable {
                reason: crate::model::graphql::unavailable::UserUnavailableReason::Suspended,
                message: Some("User is suspended"),
                unavailable_message: Some(_),
            }
        ));
    }

    #[test]
    fn reject_unknown_unavailable_field() {
        let json = r#"{"__typename":"UserUnavailable","reason":"Suspended","id":"VXNlcjoxMg=="}"#;

        assert!(serde_json::from_str::<UserResult<'_, User<'_>>>(json).is_err());
    }

    #[test]
    fn reject_unavailable_without_reason() {
        let json = r#"{"__typename":"UserUnavailable","message":"User is suspended"}"#;

        assert!(serde_json::from_str::<UserResult<'_, User<'_>>>(json).is_err());
    }
}
