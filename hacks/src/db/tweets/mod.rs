use rocksdb::{DB, Options};
use std::path::Path;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RocksDB error")]
    Db(#[from] rocksdb::Error),
    #[error("Invalid key")]
    InvalidKey(Vec<u8>),
    #[error("Invalid value")]
    InvalidValue(Vec<u8>),
}

#[derive(Clone)]
pub struct Database {
    db: Arc<DB>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut options = Options::default();
        options.create_if_missing(true);
        options.set_compression_type(rocksdb::DBCompressionType::Zstd);

        let db = DB::open(&options, path)?;

        Ok(Self { db: Arc::new(db) })
    }

    pub fn lookup(&self, user_id: u64) -> impl Iterator<Item = Result<(u64, bool), Error>> {
        let prefix = user_id.to_be_bytes();

        self.db
            .prefix_iterator(prefix)
            .take_while(move |result| {
                result
                    .as_ref()
                    .map_or(true, |(key, _)| key.starts_with(&prefix))
            })
            .map(|result| {
                let (key, value) = result?;

                if key.len() == 16 {
                    let tweet_id = u64::from_be_bytes(
                        key[8..16]
                            .try_into()
                            .map_err(|_| Error::InvalidKey(key.to_vec()))?,
                    );

                    Ok((tweet_id, value.is_empty()))
                } else {
                    Err(Error::InvalidKey(key.to_vec()))
                }
            })
    }

    pub fn lookup_live(&self, user_id: u64) -> impl Iterator<Item = Result<u64, Error>> {
        self.lookup(user_id).filter_map(|result| {
            result
                .map(|(tweet_id, live)| live.then_some(tweet_id))
                .map_or_else(|error| Some(Err(error)), |value| value.map(Ok))
        })
    }

    pub fn insert(&self, user_id: u64, tweet_id: u64) -> Result<bool, Error> {
        let key = Self::key(user_id, tweet_id);

        if self.db.get_pinned(key)?.is_some() {
            Ok(true)
        } else {
            self.db.put(key, [])?;

            Ok(false)
        }
    }

    pub fn mark_dead(&self, user_id: u64, tweet_id: u64) -> Result<bool, Error> {
        let key = Self::key(user_id, tweet_id);

        match self.db.get_pinned(key)? {
            Some(value) if value.is_empty() => {
                self.db.put(key, [0])?;
                Ok(true)
            }
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub fn mark_live(&self, user_id: u64, tweet_id: u64) -> Result<bool, Error> {
        let key = Self::key(user_id, tweet_id);

        match self.db.get_pinned(key)? {
            Some(value) if value.is_empty() => Ok(true),
            Some(_) => {
                self.db.put(key, [])?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn key(user_id: u64, tweet_id: u64) -> [u8; 16] {
        let mut bytes = [0; 16];

        bytes[0..8].copy_from_slice(&user_id.to_be_bytes());
        bytes[8..16].copy_from_slice(&tweet_id.to_be_bytes());

        bytes
    }
}
