pub mod rate_limits;
pub mod stream;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListType {
    Followers,
    Friends,
}

impl ListType {
    pub fn name(&self) -> &str {
        match self {
            Self::Followers => "followers",
            Self::Friends => "friends",
        }
    }
}

pub struct Client {
    client: reqwest::Client,
    followers_endpoint: String,
    friends_endpoint: String,
    bearer_token: String,
}

impl Client {
    pub fn new(followers_endpoint: String, friends_endpoint: String, bearer_token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            followers_endpoint,
            friends_endpoint,
            bearer_token,
        }
    }

    pub fn list(&self, user_id: u64, list_type: ListType) -> stream::FollowsStream<'_> {
        let endpoint = match list_type {
            ListType::Followers => &self.followers_endpoint,
            ListType::Friends => &self.friends_endpoint,
        };

        stream::FollowsStream::new(&self.client, endpoint, &self.bearer_token, user_id)
    }
}
