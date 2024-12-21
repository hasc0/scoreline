// teams module defined in ./main.rs

use std::collections::HashMap;

pub struct Team {
    team: String,
    city: String,
    slug: String,
}

pub fn mlb_teams() -> HashMap<String, Team> {
    let mut teams: HashMap<String, Team> = HashMap::new();

    teams.insert(
        String::from("angels"),
        Team {
            team: String::from("Angels"),
            city: String::from("Los Angeles"),
            slug: String::from("LAA"),
        },
    );

    teams.insert(
        String::from("astros"),
        Team {
            team: String::from("Astros"),
            city: String::from("Houston"),
            slug: String::from("HOU"),
        },
    );

    teams.insert(
        String::from("athletics"),
        Team {
            team: String::from("Athletics"),
            city: String::from("Oakland"),
            slug: String::from("OAK"),
        },
    );

    teams.insert(
        String::from("bluejays"),
        Team {
            team: String::from("Blue Jays"),
            city: String::from("Toronto"),
            slug: String::from("TOR"),
        },
    );

    teams.insert(
        String::from("braves"),
        Team {
            team: String::from("Braves"),
            city: String::from("Atlanta"),
            slug: String::from("ATL"),
        },
    );

    teams.insert(
        String::from("brewers"),
        Team {
            team: String::from("Brewers"),
            city: String::from("Milwaukee"),
            slug: String::from("MIL"),
        },
    );

    teams.insert(
        String::from("cardinals"),
        Team {
            team: String::from("Cardinals"),
            city: String::from("St. Louis"),
            slug: String::from("STL"),
        },
    );

    teams.insert(
        String::from("cubs"),
        Team {
            team: String::from("Cubs"),
            city: String::from("Chicago"),
            slug: String::from("CHC"),
        },
    );

    teams.insert(
        String::from("diamondbacks"),
        Team {
            team: String::from("Diamondbacks"),
            city: String::from("Arizona"),
            slug: String::from("ARI"),
        },
    );

    teams.insert(
        String::from("dodgers"),
        Team {
            team: String::from("Dodgers"),
            city: String::from("Los Angeles"),
            slug: String::from("LAD"),
        },
    );

    teams.insert(
        String::from("giants"),
        Team {
            team: String::from("Giants"),
            city: String::from("San Francisco"),
            slug: String::from("SF"),
        },
    );

    teams.insert(
        String::from("guardians"),
        Team {
            team: String::from("Guardians"),
            city: String::from("Cleveland"),
            slug: String::from("CLE"),
        },
    );

    teams.insert(
        String::from("mariners"),
        Team {
            team: String::from("Mariners"),
            city: String::from("Seattle"),
            slug: String::from("SEA"),
        },
    );

    teams.insert(
        String::from("marlins"),
        Team {
            team: String::from("Marlins"),
            city: String::from("Miami"),
            slug: String::from("MIA"),
        },
    );

    teams.insert(
        String::from("mets"),
        Team {
            team: String::from("Mets"),
            city: String::from("New York"),
            slug: String::from("NYM"),
        },
    );

    teams.insert(
        String::from("nationals"),
        Team {
            team: String::from("Nationals"),
            city: String::from("Washington"),
            slug: String::from("WSH"),
        },
    );

    teams.insert(
        String::from("orioles"),
        Team {
            team: String::from("Orioles"),
            city: String::from("Baltimore"),
            slug: String::from("BAL"),
        },
    );

    teams.insert(
        String::from("padres"),
        Team {
            team: String::from("Padres"),
            city: String::from("San Diego"),
            slug: String::from("SD"),
        },
    );

    teams.insert(
        String::from("phillies"),
        Team {
            team: String::from("Phillies"),
            city: String::from("Philadelphia"),
            slug: String::from("PHI"),
        },
    );

    teams.insert(
        String::from("pirates"),
        Team {
            team: String::from("Pirates"),
            city: String::from("Pittsburgh"),
            slug: String::from("PIT"),
        },
    );

    teams.insert(
        String::from("rangers"),
        Team {
            team: String::from("Rangers"),
            city: String::from("Texas"),
            slug: String::from("TEX"),
        },
    );

    teams.insert(
        String::from("rays"),
        Team {
            team: String::from("Rays"),
            city: String::from("Tampa Bay"),
            slug: String::from("TB"),
        },
    );

    teams.insert(
        String::from("redsox"),
        Team {
            team: String::from("Red Sox"),
            city: String::from("Boston"),
            slug: String::from("BOS"),
        },
    );

    teams.insert(
        String::from("reds"),
        Team {
            team: String::from("Reds"),
            city: String::from("Cincinnati"),
            slug: String::from("CIN"),
        },
    );

    teams.insert(
        String::from("rockies"),
        Team {
            team: String::from("Rockies"),
            city: String::from("Colorado"),
            slug: String::from("COL"),
        },
    );

    teams.insert(
        String::from("royals"),
        Team {
            team: String::from("Royals"),
            city: String::from("Kansas City"),
            slug: String::from("KC"),
        },
    );

    teams.insert(
        String::from("tigers"),
        Team {
            team: String::from("Tigers"),
            city: String::from("Detroit"),
            slug: String::from("DET"),
        },
    );

    teams.insert(
        String::from("twins"),
        Team {
            team: String::from("Twins"),
            city: String::from("Minnesota"),
            slug: String::from("MIN"),
        },
    );

    teams.insert(
        String::from("whitesox"),
        Team {
            team: String::from("White Sox"),
            city: String::from("Chicago"),
            slug: String::from("CHW"),
        },
    );

    teams.insert(
        String::from("yankees"),
        Team {
            team: String::from("Yankees"),
            city: String::from("New York"),
            slug: String::from("NYY"),
        },
    );

    return teams;
}

