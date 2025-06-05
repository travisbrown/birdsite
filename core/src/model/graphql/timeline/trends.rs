use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trend<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub url: crate::model::graphql::trends::TrendUrl<'a>,
}
