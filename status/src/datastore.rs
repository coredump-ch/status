use std::fmt;
use std::error::Error;

use redis;


/// A DataStore needs to implement store and retrieve methods.
pub trait DataStore {
    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError>;
    fn retrieve(&self, key: &str) -> Result<String, DataStoreError>;
}

/// A struct representing a datastore error.
#[derive(Debug)]
pub struct DataStoreError {
    repr: ErrorKind,
}

/// An enum containing all possible error kinds.
#[derive(Debug)]
enum ErrorKind {
    RedisError(redis::RedisError),
}

impl From<redis::RedisError> for DataStoreError {
    fn from(err: redis::RedisError) -> DataStoreError {
        DataStoreError { repr: ErrorKind::RedisError(err) }
    }
}
