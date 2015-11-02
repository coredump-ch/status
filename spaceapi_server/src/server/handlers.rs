//! Handlers for the server.

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};
use iron::prelude::*;
use iron::{status, headers, middleware};
use iron::modifiers::Header;
use router::Router;

use urlencoded;

use ::api;
use ::api::optional::Optional;
use ::datastore;
use ::sensors;


#[derive(Debug)]
struct ErrorResponse {
    reason: String,
}

impl ToJson for ErrorResponse {
    /// Serialize an ErrorResponse object into a proper JSON structure.
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("status".to_string(), "error".to_json());
        d.insert("reason".to_string(), self.reason.to_json());
        Json::Object(d)
    }
}


pub struct ReadHandler {
    status: api::Status,
    datastore: datastore::SafeDataStore,
    sensor_specs: sensors::SafeSensorSpecs,
}

impl ReadHandler {
    pub fn new(status: api::Status, datastore: datastore::SafeDataStore, sensor_specs: sensors::SafeSensorSpecs) -> ReadHandler {
        ReadHandler {
            status: status,
            datastore: datastore,
            sensor_specs: sensor_specs,
        }
    }

    fn build_response_json(&self) -> Json {

        // Create a mutable copy of the status struct
        let mut status_copy = self.status.clone();

        // Process registered sensors
        for sensor_spec in self.sensor_specs.lock().unwrap().iter() {
            sensor_spec.get_sensor_value(self.datastore.clone()).map(|value| {
                if status_copy.sensors.is_absent() {
                    status_copy.sensors = Optional::Value(api::Sensors {
                        people_now_present: Optional::Absent,
                        temperature: Optional::Absent,
                    });
                }
                sensor_spec.template.to_sensor(&value, &mut status_copy.sensors.as_mut().unwrap());
            });
        }

        status_copy.state.open = status_copy.sensors.as_ref()
            .and_then(|sensors| sensors.people_now_present.as_ref())
            .and_then(|people_now_present| {
                match people_now_present[0].value {
                    0i64 => Optional::Value(false),
                    _ => Optional::Value(true),
                }
            }).into();

        // Serialize to JSON
        status_copy.to_json()
    }
}

impl middleware::Handler for ReadHandler {

    /// Return the current status JSON.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        println!("{} /{} from {}", req.method, req.url.path[0], req.remote_addr);

        // Get response body
        let body = self.build_response_json().to_string();

        // Create response
        let response = Response::with((status::Ok, body))
            // Set headers
            .set(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())))
            .set(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])))
            .set(Header(headers::AccessControlAllowOrigin::Any));

        Ok(response)
    }

}


pub struct UpdateHandler {
    datastore: datastore::SafeDataStore,
    sensor_specs: sensors::SafeSensorSpecs,
}

impl UpdateHandler {
    pub fn new(datastore: datastore::SafeDataStore, sensor_specs: sensors::SafeSensorSpecs) -> UpdateHandler {
        UpdateHandler {
            datastore: datastore,
            sensor_specs: sensor_specs,
        }
    }

    fn ok_response(&self) -> Response {
        Response::with((status::NoContent))
            // Set headers
            .set(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())))
            .set(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])))
            .set(Header(headers::AccessControlAllowOrigin::Any))
    }

    fn err_response(&self, reason: &str) -> Response {
        let error = ErrorResponse { reason: reason.to_string() };
        Response::with((status::BadRequest, error.to_json().to_string()))
            // Set headers
            .set(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())))
            .set(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])))
            .set(Header(headers::AccessControlAllowOrigin::Any))
    }

}

impl middleware::Handler for UpdateHandler {

    /// Update the sensor, return correct status code.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        // TODO: create macro for these println! invocations.
        println!("{} /{} from {}", req.method, req.url.path[0], req.remote_addr);

        // Get sensor name
        let sensor_name;
        {
            // TODO: Properly propagate errors
            let params = req.extensions.get::<Router>().unwrap();
            sensor_name = params.find("sensor").unwrap().to_string();
        }

        // Get sensor value
        let sensor_value;
        {
            let params = req.get_ref::<urlencoded::UrlEncodedBody>().unwrap();
            sensor_value = match params.get("value") {
                Some(ref values) =>  match values.len() {
                    1 => values[0].to_string(),
                    _ => return Ok(self.err_response("Too many values specified")),
                },
                None => return Ok(self.err_response("\"value\" parameter not specified")),
            }
        }

        // Create response
        Ok(self.ok_response())
    }

}