//! Some useful utils used in the SpaceAPI implementation.

extern crate rustc_serialize;

use std::net::Ipv4Addr;
use std::str::FromStr;


// We have to create a wrapper because we cannot implement Decodable for types
// outside this crate (E0117).
#[derive(Debug)]
pub struct Ipv4 {
    pub ip: Ipv4Addr,
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
