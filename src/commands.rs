use std::io::{stdin, Result};
use std::process::{exit, Command};

pub fn select_channel() -> std::result::Result<String, failure::Error> {
    println!("Select channel");
    println!("[stable/beta/nightly]");
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.to_lowercase().trim().to_owned())
}

pub fn print_rust_and_rls_install(v: &str, yes: bool) {
    if v == "stable" || v == "beta" {
        println!(
            r"
Requested Rust channel

    => {}
    ",
            v
        );
    } else {
        println!(
            r"
Recommended Nightly Rust version for using rls

    => {}
    ",
            v
        );
    }

    // Operation 1
    match rust_install(&v, yes) {
        Ok(()) => (),
        Err(e) => eprintln!("{:?}", e),
    }

    // Operation 2
    match rls_install(&v, yes) {
        Ok(()) => (),
        Err(e) => eprintln!("{:?}", e),
    }

    // Operation 3
    match rust_set_default(&v, yes) {
        Ok(()) => (),
        Err(e) => eprintln!("{:?}", e),
    }
}

fn rust_install(v: &str, yes: bool) -> Result<()> {
    println!(
        r"
    1. Rust installation:

        $ rustup install {}
    ",
        v
    );
    if yes {
        command_rust(&v);
    } else {
        println!("Command execution (y/n)");
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            command_rust(&v);
        } else {
            println!("Cancel");
            exit(0);
        }
    }
    Ok(())
}

pub fn rls_install(v: &str, yes: bool) -> Result<()> {
    println!(
        r"
    2. RLS installation:

        $ rustup component add rls --toolchain {}
        $ rustup component add rust-analysis --toolchain {}
        $ rustup component add rust-src --toolchain {}
",
        v, v, v
    );
    if yes {
        // rls install
        command_rls(&v);

        // rust-analysis install
        command_analysis(&v);

        // rust-src install
        command_src(&v);
    } else {
        println!("Command execution (y/n)");
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            // rls install
            command_rls(&v);

            // rust-analysis install
            command_analysis(&v);

            // rust-src install
            command_src(&v);
        } else {
            println!("Cancel");
            exit(0);
        }
    }
    Ok(())
}

pub fn rust_set_default(v: &str, yes: bool) -> Result<()> {
    println!(
        r"
    3. Set default:

        $ rustup default {}
    ",
        v
    );
    if yes {
        command_rust_default(&v);
    } else {
        println!("Command execution (y/n)");
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            command_rust_default(&v);
        } else {
            println!("Cancel");
            exit(0);
        }
    }
    Ok(())
}

fn command_rust(v: &str) {
    println!("$ rustup install {}", v);
    Command::new("rustup")
        .args(&["install", v])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_rls(v: &str) {
    println!("$ rustup component add rls --toolchain {}", v);
    Command::new("rustup")
        .args(&["component", "add", "rls", "--toolchain", v])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_analysis(v: &str) {
    println!("$ rustup component add rust-analysis --toolchain {}", v);
    Command::new("rustup")
        .args(&["component", "add", "rust-analysis", "--toolchain", v])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_src(v: &str) {
    println!("$ rustup component add rust-src --toolchain {}", v);
    Command::new("rustup")
        .args(&["component", "add", "rust-src", "--toolchain", v])
        .status()
        .expect("Abort installation");
    println!("OK");
}

fn command_rust_default(v: &str) {
    println!("$ rustup default {}", v);
    Command::new("rustup")
        .args(&["default", v])
        .status()
        .expect("Abort installation");
    println!("OK");
}
