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
    print!(" [0.stable, 1.beta, 2.nightly] -> ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.to_lowercase().trim().to_owned())
}

pub fn print_rust_and_rls_install(channel: &str, yes: bool) {
    if channel == "stable" || channel == "beta" {
        println!(" * Requested Rust channel: {}\n", channel);
    } else {
        println!(" * Recommended Nightly Rust: {}\n", channel);
    }

    // Operation 1
    rust_install(&channel, yes);

    // Operation 2
    rls_install(&channel, yes);

    // Operation 3
    rust_set_default(&channel, yes);
}

fn rust_install(channel: &str, yes: bool) {
    println!(" 1. Rust installation command.\n");
    println!("$ rustup install {}\n", channel);
    match excecution(yes) {
        Ok(()) => command_rust(&channel),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

pub fn rls_install(channel: &str, yes: bool) {
    println!(" 2. RLS installation commands.\n");
    println!("$ rustup component add rls --toolchain {}", channel);
    println!(
        "$ rustup component add rust-analysis --toolchain {}",
        channel
    );
    println!("$ rustup component add rust-src --toolchain {}\n", channel);
    match excecution(yes) {
        Ok(()) => {
            // rls install
            command_rls(&channel);

            // rust-analysis install
            command_rust_analysis(&channel);

            // rust-src install
            command_rust_src(&channel);
        }
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

pub fn rust_set_default(channel: &str, yes: bool) {
    println!("\n 3. Set default:\n");
    println!("$ rustup default {}\n", channel);

    match excecution(yes) {
        Ok(()) => command_rust_default(&channel),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

fn command_rust(channel: &str) {
    println!("\n$ rustup install {}", channel);
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
