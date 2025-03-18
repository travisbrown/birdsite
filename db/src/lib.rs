use birdsite::age::snowflake_to_date_time;
use chrono::{DateTime, Utc};
use rocksdb::{
    BlockBasedOptions, ColumnFamily, ColumnFamilyDescriptor, DBCompressionType,
    DBIteratorWithThreadMode, IteratorMode, Options, Transaction, TransactionDB,
    TransactionDBOptions,
};
use std::path::Path;
use std::sync::Arc;

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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TweetMetadata {
    pub id: u64,
    pub user: UserMetadata,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserMetadata {
    pub id: u64,
    pub created_at: DateTime<Utc>,
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

        let mut cfs = vec![
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

    pub fn insert_tweet(
        &self,
        tweet_metadata: TweetMetadata,
        retweet_target_metadata: Option<TweetMetadata>,
        reply_target_metadata: Option<TweetMetadata>,
        quote_target_metadata: Option<TweetMetadata>,
        mentions_metadata: Vec<UserMetadata>,
    ) -> Result<(), Error> {
        let tweet_id_bytes = tweet_metadata.id.to_be_bytes();
        let user_id_bytes = tweet_metadata.user.id.to_be_bytes();

        let transaction = self.db.transaction();
        let cfs = ColumnFamilies::get(&self.db);

        Self::insert_single_tweet(&transaction, &cfs, tweet_metadata)?;

        let mut key = [0u8; 16];

        if let Some(target_metadata) = retweet_target_metadata {
            Self::insert_single_tweet(&transaction, &cfs, target_metadata)?;

            let target_id_bytes = target_metadata.id.to_be_bytes();

            Self::checked_update(&transaction, cfs.retweet0, tweet_id_bytes, &target_id_bytes)?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&tweet_id_bytes);

            transaction.put_cf(cfs.retweet1, key, [])?;
        }

        if let Some(target_metadata) = reply_target_metadata {
            Self::insert_single_tweet(&transaction, &cfs, target_metadata)?;

            let target_id_bytes = target_metadata.id.to_be_bytes();

            Self::checked_update(&transaction, cfs.reply0, tweet_id_bytes, &target_id_bytes)?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&tweet_id_bytes);

            transaction.put_cf(cfs.reply1, key, [])?;
        }

        if let Some(target_metadata) = quote_target_metadata {
            Self::insert_single_tweet(&transaction, &cfs, target_metadata)?;

            let target_id_bytes = target_metadata.id.to_be_bytes();

            Self::checked_update(&transaction, cfs.quote0, tweet_id_bytes, &target_id_bytes)?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&tweet_id_bytes);

            transaction.put_cf(cfs.quote1, key, [])?;
        }

        for user_metadata in mentions_metadata {
            let target_id_bytes = user_metadata.id.to_be_bytes();

            key[0..8].copy_from_slice(&user_id_bytes);
            key[8..16].copy_from_slice(&target_id_bytes);

            transaction.put_cf(cfs.mention0, key, [])?;

            key[0..8].copy_from_slice(&target_id_bytes);
            key[8..16].copy_from_slice(&user_id_bytes);

            transaction.put_cf(cfs.mention1, key, [])?;
        }

        Ok(transaction.commit()?)
    }

    fn insert_single_tweet<'a>(
        transaction: &Transaction<'a, TransactionDB>,
        cfs: &ColumnFamilies<'a>,
        tweet_metadata: TweetMetadata,
    ) -> Result<(), Error> {
        let tweet_id_bytes = tweet_metadata.id.to_be_bytes();
        let user_id_bytes = tweet_metadata.user.id.to_be_bytes();

        let mut value = Vec::with_capacity(12);

        value.extend_from_slice(&tweet_id_bytes);

        if snowflake_to_date_time(tweet_metadata.id) != Some(tweet_metadata.created_at) {
            value.extend_from_slice(&(tweet_metadata.created_at.timestamp() as u32).to_be_bytes());
        }

        Self::checked_update(transaction, &cfs.tweet0, tweet_id_bytes, &value)?;

        let mut tweet1_key = [0u8; 12];

        tweet1_key[0..4].copy_from_slice(&user_id_bytes);
        tweet1_key[4..12].copy_from_slice(&tweet_id_bytes);

        transaction.put_cf(cfs.tweet1, tweet1_key, []);

        value.clear();

        if snowflake_to_date_time(tweet_metadata.user.id) != Some(tweet_metadata.user.created_at) {
            value.extend_from_slice(
                &(tweet_metadata.user.created_at.timestamp() as u32).to_be_bytes(),
            );
        }

        transaction.put_cf(cfs.user0, tweet_metadata.user.id.to_be_bytes(), &value)?;

        Ok(())
    }

    pub fn lookup_tweet(&self, id: u64) -> Result<Option<(u64, DateTime<Utc>)>, Error> {
        todo![]
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

    pub fn lookup_user(&self, id: u64) -> Result<Option<DateTime<Utc>>, Error> {
        todo![]
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

    fn checked_update<'a>(
        transaction: &'a Transaction<'a, TransactionDB>,
        cf: &'a ColumnFamily,
        key: [u8; 8],
        value: &'a [u8],
    ) -> Result<bool, Error> {
        let exists = if let Some(old) = transaction.get_pinned_for_update_cf(cf, key, false)? {
            if old.as_ref() == value {
                return Err(Error::DuplicateValues {
                    key: u64::from_be_bytes(key),
                    old: old.to_vec(),
                    new: value.to_vec(),
                });
            }

            true
        } else {
            false
        };

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
                                .map(|slice| u64::from_be_bytes(slice))
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
