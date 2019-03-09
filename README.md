# cargo-rls-install

## Purpose

- Rust Language Server easy install.

Every night, I look for a day when I can use RLS which may be broken.  
In order to realize it, this program goes to see [the page](https://mexus.github.io/rustup-components-history/) that [mexus](https://github.com/mexus/)
updates everyday,  and scrapes it.

If we had no mexus page in the first place, we would have never made Cargo subcommand,  
nor did I feel like using Rust.

## Installation

```bash
cargo install cargo-rls-install
```

## Preparing for installation (Linux only)

For Linux, it is necessary to install OpenSSL for installation.

```bash
# Ubuntu or Debian
sudo apt install libssl-dev

# WSL(Windows Subsystem for Linux)
sudo apt install libssl-dev
sudo apt install build-essential
sudo apt install pkg-config

# RedHut
sudo yum install openssl-devel

# RedHut(Fedora etc.)
sudo dnf install openssl
```

## Usage

```bash
cargo rls-install [FLAGS]
```

### Example1

```bash
cargo rls-install -n
```

Three operations are executed by the above command.

1. Rust Language(Nightly channel) install.
2. RLS(Rust Language Server) install.
3. Set default use channel.

Before executing each operation, ask whether to execute it.

### Example2

```bash
cargo rls-install -ny
```

All operations are done without approval until the end.

## Flags

```bash
-b, --beta       Install beta channel Rust and RLS
-h, --help       Prints help information
-n, --nightly    Install nightly channel Rust and RLS
-s, --stable     Install stable channel Rust and RLS
-V, --version    Prints version information
-y, --yes        Pre-approval Rust and RLS install command
```

## Special thanks

- [mexus](https://github.com/mexus/)

## Release note

| Version | Change                                                                                                               |
| ------- | :------------------------------------------------------------------------------------------------------------------- |
| v1.0.12 | Version acquisition by env!("CARGO_PKG_VERSION").                                                                    |
| v1.0.11 | The repository "rustup-components-history" was incorporated into Rust's official Repository, so the URL was changed. |
| v1.0.10 | Add installation Instructions.                                                                                       |
| v1.0.9  | README.md: There was an error in the description about dnf                                                           |
| v1.0.8  | Add description for installation with wsl in README.md.                                                              |
| v1.0.7  | Fixed bug of v1.0.3 was not fixed.                                                                                   |
| v1.0.6  | README.md: fix tables.                                                                                               |
| v1.0.5  | I set the installation location of latest.txt in the cargo-rls-install source folder in .cargo.                      |
| v1.0.4  | Create latest.txt in .cargo.                                                                                         |
| v1.0.3  | Fix fatal bug. Parse error always occurs in initial operation.                                                       |
| v1.0.2  | Minor change.                                                                                                        |
| v1.0.1  | Minor change.                                                                                                        |
| v1.0.0  | First release.                                                                                                       |