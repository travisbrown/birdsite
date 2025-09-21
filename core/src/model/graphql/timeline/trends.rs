use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trend<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub url: crate::model::graphql::trends::TrendUrl<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum TrendImage<'a> {
    Object { url: &'a str },
    Direct(&'a str),
}

impl<'a> TrendImage<'a> {
    #[must_use]
    pub const fn url(&self) -> &'a str {
        match self {
            Self::Object { url } | Self::Direct(url) => url,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_direct_trend_image() {
        let image = serde_json::from_str::<super::TrendImage<'_>>(
            "\"https://pbs.twimg.com/media/Gk63rFLXgAAxd8-.jpg\"",
        )
        .unwrap();

        assert_eq!(
            image.url(),
            "https://pbs.twimg.com/media/Gk63rFLXgAAxd8-.jpg"
        );
    }

    #[test]
    fn deserialize_object_trend_image() {
        let image = serde_json::from_str::<super::TrendImage<'_>>(
            "{\"url\":\"https://pbs.twimg.com/media/Gk63rFLXgAAxd8-.jpg\"}",
        )
        .unwrap();

        assert_eq!(
            image.url(),
            "https://pbs.twimg.com/media/Gk63rFLXgAAxd8-.jpg"
        );
    }
}
