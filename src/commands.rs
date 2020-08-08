use std::io::{stdin, stdout, Result, Write};
use std::process::{exit, Command};
use std::str::from_utf8;

fn execution(yes: bool) -> Result<()> {
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

pub fn rust_and_rls_install(ch: &str, yes: bool) {
    // Operation 1: Rust install
    rust_install(ch, yes);

    // Operation 2: RLS install
    rls_install(yes);

    // Operation 3: Default setting
    rust_set_default(ch, yes);
}

pub fn print_rust_and_rls_install(
    ch: &str,
    yes: bool,
    skip_rust_install: bool,
    skip_default_setting: bool,
) {
    let channel: String = if ch == "stable" || ch == "beta" {
        println!("\n * Requested Rust channel: {}", ch);
        ch.to_owned()
    } else {
        // YYYY-MM-DD
        println!("\n * Recommended Nightly Rust: {}", ch);
        format!("{}{}", "nightly-", ch)
    };

    // Operation 1: Rust install
    if skip_rust_install {
        println!("\n   1. Rust version: OK({} installed)", channel);
    } else {
        rust_install(&channel, yes);
    }

    // Operation 2: RLS install
    rls_install(yes);

    // Operation 3: Default setting
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

    match execution(yes) {
        Ok(()) => {
            println!("$ rustup install {}", channel);
            Command::new("rustup")
                .args(&["install", channel])
                .status()
                .expect("Abort installation");
        }
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

fn rls_install(yes: bool) {
    println!("\n   2. RLS installation commands:");

    // rls install
    if !yes {
        println!("\n$ rustup component add rls\n");
    }

    match execution(yes) {
        Ok(()) => component_add("rls"),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }

    // rust-analysis install
    if !yes {
        println!("\n$ rustup component add rust-analysis\n");
    }

    match execution(yes) {
        Ok(()) => component_add("rust-analysis"),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }

    // rust-src install
    if !yes {
        println!("\n$ rustup component add rust-src\n");
    }

    match execution(yes) {
        Ok(()) => component_add("rust-src"),
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

    match execution(yes) {
        Ok(()) => command_rust_default(&channel),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

pub fn component_add(component: &str) {
    println!("\n$ rustup component add {}", component);

    Command::new("rustup")
        .args(&["component", "add", component])
        .status()
        .expect("Abort installation");
}

pub fn component_add_and_get_output(component: &str) -> String {
    println!("\n$ rustup component add {}", component);

    let output = Command::new("rustup")
        .args(&["component", "add", component])
        .output()
        .expect("Abort installation");

    from_utf8(&output.stderr).unwrap().to_owned()
}

pub fn command_rust_default(channel: &str) {
    println!("\n$ rustup default {}", channel);
    Command::new("rustup")
        .args(&["default", channel])
        .status()
        .expect("Abort installation");
}

pub fn command_rust_uninstall(channel: &str) {
    println!("\n$ rustup uninstall {}", channel);
    match execution(false) {
        Ok(()) => {
            Command::new("rustup")
                .args(&["uninstall", channel])
                .status()
                .expect("Abort installation");
        }
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}

pub fn command_rust_multiple_uninstall(dated_nightly: Vec<String>) {
    println!();
    println!(" * Uninstall list\n");
    for dt in &dated_nightly {
        println!(" => {}", dt);
    }
    println!(" -------------------------------------------\n");
    for dt in &dated_nightly {
        println!("$ rustup uninstall {}", dt);
    }
    match execution(false) {
        Ok(()) => {
            for dt in &dated_nightly {
                Command::new("rustup")
                    .args(&["uninstall", dt])
                    .status()
                    .expect("Abort installation");
            }
        }
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    }
}
