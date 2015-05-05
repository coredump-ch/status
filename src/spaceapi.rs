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

/// Adapted from the generated code.
/// This is required to translate the `_type` field in the struct to a JSON field called `type`.
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
pub struct Sensors {
    pub people_now_present: Vec<PeopleNowPresentSensor>,
    pub temperature: Vec<TemperatureSensor>,
}

#[derive(RustcEncodable)]
pub struct PeopleNowPresentSensor {
    pub value: u32,
    pub location: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(RustcEncodable)]
pub struct TemperatureSensor {
    pub value: f32,
    pub unit: String,
    pub location: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(RustcEncodable)]
pub struct Cache {
    pub schedule: String,
}

#[derive(RustcEncodable)]
pub struct Status {
    pub api: String,
    pub space: String,
    pub logo: String,
    pub url: String,
    pub location: Location,
    pub spacefed: SpaceFED,
    pub cache: Cache,

    pub state: State,
    pub contact: Contact,
    pub issue_report_channels: Vec<String>,

    pub feeds: Feeds,
    pub projects: Vec<String>,
    pub sensors: Sensors,
}
