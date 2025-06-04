use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image<'a> {
    pub url: Cow<'a, str>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct OriginalImage<'a> {
    pub original_img_url: Cow<'a, str>,
    pub original_img_width: usize,
    pub original_img_height: usize,
}
