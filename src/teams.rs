// teams module defined in ./main.rs

use std::collections::HashMap;

#[derive(Clone)]
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
        String::from("jazz"),
        Team {
            team: String::from("Jazz"),
            city: String::from("Utah"),
            slug: String::from("UTH"),
        },
    );

    teams.insert(
        String::from("kings"),
        Team {
            team: String::from("Kings"),
            city: String::from("Sacramento"),
            slug: String::from("SAC"),
        },
    );

    teams.insert(
        String::from("knicks"),
        Team {
            team: String::from("Knicks"),
            city: String::from("New York"),
            slug: String::from("NYK"),
        },
    );

    teams.insert(
        String::from("lakers"),
        Team {
            team: String::from("Lakers"),
            city: String::from("Los Angeles"),
            slug: String::from("LAL"),
        },
    );

    teams.insert(
        String::from("magic"),
        Team {
            team: String::from("Magic"),
            city: String::from("Orlando"),
            slug: String::from("ORL"),
        },
    );

    teams.insert(
        String::from("mavericks"),
        Team {
            team: String::from("Mavericks"),
            city: String::from("Dallas"),
            slug: String::from("DAL"),
        },
    );

    teams.insert(
        String::from("nets"),
        Team {
            team: String::from("Nets"),
            city: String::from("Brooklyn"),
            slug: String::from("BKN"),
        },
    );

    teams.insert(
        String::from("nuggets"),
        Team {
            team: String::from("Nuggets"),
            city: String::from("Denver"),
            slug: String::from("DEN"),
        },
    );

    teams.insert(
        String::from("pacers"),
        Team {
            team: String::from("Pacers"),
            city: String::from("Indiana"),
            slug: String::from("IND"),
        },
    );

    teams.insert(
        String::from("pelicans"),
        Team {
            team: String::from("Pelicans"),
            city: String::from("New Orleans"),
            slug: String::from("NO"),
        },
    );

    teams.insert(
        String::from("pistons"),
        Team {
            team: String::from("Pistons"),
            city: String::from("Detroit"),
            slug: String::from("DET"),
        },
    );

    teams.insert(
        String::from("raptors"),
        Team {
            team: String::from("Raptors"),
            city: String::from("Toronto"),
            slug: String::from("TOR"),
        },
    );

    teams.insert(
        String::from("rockets"),
        Team {
            team: String::from("Rockets"),
            city: String::from("Houston"),
            slug: String::from("HOU"),
        },
    );

    teams.insert(
        String::from("spurs"),
        Team {
            team: String::from("Spurs"),
            city: String::from("San Antonio"),
            slug: String::from("SA"),
        },
    );

    teams.insert(
        String::from("suns"),
        Team {
            team: String::from("Suns"),
            city: String::from("Phoenix"),
            slug: String::from("PHX"),
        },
    );

    teams.insert(
        String::from("thunder"),
        Team {
            team: String::from("Thunder"),
            city: String::from("Oklahoma City"),
            slug: String::from("OKC"),
        },
    );

    teams.insert(
        String::from("timberwolves"),
        Team {
            team: String::from("Timberwolves"),
            city: String::from("Minnesota"),
            slug: String::from("MIN"),
        },
    );

    teams.insert(
        String::from("trailblazers"),
        Team {
            team: String::from("Trail Blazers"),
            city: String::from("Portland"),
            slug: String::from("POR"),
        },
    );

    teams.insert(
        String::from("warriors"),
        Team {
            team: String::from("Warriors"),
            city: String::from("Golden State"),
            slug: String::from("GS"),
        },
    );

    teams.insert(
        String::from("wizards"),
        Team {
            team: String::from("Wizards"),
            city: String::from("Washington"),
            slug: String::from("WSH"),
        },
    );

    return teams;
}

