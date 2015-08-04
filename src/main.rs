//! Coredump implementation of the Space API server.
//!
//! Start this via the commandline:
//!
//!     ./coredump_status [-p PORT] [-i IP]

extern crate docopt;
extern crate rustc_serialize;
extern crate spaceapi_server;

mod utils;

use std::sync::{Mutex,Arc};
use docopt::Docopt;
use spaceapi_server::SpaceapiServer;
use spaceapi_server::api;
use spaceapi_server::api::sensors::{TemperatureSensorTemplate, PeopleNowPresentSensorTemplate};
use spaceapi_server::datastore::{DataStore, RedisStore};
use utils::Ipv4;


static USAGE: &'static str = "
Usage: coredump_status [-p PORT] [-i IP]

Options:
    -p PORT  The port to listen on [default: 3000].
    -i IP    The ipv4 address to listen on [default: 127.0.0.1].
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_p: u16,
    flag_i: Ipv4,
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    // Parse arguments
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());
    let host = args.flag_i.ip;
    let port = args.flag_p;

    // TODO: Create variables for all params
    let status = api::Status::new(
        "coredump".to_string(),
        "https://www.coredump.ch/logo.png".to_string(),
        "https://www.coredump.ch/".to_string(),
        api::Location {
            address: api::Optional::Value("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".to_string()),
            lat: 47.22936,
            lon: 8.82949,
        },
        api::Contact {
            irc: api::Optional::Value("irc://freenode.net/#coredump".to_string()),
            twitter: api::Optional::Value("@coredump_ch".to_string()),
            foursquare: api::Optional::Value("525c20e5498e875d8231b1e5".to_string()),
            email: api::Optional::Value("danilo@coredump.ch".to_string()),
        },
        vec![
            "email".to_string(),
            "twitter".to_string(),
        ],
    );

    // Set up datastore
    let datastore = Arc::new(Mutex::new(Box::new(RedisStore::new().unwrap()) as Box<DataStore>));

    // Set up server
    let mut server = SpaceapiServer::new(host, port, status, datastore);

    // Register sensors
    server.register_sensor(Box::new(TemperatureSensorTemplate {
        unit: "°C".to_string(),
        location: "Hackerspace".to_string(),
        name: api::Optional::Value("Raspberry CPU".to_string()),
        description: api::Optional::Absent,
    }), "raspi_temperature".to_string());
    server.register_sensor(Box::new(PeopleNowPresentSensorTemplate {
        location: api::Optional::Value("Hackerspace".to_string()),
        name: api::Optional::Absent,
        description: api::Optional::Absent,
        names: api::Optional::Absent,
    }), "people_present".to_string());

    // Serve!
    server.serve();
}
