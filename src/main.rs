extern crate cargo_rls_install;
use cargo_rls_install::{
    commands::{print_rust_and_rls_install, rls_install, rust_set_default, select_channel},
    global::PRESENT_DATE,
    help, installed_toolchain, latest_txt_path, parse_args, RustupCompenentsHistory,
};
use chrono::NaiveDate;
use regex::Regex;
use std::io::{
    BufRead, BufReader, BufWriter, ErrorKind,
    ErrorKind::{NotFound, Other},
    Write,
};
use std::process::{exit, Command};
use std::{fs, result, str};

const BUILD_IN_TEXT_NAME: &str = "latest.txt";

fn main() {
    // Command line
    // lib.rs
    let o = parse_args();

    // view option
    if o.view {
        /* Local system rust version */
        let console_stdout = local_system_rust_version();
        let (_, platform_name) = sysroot_regex(&console_stdout);

        /* Search url decision and Scraping */
        url_decision(&platform_name)
            .as_str()
            .rustup_components_history();

        view_option();

        exit(0);
    }

    // Stable choice
    match (o.yes, o.stable) {
        (false, true) => {
            print_rust_and_rls_install("stable", false);
        }
        (true, true) => {
            print_rust_and_rls_install("stable", true);
        }
        _ => (),
    }

    // Beta choice
    match (o.yes, o.beta) {
        (false, true) => {
            print_rust_and_rls_install("beta", false);
        }
        (true, true) => {
            print_rust_and_rls_install("beta", true);
        }
        _ => (),
    }

    // Nightly choice
    match (o.yes, o.nightly) {
        (false, true) => {
            nightly(false);
        }
        (true, true) => {
            nightly(true);
        }
        _ => (),
    }

    // Yes only
    match (o.yes, o.stable, o.beta, o.nightly) {
        (true, false, false, false) => match select_channel() {
            Ok(ch) => {
                if ch == "stable" || ch == "beta" {
                    print_rust_and_rls_install(&ch, true);
                } else if ch == "nightly" {
                    nightly(true);
                } else {
                    println!("No matches");
                }
            }
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
    println!("End.");
}

fn view_option() {
    // local info
    let toolchains = installed_toolchain();

    // web status
    let map = PRESENT_DATE.lock().unwrap();

    /* Status table */
    println!(" * Rust information\n");
    println!(" -----Rust & RLS status-----");
    println!(" ---------------------------");
    println!(" |{:^12}|{:^12}|", "Build date", "Status");
    println!(" ---------------------------");

    // web status
    for (date, status) in map.iter() {
        if toolchains.contains(&date.format("%F").to_string()) {
            println!(
                " |{:^12}|{:^12}| <= Rust Installed",
                date.format("%F").to_string(),
                status
            );
        } else {
            println!(" |{:^12}|{:^12}|", date.format("%F").to_string(), status);
        }
    }

    // println!(" ---------------------------");
    // println!(" * Installed Nightly Rust");
    // println!(" ___________________________");

    // for tc in toolchains {
    //     println!(" |{:^12}|{:^12}|", tc, "Installed");
    // }

    println!(" -----------End-------------");
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

fn nightly(yes: bool) {
    /* Local system rust version */
    let console_stdout = local_system_rust_version();
    let (now_build_date, platform_name) = sysroot_regex(&console_stdout);

    /* Search url decision */
    let url = url_decision(&platform_name);

    // Get text version(nightly-"Date" store)
    let mut text_latest_version = match latest_text_last_line() {
        Ok(version) => version.trim().to_owned(),
        Err(_e) => "".to_owned(),
    };

    /* Latest version description */
    let file_data: &str = "";
    if !url.is_empty() {
        /* URL mode */
        text_latest_version = alive_rls(&url, &text_latest_version);
    } else if !file_data.is_empty() {
        /* Readfile mode */
        text_latest_version = alive_rls(file_data, &text_latest_version);
    }

    if text_latest_version == now_build_date {
        let version = "nightly-".to_owned() + &text_latest_version;
        println!();
        println!("    1. Rust version: OK");
        match rls_install(&version, yes) {
            Ok(()) => (),
            Err(e) => eprintln!("{:?}", e),
        }
        match rust_set_default(&version, yes) {
            Ok(()) => (),
            Err(e) => eprintln!("{:?}", e),
        }
    } else {
        match (text_latest_version.is_empty(), now_build_date.is_empty()) {
            // There is both data
            (false, false) => {
                // Local rust version date(nightly-{date}) compare
                // If you have the latest version, recommend installing
                let text_date = NaiveDate::parse_from_str(&text_latest_version, "%Y-%m-%d")
                    .expect("date type parse error");

                let toolchain_date = NaiveDate::parse_from_str(&now_build_date, "%Y-%m-%d")
                    .expect("date type parse error");

                if text_date > toolchain_date {
                    print_rust_and_rls_install(&("nightly-".to_owned() + &text_latest_version), yes)
                }
            }
            // Rust and RLS aren't installed on the local system
            (false, true) => {
                print_rust_and_rls_install(&("nightly-".to_owned() + &text_latest_version), yes)
            }
            // Text data empty
            _ => {
                println!("Can't search RLS latest version.");
            }
        }
    }
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

            let platform_name = match platform(&no_head) {
                Ok(name) => name,
                Err(_e) => "".to_owned(),
            };
            (now_build_date, platform_name)
        }
        (false, true, false) => {
            let no_head = re_beta.replace(path, "");

            println!("\n * Default use Rust toolchain: Beta\n");
            let platform_name = match platform(&no_head) {
                Ok(name) => name,
                Err(_e) => "".to_owned(),
            };
            ("".to_owned(), platform_name)
        }
        (false, false, true) => {
            let no_head = re_stable.replace(path, "");

            println!("\n * Default use Rust toolchain: Stable\n");
            let platform_name = match platform(&no_head) {
                Ok(name) => name,
                Err(_e) => "".to_owned(),
            };
            ("".to_owned(), platform_name)
        }
        _ => {
            eprintln!("Other Error");
            ("".to_owned(), "".to_owned())
        }
    }
}

fn platform(no_head: &str) -> result::Result<String, ErrorKind> {
    let re_date_plus_hyphen = Regex::new(r"\d{4}-\d{2}-\d{2}-").unwrap();
    if re_date_plus_hyphen.is_match(no_head) {
        Ok(re_date_plus_hyphen.replace(no_head, "").to_string())
    } else {
        Err(Other) // No matching. <YYYY-MM-DD>
    }
}

fn latest_text_last_line() -> result::Result<String, ErrorKind> {
    let reader_opt = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .open(latest_txt_path(BUILD_IN_TEXT_NAME))
        .expect("Can't open file.");

    let reader = BufReader::new(reader_opt);

    let mut text_vector = reader
        .lines()
        .filter_map(std::result::Result::ok) // None case validate
        .collect::<Vec<_>>();

    match text_vector.len() {
        0 => Err(NotFound),
        _ => Ok(text_vector.pop().expect("vector pop failed.")),
    }
}

fn alive_rls(url: &str, text_latest_version: &str) -> String {
    url.rustup_components_history();

    let map = PRESENT_DATE.lock().unwrap();

    let mut v = vec![];
    for (date, _) in map.iter() {
        v.push(date);
    }

    println!("{:?}", v);

    let web_latest_date = v.iter().max().unwrap().format("%F").to_string();

    println!("{}", web_latest_date);

    if text_latest_version != web_latest_date {
        // Text write newline
        let writer_opt = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(latest_txt_path(BUILD_IN_TEXT_NAME))
            .expect("Can't open file.");
        let mut writer = BufWriter::new(writer_opt);
        writeln!(writer, "{}", &web_latest_date).expect("File write failed.");
    }

    if !text_latest_version.is_empty() {
        let webpage_date =
            NaiveDate::parse_from_str(&web_latest_date, "%Y-%m-%d").expect("date type parse error");

        let text_date = NaiveDate::parse_from_str(&text_latest_version, "%Y-%m-%d")
            .expect("date type parse error");

        if webpage_date >= text_date {
            web_latest_date
        } else {
            text_latest_version.to_owned()
        }
    } else {
        // web_latest_date never gets empty
        // If this is empty, panic long ago.
        web_latest_date
    }
}
