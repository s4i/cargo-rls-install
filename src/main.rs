extern crate cargo_rls_install;
use cargo_rls_install::{
    commands::{print_rust_and_rls_install, rls_install, rust_set_default, select_channel},
    global::PRESENT_DATE,
    help, latest_txt_path, parse_args, Channel, RustupCompenentsHistory,
};
use chrono::NaiveDate;
use regex::Regex;
use std::io::{BufWriter, Write};
use std::process::{exit, Command};
use std::{fs, str};

const BUILD_IN_TEXT_NAME: &str = "latest.txt";

fn main() {
    // Command line
    // lib.rs
    let o: Channel = parse_args();

    // view option
    if o.view {
        match (o.yes, o.stable, o.beta, o.nightly) {
            (false, false, false, false) => view(),
            _ => println!("Invalid option."),
        }
        println!("End.");
        exit(0);
    }

    // Stable choice
    if o.stable {
        print_rust_and_rls_install("stable", o.yes);
    }

    // Beta choice
    if o.beta {
        print_rust_and_rls_install("beta", o.yes);
    }

    // Nightly choice
    if o.nightly {
        nightly(o.yes);
    }

    // Yes only or option nothing
    yes_only(&o);

    println!("End.");
}

fn url_decision(platform_name: &str) -> String {
    /* Switch web pages */
    if platform_name == "x86_64-unknown-linux-gnu" {
        "https://rust-lang.github.io/rustup-components-history/".to_owned()
    } else {
        format!(
            "{}{}",
            "https://rust-lang.github.io/rustup-components-history/", platform_name
        )
    }
}

fn view() {
    /* Local system rust version */
    let console_stdout = local_system_rust_version();
    let (_, platform_name) = sysroot_regex(&console_stdout);

    /* Search url decision and Scraping */
    url_decision(&platform_name)
        .as_str()
        .rustup_components_history();

    // local info
    let toolchains = installed_toolchain();

    // web status
    let map = PRESENT_DATE.lock().unwrap();

    /* Status table */
    println!(" * Rust information");

    // web status
    let mut eight_days = Vec::new();
    for (date, _) in map.iter() {
        eight_days.push(date.format("%F").to_string());
    }

    let mut has_eight_days_before = false;
    for tc in &toolchains {
        if !eight_days.contains(tc) {
            has_eight_days_before = true;
        }
    }

    if has_eight_days_before {
        println!(" ---------------------------------");
        println!(" |Old Nightly Rust(Before 9 days)|");
        println!(" ---------------------------------");
        println!(" | {:<19} {:^10}|", "Build date", "");
        println!(" ---------------------------------");

        for tc in &toolchains {
            if !eight_days.contains(tc) {
                println!(
                    " | {:<19} {:^10}| <= Installed",
                    format!("{}{}", "nightly-", tc),
                    ""
                );
            }
        }
        println!(" |_______________________________|");
    }

    println!(" |       Rust & RLS status       |");
    println!(" ---------------------------------");
    println!(" | {:<19}|{:^10}|", "Build date", "Status");
    println!(" ---------------------------------");

    for (date, status) in map.iter() {
        if toolchains.contains(&date.format("%F").to_string()) {
            println!(
                " | {:<19}|{:^10}| <= Installed",
                format!("{}{}", "nightly-", date.format("%F").to_string()),
                status
            );
        } else {
            println!(
                " | {:<19}|{:^10}|",
                format!("{}{}", "nightly-", date.format("%F").to_string()),
                status
            );
        }
    }
    println!(" |_______________________________|");
}

