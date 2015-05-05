//! The main entry point for the SpaceAPi server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

extern crate rustc_serialize;
extern crate hyper;
extern crate redis;

mod utils;
mod spaceapi;
mod datastore;
mod redis_store;

use std::io::Write;
use std::net::Ipv4Addr;

use rustc_serialize::json;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

use datastore::DataStore;


fn build_response_json(people_present: Option<u32>, raspi_temperature: Option<f32>) -> String {
    let people_present_sensor = match people_present {
        Some(count) => vec![
            spaceapi::PeopleNowPresentSensor {
                value: count,
                location: Some("Hackerspace".to_string()),
                name: None,
                description: None,
            }
        ],
        None => Vec::new(),
    };

    let temperature_sensor = match raspi_temperature {
        Some(degrees) => vec![
            spaceapi::TemperatureSensor {
                value: degrees,
                unit: "°C".to_string(),
                location: "Hackerspace".to_string(),
                name: Some("Raspberry CPU".to_string()),
                description: None,
            }
        ],
        None => Vec::new(),
    };

    let status = spaceapi::Status {
        api: "0.13".to_string(),
        space: "coredump".to_string(),
        logo: "https://www.coredump.ch/logo.png".to_string(),
        url: "https://www.coredump.ch/".to_string(),
        location: spaceapi::Location {
            address: "Spinnereistrasse 2, 8640 Rapperswil, Switzerland".to_string(),
            lat: 47.22936,
            lon: 8.82949,
        },
        spacefed: spaceapi::SpaceFED {
            spacenet: false,
            spacesaml: false,
            spacephone: false,
        },
        cache: spaceapi::Cache {
            schedule: "m.02".to_string(),
        },
        state: spaceapi::State {
            open: false,
            message: "Open every Monday from 20:00".to_string(),
        },
        contact: spaceapi::Contact {
            irc: "irc://freenode.net/#coredump".to_string(),
            twitter: "@coredump_ch".to_string(),
            foursquare: "525c20e5498e875d8231b1e5".to_string(),
            email: "danilo@coredump.ch".to_string(),
        },
        issue_report_channels: vec![
            "email".to_string(),
            "twitter".to_string(),
        ],
        feeds: spaceapi::Feeds {
            blog: spaceapi::Feed {
                _type: "rss".to_string(),
                url: "https://www.coredump.ch/feed/".to_string(),
            },
        },
        projects: vec![
            "https://www.coredump.ch/projekte/".to_string(),
            "https://discourse.coredump.ch/c/projects".to_string(),
            "https://github.com/coredump-ch/".to_string(),
        ],
        sensors: spaceapi::Sensors {
            people_now_present: people_present_sensor,
            temperature: temperature_sensor,
        },
    };
    json::encode(&status).unwrap()
}

fn status_endpoint(_: Request, res: Response<Fresh>) {
    let mut res = res.start().unwrap();

    let datastore = redis_store::RedisStore::new().unwrap();
    let people_present: Option<u32> = match datastore.retrieve("people_present") {
        Ok(v) => match v.parse::<u32>() {
            Ok(i) => Some(i),
            Err(_) => None,
        },
        Err(_) => None,
    };
    let raspi_temperature: Option<f32> = match datastore.retrieve("raspi_temperature") {
        Ok(v) => match v.parse::<f32>() {
            Ok(i) => Some(i),
            Err(_) => None,
        },
        Err(_) => None,
    };

    let response_body = build_response_json(people_present, raspi_temperature);
    res.write_all(response_body.as_bytes()).unwrap();
    res.end().unwrap();
}

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = utils::get_port();

    println!("Starting HTTP server on {}:{}...", ip, port);
    Server::http(status_endpoint).listen((ip, port)).unwrap();
}
