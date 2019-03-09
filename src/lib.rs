extern crate failure;
extern crate regex;
extern crate reqwest;
extern crate select;
extern crate structopt;
#[macro_use]
extern crate lazy_static;
// extern crate cargo_toml;
extern crate dirs;

pub mod global;
pub mod options;
pub mod own_dir;

pub use options::help;
pub use options::parse_args;
pub use own_dir::latest_txt_path;

pub fn app_src_dir() -> String {
    format!(
        "{}{}{}",
        "cargo-rls-install",
        '-',
        env!("CARGO_PKG_VERSION")
    )
}
