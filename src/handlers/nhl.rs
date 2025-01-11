// nhl module defined in ../handlers.rs

use ureq::serde_json;
use ureq::serde_json::{Value, Map};

use crate::RequestInfo;
use crate::teams::Team;

pub fn nhl_handler(response: Value, request: RequestInfo) {
    let response_obj: &Map<String, Value> = response.as_object().unwrap();
    let events: Value = response_obj.get("events").unwrap().clone();
    let events_obj: &Map<String, Value> = events.as_object().unwrap();
    // for event in events_obj {
    //     println!("event: {:?}", event);
    // }
    println!("isobj: {}", events.is_object());
}
