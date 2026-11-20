#![allow(
    clippy::empty_enums,
    reason = "Serde derives for empty objects generate empty field enums outside the annotated type."
)]

use std::borrow::Cow;

use bounded_static_derive_more::ToStatic;
use serde_json::Number;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Place<'a> {
    #[serde(borrow)]
    pub id: Cow<'a, str>,
    pub place_type: crate::model::place::PlaceType,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub country_code: crate::model::country::PossibleCountry,
    #[serde(borrow)]
    pub country: Cow<'a, str>,
    #[serde(borrow)]
    pub full_name: Cow<'a, str>,
    pub geo: Option<PlaceGeo>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceGeo {
    #[serde(rename = "type")]
    pub place_geo_type: PlaceGeoType,
    pub bbox: BoundingBox,
    pub properties: PlaceGeoProperties,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PlaceGeoType {
    Feature,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundingBox(pub Number, pub Number, pub Number, pub Number);

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceGeoProperties {}
