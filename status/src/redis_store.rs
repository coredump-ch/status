use redis::Client;
use redis::Commands;

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

/// Implement the DataStore methods for Redis
impl DataStore for RedisStore {

    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError> {
        let con = try!(self.client.get_connection());

        println!("Storing {} in {}", value, key);
        try!(con.set("key", "value"));
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<String, DataStoreError> {
        let con = try!(self.client.get_connection());

        println!("Return {}", key);
        Ok(try!(con.get("key")))
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
