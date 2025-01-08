use std::env;

mod utils;
use utils::{parse_args, RequestInfo};

mod teams;

mod request;
use request::parse_request;

mod handlers;

fn main() {
    let args: Vec<String> = env::args().collect();

    let request: Option<RequestInfo> = parse_args(args);
    match request {
        Some(req) => parse_request(req),
        None => {
            return;
        }
    }
}
