use num_rational::Ratio;
use serde_field_attributes::ratio_i64;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Place<'a> {
    pub id: &'a str,
    pub url: Cow<'a, str>,
    pub place_type: PlaceType,
    pub name: Cow<'a, str>,
    pub country_code: crate::model::country::PossibleCountry,
    pub country: Cow<'a, str>,
    pub full_name: Cow<'a, str>,
    pub bounding_box: Option<BoundingBox>,
    pub attributes: Option<Attributes>,
    pub contained_within: Option<Vec<()>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PlaceType {
    #[serde(rename = "city")]
    City,
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "neighborhood")]
    Neighborhood,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "poi")]
    PointOfInterest,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundingBox {
    #[serde(rename = "type")]
    pub bounding_box_type: BoundingBoxType,
    pub coordinates: Option<Vec<Vec<Coordinates>>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BoundingBoxType {
    Polygon,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Coordinates(
    #[serde(with = "ratio_i64")] pub Ratio<i64>,
    #[serde(with = "ratio_i64")] pub Ratio<i64>,
);

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedCoordinates {
    #[serde(rename = "type")]
    pub coordinates_type: CoordinatesType,
    pub coordinates: Coordinates,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CoordinatesType {
    Point,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attributes {}
