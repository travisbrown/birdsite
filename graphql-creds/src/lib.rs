#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![forbid(unsafe_code)]
use birdsite_graphql_ctid::{Endpoint, TransactionId, client::Client};
use chrono::Utc;
use rusqlite::{Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod types;

const LOOKUP_CTID_FOR_ENDPOINT: &str = "SELECT value, added FROM ctid WHERE name = ? AND version = ? AND NOT expired ORDER BY added DESC LIMIT 1";
const ADD_CTID: &str = "INSERT INTO ctid (name, version, value) VALUES (?, ?, ?) RETURNING id";
const EXPIRE_CTID: &str = "UPDATE ctid SET expired = 1 WHERE value = ?";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SQLite error")]
    Sqlite(#[from] rusqlite::Error),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Path exists")]
    PathExists(PathBuf),
    #[error("Client error")]
    Client(#[from] birdsite_graphql_ctid::client::Error),
}

#[derive(Clone, Debug)]
pub struct Store {
    connection: Arc<Mutex<Connection>>,
    client: Client,
}

impl Store {
    pub fn new(connection: Connection, client: Client) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
            client,
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, rusqlite::Error> {
        Ok(Self::new(Connection::open(path)?, Client::default()))
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        if path.as_ref().exists() {
            Err(Error::PathExists(path.as_ref().to_path_buf()))
        } else {
            if let Some(parent) = path.as_ref().parent() {
                std::fs::create_dir_all(parent)?;
            }

            let store = Self::new(Connection::open(path)?, Client::default());
            store.load_schema()?;

            Ok(store)
        }
    }

    pub fn open_or_create<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        if path.as_ref().exists() {
            Ok(Self::open(path)?)
        } else {
            Self::create(path)
        }
    }

    pub fn create_in_memory() -> Result<Self, Error> {
        let store = Self::new(Connection::open_in_memory()?, Client::default());
        store.load_schema()?;

        Ok(store)
    }

    pub fn load_schema(&self) -> Result<(), rusqlite::Error> {
        let schema = include_str!("schemas/ctid.sql");
        let connection = self.connection.lock().unwrap();

        connection.execute_batch(schema)
    }

    pub async fn get_ctid(&self, endpoint: &Endpoint<'_>) -> Result<TransactionId, Error> {
        if let Some(transaction_id) = self.lookup_ctid(endpoint)? {
            Ok(transaction_id)
        } else {
            let transaction_id = self.client.generate(endpoint).await?;

            self.add_ctid(endpoint, &transaction_id)?;

            Ok(transaction_id)
        }
    }

    pub async fn get_ctid_with_max_age(
        &self,
        endpoint: &Endpoint<'_>,
        max_age: Duration,
    ) -> Result<TransactionId, Error> {
        if let Some(transaction_id) = self.lookup_ctid(endpoint)? {
            let age = Utc::now() - transaction_id.timestamp;

            if age.to_std().map(|age| age <= max_age).unwrap_or(false) {
                Ok(transaction_id)
            } else {
                let transaction_id = self.client.generate(endpoint).await?;

                self.add_ctid(endpoint, &transaction_id)?;

                Ok(transaction_id)
            }
        } else {
            let transaction_id = self.client.generate(endpoint).await?;

            self.add_ctid(endpoint, &transaction_id)?;

            Ok(transaction_id)
        }
    }

    pub fn lookup_ctid(
        &self,
        endpoint: &Endpoint<'_>,
    ) -> Result<Option<TransactionId>, rusqlite::Error> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached(LOOKUP_CTID_FOR_ENDPOINT)?;

        statement
            .query_one((&endpoint.name, &endpoint.version), |row| {
                let value = row.get(0)?;
                let added: types::Timestamp = row.get(1)?;

                Ok(TransactionId::new(value, added.into()))
            })
            .optional()
    }

    pub fn add_ctid(
        &self,
        endpoint: &Endpoint<'_>,
        transaction_id: &TransactionId,
    ) -> Result<i64, rusqlite::Error> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached(ADD_CTID)?;

        statement.query_one(
            (&endpoint.name, &endpoint.version, &transaction_id.value),
            |row| row.get(0),
        )
    }

    pub fn expire_ctid(&self, value: &str) -> Result<bool, rusqlite::Error> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached(EXPIRE_CTID)?;

        let result = statement.execute((value,))?;

        Ok(result == 1)
    }
}

#[cfg(test)]
mod tests {
    use birdsite_graphql_ctid::{Endpoint, TransactionId};
    use chrono::Utc;

    #[test]
    fn load_schema_in_memory() -> Result<(), super::Error> {
        let store = super::Store::create_in_memory()?;
        let endpoint = Endpoint::new("Test", "AAAaaA");
        let transaction_id = TransactionId::new("XyZaBc".to_string(), Utc::now());

        let found_transaction_id = store.lookup_ctid(&endpoint)?;
        assert_eq!(found_transaction_id, None);

        let new_id = store.add_ctid(&endpoint, &transaction_id)?;
        assert_eq!(new_id, 1);

        let found_transaction_id = store.lookup_ctid(&endpoint)?;
        assert_eq!(found_transaction_id.as_ref(), Some(&transaction_id));

        Ok(())
    }

    #[test]
    fn load_schema_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("test.db");

        let store = super::Store::open_or_create(&path)?;
        let endpoint = Endpoint::new("Test", "AAAaaA");
        let transaction_id = TransactionId::new("XyZaBc".to_string(), Utc::now());

        let found_transaction_id = store.lookup_ctid(&endpoint)?;
        assert_eq!(found_transaction_id, None);

        let new_id = store.add_ctid(&endpoint, &transaction_id)?;
        assert_eq!(new_id, 1);

        let found_transaction_id = store.lookup_ctid(&endpoint)?;
        assert_eq!(found_transaction_id.as_ref(), Some(&transaction_id));

        let reopened_store = super::Store::open_or_create(&path)?;

        let found_transaction_id = reopened_store.lookup_ctid(&endpoint)?;
        assert_eq!(found_transaction_id.as_ref(), Some(&transaction_id));

        Ok(())
    }
}
