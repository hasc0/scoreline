use std::env;

mod utils;

mod request;
use request::parse_request;

mod deserialize;

fn main() {
    let args: Vec<String> = env::args().collect();
}
