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

    pub fn user_info(&self) -> Vec<UserInfo> {
        match self {
            Self::Data(value) => value
                .includes
                .users
                .iter()
                .map(|user| UserInfo::new(user.id, user.username.clone()))
                .collect(),
            Self::V1(value) => vec![],
        }
    }

    pub fn tweet_info(&self) -> Vec<TweetInfo> {
        match self {
            Self::Data(value) => {
                let mut values = vec![TweetInfo::from_data(&value.data)];

                values.extend(
                    value
                        .includes
                        .tweets
                        .as_ref()
                        .map(|tweets| {
                            tweets
                                .iter()
                                .map(|value| TweetInfo::from_data(value))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default(),
                );

                values
            }
            Self::V1(value) => vec![],
        }
    }
}

pub struct UserInfo {
    pub id: u64,
    pub screen_name: String,
}

impl UserInfo {
    pub fn new<S: Into<String>>(id: u64, screen_name: S) -> Self {
        Self {
            id,
            screen_name: screen_name.into(),
        }
    }
}

pub struct TweetInfo {
    pub status_id: u64,
    pub user_id: u64,
    pub in_reply_to_user_id: Option<u64>,
}

impl TweetInfo {
    pub fn new(status_id: u64, user_id: u64, in_reply_to_user_id: Option<u64>) -> Self {
        Self {
            status_id,
            user_id,
            in_reply_to_user_id,
        }
    }

    fn from_data(value: &data::TweetData) -> Self {
        Self::new(value.id, value.author_id, value.in_reply_to_user_id)
    }
}
