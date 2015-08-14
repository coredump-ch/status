extern crate redis;

use std::sync::{Mutex,Arc};
use std::any::Any;

#[doc(no_inline)]
pub use self::redis::RedisError;


/// A ``DataStore`` needs to implement ``store`` and ``retrieve`` methods.
pub trait DataStore : Send + Any {
    fn store(&mut self, key: &str, value: &str) -> Result<(), DataStoreError>;
    fn retrieve(& self, key: &str) -> Result<String, DataStoreError>;
    fn delete(&mut self, key: &str) -> Result<(), DataStoreError>;
}

/// A datastore wrapped in an Arc and a Mutex. Safe for use in multithreaded situations.
pub type SafeDataStore<DS: DataStore> = Arc<Mutex<DS>>;

/// An enum representing a datastore error.
#[derive(Debug)]
pub enum DataStoreError {
    RedisError(redis::RedisError),
    HashMapError,
}

impl From<redis::RedisError> for DataStoreError {
    fn from(err: redis::RedisError) -> DataStoreError {
        DataStoreError::RedisError(err)
    }
}
