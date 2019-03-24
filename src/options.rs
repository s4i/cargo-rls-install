use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cargo {
    #[structopt(
        name = "rls-install",
        about = "Install the Rust and Rust Language Server.",
        author = ""
    )]
    Install(Channel),
}

#[derive(Debug, StructOpt)]
pub struct Channel {
    #[structopt(short, long, help = "Install stable channel Rust and RLS")]
    pub stable: bool,
    #[structopt(short, long, help = "Install beta channel Rust and RLS")]
    pub beta: bool,
    #[structopt(short, long, help = "Install nightly channel Rust and RLS")]
    pub nightly: bool,
    #[structopt(short, long, help = "RLS build status view")]
    pub view: bool,
    #[structopt(
        short,
        long,
        help = "Pre-approval Rust and RLS install and rustup default command"
    )]
    pub yes: bool,
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
    -v, --view       RLS build status view
    -y, --yes        Pre-approval Rust and RLS install and rustup default command
    "
    );
}
