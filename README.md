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
cargo rls-install [FLAGS]
```

### Example1

```bash
cargo rls-install -n # --nightly
```

Three operations are executed by the above command.

1. Rust Language(Nightly channel) install.
2. RLS(Rust Language Server) install.
3. Change default toolchain.

Before executing each operation, ask whether to execute it.

### Example2

```bash
cargo rls-install -ny # --nightly --yes
```

All operations are done without approval until the end.

### Example3

```bash
cargo rls-install -s # --stable
```

Install Stable Rust and RLS and change the default toolchain.

### Example4

```bash
cargo rls-install -b # --beta
```

Install Beta Rust and RLS and change the default toolchain.

### Example5

```bash
cargo rls-install -ysbn
```

Install RLS on all Rust channels.
The default toolchain is Nightly Rust, as the last operation on Nightly Rust is done.

### Example6(v1.0.17-)

```bash
cargo rls-install -v # --view
```

Check the build status of Rust and RLS.
This command is intended to be used only by itself.

### Example7(v1.0.23-)

```bash
cargo rls-install -f # --rustfmt
cargo rls-install -c rustfmt # --comp-add rustfmt
```

Install component(example: rustfmt). Use `rustup component add` command.

### Example8(v1.0.25-)

```bash
# Use stable
cargo rls-install -ds
# Use beta
cargo rls-install -db
# Use target beta(v1.0.28-)
cargo rls-install -dbeta-2019-05-19
# Use nightly
cargo rls-install -dnightly
# Use latest nightly rust toolchain
cargo rls-install -dn
# Use target nightly rust toolchain
cargo rls-install -dnightly-2019-05-21
```

Change default toolchain. Use `rustup default` command.

### Example9(v1.0.28-)

```bash
# Uninstall stable
cargo rls-install -us
# Uninstall beta
cargo rls-install -ub
# Uninstall target beta
cargo rls-install -ubbeta-2019-05-19
# Uninstall nightly
cargo rls-install -un
# Uninstall target nightly
cargo rls-install -unightly-2019-5-21
# Uninstall all but the latest nightly rust dated
cargo rls-install -ua # a or all
```

Uninstall toolchain. Use `rustup uninstall` command.  
Note: Latest nightly rust and default toolchain isn't eligible for uninstallation.

## Flags

```bash
USAGE:
    cargo-rls-install.exe rls-install [FLAGS] [OPTIONS]

FLAGS:
    -b, --beta       Install beta channel Rust and RLS
    -h, --help       Prints help information
    -n, --nightly    Install nightly channel Rust and RLS
    -f, --rustfmt    Install rustfmt
    -s, --stable     Install stable channel Rust and RLS
    -V, --version    Prints version information
    -v, --view       RLS build status view
    -y, --yes        Pre-approval: Install Rust, RLS and change toolchain

OPTIONS:
    -c, --component-add <comp_add>           Wrapper(rustup component add)
    -d, --default-toolchain <default>        Wrapper(rustup default)
    -u, --uninstall-toolchain <uninstall>    Wrapper(rustup uninstall)
```

## Special thanks

[mexus](https://github.com/mexus/)
