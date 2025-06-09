#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum WithheldScope {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "")]
    Empty,
}
