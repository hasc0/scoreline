use std::env;

mod utils;
use utils::{parse_args, RequestInfo};

mod teams;

mod request;
use request::parse_request;

mod handler;
use handler::handle_response;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();

    let request: Option<RequestInfo> = parse_args(args);
    match request {
        Some(req) => parse_request(req),
        None => return,
    }
}