pub fn nfl_teams() -> HashMap<String, Team> {
    let mut teams: HashMap<String, Team> = HashMap::new();

    teams.insert(
        String::from("49ers"),
        Team {
            team: String::from("49ers"),
            city: String::from("San Francisco"),
            slug: String::from("SF"),
        },
    );

    teams.insert(
        String::from("bears"),
        Team {
            team: String::from("Bears"),
            city: String::from("Chicago"),
            slug: String::from("CHI"),
        },
    );

    teams.insert(
        String::from("bengals"),
        Team {
            team: String::from("Bengals"),
            city: String::from("Cincinnati"),
            slug: String::from("CIN"),
        },
    );

    teams.insert(
        String::from("bills"),
        Team {
            team: String::from("Bills"),
            city: String::from("Buffalo"),
            slug: String::from("BUF"),
        },
    );

    teams.insert(
        String::from("broncos"),
        Team {
            team: String::from("Broncos"),
            city: String::from("Denver"),
            slug: String::from("DEN"),
        },
    );

    teams.insert(
        String::from("browns"),
        Team {
            team: String::from("Browns"),
            city: String::from("Cleveland"),
            slug: String::from("CLE"),
        },
    );

    teams.insert(
        String::from("buccaneers"),
        Team {
            team: String::from("Buccaneers"),
            city: String::from("Tampa Bay"),
            slug: String::from("TB"),
        },
    );

    teams.insert(
        String::from("cardinals"),
        Team {
            team: String::from("Cardinals"),
            city: String::from("Arizona"),
            slug: String::from("ARI"),
        },
    );

    teams.insert(
        String::from("chargers"),
        Team {
            team: String::from("Chargers"),
            city: String::from("Los Angeles"),
            slug: String::from("LAC"),
        },
    );

    teams.insert(
        String::from("chiefs"),
        Team {
            team: String::from("Chiefs"),
            city: String::from("Kansas City"),
            slug: String::from("KC"),
        },
    );

    teams.insert(
        String::from("colts"),
        Team {
            team: String::from("Colts"),
            city: String::from("Indianapolis"),
            slug: String::from("IND"),
        },
    );

    teams.insert(
        String::from("commanders"),
        Team {
            team: String::from("Commanders"),
            city: String::from("Washington"),
            slug: String::from("WSH"),
        },
    );

    teams.insert(
        String::from("cowboys"),
        Team {
            team: String::from("Cowboys"),
            city: String::from("Dallas"),
            slug: String::from("DAL"),
        },
    );

    teams.insert(
        String::from("dolphins"),
        Team {
            team: String::from("Dolphins"),
            city: String::from("Miami"),
            slug: String::from("MIA"),
        },
    );

    teams.insert(
        String::from("eagles"),
        Team {
            team: String::from("Eagles"),
            city: String::from("Philadelphia"),
            slug: String::from("PHI"),
        },
    );

    teams.insert(
        String::from("falcons"),
        Team {
            team: String::from("Falcons"),
            city: String::from("Atlanta"),
            slug: String::from("ATL"),
        },
    );

    teams.insert(
        String::from("giants"),
        Team {
            team: String::from("Giants"),
            city: String::from("New York"),
            slug: String::from("NYG"),
        },
    );

    teams.insert(
        String::from("jaguars"),
        Team {
            team: String::from("Jaguars"),
            city: String::from("Jacksonville"),
            slug: String::from("JAX"),
        },
    );

    teams.insert(
        String::from("jets"),
        Team {
            team: String::from("Jets"),
            city: String::from("New York"),
            slug: String::from("NYJ"),
        },
    );

    teams.insert(
        String::from("lions"),
        Team {
            team: String::from("Lions"),
            city: String::from("Detroit"),
            slug: String::from("DET"),
        },
    );

    teams.insert(
        String::from("packers"),
        Team {
            team: String::from("Packers"),
            city: String::from("Green Bay"),
            slug: String::from("GB"),
        },
    );

    teams.insert(
        String::from("panthers"),
        Team {
            team: String::from("Panthers"),
            city: String::from("Carolina"),
            slug: String::from("CAR"),
        },
    );

    teams.insert(
        String::from("patriots"),
        Team {
            team: String::from("Patriots"),
            city: String::from("New England"),
            slug: String::from("NE"),
        },
    );

    teams.insert(
        String::from("raiders"),
        Team {
            team: String::from("Raiders"),
            city: String::from("Las Vegas"),
            slug: String::from("LV"),
        },
    );

    teams.insert(
        String::from("rams"),
        Team {
            team: String::from("Rams"),
            city: String::from("Los Angeles"),
            slug: String::from("LAR"),
        },
    );

    teams.insert(
        String::from("ravens"),
        Team {
            team: String::from("Ravens"),
            city: String::from("Ravens"),
            slug: String::from("BAL"),
        },
    );

    teams.insert(
        String::from("saints"),
        Team {
            team: String::from("Saints"),
            city: String::from("New Orleans"),
            slug: String::from("NO"),
        },
    );

    teams.insert(
        String::from("seahawks"),
        Team {
            team: String::from("Seahawks"),
            city: String::from("Seattle"),
            slug: String::from("SEA"),
        },
    );

    teams.insert(
        String::from("steelers"),
        Team {
            team: String::from("Steelers"),
            city: String::from("Pittsburgh"),
            slug: String::from("PIT"),
        },
    );

    teams.insert(
        String::from("texans"),
        Team {
            team: String::from("Texans"),
            city: String::from("Houston"),
            slug: String::from("HOU"),
        },
    );

    teams.insert(
        String::from("titans"),
        Team {
            team: String::from("Titans"),
            city: String::from("Tennessee"),
            slug: String::from("TEN"),
        },
    );

    teams.insert(
        String::from("vikings"),
        Team {
            team: String::from("Vikings"),
            city: String::from("Minnesota"),
            slug: String::from("MIN"),
        },
    );

    return teams;
}

