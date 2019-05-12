extern crate cargo_rls_install;
use cargo_rls_install::{
    commands::{
        component_add, component_add_and_get_output, print_rust_and_rls_install, select_channel,
    },
    global::PRESENT_DATE,
    local_env::latest_txt_path,
    options::{help, parse_args, Channel},
    scraping::RustupCompenentsHistory,
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
            _ => println!("Invalid option"),
        }
        println!("End");
        exit(0);
    }

    let mut has_stable = false;
    let mut has_beta = false;

    let re_channel = Regex::new(r"(default)").unwrap();
    let mut channel_name = String::new();

    for lt in installed_toolchains() {
        if lt.starts_with("stable-") {
            has_stable = true;
        } else if lt.starts_with("beta-") {
            has_beta = true;
        }
        if re_channel.is_match(&lt) {
            channel_name = lt;
        }
    }

    // Stable choice
    if o.stable {
        print_rust_and_rls_install(
            "stable",
            o.yes,
            has_stable,
            channel_name.starts_with("stable-"),
        );
    }

    // Beta choice
    if o.beta {
        print_rust_and_rls_install("beta", o.yes, has_beta, channel_name.starts_with("beta-"));
    }

    // Nightly choice
    if o.nightly {
        nightly(o.yes);
    }

    // Default toolchain may have been changed
    for lt in installed_toolchains() {
        if re_channel.is_match(&lt) {
            channel_name = lt.replace(" (default)", "");
        }
    }

    // Install rustfmt
    if o.rustfmt && !channel_name.is_empty() {
        component_add(&channel_name, "rustfmt");
    }

    // Wrapper "rustup component add"
    let comp_add_some = o.comp_add.is_some();
    if comp_add_some && !channel_name.is_empty() {
        let require_comp = o.comp_add.unwrap();
        if o.rustfmt && require_comp != "rustfmt" {
            if component_add_and_get_output(&channel_name, &require_comp).starts_with("error") {
                println!("Not found Component: \"{}\"", require_comp);
            }
        } else if !o.rustfmt
            && component_add_and_get_output(&channel_name, &require_comp).starts_with("error")
        {
            println!("Not found Component: \"{}\"", require_comp);
        }
    }

    // Yes only or option nothing
    match (o.yes, o.stable, o.beta, o.nightly, o.rustfmt, comp_add_some) {
        (true, false, false, false, false, false) => match select_channel() {
            // &*: String -> &str
            Ok(ch) => match &*ch {
                "0" | "stable" | "0:stable" => {
                    print_rust_and_rls_install(
                        "stable",
                        o.yes,
                        has_stable,
                        channel_name.starts_with("stable-"),
                    );
                }
                "1" | "beta" | "1:beta" => {
                    print_rust_and_rls_install(
                        "beta",
                        o.yes,
                        has_beta,
                        channel_name.starts_with("beta-"),
                    );
                }
                "2" | "nightly" | "2:nightly" => nightly(o.yes),
                _ => println!("No matches"),
            },
            Err(_e) => {
                println!("Cancel");
            }
        },
        (false, false, false, false, false, false) => {
            help();
            println!("Please input option");
        }
        _ => (),
    }

    println!("End");
}

