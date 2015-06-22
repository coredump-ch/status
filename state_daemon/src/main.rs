/// The main entry point for the people_present daemon

extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json::{self, DecoderError};
use hyper::Client;
use std::io;
use std::io::Read;

#[derive(RustcDecodable)]
struct SwitchStatus {
    open: bool,
    people: u8,
}

fn update_people_temp_raspi(host: &str, people: u8, temp_raspi: f64) {
    let mut client = Client::new();

    let values =format!("people={}&temp_raspi={}", people, temp_raspi);

    let res = client.post(host)
        .body(&values)
        .send();
    match res {
        Ok(res)  => println!("Response: {}", res.status),
        Err(e) => println!("Err: {:?}", e)
    }
}

#[derive(Debug)]
enum PeoplePresentError {
    Io(io::Error),
    Decoder(DecoderError)
}

fn get_people_present(host: &str) -> Result<u8, PeoplePresentError> {
    let mut client = Client::new();
    let mut json = String::new();
    try!(client.get(host).send().unwrap().read_to_string(&mut json)
         .map_err(PeoplePresentError::Io));
    json::decode::<SwitchStatus>(&json).map_err(PeoplePresentError::Decoder)
        .map(|status| status.people)
}

/// \todo read raspi_temp from raspberry pi
fn get_raspi_temp() -> Option<f64> {
    Some(45.0)
}

fn main() {
    loop {
        update_people_temp_raspi(
            //"http://localhost:8080/update",
            "http://status.coredump.ch/update",
            get_people_present("http://10.0.0.100:1337").unwrap(),
            get_raspi_temp().unwrap()
            );
        std::thread::sleep_ms(1000);
    }
}

