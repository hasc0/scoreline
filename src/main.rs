use std::env;

mod utils;
use utils::{parse_args, RequestInfo};

mod teams;

mod request;
use request::parse_request;

mod deserialize;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let request: Option<RequestInfo> = parse_args(args);
    match request {
        Some(req) => parse_request(req),
        None => {
            println!("Please provide valid arguments.");
            return;
        }
    }
}