fn view() {
    /* Local system rust version */
    let (_, platform_name) = sysroot_regex();

    /* Search url decision and Scraping */
    url_decision(&platform_name)
        .as_str()
        .rustup_components_history();

    // local info
    let mut local_nightlys = vec![];

    let re_nightly = Regex::new(r"^nightly-\d{4}-\d{2}-\d{2}-").unwrap();
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    for t in installed_toolchains() {
        if re_nightly.is_match(&t) {
            local_nightlys.push(re_date.find(&t).unwrap().as_str().to_owned());
        }
    }

    // web status
    let map = PRESENT_DATE.lock().unwrap();

    /* Status table */
    println!(" * Rust information");

    // web status
    let mut seven_days = Vec::new();
    for (date, _) in map.iter() {
        seven_days.push(date);
    }

    let mut has_seven_days_before = false;
    for lt in &local_nightlys {
        if !seven_days.contains(&lt) {
            has_seven_days_before = true;
        }
    }

    if has_seven_days_before {
        println!(" ---------------------------------");
        println!(" |    Old Rust(Before 8 days)    |");
        println!(" ---------------------------------");
        println!(" | {:<19} {:^10}|", "Build date", "");
        println!(" ---------------------------------");

        for tc in &local_nightlys {
            if !seven_days.contains(&tc) {
                println!(
                    " | {:<19} {:^10}| <= Installed",
                    format!("{}{}", "nightly-", tc),
                    ""
                );
            }
        }
    }

    println!(" ---------------------------------");
    println!(" |       Rust & RLS status       |");
    println!(" ---------------------------------");
    println!(" | {:<19}|{:^10}|", "Build date", "Status");
    println!(" ---------------------------------");

    for (date, status) in map.iter() {
        if local_nightlys.contains(&date) {
            println!(
                " | {:<19}|{:^10}| <= Installed",
                format!("{}{}", "nightly-", date),
                status
            );
        } else if date.starts_with("Last") {
            println!(" ---------------------------------");
            println!(
                " |{:^31}|",
                format!("{}{}{}", date, ": ".to_owned(), status)
            );
        } else {
            println!(
                " | {:<19}|{:^10}|",
                format!("{}{}", "nightly-", date),
                status
            );
        }
    }
    println!(" ---------------------------------");
}

