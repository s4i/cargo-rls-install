# cargo-rls-install

## Purpose

- Rust Language Server easy install.

Every night, I look for a day when I can use RLS which may be broken.  
In order to realize it, this program goes to see [<font color="Gold">the page</font>](https://mexus.github.io/rustup-components-history/) that [mexus](https://github.com/mexus/)
updates everyday,  and scrapes it.

If we had no mexus page in the first place, we would have never made Cargo subcommand,  
nor did I feel like using Rust.

## Usage

```
$ cargo rls-install [FLAGS]
```

### Example1

```
$ cargo rls-install -n
```

Three operations are executed by the above command.

1. Rust Language(Nightly channel) install.
2. RLS(Rust Language Server) install.
3. Set default use channel.

Before executing each operation, ask whether to execute it.

### Example2

```
$ cargo rls-install -ny
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

- mexus
