pub mod data;
pub mod flat;
pub mod metadata;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TweetSnapshot<'a> {
    Data(data::TweetSnapshot<'a>),
    Flat(flat::TweetSnapshot<'a>),
}

impl<'a> TweetSnapshot<'a> {
    pub fn id(&self) -> u64 {
        match self {
            Self::Data(snapshot) => snapshot.data.id,
            Self::Flat(snapshot) => snapshot.id,
        }
    }

    pub fn user_id(&self) -> u64 {
        match self {
            Self::Data(snapshot) => snapshot.data.author_id,
            Self::Flat(snapshot) => snapshot.user.id,
        }
    }

    pub fn user_screen_name(&self) -> Option<&str> {
        match self {
            Self::Data(snapshot) => snapshot
                .lookup_user(self.user_id())
                .map(|user| user.username.as_ref()),
            Self::Flat(snapshot) => Some(&snapshot.user.screen_name),
        }
    }

    pub fn canonical_url(&self, use_x: bool) -> Option<String> {
        self.user_screen_name().map(|screen_name| {
            format!(
                "https://{}.com/{}/status/{}",
                if use_x { "x" } else { "twitter" },
                screen_name,
                self.id()
            )
        })
    }
}
