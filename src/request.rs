// request module defined in ./main.rs

use ureq;
use chrono::prelude::*;
use chrono::NaiveDate;

use crate::utils::{RequestInfo, League, Stat};
use crate::deserialize::{mlb_handler, nba_handler, nfl_handler, nhl_handler};

pub fn parse_request(request: RequestInfo) {
    let league: League =  request.league.unwrap();

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

    let response: String;

    match request.stat.unwrap() {
        Stat::Score => response = score_request(sport_str, league_str, request.date),
        Stat::Record => response = record_request(sport_str, league_str, request.date),
    }

    match league {
        League::MLB => mlb_handler(response, request.team),
        League::NBA => nba_handler(response, request.team),
        League::NFL => nfl_handler(response, request.team),
        League::NHL => nhl_handler(response, request.team),
    }
}

fn score_request(sport: &str, league: &str, date: Option<NaiveDate>) -> String {
    let date_str: String;

    match date {
        Some(date) => date_str = date.to_string(),
        None => date_str = String::from(""),
    }

    let date_url: String;
    if date_str == "" {
        let curr_date: NaiveDate = Local::now().naive_local().into();
        let curr_date_str: String = curr_date.to_string();
        let date_split: Vec<&str> = curr_date_str.split("-").collect();
        date_url = format!("?dates={}{}{}", date_split[0], date_split[1], date_split[2]);
    } else {
        let date_split: Vec<&str> = date_str.split("-").collect();
        date_url = format!("?dates={}{}{}", date_split[0], date_split[1], date_split[2]);
    }

    let req_url: String = format!("site.api.espn.com/apis/site/v2/sports/{sport}/{league}/scoreboard{date_url}");

    return String::from("temp");
}

fn record_request(sport: &str, league: &str, date: Option<NaiveDate>) -> String {
    let year_str: String;

    match date {
        Some(date) => year_str = date.year().to_string(),
        None => year_str = String::from(""),
    }

    let date_url: String;
    if year_str == "" {
        let curr_date: NaiveDate = Local::now().naive_local().into();
        let curr_year_str: String = curr_date.year().to_string();
        date_url = format!("?season={}", curr_year_str);
    } else {
        date_url = format!("?season={year_str}");
    }

    let req_url: String = format!("site.api.espn.com/apis/site/v2/sports/{sport}/{league}/standings{date_url}");

    return String::from("temp");
}
