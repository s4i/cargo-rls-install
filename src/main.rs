use cargo_rls_install::{
    commands::{
        command_rust_default, command_rust_multiple_uninstall, command_rust_uninstall,
        component_add, component_add_and_get_output, print_rust_and_rls_install, select_channel,
    },
    options::{help, parse_args, Channel},
    scraping::RustupCompenentsHistory,
};

use regex::Regex;
use std::process::Command;
use std::str;

fn main() {
    // Command line
    // options.rs
    let o: Channel = parse_args();

    // Check if component name isn't empty
    let comp_add_some = o.comp_add.is_some();
    let rustup_default = o.default.is_some();
    let rustup_uninstall = o.uninstall.is_some();

    let re_channel = Regex::new(r"(default)").unwrap();
    let mut default_channel_name = String::new();

    for lt in installed_toolchains() {
        if re_channel.is_match(&lt) {
            default_channel_name = lt.replace(" (default)", "");
        }
    }

    // view option
    if o.view {
        match (
            o.yes,
            o.stable,
            o.beta,
            o.nightly,
            o.rustfmt,
            comp_add_some,
            rustup_default,
            rustup_uninstall,
        ) {
            (false, false, false, false, false, false, false, false) => view(&default_channel_name),
            _ => println!("Invalid option"),
        }
        println!("End");
        return;
    }

    // Stable choice
    if o.stable {
        print_rust_and_rls_install(
            "stable",
            o.yes,
            false,
            default_channel_name.starts_with("stable-"),
        );
    }

    // Beta choice
    if o.beta {
        print_rust_and_rls_install(
            "beta",
            o.yes,
            false,
            default_channel_name.starts_with("beta-"),
        );
    }

    // Nightly choice
    if o.nightly {
        nightly(o.yes);
    }

    // Default toolchain may have been changed
    for lt in installed_toolchains() {
        if re_channel.is_match(&lt) {
            default_channel_name = lt.replace(" (default)", "");
        }
    }

    // Install rustfmt
    if o.rustfmt {
        component_add(&default_channel_name, "rustfmt");
    }

    // Wrapper "rustup component add"
    if comp_add_some {
        let require_comp = o.comp_add.unwrap();
        if require_comp != "rustfmt" || !o.rustfmt {
            // Catch error message returned to stderr
            output_command_message(&default_channel_name, &require_comp);
        }
    }

    // Wrapper "rustup default [toolchain]"
    if rustup_default {
        let toolchain = o.default.unwrap();
        if toolchain.is_ascii() {
            change_defalt_toolchain(&toolchain.to_lowercase());
        } else {
            println!("Nonexistent toolchain");
        }
    }

    // Wrapper "rustup uninstall [toolchain]"
    if rustup_uninstall {
        let toolchain = o.uninstall.unwrap();
        if toolchain.is_ascii() {
            uninstall_toolchain(&toolchain.to_lowercase(), &default_channel_name);
        } else {
            println!("Nonexistent toolchain");
        }
    }

    // Yes only or option nothing
    match (
        o.yes,
        o.stable,
        o.beta,
        o.nightly,
        o.rustfmt,
        comp_add_some,
        rustup_default,
        rustup_uninstall,
    ) {
        // Yes only
        (true, false, false, false, false, false, false, false) => match select_channel() {
            // &*: String -> &str
            Ok(ch) => match &*ch {
                "0" | "stable" | "0:stable" => {
                    print_rust_and_rls_install(
                        "stable",
                        o.yes,
                        false,
                        default_channel_name.starts_with("stable-"),
                    );
                }
                "1" | "beta" | "1:beta" => {
                    print_rust_and_rls_install(
                        "beta",
                        o.yes,
                        false,
                        default_channel_name.starts_with("beta-"),
                    );
                }
                "2" | "nightly" | "2:nightly" => nightly(o.yes),
                _ => println!("No matches"),
            },
            Err(_e) => {
                println!("Cancel");
            }
        },
        (false, false, false, false, false, false, false, false) => {
            help();
            println!("Please input option");
        }
        _ => (),
    }
    println!("End");
}

