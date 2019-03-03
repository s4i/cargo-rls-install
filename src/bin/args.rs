extern crate structopt;

pub mod args {
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    pub enum Cargo {
        #[structopt(
            name = "rls-install",
            about = "Install the Rust and Rust Language Server",
            author = ""
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

    pub fn parse() -> Channel {
        match StructOpt::from_args() {
            Cargo::Install(opt) => opt,
        }
    }
}
