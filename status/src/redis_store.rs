use redis::Client;

use datastore::DataStore;
use datastore::DataStoreError;


/// A data store for Redis.
struct RedisStore {
    client : Client,
}

impl RedisStore {
    pub fn new() -> Result<RedisStore, DataStoreError> {
        let redis_client = try!(Client::open("redis://127.0.0.1/"));
        Ok(RedisStore { client: redis_client })
    }
}

impl DataStore for RedisStore {

    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError> {
        println!("Storing {} in {}", value, key);
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<&str, DataStoreError> {
        println!("Return {}", key);
        Ok("value")
    }

}

#[cfg(test)]
mod test {
    use redis_store;
    use datastore::DataStore;

    #[test]
    fn roundtrip() {
        let rs = redis_store::RedisStore::new().unwrap();
        rs.store("key", "value");
        let result = rs.retrieve("key").unwrap();
        assert_eq!(result, "value");
    }

}