fn nightly(yes: bool) {
    /* Local system rust version */
    let console_stdout = local_system_rust_version();
    let (now_build_date, platform_name) = sysroot_regex(&console_stdout);

    // Get web page date(nightly-"Date" store) - global variable
    url_decision(&platform_name)
        .as_str()
        .rustup_components_history();

    // Get text date(nightly-"Date" store)
    let mut latest_txt_lines = Vec::new();
    match latest_text_lines() {
        Ok(text_date) => latest_txt_lines = text_date,
        Err(e) => println!("{:?}", e),
    };

    // line tail(=latest date) get
    latest_txt_lines.retain(|s| s != ""); // remove empty
    let text_latest = latest_txt_lines.last().unwrap(); // get latest
    let chrono_text = NaiveDate::parse_from_str(&text_latest, "%Y-%m-%d").expect("date");

    // Display
    println!(" {:<20} Status", "Build date");
    println!(" -----------------------------");

    let mut present_vec = Vec::new();

    // global variable
    let map = PRESENT_DATE.lock().unwrap();

    for (date, status) in map.clone().into_iter() {
        println!(" {:<20}{:>8}", format!("{}{}", "nightly-", date), status);
        if status == "present" {
            present_vec.push(date);
        }
    }

    println!(" -----------------------------");

    let web_latest = if !present_vec.is_empty() {
        present_vec
            .into_iter()
            .max()
            .unwrap()
            .format("%F")
            .to_string()
    } else {
        // eight days missing
        // Rust update unavailable
        println!("For RLS, unfortunate 8 days.");
        println!("It is impossible to find the latest version.");
        println!("The following version is written in the built-in text.");
        "".to_owned()
    };

    if text_latest.is_empty() {
        println!("Can't search Rust and RLS latest version.");
        exit(1);
    }

    match (!now_build_date.is_empty(), !web_latest.is_empty()) {
        (false, true) => {
            let chrono_web = NaiveDate::parse_from_str(&web_latest, "%Y-%m-%d").unwrap();

            // Rust and RLS aren't installed on the local system
            // Case: first use or not default channel nightly
            if chrono_web > chrono_text {
                print_rust_and_rls_install(&web_latest, yes);
                // Text write newline
                text_write(&web_latest);
            } else if chrono_web <= chrono_text {
                print_rust_and_rls_install(&text_latest, yes);
            }
        }
        (true, true) => {
            let chrono_now = NaiveDate::parse_from_str(&now_build_date, "%Y-%m-%d").unwrap();
            let chrono_web = NaiveDate::parse_from_str(&web_latest, "%Y-%m-%d").unwrap();

            // Rust update check
            if chrono_now > chrono_web && chrono_now > chrono_text {
                println!("Can't search Rust and RLS latest version.");
            } else if chrono_now >= chrono_web {
                skip_rust_install(&now_build_date, yes);
            } else if chrono_web > chrono_text {
                print_rust_and_rls_install(&web_latest, yes);
                // Text write newline
                text_write(&web_latest);
            } else if chrono_web <= chrono_text {
                print_rust_and_rls_install(&text_latest, yes);
            }
        }
        (true, false) => {
            let chrono_now = NaiveDate::parse_from_str(&now_build_date, "%Y-%m-%d").unwrap();

            if chrono_text > chrono_now {
                print_rust_and_rls_install(&text_latest, yes);
            } else {
                skip_rust_install(&now_build_date, yes);
            }
        }
        (false, false) => {
            print_rust_and_rls_install(&text_latest, yes);
        }
    }
}

fn skip_rust_install(date: &str, yes: bool) {
    let target = format!("{}{}", "nightly-", date);
    println!(" 1. Rust version: OK({})\n", target);
    rls_install(&target, yes);
    rust_set_default(&target, yes);
}

fn yes_only(o: &Channel) {
    match (o.yes, o.stable, o.beta, o.nightly) {
        (true, false, false, false) => match select_channel() {
            // &*: String -> &str
            Ok(ch) => match &*ch {
                "0" => print_rust_and_rls_install("stable", o.yes),
                "0.stable" => print_rust_and_rls_install("stable", o.yes),
                "stable" => print_rust_and_rls_install(&ch, o.yes),
                "1" => print_rust_and_rls_install("beta", o.yes),
                "1.beta" => print_rust_and_rls_install("beta", o.yes),
                "beta" => print_rust_and_rls_install(&ch, o.yes),
                "2" => nightly(o.yes),
                "2.nightly" => nightly(o.yes),
                "nightly" => nightly(o.yes),
                _ => println!("No matches"),
            },
            Err(_e) => {
                println!("Cancel");
            }
        },
        (false, false, false, false) => {
            help();
            println!("Please input option.");
        }
        _ => (),
    }
}

