use std::io::{stdin, stdout, Result, Write};
use std::process::{exit, Command};

fn excecution(yes: bool) -> Result<()> {
    if !yes {
        print!("Execution(y/n) ");
        stdout().flush().unwrap();
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        match buf.to_lowercase().trim() {
            "" | "y" | "yes" => (),
            _ => {
                println!("Cancel");
                exit(0);
            }
        }
    }
    Ok(())
}

pub fn select_channel() -> std::result::Result<String, failure::Error> {
    println!("\n * Select channel");
    print!(" [0:stable, 1:beta, 2:nightly] -> ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.to_lowercase().trim().to_owned())
}

pub fn print_rust_and_rls_install(
    ch: &str,
    yes: bool,
    skip_rust_install: bool,
    skip_default_setting: bool,
) {
    let channel = if ch == "stable" || ch == "beta" {
        println!("\n * Requested Rust channel: {}", ch);
        ch.to_owned()
    } else {
        // YYYY-MM-DD
        println!("\n * Recommended Nightly Rust: {}", ch);
        format!("{}{}", "nightly-", ch)
    };

    // Operation 1
    if skip_rust_install {
        println!("\n   1. Rust version: OK({} installed)", channel);
    } else {
        rust_install(&channel, yes);
    }

    // Operation 2
    rls_install(&channel, yes);

    // Operation 3
    if skip_default_setting {
        println!("\n   3. Set default: Already set\n");
    } else {
        rust_set_default(&channel, yes);
    }
}

fn rust_install(channel: &str, yes: bool) {
    println!("\n   1. Rust installation command:\n");

    if !yes {
        println!("$ rustup install {}\n", channel);
    }

    match excecution(yes) {
        Ok(()) => command_rust(&channel),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

fn rls_install(channel: &str, yes: bool) {
    println!("\n   2. RLS installation commands:");

    if !yes {
        println!("\n$ rustup component add rls --toolchain {}\n", channel);
    }

    match excecution(yes) {
        Ok(()) => command_rls(&channel), // rls install
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }

    if !yes {
        println!(
            "\n$ rustup component add rust-analysis --toolchain {}\n",
            channel
        );
    }

    match excecution(yes) {
        Ok(()) => command_rust_analysis(&channel), // rust-analysis install
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }

    if !yes {
        println!(
            "\n$ rustup component add rust-src --toolchain {}\n",
            channel
        );
    }

    match excecution(yes) {
        Ok(()) => command_rust_src(&channel), // rust-src install
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

fn rust_set_default(channel: &str, yes: bool) {
    println!("\n   3. Set default:\n");

    if !yes {
        println!("$ rustup default {}\n", channel);
    }

    match excecution(yes) {
        Ok(()) => command_rust_default(&channel),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

fn command_rust(channel: &str) {
    println!("$ rustup install {}", channel);
    Command::new("rustup")
        .args(&["install", channel])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_rls(channel: &str) {
    println!("\n$ rustup component add rls --toolchain {}", channel);
    Command::new("rustup")
        .args(&["component", "add", "rls", "--toolchain", channel])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_rust_analysis(channel: &str) {
    println!(
        "\n$ rustup component add rust-analysis --toolchain {}",
        channel
    );
    Command::new("rustup")
        .args(&["component", "add", "rust-analysis", "--toolchain", channel])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_rust_src(channel: &str) {
    println!("\n$ rustup component add rust-src --toolchain {}", channel);
    Command::new("rustup")
        .args(&["component", "add", "rust-src", "--toolchain", channel])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_rust_default(channel: &str) {
    println!("\n$ rustup default {}", channel);
    Command::new("rustup")
        .args(&["default", channel])
        .status()
        .expect("Abort installation");
    println!("OK");
}
