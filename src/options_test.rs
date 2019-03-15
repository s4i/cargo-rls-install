#[test]
fn help() {
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
    -y, --yes        Pre-approval Rust and RLS install and rustup default command
    "
    );
}
