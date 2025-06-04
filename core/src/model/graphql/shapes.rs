#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rectangle {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}
