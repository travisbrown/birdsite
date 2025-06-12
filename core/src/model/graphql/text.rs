//! TODO: Move this to the `model` level?
use crate::model::entity::TypedEntity;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Text<'a> {
    pub text: Cow<'a, str>,
    pub rtl: Option<bool>,
    #[serde(borrow)]
    pub entities: Vec<TypedEntity<'a>>,
}
