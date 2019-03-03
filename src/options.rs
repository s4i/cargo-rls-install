use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Cargo {
    #[structopt(
        name = "rls-install",
        about = "Install the Rust and Rust Language Server.",
        author = "",
        // version = "",
    )]
    Install(Channel),
}

#[derive(Debug, StructOpt)]
pub struct Channel {
    #[structopt(short, long, help = "Pre-approval Rust and RLS install command")]
    pub yes: bool,
    #[structopt(short, long, help = "Install stable channel Rust and RLS")]
    pub stable: bool,
    #[structopt(short, long, help = "Install beta channel Rust and RLS")]
    pub beta: bool,
    #[structopt(short, long, help = "Install nightly channel Rust and RLS")]
    pub nightly: bool,
}

pub fn parse_args() -> Channel {
    match StructOpt::from_args() {
        Cargo::Install(opt) => opt,
    }
}

pub fn help() {
    println!(
        r"
USAGE:
    cargo-rls-install.exe rls-install [FLAGS]

FLAGS:
    -b, --beta       Install beta channel Rust and RLS
    -h, --help       Prints help information
    -n, --nightly    Install nightly channel Rust and RLS
    -s, --stable     Install stable channel Rust and RLS
    -V, --version    Prints version information
    -y, --yes        Pre-approval Rust and RLS install command
    "
    );
}
