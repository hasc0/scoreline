// utils module defined in ./main.rs

use std::collections::HashMap;

use chrono::{NaiveDate, ParseResult};
use chrono::prelude::Local;

use crate::teams::{mlb_teams, nba_teams, nfl_teams, nhl_teams, Team};

#[derive(Debug, Default)]
pub struct RequestInfo {
    pub league: Option<League>,
    pub stat: Option<Stat>,
    pub team: Option<Team>,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub enum League {
    MLB,
    NBA,
    NFL,
    NHL,
}

#[derive(Debug, Clone)]
pub enum Stat {
    Score,
    // Record,
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
            // "record" => request.stat = Some(Stat::Record),
            _ => {
                println!("Only score is currently supported.");
            }
        }
    }

    if args.len() > 3 {
        if !parse_team(&mut request, args[1].as_str(), args[3].as_str()) && !parse_date(&mut request, args[3].as_str()) {
            println!("Enter a valid team and/or date.");
            return None;
        }
    }

    if args.len() > 4 {
        if !parse_date(&mut request, args[4].as_str()) {
            println!("Date must be entered in MM/DD/YYYY format.");
            return None;
        }
    }

    if args.len() > 5 {
        println!("Too many arguments supplied.");
        return None;
    }

    return Some(request);
}

fn parse_team(request: &mut RequestInfo, league: &str, team: &str) -> bool {
    let team_info: Option<Team>;

    if league == "mlb" {
        let mlb_teams: HashMap<&str, Team> = mlb_teams();
        team_info = mlb_teams.get(team).cloned();
    } else if league == "nba" {
        let nba_teams: HashMap<&str, Team> = nba_teams();
        team_info = nba_teams.get(team).cloned();
    } else if league == "nfl" {
        let nfl_teams: HashMap<&str, Team> = nfl_teams();
        team_info = nfl_teams.get(team).cloned();
    } else if league == "nhl" {
        let nhl_teams: HashMap<&str, Team> = nhl_teams();
        team_info = nhl_teams.get(team).cloned();
    } else {
        println!("Enter a valid league.");
        return false;
    }

    match team_info {
        Some(team) =>  request.team = Some(team),
        None => return false,
    }

    return true;
}

// TODO: Add default date entry, remove the need for Option<NaiveDate>
fn parse_date(request: &mut RequestInfo, date: &str) -> bool {
    let date: ParseResult<NaiveDate> = NaiveDate::parse_from_str(date, "%m/%d/%Y");
    let curr_date: NaiveDate = Local::now().naive_local().into();
    let user_date: NaiveDate;

    match date {
        Ok(date) => user_date = date,
        Err(..) => {
            return false;
        }
    }

    if user_date > curr_date {
        println!("Entered must not exceed the current date.");
        return false;
    } else {
        request.date = Some(user_date);
    }

    return true;
}
