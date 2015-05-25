/// The main entry point for the people_present daemon

extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json::{self, DecoderError};
use hyper::Client;

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

/// \todo get people_present from switches inside coredump
fn get_people_present() -> Result<u8, DecoderError> {
    // get presence counter port 1337
    json::decode::<SwitchStatus>("{\"open\": true, \"people\": 2}")
        .map(|status| status.people)
}

/// \todo read raspi_temp from raspberry pi
fn get_raspi_temp() -> Option<f64> {
    Some(45.0)
}

fn main() {
    loop {
        update_people_temp_raspi(
            "http://localhost:8080/update",
            get_people_present().unwrap(),
            get_raspi_temp().unwrap()
            );
        std::thread::sleep_ms(1000);
    }
}

