#[macro_use]
extern crate lazy_static;

pub mod commands;
pub mod global;
pub mod local_env;
pub mod options;
pub mod scraping;

pub use commands::*;
pub use local_env::latest_txt_path;
pub use options::*;
pub use scraping::RustupCompenentsHistory;
