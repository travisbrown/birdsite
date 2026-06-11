use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrendMetadata<'a> {
    #[serde(borrow)]
    pub domain_context: Option<Cow<'a, str>>,
    pub meta_description: Option<Cow<'a, str>>,
    pub url: Option<crate::model::url::Url<'a>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_trend_url_options() {
        // Regression: a `TrendUrl` wrapper used to declare its own `urtEndpointOptions` alongside
        // the flattened `Url`'s identically renamed field, so serialization emitted the key twice
        // and the output could not be re-deserialized.
        let json = concat!(
            r#"{"domain_context":"Trending","meta_description":null,"#,
            r#""url":{"urlType":"UrtEndpoint","url":"twitter://search","#,
            r#""urtEndpointOptions":{"requestParams":[],"title":"Search"}}}"#
        );

        let value = serde_json::from_str::<TrendMetadata<'_>>(json).unwrap();
        let reserialized = serde_json::to_string(&value).unwrap();

        assert_eq!(reserialized.matches("urtEndpointOptions").count(), 1);
        assert_eq!(
            serde_json::from_str::<TrendMetadata<'_>>(&reserialized).unwrap(),
            value
        );
    }
}
