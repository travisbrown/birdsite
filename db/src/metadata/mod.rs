use birdsite::age::snowflake_to_date_time;
use chrono::{DateTime, TimeZone, Utc};
use rocksdb::{
    ColumnFamily, ColumnFamilyDescriptor, DBIteratorWithThreadMode, IteratorMode, Options,
    Transaction, TransactionDB, TransactionDBOptions,
};
use std::path::Path;
use std::sync::Arc;

pub mod tweet;

use tweet::{TweetMetadata, UserMetadata};

const TWEET0_CF_NAME: &str = "tweet0";
const TWEET1_CF_NAME: &str = "tweet1";
const USER0_CF_NAME: &str = "user0";
const RETWEET0_CF_NAME: &str = "retweet0";
const RETWEET1_CF_NAME: &str = "retweet1";
const REPLY0_CF_NAME: &str = "reply0";
const REPLY1_CF_NAME: &str = "reply1";
const QUOTE0_CF_NAME: &str = "quote0";
const QUOTE1_CF_NAME: &str = "quote1";
const MENTION0_CF_NAME: &str = "mention0";
const MENTION1_CF_NAME: &str = "mention1";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RocksDB error")]
    Db(#[from] rocksdb::Error),
    #[error("Invalid key")]
    InvalidKey(Vec<u8>),
    #[error("Invalid value")]
    InvalidValue(Vec<u8>),
    #[error("Duplicate values")]
    DuplicateValues {
        key: u64,
        old: Vec<u8>,
        new: Vec<u8>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Timestamp {
    Stored(DateTime<Utc>),
    Computed(DateTime<Utc>),
}

impl Timestamp {
    pub fn is_stored(&self) -> bool {
        matches!(self, Self::Stored(_))
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(value: Timestamp) -> Self {
        match value {
            Timestamp::Stored(timestamp) => timestamp,
            Timestamp::Computed(timestamp) => timestamp,
        }
    }
}

#[derive(Clone)]
pub struct Database {
    db: Arc<TransactionDB>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut options = Options::default();
        options.create_missing_column_families(true);
        options.create_if_missing(true);

        let transactions_options = TransactionDBOptions::new();

        let tweet0_cf = ColumnFamilyDescriptor::new(TWEET0_CF_NAME, Self::default_cf_options());
        let tweet1_cf = ColumnFamilyDescriptor::new(TWEET1_CF_NAME, Self::default_cf_options());
        let user0_cf = ColumnFamilyDescriptor::new(USER0_CF_NAME, Self::default_cf_options());
        let retweet0_cf = ColumnFamilyDescriptor::new(RETWEET0_CF_NAME, Self::default_cf_options());
        let retweet1_cf = ColumnFamilyDescriptor::new(RETWEET1_CF_NAME, Self::default_cf_options());
        let reply0_cf = ColumnFamilyDescriptor::new(REPLY0_CF_NAME, Self::default_cf_options());
        let reply1_cf = ColumnFamilyDescriptor::new(REPLY1_CF_NAME, Self::default_cf_options());
        let quote0_cf = ColumnFamilyDescriptor::new(QUOTE0_CF_NAME, Self::default_cf_options());
        let quote1_cf = ColumnFamilyDescriptor::new(QUOTE1_CF_NAME, Self::default_cf_options());
        let mention0_cf = ColumnFamilyDescriptor::new(MENTION0_CF_NAME, Self::default_cf_options());
        let mention1_cf = ColumnFamilyDescriptor::new(MENTION1_CF_NAME, Self::default_cf_options());

        let cfs = vec![
            tweet0_cf,
            tweet1_cf,
            user0_cf,
            retweet0_cf,
            retweet1_cf,
            reply0_cf,
            reply1_cf,
            quote0_cf,
            quote1_cf,
            mention0_cf,
            mention1_cf,
        ];

        let db = TransactionDB::open_cf_descriptors(&options, &transactions_options, path, cfs)?;

        Ok(Self { db: Arc::new(db) })
    }

    pub fn known_tweets(&self) -> TweetIterator<'_> {
        TweetIterator {
            underlying: self
                .db
                .iterator_cf(self.cf_handle(TWEET0_CF_NAME), IteratorMode::Start),
        }
    }

    pub fn known_users(&self) -> UserIterator<'_> {
        UserIterator {
            underlying: self
                .db
                .iterator_cf(self.cf_handle(USER0_CF_NAME), IteratorMode::Start),
        }
    }

    pub fn seen_tweets(&self) -> impl Iterator<Item = Result<u64, Error>> {
        self.known_tweets()
            .map(|result| result.map(|(id, _, _)| id))
            .chain(
                IdPairIterator {
                    underlying: self
                        .db
                        .iterator_cf(self.cf_handle(RETWEET1_CF_NAME), IteratorMode::Start),
                }
                .ids(),
            )
            .chain(
                IdPairIterator {
                    underlying: self
                        .db
                        .iterator_cf(self.cf_handle(REPLY1_CF_NAME), IteratorMode::Start),
                }
                .ids(),
            )
            .chain(
                IdPairIterator {
                    underlying: self
                        .db
                        .iterator_cf(self.cf_handle(QUOTE1_CF_NAME), IteratorMode::Start),
                }
                .ids(),
            )
            .chain(
                IdPairIterator {
                    underlying: self
                        .db
                        .iterator_cf(self.cf_handle(MENTION0_CF_NAME), IteratorMode::Start),
                }
                .map(|result| result.map(|(id, _)| id)),
            )
    }

    pub fn seen_users(&self) -> impl Iterator<Item = Result<u64, Error>> {
        self.known_users()
            .map(|result| result.map(|(id, _)| id))
            .chain(
                self.known_tweets()
                    .map(|result| result.map(|(_, user_id, _)| user_id)),
            )
            .chain(
                IdPairIterator {
                    underlying: self
                        .db
                        .iterator_cf(self.cf_handle(MENTION0_CF_NAME), IteratorMode::Start),
                }
                .map(|result| result.map(|(_, user_id)| user_id)),
            )
    }

    pub fn insert_snapshot(&self, tweets: &[TweetMetadata]) -> Result<(), Error> {
        let transaction = self.db.transaction();
        let cfs = ColumnFamilies::get(&self.db);

        for tweet in tweets {
            Self::insert_single_tweet(&transaction, &cfs, tweet)?;
        }

        Ok(transaction.commit()?)
    }

    fn insert_single_tweet<'a>(
        transaction: &Transaction<'a, TransactionDB>,
        cfs: &ColumnFamilies<'a>,
        tweet_metadata: &TweetMetadata,
    ) -> Result<(), Error> {
        let tweet_id_bytes = tweet_metadata.id.to_be_bytes();
        let user_id_bytes = tweet_metadata.user.id.to_be_bytes();

        let mut value = Vec::with_capacity(12);

        value.extend_from_slice(&user_id_bytes);

        if snowflake_to_date_time(tweet_metadata.id) != Some(tweet_metadata.created_at) {
            value.extend_from_slice(&(tweet_metadata.created_at.timestamp() as u32).to_be_bytes());
        }

        Self::checked_update(transaction, cfs.tweet0, tweet_id_bytes, &value)?;

        let mut tweet1_key = [0u8; 16];

        tweet1_key[0..8].copy_from_slice(&user_id_bytes);
        tweet1_key[8..16].copy_from_slice(&tweet_id_bytes);

        transaction.put_cf(cfs.tweet1, tweet1_key, [])?;

        Self::insert_user(transaction, cfs.user0, &tweet_metadata.user)?;

        let mut key = [0u8; 16];

        if let Some(target_id) = tweet_metadata.retweeted_id {
            let target_id_bytes = target_id.to_be_bytes();

            Self::checked_update(transaction, cfs.retweet0, tweet_id_bytes, &target_id_bytes)?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&tweet_id_bytes);

            transaction.put_cf(cfs.retweet1, key, [])?;
        }

        if let Some(target_id) = tweet_metadata.replied_to_id {
            let target_id_bytes = target_id.to_be_bytes();

            Self::checked_update(transaction, cfs.reply0, tweet_id_bytes, &target_id_bytes)?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&tweet_id_bytes);

            transaction.put_cf(cfs.reply1, key, [])?;
        }

        if let Some(target_id) = tweet_metadata.quoted_id {
            let target_id_bytes = target_id.to_be_bytes();

            Self::checked_update(transaction, cfs.quote0, tweet_id_bytes, &target_id_bytes)?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&tweet_id_bytes);

            transaction.put_cf(cfs.quote1, key, [])?;
        }

        for user_metadata in &tweet_metadata.mentions {
            Self::insert_user(transaction, cfs.user0, user_metadata)?;

            let target_id_bytes = user_metadata.id.to_be_bytes();

            key[0..8].copy_from_slice(&tweet_id_bytes);
            key[8..16].copy_from_slice(&target_id_bytes);

            transaction.put_cf(cfs.mention0, key, [])?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&tweet_id_bytes);

            transaction.put_cf(cfs.mention1, key, [])?;
        }

        Ok(())
    }

    fn insert_user(
        transaction: &Transaction<'_, TransactionDB>,
        cf: &ColumnFamily,
        user_metadata: &UserMetadata,
    ) -> Result<(), Error> {
        if let Some(created_at) = user_metadata.created_at {
            let value = if snowflake_to_date_time(user_metadata.id) != Some(created_at) {
                (created_at.timestamp() as u32).to_be_bytes().to_vec()
            } else {
                vec![]
            };

            Self::checked_update(transaction, cf, user_metadata.id.to_be_bytes(), &value)?;
        }

        Ok(())
    }

    pub fn lookup_tweet(&self, id: u64) -> Result<Option<(u64, DateTime<Utc>)>, Error> {
        self.db
            .get_pinned_cf(self.cf_handle(TWEET0_CF_NAME), id.to_be_bytes())
            .map_err(Error::from)
            .and_then(|value| {
                value.map_or_else(
                    || Ok(None),
                    |value| {
                        decode_tweet0_value(id, value)
                            .map(|(user_id, timestamp)| Some((user_id, timestamp.into())))
                    },
                )
            })
    }

    pub fn lookup_user(&self, id: u64) -> Result<Option<DateTime<Utc>>, Error> {
        self.db
            .get_pinned_cf(self.cf_handle(USER0_CF_NAME), id.to_be_bytes())
            .map_err(Error::from)
            .and_then(|value| {
                value.map_or_else(
                    || Ok(None),
                    |value| decode_user0_value(id, value).map(|timestamp| Some(timestamp.into())),
                )
            })
    }

    pub fn lookup_tweets_by_user(&self, id: u64) -> IdIterator {
        let prefix = id.to_be_bytes();

        IdIterator {
            underlying: self
                .db
                .prefix_iterator_cf(self.cf_handle(TWEET1_CF_NAME), prefix),
            prefix,
        }
    }

    pub fn lookup_retweet_target(&self, id: u64) -> Result<Option<u64>, Error> {
        self.lookup_m2o_target(self.cf_handle(RETWEET0_CF_NAME), id)
    }

    pub fn lookup_retweet_sources(&self, id: u64) -> IdIterator {
        let prefix = id.to_be_bytes();

        IdIterator {
            underlying: self
                .db
                .prefix_iterator_cf(self.cf_handle(RETWEET1_CF_NAME), prefix),
            prefix,
        }
    }

    pub fn lookup_reply_target(&self, id: u64) -> Result<Option<u64>, Error> {
        self.lookup_m2o_target(self.cf_handle(REPLY0_CF_NAME), id)
    }

    pub fn lookup_reply_sources(&self, id: u64) -> IdIterator {
        let prefix = id.to_be_bytes();

        IdIterator {
            underlying: self
                .db
                .prefix_iterator_cf(self.cf_handle(REPLY1_CF_NAME), prefix),
            prefix,
        }
    }

    pub fn lookup_quote_target(&self, id: u64) -> Result<Option<u64>, Error> {
        self.lookup_m2o_target(self.cf_handle(QUOTE0_CF_NAME), id)
    }

    pub fn lookup_quote_sources(&self, id: u64) -> IdIterator {
        let prefix = id.to_be_bytes();

        IdIterator {
            underlying: self
                .db
                .prefix_iterator_cf(self.cf_handle(QUOTE1_CF_NAME), prefix),
            prefix,
        }
    }

    pub fn lookup_mention_targets(&self, id: u64) -> IdIterator {
        let prefix = id.to_be_bytes();

        IdIterator {
            underlying: self
                .db
                .prefix_iterator_cf(self.cf_handle(MENTION0_CF_NAME), prefix),
            prefix,
        }
    }

    pub fn lookup_mention_sources(&self, id: u64) -> IdIterator {
        let prefix = id.to_be_bytes();

        IdIterator {
            underlying: self
                .db
                .prefix_iterator_cf(self.cf_handle(MENTION1_CF_NAME), prefix),
            prefix,
        }
    }

    fn lookup_m2o_target(&self, cf: &ColumnFamily, id: u64) -> Result<Option<u64>, Error> {
        match self.db.get_cf(cf, id.to_be_bytes()) {
            Ok(Some(bytes)) => Ok(Some(u64::from_be_bytes(
                bytes.try_into().map_err(Error::InvalidKey)?,
            ))),
            Ok(None) => Ok(None),
            Err(error) => Err(Error::from(error)),
        }
    }

    fn default_cf_options() -> Options {
        Options::default()
    }

    /// Panics on invalid name.
    ///
    /// Only for internal use.
    fn cf_handle(&self, name: &str) -> &ColumnFamily {
        self.db.cf_handle(name).unwrap()
    }

    /// Insert a key-value pair, checking whether an entry already exists, and failing if it does not have the same value.
    fn checked_update<'a>(
        transaction: &'a Transaction<'a, TransactionDB>,
        cf: &'a ColumnFamily,
        key: [u8; 8],
        value: &'a [u8],
    ) -> Result<bool, Error> {
        let exists = match transaction.get_pinned_for_update_cf(cf, key, false)? {
            Some(old) => {
                if old.as_ref() != value {
                    Err(Error::DuplicateValues {
                        key: u64::from_be_bytes(key),
                        old: old.to_vec(),
                        new: value.to_vec(),
                    })
                } else {
                    Ok(true)
                }
            }
            None => Ok(false),
        }?;

        transaction.put_cf(cf, key, value)?;

        Ok(exists)
    }
}

