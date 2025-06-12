pub mod ads;
pub mod community;
pub mod image;
pub mod properties;
pub mod shapes;
pub mod text;
pub mod trends;
pub mod unavailable;
pub mod user;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResultWrapper<A> {
    pub result: Option<A>,
}