fn view(default_toolchain: &str) {
    /* Local system rust version */
    let (_, platform_name) = sysroot_regex();

    // local info
    let mut local_nightlys = vec![];

    let re_nightly = Regex::new(r"^nightly-\d{4}-\d{2}-\d{2}-").unwrap();
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    for t in installed_toolchains() {
        if re_nightly.is_match(&t) {
            local_nightlys.push(re_date.find(&t).unwrap().as_str().to_owned());
        }
    }

    /* Status table */
    println!(" * Rust information");

    // web status
    /* Search url decision and Scraping */
    let web_status = match url_decision(&platform_name)
        .as_str()
        .rustup_components_history()
    {
        Ok(map) => map,
        Err(_) => {
            println!(" >>> May not be connected to the network\n");
            return;
        }
    };

    let mut seven_days = Vec::new();
    for date in web_status[0].keys() {
        seven_days.push(date);
    }

    let mut has_seven_days_before = false;
    for ln in &local_nightlys {
        if !seven_days.contains(&ln) {
            has_seven_days_before = true;
            break;
        }
    }

    println!(" --------------------------------------------");

    if has_seven_days_before {
        println!(" | {:^40} |", "Old Rust(Before 8 days)");
        println!(" --------------------------------------------");
        println!(" | {:<40} |", "Build date");
        println!(" --------------------------------------------");

        for tc in &local_nightlys {
            if !seven_days.contains(&tc) {
                if default_toolchain.starts_with(&format!("{}{}", "nightly-", tc)) {
                    println!(
                        " | {:<40} | <= Installed(Default)",
                        format!("{}{}", "nightly-", tc)
                    );
                } else {
                    println!(" | {:<40} | <= Installed", format!("{}{}", "nightly-", tc));
                }
            }
        }
        println!(" --------------------------------------------");
    }

    println!(" | {:<40} |", "One week Rust & RLS status");
    println!(" --------------------------------------------");
    println!(" | {:<19}| {:^9}| {:^9}|", "Build date", "Clippy", "RLS");
    println!(" --------------------------------------------");

    for date in seven_days.iter() {
        if local_nightlys.contains(&date) {
            let build_date = format!("{}{}", "nightly-", date);
            if default_toolchain.starts_with(&build_date) {
                println!(
                    " | {:<19}| {:^9}| {:^9}| <= Installed(Default)",
                    build_date,
                    web_status[0].get(date.to_owned()).unwrap(),
                    web_status[1].get(date.to_owned()).unwrap(),
                );
            } else {
                println!(
                    " | {:<19}| {:^9}| {:^9}| <= Installed",
                    format!("{}{}", "nightly-", date),
                    web_status[0].get(date.to_owned()).unwrap(),
                    web_status[1].get(date.to_owned()).unwrap(),
                );
            }
        } else if date.starts_with("Last") {
            println!(" --------------------------------------------");
            println!(" | {:^40} |", date);
            println!(
                " | {:^40} |",
                format!(
                    "Clippy:{}, RLS:{}",
                    web_status[0].get(date.to_owned()).unwrap(),
                    web_status[1].get(date.to_owned()).unwrap(),
                )
            );
        } else {
            println!(
                " | {:<19}| {:^9}| {:^9}|",
                format!("{}{}", "nightly-", date),
                web_status[0].get(date.to_owned()).unwrap(),
                web_status[1].get(date.to_owned()).unwrap(),
            );
        }
    }
    println!(" --------------------------------------------");
}

