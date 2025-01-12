// mlb module defined in ../handlers.rs

use ureq::{Response, Error, serde_json};

use serde::Deserialize;

use crate::RequestInfo;
use crate::teams::Team;

pub fn mlb_handler(response: Response, request: RequestInfo) -> Result<(), serde_json::Error> {
    Ok(())
}
