// handler module defined in ../main.rs

use std::io;

use ureq::serde_json;

use serde::Deserialize;

use crate::RequestInfo;

#[derive(Debug, Deserialize)]
struct Event {
    // id: String,
    // date: String,
    // short_name: String,
    competitions: Vec<String>,
}

pub fn handle_response(response: Result<String, io::Error>, request: RequestInfo) -> Result<(), serde_json::Error> {
    let response_str: String = match response {
        Ok(str) => str,
        Err(err) => panic!("An error occurred while deserializing the response: {}", err),
    };

    let events: Event = match serde_json::from_str(&response_str) {
        Ok(c) => c,
        Err(e) => panic!("An error occurred while deserializing the response: {}", e),
    };

    for c in events.competitions {
        println!("{:?}", c);
    }

    let has_team: bool;
    let header: String = match request.team {
        Some(team) => {
            has_team = true;
            format!("Score for {}, {}", team.team, request.date.unwrap().to_string())
        }
        None => {
            has_team = false;
            format!("Scores for {}", request.date.unwrap().to_string())
        }
    };

    match has_team {
        true => {

        }
        false => {

        }
    }

    Ok(())
}
