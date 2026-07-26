//! Models for the two JSON tweet-snapshot formats found in the Wayback Machine.
//!
//! The archive uses two distinct shapes over time: the nested [`data`] format (a `data` envelope
//! mirroring the v2 API, seen from roughly December 2022 onward) and the older [`flat`] format (a
//! single object mirroring the v1.1 API). [`TweetSnapshot`] unifies both.
use bounded_static_derive_more::ToStatic;

pub mod data;
pub mod flat;
pub mod metadata;

/// A tweet snapshot in either of the two archived JSON formats.
// Boxing the larger variant would break the `const fn` accessors below (dereferencing a `Box` is
// not allowed in const contexts).
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, ToStatic)]
pub enum TweetSnapshot<'a> {
    Data(data::TweetSnapshot<'a>),
    Flat(flat::TweetSnapshot<'a>),
}

impl TweetSnapshot<'_> {
    #[must_use]
    pub const fn id(&self) -> u64 {
        match self {
            Self::Data(snapshot) => snapshot.data.id,
            Self::Flat(snapshot) => snapshot.id,
        }
    }

    #[must_use]
    pub const fn user_id(&self) -> u64 {
        match self {
            Self::Data(snapshot) => snapshot.data.author_id,
            Self::Flat(snapshot) => snapshot.user.id,
        }
    }

    #[must_use]
    pub fn user_screen_name(&self) -> Option<&str> {
        match self {
            Self::Data(snapshot) => snapshot
                .lookup_user(self.user_id())
                .map(|user| user.username.as_ref()),
            Self::Flat(snapshot) => Some(&snapshot.user.screen_name),
        }
    }

    #[must_use]
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
