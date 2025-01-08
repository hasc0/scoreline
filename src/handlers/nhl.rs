// nhl module defined in ../handlers.rs

use ureq::serde_json;
use ureq::serde_json::{Value, Map};

use crate::RequestInfo;
use crate::teams::Team;

pub fn nhl_handler(response: Value, request: RequestInfo) {
    println!("team: {}", request.team.unwrap().team);

    let v: &Map<String, Value> = response.as_object().unwrap();
    let w: Value = v.get("events").unwrap().clone();
    println!("w: {}", w);
}
