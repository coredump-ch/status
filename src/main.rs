//! Coredump implementation of the Space API server.
//!
//! Start this via the commandline:
//!
//!     ./coredump_status [-p PORT] [-i IP]

use docopt::Docopt;
use serde_derive::Deserialize;
use spaceapi_server::api;
use spaceapi_server::api::sensors::{
    PeopleNowPresentSensorTemplate, SensorMetadata, SensorMetadataWithLocation, TemperatureSensorTemplate,
};
use spaceapi_server::modifiers::StateFromPeopleNowPresent;
use spaceapi_server::SpaceapiServerBuilder;
use std::env;

static USAGE: &str = "
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
    env_logger::init();

    // Parse arguments
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let host = args.flag_i;
    let port = args.flag_p;

    // Create new Status instance
    let status = api::StatusBuilder::mixed("coredump")
        .logo("https://www.coredump.ch/wp-content/uploads/2016/11/logo.png")
        .url("https://www.coredump.ch/")
        .location(api::Location {
            address: Some("Lenzikon 32B, 8732 Neuhaus, Switzerland".into()),
            lat: 47.225_1,
            lon: 8.833_9,
            timezone: None,
        })
        .contact(api::Contact {
            irc: Some("irc://irc.libera.chat/#coredump".into()),
            twitter: Some("@coredump_ch".into()),
            email: Some("vorstand@lists.coredump.ch".into()),
            ..Default::default()
        })
        .add_issue_report_channel(api::IssueReportChannel::Email)
        .add_issue_report_channel(api::IssueReportChannel::Twitter)
        .spacefed(api::Spacefed {
            spacenet: false,
            spacesaml: false,
            spacephone: Some(false),
        })
        .feeds(api::Feeds {
            blog: Some(api::Feed {
                type_: Some("rss".into()),
                url: "https://www.coredump.ch/feed/".into(),
            }),
            wiki: None,
            calendar: Some(api::Feed {
                type_: Some("ical".into()),
                url: "https://www.coredump.ch/events/?ical=1".into(),
            }),
            flickr: None,
        })
        .add_project("https://www.coredump.ch/projekte/")
        .add_project("https://forum.coredump.ch/c/projects")
        .add_project("https://github.com/coredump-ch/")
        .add_cam("https://webcam.coredump.ch/cams/ultimaker_0.jpg")
        .add_extension("ccc", "chaostreff")
        .state(api::State {
            message: Some("Open Mondays from 20:00".into()),
            ..api::State::default()
        })
        .build()
        .expect("Couldn't create status object");

    // Redis connection info
    let redis_host: String = env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let redis_port: u16 = env::var("REDIS_PORT")
        .unwrap_or_else(|_| "6379".to_string())
        .parse()
        .unwrap_or(6379);
    let redis_db: i64 = env::var("REDIS_DB")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .unwrap_or(0);
    let redis_url = format!("redis://{}:{}/{}", redis_host, redis_port, redis_db);

    // Create server
    let server = SpaceapiServerBuilder::new(status)
        .redis_connection_info(&*redis_url)
        .add_status_modifier(StateFromPeopleNowPresent)
        .add_sensor(
            TemperatureSensorTemplate {
                unit: "°C".into(),
                metadata: SensorMetadataWithLocation {
                    location: "Hackerspace".into(),
                    name: Some("Raspberry CPU".into()),
                    description: None,
                },
            },
            "temperature_raspi".into(),
        )
        .add_sensor(
            TemperatureSensorTemplate {
                unit: "°C".into(),
                metadata: SensorMetadataWithLocation {
                    location: "Hackerspace".into(),
                    name: Some("Room Temperature (Sensor 1, Entrance)".into()),
                    description: None,
                },
            },
            "temperature_entrance".into(),
        )
        .add_sensor(
            TemperatureSensorTemplate {
                unit: "°C".into(),
                metadata: SensorMetadataWithLocation {
                    location: "Hackerspace".into(),
                    name: Some("Room Temperature (Sensor 2, Windows)".into()),
                    description: None,
                },
            },
            "temperature_windows".into(),
        )
        .add_sensor(
            PeopleNowPresentSensorTemplate {
                metadata: SensorMetadata {
                    location: Some("Hackerspace".into()),
                    name: None,
                    description: None,
                },
            },
            "people_now_present".into(),
        )
        .build()
        .expect("Could not build server");

    server.serve((host, port)).expect("Could not start the server");
}
