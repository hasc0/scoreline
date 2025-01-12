// nba module defined in ../handlers.rs

use ureq::{Response, Error, serde_json};

use serde::Deserialize;

use crate::RequestInfo;
use crate::teams::Team;

pub fn nba_handler(response: Response, request: RequestInfo) -> Result<(), serde_json::Error> {
    Ok(())
}
