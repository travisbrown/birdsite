use serde::de::Deserialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MentionResult<'a> {
    Full(Mention<'a>),
    ScreenName(Cow<'a, str>),
}

impl<'a> MentionResult<'a> {
    #[must_use]
    pub fn mention(&self) -> Option<Mention<'a>> {
        match self {
            Self::Full(mention) => Some(mention.clone()),
            Self::ScreenName(_) => None,
        }
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for MentionResult<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal::MentionResult {
            id,
            screen_name,
            name,
        } = internal::MentionResult::deserialize(deserializer)?;

        if id > 0 {
            if screen_name.is_empty() {
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&screen_name),
                    &"non-empty screen name string",
                ))
            } else {
                Ok(Self::Full(Mention {
                    id: id as u64,
                    screen_name,
                    name,
                }))
            }
        } else if id == -1 {
            Ok(Self::ScreenName(screen_name))
        } else {
            // If the ID is negative, we expect it to be -1.
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(id),
                &"-1",
            ))
        }
    }
}

impl bounded_static::IntoBoundedStatic for MentionResult<'_> {
    type Static = MentionResult<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Full(mention) => MentionResult::Full(mention.into_static()),
            Self::ScreenName(screen_name) => MentionResult::ScreenName(screen_name.into_static()),
        }
    }
}

impl bounded_static::ToBoundedStatic for MentionResult<'_> {
    type Static = MentionResult<'static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::Full(mention) => MentionResult::Full(mention.to_static()),
            Self::ScreenName(screen_name) => MentionResult::ScreenName(screen_name.to_static()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct Mention<'a> {
    pub id: u64,
    pub screen_name: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

mod internal {
    use serde_field_attributes::integer_str;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MentionResult<'a> {
        #[serde(rename = "id_str", with = "integer_str")]
        pub id: i64,
        pub screen_name: Cow<'a, str>,
        pub name: Cow<'a, str>,
    }
}
