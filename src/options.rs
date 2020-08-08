use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cargo {
    #[structopt(name = "rls-install")]
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
    #[structopt(short, long = "component-add", help = "Wrapper(rustup component add)")]
    pub component: Option<String>,
    #[structopt(
        short,
        long,
        help = "Pre-approval: Install Rust, RLS and change toolchain"
    )]
    pub yes: bool,
    #[structopt(short, long = "default-toolchain", help = "Wrapper(rustup default)")]
    pub default: Option<String>,
    #[structopt(
        short,
        long = "uninstall-toolchain",
        help = "Wrapper(rustup uninstall)"
    )]
    pub uninstall: Option<String>,
    #[structopt(short, long, help = "Install user specified target nightly channel")]
    pub install: Option<String>,
    #[structopt(subcommand)]
    pub subcommands: Option<SubCommandsEnum>,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum SubCommandsEnum {
    #[structopt(about = "Wrapper(rustup show)")]
    Show,
    #[structopt(about = "Install clippy and rustfmt")]
    Formatter,
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
    cargo-rls-install.exe rls-install [FLAGS] [OPTIONS]

FLAGS:
    -b, --beta       Install beta channel Rust and RLS
    -h, --help       Prints help information
    -i, --install    Install user specified target nightly channel
    -n, --nightly    Install nightly channel Rust and RLS
    -s, --stable     Install stable channel Rust and RLS
    -V, --version    Prints version information
    -v, --view       RLS build status view
    -y, --yes        Pre-approval: Install Rust, RLS and change toolchain

OPTIONS:
    -c, --component-add <component>          Wrapper(rustup component add)
    -d, --default-toolchain <default>        Wrapper(rustup default)
    -u, --uninstall-toolchain <uninstall>    Wrapper(rustup uninstall)

SUBCOMMANDS:
    formatter    Install clippy and rustfmt
    help         Prints this message or the help of the given subcommand(s)
    show         Wrapper(rustup show)
"
    );
}
