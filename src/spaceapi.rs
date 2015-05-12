extern crate rustc_serialize;
use rustc_serialize::{Encodable, Encoder};


/// An Optional value can contain Optional::Some<T> or Optional::Absent. It is similar to an
/// Option, but Optional::Absent means it will be omitted when serialized.
pub enum Optional<T> {
    Value(T),
    Absent,
}

pub struct Location {
    pub address: Optional<String>,
    pub lat: f64,
    pub lon: f64,
}

pub struct Spacefed {
    pub spacenet: bool,
    pub spacesaml: bool,
    pub spacephone: bool,
}

pub struct Icon {
    pub open: String,
    pub close: String,
}

pub struct State {
    pub open: Option<bool>,
    pub lastchange: Optional<u64>,
    pub trigger_person: Optional<String>,
    pub message: Optional<String>,
    pub icon: Optional<Icon>,
}

pub struct Event {
    pub name: String,
    pub _type: String,
    pub timestamp: u64,
    pub extra: Optional<String>,
}

pub struct Contact {
    pub irc: Optional<String>,
    pub twitter: Optional<String>,
    pub foursquare: Optional<String>,
    pub email: Optional<String>,
}

pub struct Feed {
    pub _type: Optional<String>,
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

pub struct Feeds {
    pub blog: Optional<Feed>,
    pub wiki: Optional<Feed>,
    pub calendar: Optional<Feed>,
    pub flickr: Optional<Feed>,
}

pub struct Sensors {
    pub people_now_present: Vec<PeopleNowPresentSensor>,
    pub temperature: Vec<TemperatureSensor>,
}

pub struct PeopleNowPresentSensor {
    pub value: u32,
    pub location: Optional<String>,
    pub name: Optional<String>,
    pub names: Optional<Vec<String>>,
    pub description: Optional<String>,
}

pub struct TemperatureSensor {
    pub value: f32,
    pub unit: String,
    pub location: String,
    pub name: Optional<String>,
    pub description: Optional<String>,
}

pub struct Cache {
    pub schedule: String,
}

pub struct RadioShow {
    pub name: String,
    pub url: String,
    pub _type: String,
    pub start: String,
    pub end: String,
}

pub struct Status {

    // Hackerspace properties
    pub api: String,
    pub space: String,
    pub logo: String,
    pub url: String,
    pub location: Location,
    pub contact: Contact,

    // Hackerspace features / projects
    pub spacefed: Optional<Spacefed>,
    pub projects: Optional<Vec<String>>,
    pub cam: Optional<Vec<String>>,
    pub feeds: Optional<Feeds>,
    pub events: Optional<Vec<Event>>,
    pub radio_show: Optional<Vec<RadioShow>>,

    // SpaceAPI internal usage
    pub cache: Optional<Cache>,
    pub issue_report_channels: Vec<String>,

    // Mutable data
    pub state: State,
    pub sensors: Optional<Sensors>,

}
