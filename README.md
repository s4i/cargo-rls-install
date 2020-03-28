# cargo-rls-install

[![Build Status](https://travis-ci.org/s4i/cargo-rls-install.svg?branch=master)](https://travis-ci.org/s4i/cargo-rls-install)
[![Build status](https://ci.appveyor.com/api/projects/status/jrfl8f3yuu9hsbml?svg=true)](https://ci.appveyor.com/project/s4i/cargo-rls-install)

## Purpose

### Rust Language Server easy install

Every night, I look for a day when I can use RLS which may be broken.  
In order to realize it, this program goes to see [the page](https://rust-lang.github.io/rustup-components-history/) that [mexus](https://github.com/mexus/)'s scripts
updates everyday, and scrapes it.

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
cargo rls-install [FLAGS] [OPTIONS]
```

### Example1

Three operations are executed by the above command.

1. Rust Language(Nightly channel) install.
2. RLS(Rust Language Server) install.
3. Change default toolchain.

Before executing each operation, ask whether to execute it.

```bash
cargo rls-install -n # --nightly
```

### Example2

All operations are done without approval until the end.

```bash
cargo rls-install -ny # --nightly --yes
```

### Example3

Install Stable Rust, RLS and change the default toolchain.

```bash
cargo rls-install -s # --stable
```

### Example4

Install Beta Rust, RLS and change the default toolchain.

```bash
cargo rls-install -b # --beta
```

### Example5

Install RLS on all Rust channels.  
_The default toolchain is Nightly Rust, as the last operation on Nightly Rust is done._

```bash
cargo rls-install -ysbn
```

### Example6(v1.0.17-)

Check the build status of Rust and RLS.  
_This command is intended for use alone._

```bash
cargo rls-install -v # --view
```

### Example7(v1.0.23-)

Installs the specified component. Use `rustup component add` command.

```bash
# example: rustfmt
cargo rls-install -f # --rustfmt
cargo rls-install -c rustfmt # --component-add rustfmt
```

### Example8(v1.0.25-)

Changes the selected Rust channel to the default toolchain.  
 _If you specify `cargo rls-instrall -d n`, Nightly Rust with the most recent date will be specified as the default toolchain._

```bash
# Use stable
cargo rls-install -d s
# Use beta
cargo rls-install -d b
# Use target beta(v1.0.28-)
cargo rls-install -d beta-2019-05-19
# Use nightly
cargo rls-install -d nightly
# Use latest nightly rust toolchain
cargo rls-install -d n
# Use target nightly rust toolchain
cargo rls-install -d nightly-2019-05-21
```

Change default toolchain. Use `rustup default` command.

### Example9(v1.0.28-)

Delete the selected Rust channel.

```bash
# Uninstall stable
cargo rls-install -u s
# Uninstall beta
cargo rls-install -u b
# Uninstall target beta
cargo rls-install -u beta-2019-05-19
# Uninstall nightly
cargo rls-install -u n
# Uninstall target nightly
cargo rls-install -u nightly-2019-5-21
# Uninstall all but the latest nightly rust dated
cargo rls-install -u a # a or all
```

Uninstall toolchain. Use `rustup uninstall` command.  
Note: Latest nightly rust and default toolchain isn't eligible for uninstallation.

### Example10(v2.0.4-)

Nightly Rust will be installed on the date you choose.

```bash
cargo rls-install -i nightly-2020-03-19
```

## Flags

```bash
USAGE:
    cargo rls-install [FLAGS] [OPTIONS]

FLAGS:
    -b, --beta       Install beta channel Rust and RLS
    -h, --help       Prints help information
    -i, --install    Install user specified target nightly channel
    -n, --nightly    Install nightly channel Rust and RLS
    -f, --rustfmt    Install rustfmt
    -s, --stable     Install stable channel Rust and RLS
    -V, --version    Prints version information
    -v, --view       RLS build status view
    -y, --yes        Pre-approval: Install Rust, RLS and change toolchain

OPTIONS:
    -c, --component-add <component>          Wrapper(rustup component add)
    -d, --default-toolchain <default>        Wrapper(rustup default)
    -u, --uninstall-toolchain <uninstall>    Wrapper(rustup uninstall)
```

## Special thanks

[mexus](https://github.com/mexus/)