pub struct IdIterator<'a> {
    underlying: DBIteratorWithThreadMode<'a, TransactionDB>,
    prefix: [u8; 8],
}

impl Iterator for IdIterator<'_> {
    type Item = Result<u64, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().and_then(|result| {
            result.map_or_else(
                |error| Some(Err(Error::from(error))),
                |(key_bytes, _)| {
                    if key_bytes.starts_with(&self.prefix) {
                        Some(
                            key_bytes[8..]
                                .try_into()
                                .map(u64::from_be_bytes)
                                .map_err(|_| Error::InvalidKey(key_bytes.to_vec())),
                        )
                    } else {
                        None
                    }
                },
            )
        })
    }
}

pub struct TweetIterator<'a> {
    underlying: DBIteratorWithThreadMode<'a, TransactionDB>,
}

impl<'a> Iterator for TweetIterator<'a> {
    type Item = Result<(u64, u64, Timestamp), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().map(|result| {
            let (key, value) = result?;

            let id = u64::from_be_bytes(
                key.as_ref()
                    .try_into()
                    .map_err(|_| Error::InvalidKey(key.to_vec()))?,
            );

            let (user_id, timestamp) = decode_tweet0_value(id, value)?;

            Ok((id, user_id, timestamp))
        })
    }
}

pub struct UserIterator<'a> {
    underlying: DBIteratorWithThreadMode<'a, TransactionDB>,
}

