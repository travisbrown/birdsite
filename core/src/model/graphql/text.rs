//! TODO: Move this to the `model` level?
use std::borrow::Cow;

use crate::model::entity::TypedEntity;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Text<'a> {
    #[serde(borrow)]
    pub text: Cow<'a, str>,
    pub rtl: Option<bool>,
    #[serde(borrow)]
    pub entities: Vec<TypedEntity<'a>>,
}
