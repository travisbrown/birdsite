use bounded_static_derive_more::ToStatic;
use num_rational::Ratio;
use serde_field_attributes::ratio_i64;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Place<'a> {
    pub id: Cow<'a, str>,
    pub place_type: crate::model::place::PlaceType,
    pub name: Cow<'a, str>,
    pub country_code: crate::model::country::PossibleCountry,
    pub country: Cow<'a, str>,
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
pub struct BoundingBox(
    #[serde(with = "ratio_i64")] pub Ratio<i64>,
    #[serde(with = "ratio_i64")] pub Ratio<i64>,
    #[serde(with = "ratio_i64")] pub Ratio<i64>,
    #[serde(with = "ratio_i64")] pub Ratio<i64>,
);

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceGeoProperties {}