impl<'a> Iterator for UserIterator<'a> {
    type Item = Result<(u64, Timestamp), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().map(|result| {
            let (key, value) = result?;

            let id = u64::from_be_bytes(
                key.as_ref()
                    .try_into()
                    .map_err(|_| Error::InvalidKey(key.to_vec()))?,
            );

            let timestamp = decode_user0_value(id, value)?;

            Ok((id, timestamp))
        })
    }
}

pub struct IdPairIterator<'a> {
    underlying: DBIteratorWithThreadMode<'a, TransactionDB>,
}

impl<'a> IdPairIterator<'a> {
    pub fn ids(self) -> impl Iterator<Item = Result<u64, Error>> {
        self.flat_map(|result| {
            result.map_or_else(
                |error| either::Left(std::iter::once(Err(error))),
                |(id0, id1)| either::Right([Ok(id0), Ok(id1)].into_iter()),
            )
        })
    }
}

impl<'a> Iterator for IdPairIterator<'a> {
    type Item = Result<(u64, u64), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.underlying.next().map(|result| {
            let (key, _) = result?;

            if key.len() == 16 {
                let id0 = u64::from_be_bytes(
                    key[0..8]
                        .try_into()
                        .map_err(|_| Error::InvalidKey(key.to_vec()))?,
                );

                let id1 = u64::from_be_bytes(
                    key[8..16]
                        .try_into()
                        .map_err(|_| Error::InvalidKey(key.to_vec()))?,
                );

                Ok((id0, id1))
            } else {
                Err(Error::InvalidKey(key.to_vec()))
            }
        })
    }
}