fn text_write(web_latest: &str) {
    let writer_opt = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(latest_txt_path(BUILD_IN_TEXT_NAME))
        .expect("Can't open file.");
    let mut writer = BufWriter::new(writer_opt);
    writeln!(writer, "{}", web_latest).expect("File write failed.");
}

fn local_system_rust_version() -> String {
    let sysroot = Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .expect("failed to execute");

    // forced linux path format
    String::from_utf8(sysroot.stdout)
        .expect("Encode failed")
        .trim_start_matches('/')
        .trim_end()
        .replace("\\", "/")
}

// ex. Return: ("2019-03-23", "x86_64-pc-windows-msvc")
fn sysroot_regex(path: &str) -> (String, String) {
    let re_stable = Regex::new(r"\b.+stable-").unwrap();
    let re_beta = Regex::new(r"\b.+beta-").unwrap();
    let re_nightly = Regex::new(r"\b.+nightly-").unwrap();

    // Get platform name
    match (
        re_nightly.is_match(path),
        re_beta.is_match(path),
        re_stable.is_match(path),
    ) {
        (true, false, false) => {
            // Forward path string delete
            let no_head = re_nightly.replace(path, "");

            let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

            let now_build_date = if re_date.is_match(&no_head) {
                re_date.find(&no_head).unwrap().as_str().to_owned()
            } else {
                "".to_owned()
            };

            if now_build_date.is_empty() {
                println!("\n * Default use Rust toolchain: Nightly\n");
            } else {
                println!(
                    "\n * Default use Rust toolchain: Nightly-{}\n",
                    now_build_date
                );
            }
            (now_build_date, platform(&no_head))
        }
        (false, true, false) => {
            println!("\n * Default use Rust toolchain: Beta\n");
            let no_head = re_beta.replace(path, "");
            let platform_name = platform(&no_head);
            ("".to_owned(), platform_name)
        }
        (false, false, true) => {
            println!("\n * Default use Rust toolchain: Stable\n");
            let no_head = re_stable.replace(path, "");
            ("".to_owned(), platform(&no_head))
        }
        _ => {
            eprintln!("Other Error");
            ("".to_owned(), "".to_owned())
        }
    }
}

fn platform(no_head: &str) -> String {
    let re_date_plus_hyphen = Regex::new(r"\d{4}-\d{2}-\d{2}-").unwrap();
    re_date_plus_hyphen.replace(no_head, "").to_string() // matching. <YYYY-MM-DD>
}

fn latest_text_lines() -> std::result::Result<Vec<String>, std::io::ErrorKind> {
    use std::fs::read;
    let text_vector = read(latest_txt_path(BUILD_IN_TEXT_NAME)).unwrap();

    match text_vector.len() {
        0 => Err(std::io::ErrorKind::NotFound),
        _ => {
            let text_str = String::from_utf8(text_vector).unwrap();
            let lines: Vec<_> = text_str.split('\n').map(|s| s.trim().to_owned()).collect();
            Ok(lines)
        }
    }
}

pub fn installed_toolchain() -> Vec<String> {
    let output = String::from_utf8(
        Command::new("rustup")
            // .arg("show")
            .args(&["toolchain", "list"])
            .output()
            .expect("rustup show failed")
            .stdout,
    )
    .unwrap();

    let lines: Vec<&str> = output.split('\n').collect();

    let re_nightly = Regex::new(r"^nightly-\d{4}-\d{2}-\d{2}-").unwrap();
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    let mut installed_toolchain = vec![];

    for line in &lines {
        if re_nightly.is_match(line) {
            installed_toolchain.push(re_date.find(line).unwrap().as_str().to_owned());
        }
    }
    installed_toolchain
}
