//! The main entry point for the SpaceAPi server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

extern crate spaceapi;
extern crate spaceapi_server;

use std::sync::{Mutex,Arc};

use std::net::Ipv4Addr;
use spaceapi::{Status, Location, Contact, Optional};
use spaceapi_server::SpaceapiServer;
use spaceapi_server::datastore::DataStore;
use spaceapi_server::redis_store::RedisStore;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let host = Ipv4Addr::new(127, 0, 0, 1);

    // TODO: Create variables for all params
    let status = Status::new(
        "coredump".to_string(),
        "https://www.coredump.ch/logo.png".to_string(),
        "https://www.coredump.ch/".to_string(),
        Location {
            address: Optional::Value("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".to_string()),
            lat: 47.22936,
            lon: 8.82949,
        },
        Contact {
            irc: Optional::Value("irc://freenode.net/#coredump".to_string()),
            twitter: Optional::Value("@coredump_ch".to_string()),
            foursquare: Optional::Value("525c20e5498e875d8231b1e5".to_string()),
            email: Optional::Value("danilo@coredump.ch".to_string()),
        },
        vec![
            "email".to_string(),
            "twitter".to_string(),
        ],
    );

    let datastore = Arc::new(Mutex::new( Box::new( RedisStore::new().unwrap()) as Box<DataStore> ));
    let server = SpaceapiServer::new(host, status, datastore);
    server.serve();
}