pub fn nba_teams() -> HashMap<String, Team> {
    let mut teams: HashMap<String, Team> = HashMap::new();

    teams.insert(
        String::from("76ers"),
        Team {
            team: String::from("76ers"),
            city: String::from("Philadelphia"),
            slug: String::from("PHI"),
        },
    );

    teams.insert(
        String::from("bucks"),
        Team {
            team: String::from("Bucks"),
            city: String::from("Milwaukee"),
            slug: String::from("MIL"),
        },
    );

    teams.insert(
        String::from("bulls"),
        Team {
            team: String::from("Bulls"),
            city: String::from("Chicago"),
            slug: String::from("CHI"),
        },
    );

    teams.insert(
        String::from("cavaliers"),
        Team {
            team: String::from("Cavaliers"),
            city: String::from("Cleveland"),
            slug: String::from("CLE"),
        },
    );

    teams.insert(
        String::from("celtics"),
        Team {
            team: String::from("Celtics"),
            city: String::from("Boston"),
            slug: String::from("BOS"),
        },
    );

    teams.insert(
        String::from("clippers"),
        Team {
            team: String::from("Clippers"),
            city: String::from("Los Angeles"),
            slug: String::from("LAC"),
        },
    );

    teams.insert(
        String::from("grizzlies"),
        Team {
            team: String::from("Grizzlies"),
            city: String::from("Memphis"),
            slug: String::from("MEM"),
        },
    );

    teams.insert(
        String::from("hawks"),
        Team {
            team: String::from("Hawks"),
            city: String::from("Atlanta"),
            slug: String::from("ATL"),
        },
    );

    teams.insert(
        String::from("heat"),
        Team {
            team: String::from("Heat"),
            city: String::from("Miami"),
            slug: String::from("MIA"),
        },
    );

    teams.insert(
        String::from("hornets"),
        Team {
            team: String::from("Hornets"),
            city: String::from("Charlotte"),
            slug: String::from("CHA"),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    return teams;
}

pub fn nfl_teams() -> HashMap<String, Team> {
    let mut teams: HashMap<String, Team> = HashMap::new();

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    return teams;
}

pub fn nhl_teams() -> HashMap<String, Team> {
    let mut teams: HashMap<String, Team> = HashMap::new();

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    teams.insert(
        String::from(""),
        Team {
            team: String::from(""),
            city: String::from(""),
            slug: String::from(""),
        },
    );

    return teams;
}