fn nightly(yes: bool) {
    /* Local system rust version */
    let (_, platform_name) = sysroot_regex();

    let web_status = match url_decision(&platform_name)
        .as_str()
        .rustup_components_history()
    {
        Ok(map) => map,
        Err(_) => {
            println!(" >>> May not be connected to the network");
            return;
        }
    };

    let mut clippy_present_last = String::new();
    let mut rls_present_last = String::new();

    // Table
    println!(" {:<22}{:<8} {:<8}", "Build date", "Clippy", "RLS");
    println!(" --------------------------------------------");

    for date in web_status[0].keys() {
        let clippy = web_status[0].get(&date.to_owned()).unwrap();
        let rls = web_status[1].get(&date.to_owned()).unwrap();
        if date.starts_with("Last") {
            clippy_present_last = web_status[0].get(date).unwrap().to_owned();
            rls_present_last = web_status[1].get(date).unwrap().to_owned();
        } else {
            println!(
                " {:<20} {:>8} {:>8}",
                format!("{}{}", "nightly-", date),
                clippy,
                rls
            );
        }
    }

    println!(" --------------------------------------------");

    // Rust and RLS aren't installed on the local system.
    let now_vec = match installed_nightly() {
        Ok(vec) => vec,
        Err(_e) => vec![String::new()],
    };

    if !clippy_present_last.is_empty() && !rls_present_last.is_empty() {
        let clippy_repl: i32 = clippy_present_last.replace("-", "").parse().unwrap();
        let rls_repl: i32 = rls_present_last.replace("-", "").parse().unwrap();
        if clippy_repl >= rls_repl {
            print_rust_and_rls_install(
                &rls_present_last,
                yes,
                now_vec.contains(&rls_present_last),
                false,
            );
        } else if clippy_repl < rls_repl {
            print_rust_and_rls_install(
                &clippy_present_last,
                yes,
                now_vec.contains(&clippy_present_last),
                false,
            );
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

fn installed_nightly() -> Result<Vec<String>, String> {
    let re_default_nightly = Regex::new(r"^nightly-\d{4}-\d{2}-\d{2}-").unwrap();
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    let mut now_vec = Vec::new();

    for lt in installed_toolchains() {
        if re_default_nightly.is_match(&lt) {
            now_vec.push(re_date.find(&lt).unwrap().as_str().to_owned());
        }
    }

    if !now_vec.is_empty() {
        Ok(now_vec)
    } else {
        Err("Not installed".to_owned())
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

            let now_default_build_date = if re_date.is_match(&no_head) {
                re_date.find(&no_head).unwrap().as_str().to_owned()
            } else {
                String::new()
            };

            if now_default_build_date.is_empty() {
                println!("\n * Default use toolchain: nightly\n");
            } else {
                println!(
                    "\n * Default use toolchain: nightly-{}\n",
                    now_default_build_date
                );
            }
            (now_default_build_date, platform(&no_head))
        }
        (false, true, false) => {
            println!("\n * Default use toolchain: beta\n");
            let no_head = re_beta.replace(path, "");
            let platform_name = platform(&no_head);
            (String::new(), platform_name)
        }
        (false, false, true) => {
            println!("\n * Default use toolchain: stable\n");
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

fn output_command_message(default_channel_name: &str, require_comp: &str) {
    let message = component_add_and_get_output(&default_channel_name, &require_comp);
    if message.starts_with("error") {
        println!("Not found component: \"{}\"", require_comp);
    } else {
        println!("{}", message.trim_end());
        println!("OK");
    }
}

fn change_defalt_toolchain(toolchain_name: &str) {
    if toolchain_name.starts_with('s') {
        command_rust_default("stable");
    } else if toolchain_name.starts_with("beta-") {
        command_rust_default(&toolchain_name);
    } else if toolchain_name.starts_with('b') {
        command_rust_default("beta");
    } else if toolchain_name == "nightly" {
        command_rust_default(&"nightly".to_owned());
    } else if toolchain_name.starts_with("nightly-") {
        command_rust_default(&toolchain_name);
    } else if toolchain_name.starts_with('n') {
        let get_tail_toolchain = installed_toolchains();
        command_rust_default(
            &get_tail_toolchain
                .last()
                .unwrap_or(&"nightly".to_owned())
                .replace(" (default)", ""),
        );
    } else {
        println!("Not found toolchain: \"{}\"", toolchain_name);
    }
}

// If default toolchain, don't uninstall
fn uninstall_toolchain(toolchain_name: &str, default_channel_name: &str) {
    if toolchain_name.starts_with('s') {
        // Check default toolchain
        if !default_channel_name.starts_with('s') {
            command_rust_uninstall("stable");
        } else {
            println!("Currently set to default toolchain");
        }
    } else if toolchain_name.starts_with("beta-") {
        // Check default toolchain
        if !default_channel_name.starts_with(&toolchain_name) {
            command_rust_uninstall(&toolchain_name);
        } else {
            println!("Currently set to default toolchain");
        }
    } else if toolchain_name.starts_with('b') {
        // Check default toolchain
        if !default_channel_name.starts_with('b') {
            command_rust_uninstall("beta");
        } else {
            println!("Currently set to default toolchain");
        }
    } else if toolchain_name.starts_with("nightly-") {
        // Check default toolchain
        if !default_channel_name.starts_with(&toolchain_name) {
            command_rust_uninstall(&toolchain_name);
        } else {
            println!("Currently set to default toolchain");
        }
    } else if toolchain_name.starts_with('n') {
        command_rust_uninstall(&"nightly".to_owned());
    } else if toolchain_name == "a" || toolchain_name == "all" {
        uninstall_all_dated_nightly();
    } else {
        println!("Not found toolchain: \"{}\"", toolchain_name);
    }
}

fn uninstall_all_dated_nightly() {
    let mut dated_nightly = vec![];
    let re_channel = Regex::new(r"(default)").unwrap();
    let re_nightly = Regex::new(r"nightly-\d{4}-\d{2}-\d{2}-").unwrap();

    for val in installed_toolchains() {
        if re_nightly.is_match(&val) {
            dated_nightly.push(val);
        }
    }

    if dated_nightly.len() >= 2 {
        // Eliminate the latest
        dated_nightly.pop();
    }

    if dated_nightly.is_empty() {
        println!("Can't find what to uninstall.");
        println!("Note: Nightly rust of latest version isn't eligible for uninstallation.");
    } else {
        let mut uninstall_count = 0;
        let mut uninstall_targets = vec![];
        for dn in dated_nightly {
            if !re_channel.is_match(&dn) {
                uninstall_targets.push(dn);
                uninstall_count += 1;
            }
        }
        // Case: When dated nightly rust was two installed, default toolchain and latest dated nightly
        if uninstall_count == 0 {
            println!("Can't find what to uninstall.");
            println!("Note: Latest nightly rust and default toolchain isn't eligible for uninstallation.");
        } else {
            command_rust_multiple_uninstall(uninstall_targets);
        }
    }
}
