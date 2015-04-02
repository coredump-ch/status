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

#[derive(RustcEncodable)]
pub struct Feed {
    pub _type: String,  // TODO: Convert this to "type" somehow
    pub url: String,
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
