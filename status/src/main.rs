extern crate rustc_serialize;
extern crate hyper;

use std::io::Write;
use std::net::Ipv4Addr;

use rustc_serialize::json;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;


#[derive(RustcEncodable)]
pub struct Location {
    address: String,
    lat: f64,
    lon: f64,
}

#[derive(RustcEncodable)]
pub struct State {
    open: bool,
    message: String,
}

#[derive(RustcEncodable)]
pub struct Contact {
    irc: String,
    twitter: String,
    foursquare: String,
    email: String,
}

#[derive(RustcEncodable)]
pub struct SpaceFED {
    spacenet: bool,
    spacesaml: bool,
    spacephone: bool,
}

#[derive(RustcEncodable)]
pub struct Feed {
    _type: String,  // TODO: Convert this to "type" somehow
    url: String,
}

#[derive(RustcEncodable)]
pub struct Feeds {
    blog: Feed,
}

#[derive(RustcEncodable)]
pub struct Status {
    api: String,
    space: String,
    logo: String,
    url: String,
    location: Location,
    spacefed: SpaceFED,
    
    state: State,
    contact: Contact,
    issue_report_channels: [&'static str; 2],

    feeds: Feeds,
    projects: [&'static str; 3],
}

fn build_response_json() -> String {

    let status = Status {
        api: "0.13".to_string(),
        space: "coredump".to_string(),
        logo: "https://www.coredump.ch/logo.png".to_string(),
        url: "https://www.coredump.ch/".to_string(),
        location: Location {
            address: "Spinnereistrasse 2, 8640 Rapperswil, Switzerland".to_string(),
            lat: 47.22936,
            lon: 8.82949,
        },
        spacefed: SpaceFED {
            spacenet: false,
            spacesaml: false,
            spacephone: false,
        },
        state: State {
            open: false,
            message: "Open every Monday from 20:00".to_string(),
        },
        contact: Contact {
            irc: "irc://freenode.net/#coredump".to_string(),
            twitter: "@coredump_ch".to_string(),
            foursquare: "525c20e5498e875d8231b1e5".to_string(),
            email: "danilo@coredump.ch".to_string(),
        },
        issue_report_channels: ["email", "twitter"],
        feeds: Feeds {
            blog: Feed {
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
    let response_body = build_response_json();
    res.write_all(response_body.as_bytes()).unwrap();
    res.end().unwrap();
}

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 3000;

    println!("Starting HTTP server on {}:{}...", ip, port);
    Server::http(status_endpoint).listen((ip, port)).unwrap();
}
