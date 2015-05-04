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


fn build_response_json() -> String {

    /*let result : i16 = match con {
        Some(c) => match c.get("people_present") {
            Ok(count) => count,
            Err(e) => -2
        },
        None => -1
    };
    println!("People present: {}", result);
    */

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
        issue_report_channels: ["email", "twitter"],
        feeds: spaceapi::Feeds {
            blog: spaceapi::Feed {
                _type: "rss".to_string(),
                url: "https://www.coredump.ch/feed/".to_string(),
            },
        },
        projects: [
            "https://www.coredump.ch/projekte/",
            "https://discourse.coredump.ch/c/projects",
            "https://github.com/coredump-ch/"
        ]
    };
    json::encode(&status).unwrap()
}

fn status_endpoint(_: Request, res: Response<Fresh>) {
    let mut res = res.start().unwrap();


    /*
    let con : Option<Connection> = match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => match client.get_connection() {
            Ok(con) => Some(con),
            Err(_) => None
        },
        Err(_) => None
    };
    */

    let response_body = build_response_json();
    res.write_all(response_body.as_bytes()).unwrap();
    res.end().unwrap();
}

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = utils::get_port();

    println!("Starting HTTP server on {}:{}...", ip, port);
    Server::http(status_endpoint).listen((ip, port)).unwrap();
}
