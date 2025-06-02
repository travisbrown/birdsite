#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum VerifiedType {
    Business,
    Government,
    User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProfileImageShape {
    Circle,
    Square,
    Hexagon,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ParodyCommentaryFanLabel {
    None,
    Parody,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HighlightsInfo {
    pub can_highlight_tweets: bool,
    pub highlighted_tweets: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProfileInterstitialType {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "fake_account")]
    FakeAccount,
    #[serde(rename = "offensive_profile_content")]
    OffensiveProfileContent,
    #[serde(rename = "sensitive_media")]
    SensitiveMedia,
    #[serde(rename = "timeout")]
    Timeout,
}
