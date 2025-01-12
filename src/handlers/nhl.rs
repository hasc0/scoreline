// nhl module defined in ../handlers.rs

use ureq::{Response, Error, serde_json};

use serde::Deserialize;

use crate::RequestInfo;
use crate::teams::Team;

#[derive(Deserialize)]
struct Events {

}

pub fn nhl_handler(response: Response, request: RequestInfo) -> Result<(), serde_json::Error> {
    let response_de: Events;
    match response.into_json() {
        Ok(json) => response_de = json,
        Err(err) => println!("An error occurred while deserializing the response: {}", err),
    }

    let header: String;
    match request.team {
        Some(team) => header = String::from(""),
        None => header = String::from(""),
    }

    return Ok(())
}
