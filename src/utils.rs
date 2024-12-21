// utils module defined in ./main.rs

use chrono;

use crate::teams::{mlb_teams, nba_teams, nfl_teams, nhl_teams, Team};

#[derive(Default)]
pub struct RequestInfo {
    league: Option<League>,
    stat: Option<Stat>,
    date: Option<String>,
    team: Option<Team>,
}

pub enum League {
    MLB,
    NBA,
    NFL,
    NHL,
}

pub enum Stat {
    Score,
    Record,
}

pub fn parse_args(args: Vec<String>) -> Option<RequestInfo> {
    let mut request: RequestInfo = RequestInfo::default();

    if args.len() <= 2 {
        println!("Input a league (e.g. nhl) followed by a stat type (e.g. score). Optionally include a team (e.g. sabres) and/or a date (MM/DD/YYYY).");
        // TODO: Move these print statements to main, maybe prompt for each individual input.
        println!("Note: All entries should be entered in lowercase.");
        println!("Note: Team names comprised of multiple words (e.g. Blue Jays) should be entered as one word (e.g. bluejays)");
        return None;
    }

    if args.len() > 2 {
        match args[1].as_str() {
            "mlb" => request.league = Some(League::MLB),
            "nba" => request.league = Some(League::NBA),
            "nfl" => request.league = Some(League::NFL),
            "nhl" => request.league = Some(League::NHL),
            _ => {
                println!("Enter a valid league.");
                return None;
            }
        }

        match args[2].as_str() {
            "score" => request.stat = Some(Stat::Score),
            "record" => request.stat = Some(Stat::Record),
            _ => {
                println!("Only score and record are currently supported.");
            }
        }
    }

    if args.len() > 3 {
        if parse_team(&request, &args[2], &args[3]) == false {
            return None;
        }
    }

    if args.len() > 4 {}

    if args.len() > 5 {
        println!("Too many arguments supplied.");
        return None;
    }

    return Some(request);
}

fn parse_team(request: &RequestInfo, league: &str, team: &str) -> bool {
    return true;
}