struct ColumnFamilies<'a> {
    tweet0: &'a ColumnFamily,
    tweet1: &'a ColumnFamily,
    user0: &'a ColumnFamily,
    retweet0: &'a ColumnFamily,
    retweet1: &'a ColumnFamily,
    reply0: &'a ColumnFamily,
    reply1: &'a ColumnFamily,
    quote0: &'a ColumnFamily,
    quote1: &'a ColumnFamily,
    mention0: &'a ColumnFamily,
    mention1: &'a ColumnFamily,
}

impl<'a> ColumnFamilies<'a> {
    fn get(db: &'a TransactionDB) -> Self {
        Self {
            tweet0: db.cf_handle(TWEET0_CF_NAME).unwrap(),
            tweet1: db.cf_handle(TWEET1_CF_NAME).unwrap(),
            user0: db.cf_handle(USER0_CF_NAME).unwrap(),
            retweet0: db.cf_handle(RETWEET0_CF_NAME).unwrap(),
            retweet1: db.cf_handle(RETWEET1_CF_NAME).unwrap(),
            reply0: db.cf_handle(REPLY0_CF_NAME).unwrap(),
            reply1: db.cf_handle(REPLY1_CF_NAME).unwrap(),
            quote0: db.cf_handle(QUOTE0_CF_NAME).unwrap(),
            quote1: db.cf_handle(QUOTE1_CF_NAME).unwrap(),
            mention0: db.cf_handle(MENTION0_CF_NAME).unwrap(),
            mention1: db.cf_handle(MENTION1_CF_NAME).unwrap(),
        }
    }
}