fn nightly(yes: bool) {
    /* Local system rust version */
    let (now_build_date, platform_name) = sysroot_regex();

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
    let text_latest = latest_txt_lines.last().unwrap(); // get last line(latest)
    let chrono_text =
        NaiveDate::parse_from_str(&text_latest, "%Y-%m-%d").expect("Parse error: NaiveData type");

    // Display
    println!(" {:<20} Status", "Build date");
    println!(" -----------------------------");

    let mut present_vec = Vec::new();

    // global variable
    let map = PRESENT_DATE.lock().unwrap();

    for (date, status) in map.iter() {
        if !date.starts_with("Last") {
            println!(" {:<20}{:>8}", format!("{}{}", "nightly-", date), status);
        }
        if status == "present" {
            present_vec.push(
                NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Parse error: NaiveData type"),
            );
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
        // Seven days missing all
        // Rust update unavailable
        println!("\nFor RLS, unfortunate 7 days");
        println!("It is impossible to find the latest version");
        println!("The following version is written in the built-in text");
        String::new()
    };

    if text_latest.is_empty() {
        println!("Can't search Rust and RLS latest version");
        exit(1);
    }

    // left==true: Installed rust-YYYY-MM-DD.
    // right==true: Scraping sucessed.
    // chrono_text and text_latest: Absolutely obtainable.
    match (!now_build_date.is_empty(), !web_latest.is_empty()) {
        (false, true) => {
            // Rust and RLS aren't installed on the local system.
            // Case: first use or not default channel nightly.
            let chrono_now_vec = match installed_nightly() {
                Ok(vec) => vec,
                Err(_e) => vec![NaiveDate::from_ymd(2018, 12, 31)],
            };

            let chrono_web = NaiveDate::parse_from_str(&web_latest, "%Y-%m-%d").unwrap();

            if chrono_web > chrono_text {
                print_rust_and_rls_install(
                    &web_latest,
                    yes,
                    chrono_now_vec.contains(&chrono_web),
                    false,
                );
                // Text write newline
                text_write(&web_latest);
            } else if chrono_web <= chrono_text {
                print_rust_and_rls_install(
                    &text_latest,
                    yes,
                    chrono_now_vec.contains(&chrono_text),
                    false,
                );
            }
        }
        (true, true) => {
            // Case: Already nightly-YYYY-MM-DD & rls installed.
            // if chrono_now > chrono_web && chrono_now > chrono_text {
            //     println!("Can't search Rust and RLS latest version");
            let chrono_now = NaiveDate::parse_from_str(&now_build_date, "%Y-%m-%d").unwrap();
            let chrono_web = NaiveDate::parse_from_str(&web_latest, "%Y-%m-%d").unwrap();

            if chrono_web > chrono_text {
                print_rust_and_rls_install(
                    &web_latest,
                    yes,
                    chrono_now >= chrono_web,
                    chrono_now == chrono_web,
                );
                // Text write newline
                text_write(&web_latest);
            } else if chrono_web <= chrono_text {
                print_rust_and_rls_install(
                    &text_latest,
                    yes,
                    chrono_now >= chrono_text,
                    chrono_now == chrono_text,
                );
            }
        }
        (true, false) => {
            // Case: clippy won't be useful for 8 days.
            let chrono_now = NaiveDate::parse_from_str(&now_build_date, "%Y-%m-%d").unwrap();
            if chrono_now < chrono_text {
                print_rust_and_rls_install(&text_latest, yes, false, false);
            } else {
                print_rust_and_rls_install(
                    &now_build_date,
                    yes,
                    chrono_now >= chrono_text,
                    chrono_now == chrono_text,
                );
            }
        }
        (false, false) => {
            // Case: Clippy won't be useful for 8 days, when this tool first use.
            let chrono_now_vec = match installed_nightly() {
                Ok(vec) => vec,
                Err(_e) => vec![NaiveDate::from_ymd(2018, 12, 31)],
            };

            print_rust_and_rls_install(
                &text_latest,
                yes,
                chrono_now_vec.contains(&chrono_text),
                false,
            )
        }
    }
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

fn installed_nightly() -> Result<Vec<NaiveDate>, String> {
    let re_default_nightly = Regex::new(r"^nightly-\d{4}-\d{2}-\d{2}-").unwrap();
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    let mut chrono_now_vec = Vec::new();

    for lt in installed_toolchains() {
        if re_default_nightly.is_match(&lt) {
            let now_build_date = re_date.find(&lt).unwrap().as_str();
            chrono_now_vec.push(NaiveDate::parse_from_str(now_build_date, "%Y-%m-%d").unwrap());
        }
    }

    if !chrono_now_vec.is_empty() {
        Ok(chrono_now_vec)
    } else {
        Err("Not installed".to_owned())
    }
}

fn text_write(web_latest: &str) {
    let writer_opt = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(latest_txt_path(BUILD_IN_TEXT_NAME))
        .expect("Can't open file");
    let mut writer = BufWriter::new(writer_opt);
    writeln!(writer, "{}", web_latest).expect("File write failed");
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
fn sysroot_regex() -> (String, String) {
    let path = &local_system_rust_version();

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
                String::new()
            };

            if now_build_date.is_empty() {
                println!("\n * Default use Rust toolchain: nightly\n");
            } else {
                println!(
                    "\n * Default use Rust toolchain: nightly-{}\n",
                    now_build_date
                );
            }
            (now_build_date, platform(&no_head))
        }
        (false, true, false) => {
            println!("\n * Default use Rust toolchain: beta\n");
            let no_head = re_beta.replace(path, "");
            let platform_name = platform(&no_head);
            (String::new(), platform_name)
        }
        (false, false, true) => {
            println!("\n * Default use Rust toolchain: stable\n");
            let no_head = re_stable.replace(path, "");
            (String::new(), platform(&no_head))
        }
        _ => {
            eprintln!("Other Error");
            (String::new(), String::new())
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
            let lines: Vec<_> = text_str
                .trim_end()
                .split('\n')
                .map(|s| s.trim().to_owned())
                .collect();
            Ok(lines)
        }
    }
}

pub fn installed_toolchains() -> Vec<String> {
    let output = String::from_utf8(
        Command::new("rustup")
            // .arg("show")
            .args(&["toolchain", "list"])
            .output()
            .expect("rustup show failed")
            .stdout,
    )
    .unwrap();

    output
        .trim_end()
        .split('\n')
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<_>>()
}
