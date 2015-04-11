extern crate rustc_serialize;
use rustc_serialize::{Encodable, Encoder};


#[derive(RustcEncodable)]
pub struct Location {
    pub address: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(RustcEncodable)]
pub struct State {
    pub open: bool,
    pub message: String,
}

#[derive(RustcEncodable)]
pub struct Contact {
    pub irc: String,
    pub twitter: String,
    pub foursquare: String,
    pub email: String,
}

#[derive(RustcEncodable)]
pub struct SpaceFED {
    pub spacenet: bool,
    pub spacesaml: bool,
    pub spacephone: bool,
}

pub struct Feed {
    pub _type: String,
    pub url: String,
}

// adapted from the generated code
impl Encodable for Feed {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Feed", 2usize, |enc| {
            try!(
                enc.emit_struct_field("type", 0usize, |enc| self._type.encode(enc))
            );
            enc.emit_struct_field("url", 1usize, |enc| self.url.encode(enc))
        })
    }
}

#[derive(RustcEncodable)]
pub struct Feeds {
    pub blog: Feed,
}

#[derive(RustcEncodable)]
pub struct Status {
    pub api: String,
    pub space: String,
    pub logo: String,
    pub url: String,
    pub location: Location,
    pub spacefed: SpaceFED,

    pub state: State,
    pub contact: Contact,
    pub issue_report_channels: [&'static str; 2],

    pub feeds: Feeds,
    pub projects: [&'static str; 3],
}
