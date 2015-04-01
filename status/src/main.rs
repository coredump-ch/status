extern crate rustc_serialize;
use rustc_serialize::json;

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

    projects: [&'static str; 3],
}

fn main() {

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
        projects: [
            "https://www.coredump.ch/projekte/",
            "https://discourse.coredump.ch/c/projects",
            "https://github.com/coredump-ch/"
        ]
    };
    let encoded = json::encode(&status).unwrap();
    println!("{}", encoded);
}