fn decode_tweet0_value<B: AsRef<[u8]>>(id: u64, bytes: B) -> Result<(u64, Timestamp), Error> {
    let bytes = bytes.as_ref();

    match bytes.len() {
        8 => {
            let user_id = u64::from_be_bytes(bytes[0..8].try_into().unwrap());

            match snowflake_to_date_time(id) {
                Some(timestamp) => Ok((user_id, Timestamp::Computed(timestamp))),
                None => Err(Error::InvalidValue(bytes.to_vec())),
            }
        }
        12 => {
            let user_id = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
            let timestamp_s: i64 = u32::from_be_bytes(bytes[8..12].try_into().unwrap()).into();

            match Utc.timestamp_opt(timestamp_s, 0).single() {
                Some(timestamp) => Ok((user_id, Timestamp::Stored(timestamp))),
                None => Err(Error::InvalidValue(bytes.to_vec())),
            }
        }
        _ => Err(Error::InvalidValue(bytes.to_vec())),
    }
}

fn decode_user0_value<B: AsRef<[u8]>>(id: u64, bytes: B) -> Result<Timestamp, Error> {
    let bytes = bytes.as_ref();

    match bytes.len() {
        0 => match snowflake_to_date_time(id) {
            Some(timestamp) => Ok(Timestamp::Computed(timestamp)),
            None => Err(Error::InvalidValue(bytes.to_vec())),
        },
        4 => {
            let timestamp_s: i64 = u32::from_be_bytes(bytes[0..4].try_into().unwrap()).into();

            match Utc.timestamp_opt(timestamp_s, 0).single() {
                Some(timestamp) => Ok(Timestamp::Stored(timestamp)),
                None => Err(Error::InvalidValue(bytes.to_vec())),
            }
        }
        _ => Err(Error::InvalidValue(bytes.to_vec())),
    }
}
