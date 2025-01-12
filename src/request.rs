// request module defined in ./main.rs

use ureq::{get, Response, Error};
use chrono::prelude::*;
use chrono::NaiveDate;

use crate::utils::{RequestInfo, League, Stat};
use crate::handlers::{mlb_handler, nba_handler, nfl_handler, nhl_handler};

pub fn parse_request(mut request: RequestInfo) {
    let league: League =  request.league.clone().unwrap();

    let league_str: &str;
    let sport_str: &str;

    match league {
        League::MLB => {
            sport_str = "baseball";
            league_str = "mlb";
        }
        League::NBA => {
            sport_str = "basketball";
            league_str = "nba";
        }
        League::NFL => {
            sport_str = "football";
            league_str = "nfl";
        }
        League::NHL => {
            sport_str = "hockey";
            league_str = "nhl";
        }
    }

    let date_str: String;
    match request.date {
        Some(date) => date_str = date.to_string(),
        None => {
            let curr_date: NaiveDate = Local::now().naive_local().into();
            request.date = Some(curr_date);
            date_str = curr_date.to_string();
        },
    }

    let response: Response;
    match request.stat.clone().unwrap() {
        Stat::Score => response = score_request(sport_str, league_str, date_str),
        // Stat::Record => response = record_request(sport_str, league_str, request.date),
    }

    match league {
        League::MLB => _ = mlb_handler(response, request),
        League::NBA => _ = nba_handler(response, request),
        League::NFL => _ = nfl_handler(response, request),
        League::NHL => _ = nhl_handler(response, request),
    }
}

fn score_request(sport: &str, league: &str, date: String) -> Response {
    let date_split: Vec<&str> = date.split("-").collect();
    let date_url: String = format!("?dates={}{}{}", date_split[0], date_split[1], date_split[2]);

    let req_url: String = format!("https://site.api.espn.com/apis/site/v2/sports/{sport}/{league}/scoreboard{date_url}");
    let response: Result<Response, Error>  = get(&req_url).call();

    match response {
        Ok(res) => return res,
        Err(err) => panic!("An error occurred when making the request: {}", err),
    }
}
