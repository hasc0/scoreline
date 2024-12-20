// utils module defined in ./main.rs

#[derive(Default)]
pub struct RequestInfo {
    sport: String,
    league: String,
    stat: Option<String>,
    date: Option<String>,
    city: Option<String>,
    team: Option<String>,
}

pub fn parse_args(args: Vec<String>) -> RequestInfo {
    let request: RequestInfo = RequestInfo::default();

    return request;
}
