extern crate failure;
extern crate regex;
extern crate reqwest;
extern crate select;
extern crate structopt;
#[macro_use]
extern crate lazy_static;
// extern crate cargo_toml;
// extern crate dirs;

pub mod global;
pub mod options;
pub mod owndir;
pub mod scraping;

pub use options::help;
pub use options::parse_args;
pub use owndir::latest_txt_path;
pub use scraping::RustupCompenentsHistory;
