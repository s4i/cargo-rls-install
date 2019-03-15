# cargo-rls-install

[![Build Status](https://travis-ci.org/s4i/cargo-rls-install.svg?branch=master)](https://travis-ci.org/s4i/cargo-rls-install)
[![Build status](https://ci.appveyor.com/api/projects/status/jrfl8f3yuu9hsbml?svg=true)](https://ci.appveyor.com/project/s4i/cargo-rls-install)
[![codecov](https://codecov.io/gh/s4i/cargo-rls-install/branch/master/graph/badge.svg)](https://codecov.io/gh/s4i/cargo-rls-install)

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
-y, --yes        Pre-approval Rust and RLS install and rustup default command
```

## Special thanks

[mexus](https://github.com/mexus/)