pub fn nhl_teams() -> HashMap<String, Team> {
    let mut teams: HashMap<String, Team> = HashMap::new();

    teams.insert(
        String::from("avalanche"),
        Team {
            team: String::from("Avalanche"),
            city: String::from("Colorado"),
            slug: String::from("COL"),
        },
    );

    teams.insert(
        String::from("blackhawks"),
        Team {
            team: String::from("Blackhawks"),
            city: String::from("Chicago"),
            slug: String::from("CHI"),
        },
    );

    teams.insert(
        String::from("bluejackets"),
        Team {
            team: String::from("Blue Jackets"),
            city: String::from("Columbus"),
            slug: String::from("CBJ"),
        },
    );

    teams.insert(
        String::from("blues"),
        Team {
            team: String::from("Blues"),
            city: String::from("St. Louis"),
            slug: String::from("STL"),
        },
    );

    teams.insert(
        String::from("bruins"),
        Team {
            team: String::from("Bruins"),
            city: String::from("Boston"),
            slug: String::from("BOS"),
        },
    );

    teams.insert(
        String::from("canadiens"),
        Team {
            team: String::from("Canadiens"),
            city: String::from("Montreal"),
            slug: String::from("MTL"),
        },
    );

    teams.insert(
        String::from("canucks"),
        Team {
            team: String::from("Canucks"),
            city: String::from("Vancouver"),
            slug: String::from("VAN"),
        },
    );

    teams.insert(
        String::from("capitals"),
        Team {
            team: String::from("Capitals"),
            city: String::from("Washington"),
            slug: String::from("WSH"),
        },
    );

    teams.insert(
        String::from("devils"),
        Team {
            team: String::from("Devils"),
            city: String::from("New Jersey"),
            slug: String::from("NJ"),
        },
    );

    teams.insert(
        String::from("ducks"),
        Team {
            team: String::from("Ducks"),
            city: String::from("Anaheim"),
            slug: String::from("ANA"),
        },
    );

    teams.insert(
        String::from("flames"),
        Team {
            team: String::from("Flames"),
            city: String::from("Calgary"),
            slug: String::from("CGY"),
        },
    );

    teams.insert(
        String::from("flyers"),
        Team {
            team: String::from("Flyers"),
            city: String::from("Philadelphia"),
            slug: String::from("PHI"),
        },
    );

    teams.insert(
        String::from("goldenknights"),
        Team {
            team: String::from("Golden Knights"),
            city: String::from("Las Vegas"),
            slug: String::from("VGK"),
        },
    );

    teams.insert(
        String::from("hurricanes"),
        Team {
            team: String::from("Hurricanes"),
            city: String::from("Carolina"),
            slug: String::from("CAR"),
        },
    );

    teams.insert(
        String::from("islanders"),
        Team {
            team: String::from("Islanders"),
            city: String::from("New York"),
            slug: String::from("NYI"),
        },
    );

    teams.insert(
        String::from("jets"),
        Team {
            team: String::from("Jets"),
            city: String::from("Winnipeg"),
            slug: String::from("WPG"),
        },
    );

    teams.insert(
        String::from("kings"),
        Team {
            team: String::from("Kings"),
            city: String::from("Los Angeles"),
            slug: String::from("LAK"),
        },
    );

    teams.insert(
        String::from("kraken"),
        Team {
            team: String::from("Kraken"),
            city: String::from("Seattle"),
            slug: String::from("SEA"),
        },
    );

    teams.insert(
        String::from("lightning"),
        Team {
            team: String::from("Lightning"),
            city: String::from("Tampa Bay"),
            slug: String::from("TB"),
        },
    );

    teams.insert(
        String::from("mapleleafs"),
        Team {
            team: String::from("Maple Leafs"),
            city: String::from("Toronto"),
            slug: String::from("TOR"),
        },
    );

    teams.insert(
        String::from("oilers"),
        Team {
            team: String::from("Oilers"),
            city: String::from("Edmonton"),
            slug: String::from("EDM"),
        },
    );

    teams.insert(
        String::from("panthers"),
        Team {
            team: String::from("Panthers"),
            city: String::from("Florida"),
            slug: String::from("FLA"),
        },
    );

    teams.insert(
        String::from("penguins"),
        Team {
            team: String::from("Penguins"),
            city: String::from("Pittsburgh"),
            slug: String::from("PIT"),
        },
    );

    teams.insert(
        String::from("predators"),
        Team {
            team: String::from("Predators"),
            city: String::from("Nashville"),
            slug: String::from("NSH"),
        },
    );

    teams.insert(
        String::from("rangers"),
        Team {
            team: String::from("Rangers"),
            city: String::from("New York"),
            slug: String::from("NYR"),
        },
    );

    teams.insert(
        String::from("redwings"),
        Team {
            team: String::from("Red Wings"),
            city: String::from("Detroit"),
            slug: String::from("DET"),
        },
    );

    teams.insert(
        String::from("sabres"),
        Team {
            team: String::from("Sabres"),
            city: String::from("Buffalo"),
            slug: String::from("BUF"),
        },
    );

    teams.insert(
        String::from("senators"),
        Team {
            team: String::from("Senators"),
            city: String::from("Ottawa"),
            slug: String::from("OTT"),
        },
    );

    teams.insert(
        String::from("sharks"),
        Team {
            team: String::from("Sharks"),
            city: String::from("San Jose"),
            slug: String::from("SJ"),
        },
    );

    teams.insert(
        String::from("stars"),
        Team {
            team: String::from("Stars"),
            city: String::from("Dallas"),
            slug: String::from("DAL"),
        },
    );

    teams.insert(
        String::from("utahhockeyclub"),
        Team {
            team: String::from("Utah Hockey Club"),
            city: String::from("Utah"),
            slug: String::from("UTAH"),
        },
    );

    teams.insert(
        String::from("wild"),
        Team {
            team: String::from("Wild"),
            city: String::from("Minnesota"),
            slug: String::from("MIN"),
        },
    );

    return teams;
}
