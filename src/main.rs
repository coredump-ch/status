//! The main entry point for the SpaceAPi server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

extern crate rustc_serialize;
extern crate docopt;

extern crate spaceapi;
extern crate spaceapi_server;

use std::sync::{Mutex,Arc};
use std::net::Ipv4Addr;
use std::str::FromStr;

use docopt::Docopt;

use spaceapi::{Status, Location, Contact, Optional};
use spaceapi_server::SpaceapiServer;
use spaceapi_server::datastore::DataStore;
use spaceapi_server::redis_store::RedisStore;


static USAGE: &'static str = "
Usage: coredump-status [-p PORT] [-i IP]

Options:
    -p PORT  The port to listen on [default: 3000].
    -i IP    The ipv4 address to listen on [default: 127.0.0.1].
";

// We have to create a wrapper because we cannot implement Decodable for types
// outside this crate (E0117).
#[derive(Debug)]
struct Ipv4 {
    ip: Ipv4Addr,
}

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_p: u16,
    flag_i: Ipv4,
}

impl rustc_serialize::Decodable for Ipv4 {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Ipv4, D::Error> {
        // Read argument as string
        let addr_str = try!(d.read_str());

        // Parse an Ipv4Addr from the string
        match Ipv4Addr::from_str(&addr_str) {

            // Yay!
            Ok(addr) => Ok(Ipv4 { ip: addr }),

            // Failed :( Generate a useful error message
            Err(_) => {
                let err_msg = format!("Failed to parse IP address: {}", addr_str);
                Err(d.error(&err_msg))
            },
        }
    }
}


fn main() {
    // Parse arguments
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    // Set the host and port
    let host = args.flag_i.ip;
    let port = args.flag_p;

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
    let server = SpaceapiServer::new(host, port, status, datastore);
    server.serve();
}
