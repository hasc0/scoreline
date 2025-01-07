// request module defined in ./main.rs

use crate::utils::RequestInfo;
use crate::teams::Team;
use crate::deserialize::{mlb_handler, nba_handler, nfl_handler, nhl_handler};

pub fn parse_request(request: RequestInfo) {
    match request.league {
        Some(league) => println!("league: {:?}", league),
        None => println!("None"),
    }

    match request.stat {
        Some(stat) => println!("stat: {:?}", stat),
        None => println!("None"),
    }

    match request.team {
        Some(team) => println!("team: {:?}", team),
        None => println!("None"),
    }

    match request.date {
        Some(date) => println!("date: {:?}", date),
        None => println!("None"),
    }

    return;
}
