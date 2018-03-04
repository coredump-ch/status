//! Coredump implementation of the Space API server.
//!
//! Start this via the commandline:
//!
//!     ./coredump_status [-p PORT] [-i IP]
extern crate env_logger;
extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate spaceapi_server;

use std::env;
use docopt::Docopt;
use spaceapi_server::SpaceapiServerBuilder;
use spaceapi_server::api;
use spaceapi_server::modifiers::StateFromPeopleNowPresent;
use spaceapi_server::api::sensors::{TemperatureSensorTemplate, PeopleNowPresentSensorTemplate};

static USAGE: &'static str = "
Usage: coredump_status [-p PORT] [-i IP]

Options:
    -p PORT  The port to listen on [default: 3000].
    -i IP    The IP address to listen on [default: 127.0.0.1].
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_p: u16,
    flag_i: std::net::IpAddr,
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    env_logger::init().unwrap();

    // Parse arguments
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize())
                                       .unwrap_or_else(|e| e.exit());
    let host = args.flag_i;
    let port = args.flag_p;

    // Create new Status instance
    let mut status = api::StatusBuilder::new("coredump")
        .logo("https://www.coredump.ch/wp-content/uploads/2016/11/logo.png")
        .url("https://www.coredump.ch/")
        .location(api::Location {
            address: Some("Holzwiesstrasse 50, 8645 Jona, Switzerland".into()),
            lat: 47.2359607,
            lon: 8.8410057,
        })
        .contact(api::Contact {
            irc: Some("irc://freenode.net/#coredump".into()),
            twitter: Some("@coredump_ch".into()),
            email: Some("vorstand@lists.coredump.ch".into()),
            ..Default::default()
        })
        .add_issue_report_channel("email")
        .add_issue_report_channel("twitter")
        .spacefed(api::Spacefed {
            spacenet: false,
            spacesaml: false,
            spacephone: false,
        })
        .feeds(api::Feeds {
            blog: Some(api::Feed {
                type_: Some("rss".into()),
                url: "https://www.coredump.ch/feed/".into(),
            }),
            wiki: None,
            calendar: None,
            flickr: None,
        })
        .add_project("https://www.coredump.ch/projekte/")
        .add_project("https://forum.coredump.ch/c/projects")
        .add_project("https://github.com/coredump-ch/")
        .add_cam("https://webcam.coredump.ch/cams/ultimaker_0.jpg")
        .build()
        .expect("Couldn't create status object");

    status.state.message = Some("Open Mondays from 20:00".into());

    // Redis connection info
    let redis_host: String = env::var("REDIS_HOST")
        .unwrap_or("127.0.0.1".to_string());
    let redis_port: u16 = env::var("REDIS_PORT")
        .unwrap_or("6379".to_string()).parse().unwrap_or(6379);
    let redis_db: i64 = env::var("REDIS_DB")
        .unwrap_or("0".to_string()).parse().unwrap_or(0);
    let redis_url = format!("redis://{}:{}/{}", redis_host, redis_port, redis_db);

    SpaceapiServerBuilder::new(status)
        .redis_connection_info(&*redis_url)
        .add_status_modifier(StateFromPeopleNowPresent)
        .add_sensor(TemperatureSensorTemplate {
            unit: "°C".into(),
            location: "Hackerspace".into(),
            name: Some("Raspberry CPU".into()),
            description: None,
        }, "temperature_raspi".into())
        .add_sensor(TemperatureSensorTemplate {
            unit: "°C".into(),
            location: "Hackerspace".into(),
            name: Some("Room Temperature (Entrance)".into()),
            description: None,
        }, "temperature_entrance".into())
        .add_sensor(TemperatureSensorTemplate {
            unit: "°C".into(),
            location: "Hackerspace".into(),
            name: Some("Room Temperature (Tables)".into()),
            description: None,
        }, "temperature_tables".into())
        .add_sensor(PeopleNowPresentSensorTemplate {
            location: Some("Hackerspace".into()),
            name: None,
            description: None,
            names: None,
        }, "people_now_present".into())
        .build()
        .expect("Could not build server")
        .serve((host, port)).expect("Could not start the server");
}
