pub mod data;
pub mod v1;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum TweetSnapshot<'a> {
    Data(data::Tweet<'a>),
    #[serde(borrow)]
    V1(v1::Tweet<'a>),
}

impl TweetSnapshot<'_> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Data(value) => value.data.id,
            Self::V1(value) => value.id,
        }
    }
}
