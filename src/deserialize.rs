// deserialize module defined in ./main.rs

mod mlb;
pub use mlb::mlb_handler;

mod nba;
pub use nba::nba_handler;

mod nfl;
pub use nfl::nfl_handler;

mod nhl;
pub use nhl::nhl_handler;
