extern crate failure;
extern crate regex;
extern crate reqwest;
extern crate select;
extern crate structopt;
#[macro_use]
extern crate lazy_static;

pub mod options;
pub use options::parse_args;
pub use options::help;

pub mod global;
pub use global::PRESENT_DATE;

// Build-in text name
pub const BUILD_IN_TEXT: &str = "latest.txt";
