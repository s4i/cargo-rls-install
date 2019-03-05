extern crate lib_rs;

use lib_rs::cargo_home;
use lib_rs::global::PRESENT_DATE;
use lib_rs::help;
use lib_rs::parse_args;

use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::io::ErrorKind::{InvalidData, NotFound, Other};
use std::io::{stdin, BufRead, BufReader, BufWriter, ErrorKind, Read, Result, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::{fs, result, str};

// Version write: Cargo.toml and here
const APP_NAME: &str = "cargo-rls-install-1.0.9";
const BUILD_IN_TEXT_NAME: &str = "latest.txt";

fn main() {
    // Command line
    // lib.rs
    let o = parse_args();

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

fn select_channel() -> result::Result<String, failure::Error> {
    println!("Select channel");
    println!("[stable/beta/nightly]");
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.to_lowercase().trim().to_owned())
}

fn nightly(yes: bool) {
    /* File read */
    let file_data: &str = "";
    // let file_path = r#"src/x86_64-pc-windows-msvc.html"#;
    // let mut buf = vec![];
    // match read_html_file(file_path) {
    //     Ok(v) => buf = v,
    //     Err(e) => println!("{}", e.to_owned()),
    // }
    // file_data = str::from_utf8(&buf).unwrap();
    // println!("{:?}", file_data);

    /* Local system rust version */
    let console_stdout = local_system_rust_version();
    let (now_build_date, platform_name) = sysroot_regex(&console_stdout);

    /* Switch web pages */
    let mut url: &str =
        &("https://mexus.github.io/rustup-components-history/".to_owned() + &platform_name);
    if platform_name == "x86_64-unknown-linux-gnu" {
        url = "https://mexus.github.io/rustup-components-history/";
    }

    // get text version
    let mut text_latest_version;
    match latest_text_last_line() {
        Ok(version) => text_latest_version = version.trim().to_owned(),
        Err(_e) => text_latest_version = "".to_owned(),
    }

    /* Latest version description */
    if !url.is_empty() {
        /* URL mode */
        text_latest_version = alive_rls(url, &text_latest_version);
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
            Err(e) => eprintln!("{}", e.to_string()),
        }
        match rust_set_default(&version, yes) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e.to_string()),
        }
    } else {
        match (text_latest_version.is_empty(), now_build_date.is_empty()) {
            // There is both data
            (false, false) => {
                // Local rust version date(nightly-{date}) compare
                // If you have the latest version, recommend installing
                match left_ge_right_year_and_anyone(&text_latest_version, &now_build_date) {
                    true => print_rust_and_rls_install(
                        &("nightly-".to_owned() + &text_latest_version),
                        yes,
                    ),
                    false => {}
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

fn left_ge_right_year_and_anyone(left: &str, right: &str) -> bool {
    println!("Search new version: nightly-{}", left);
    println!("Current installed version: nightly-{}", right);

    let compare_date1 = left
        .split('-')
        .map(|x| x.parse().expect("parse error"))
        .collect::<Vec<i32>>(); // [2019 ,2 ,24]
    let compare_date2 = right
        .split('-')
        .map(|x| x.parse().expect("parse error"))
        .collect::<Vec<i32>>(); // [2019 ,2 ,24]
    let mut decision = (false, false, false); // YYYY | MM || DD
    let mut cnt: usize = 0;
    for (i, j) in compare_date1.iter().zip(compare_date2.iter()) {
        if i >= j {
            match cnt {
                0 => decision.0 = true,
                1 => decision.1 = true,
                2 => decision.2 = true,
                _ => {}
            }
        }
        cnt += 1;
    }
    match decision {
        (true, true, true) => true,
        (true, true, false) => true,
        (true, false, true) => true,
        (true, false, false) => {
            // year compare
            if compare_date1[0] > compare_date2[0] {
                return true;
            }
            false
        }
        _ => false,
    }
}

fn local_system_rust_version() -> String {
    let sysroot = Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .expect("failed to execute");

    let forced_linux_path_format = String::from_utf8(sysroot.stdout)
        .expect("Encode failed")
        .trim_start_matches('/')
        .trim_end()
        .replace("\\", "/");

    forced_linux_path_format
}

fn sysroot_regex(path: &str) -> (String, String) {
    let re_stable = Regex::new(r"\b.+stable-").unwrap();
    let re_beta = Regex::new(r"\b.+beta-").unwrap();
    let re_nightly = Regex::new(r"\b.+nightly-").unwrap();

    // Get platform name
    let mut platform_name = "".to_owned();

    match (
        re_nightly.is_match(path),
        re_beta.is_match(path),
        re_stable.is_match(path),
    ) {
        (true, false, false) => {
            // Forward path string delete
            let no_head = re_nightly.replace(path, "");

            let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

            let mut now_build_date = "".to_owned();
            if re_date.is_match(&no_head) {
                now_build_date = re_date.find(&no_head).unwrap().as_str().to_owned();
            }

            match platform(&no_head) {
                Ok(name) => platform_name = name,
                Err(_e) => {}
            }
            return (now_build_date, platform_name);
        }
        (false, true, false) => {
            let no_head = re_beta.replace(path, "");

            println!("Default use Rust channel: Beta");
            match platform(&no_head) {
                Ok(name) => platform_name = name,
                Err(_e) => (),
            }
            return ("".to_owned(), platform_name);
        }
        (false, false, true) => {
            let no_head = re_stable.replace(path, "");

            println!("Default use Rust channel: Stable");
            match platform(&no_head) {
                Ok(name) => platform_name = name,
                Err(_e) => (),
            }
            return ("".to_owned(), platform_name);
        }
        _ => {
            eprintln!("Other Error");
            return ("".to_owned(), "".to_owned());
        }
    }
}

fn platform(no_head: &str) -> result::Result<String, ErrorKind> {
    let re_date_plus_hyphen = Regex::new(r"\d{4}-\d{2}-\d{2}-").unwrap();
    match re_date_plus_hyphen.is_match(no_head) {
        true => {
            let platform_name = re_date_plus_hyphen.replace(no_head, "");
            Ok(platform_name.to_string())
        }
        false => Err(Other), // No matching. <YYYY-MM-DD>
    }
}

#[allow(dead_code)]
fn read_html_file<P: AsRef<Path>>(file_path: P) -> std::io::Result<Vec<u8>> {
    let mut buf = vec![];
    let mut reader = BufReader::new(fs::File::open(file_path)?);
    reader.read_to_end(&mut buf)?;
    // println!("{:?}", std::str::from_utf8(&buf).unwrap());
    Ok(buf)
}

fn latest_text_last_line() -> result::Result<String, ErrorKind> {
    let reader_opt = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .open(cargo_home(APP_NAME, BUILD_IN_TEXT_NAME).expect("cargo_home_func"))
        .expect("Can't open file.");

    let reader = BufReader::new(reader_opt);

    let mut text_vector = reader
        .lines()
        .filter_map(|line| line.ok()) // None case validate
        .collect::<Vec<_>>();

    match text_vector.len() {
        0 => Err(NotFound),
        _ => Ok(text_vector.pop().expect("vector pop failed.")),
    }
}

fn alive_rls(target: &str, text_latest_version: &str) -> String {
    let writer_opt = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(cargo_home(APP_NAME, BUILD_IN_TEXT_NAME).expect("cargo_home_func"))
        .expect("Can't open file.");

    let mut web_latest_date = "".to_owned();
    match &target.rustup_components_history() {
        Ok(()) => {
            let vec = PRESENT_DATE.lock().unwrap();
            web_latest_date = vec.first().unwrap().to_owned();
            if text_latest_version != web_latest_date {
                // Text write newline
                let mut writer = BufWriter::new(writer_opt);
                writeln!(writer, "{}", &web_latest_date).expect("File write failed.");
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    match !text_latest_version.is_empty() {
        true => {
            if text_latest_version == web_latest_date {
                return web_latest_date.to_owned();
            } else if left_ge_right_year_and_anyone(&web_latest_date, &text_latest_version) {
                return web_latest_date.to_owned();
            } else {
                return text_latest_version.to_owned();
            }
        }
        false => web_latest_date,
    }
}

// Trait: RustupCompenentsHistory
// Use URL or String
// HTML: table tag scraping
trait RustupCompenentsHistory {
    fn rustup_components_history(&self) -> result::Result<(), ErrorKind>;
    fn read_html_document(url: &str) -> result::Result<Document, ErrorKind>;
    fn scraping(document: Document);
}

impl RustupCompenentsHistory for &str {
    fn rustup_components_history(&self) -> result::Result<(), ErrorKind> {
        let url = self as &str;
        match Self::read_html_document(url) {
            Ok(document) => Self::scraping(document),
            Err(_e) => {} // "InvalidData" not print
        }
        Ok(())
    }

    fn read_html_document(url: &str) -> result::Result<Document, ErrorKind> {
        if url.starts_with("http") {
            // Get HTML from internet
            let resp = reqwest::get(url).expect("Can't get response.");
            let document = Document::from_read(resp).expect("Data read failed.");
            Ok(document)
        } else if url.starts_with("<!DOCTYPE") {
            // Get HTML local file
            let document = Document::from(url);
            Ok(document)
        } else {
            Err(InvalidData)
        }
    }

    fn scraping(document: Document) {
        let date = document
            .find(Attr("scope", "col"))
            .skip(1)
            .map(|tag| tag.text())
            .collect::<Vec<_>>();
        // println!("{:?}", date);
        // let pkg_name = document.find(Attr("scope", "row"))
        //     .map(|tag| tag.text()).collect::<Vec<_>>();
        // println!("{:?}", pkg_name);
        // let build_status = document.find(Name("td"))
        //     .map(|tag| tag.text()).collect::<Vec<_>>();
        // println!("{:?}", build_status);

        let build_status = document
            .find(Attr("scope", "row"))
            .filter(|x| x.text() == "rls")
            // .map(|tag| tag.text())
            // .collect::<Vec<_>>(); // result -> "rls"
            .next()
            .expect("iter to string failed.")
            .parent()
            .unwrap()
            .find(Name("td"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        // println!("{:?}", build_status);

        if build_status.iter().all(|x| x == "missing") {
            println!("For RLS, unfortunate 8 days.");
            println!("It is impossible to find the latest version.");
            println!("The following version is written in the built-in text.");
        } else {
            for (i, s) in build_status.iter().enumerate() {
                if s == "present" {
                    let mut vec = PRESENT_DATE.lock().unwrap();
                    vec.push(date[i].clone());
                }
            }
        }
    }
}

fn print_rust_and_rls_install(v: &str, yes: bool) {
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
        Err(e) => eprintln!("{}", e.to_string()),
    }

    // Operation 2
    match rls_install(&v, yes) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e.to_string()),
    }

    // Operation 3
    match rust_set_default(&v, yes) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e.to_string()),
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
        println!("$ rustup install {}", v);
        Command::new("rustup")
            .arg("install")
            .arg(v)
            .status()
            .expect("Abort installation");
        println!("OK");
    } else {
        println!("Command execution (y/n)");
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            println!("$ rustup install {}", v);
            Command::new("rustup")
                .arg("install")
                .arg(v)
                .status()
                .expect("Abort installation");
            println!("OK");
        } else {
            println!("Cancel");
            exit(0);
        }
    }
    Ok(())
}

fn rls_install(v: &str, yes: bool) -> Result<()> {
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
        println!("$ rustup component add rls --toolchain {}", v);
        Command::new("rustup")
            .arg("component")
            .arg("add")
            .arg("rls")
            .arg("--toolchain")
            .arg(v)
            .status()
            .expect("Abort installation");
        println!("OK");

        // rust-analysis install
        println!("$ rustup component add rust-analysis --toolchain {}", v);
        Command::new("rustup")
            .arg("component")
            .arg("add")
            .arg("rust-analysis")
            .arg("--toolchain")
            .arg(v)
            .status()
            .expect("Abort installation");
        println!("OK");

        // rust-src install
        println!("$ rustup component add rust-src --toolchain {}", v);
        Command::new("rustup")
            .arg("component")
            .arg("add")
            .arg("rust-src")
            .arg("--toolchain")
            .arg(v)
            .status()
            .expect("Abort installation");
        println!("OK");
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
            println!("$ rustup component add rls --toolchain {}", v);
            Command::new("rustup")
                .arg("component")
                .arg("add")
                .arg("rls")
                .arg("--toolchain")
                .arg(v)
                .status()
                .expect("Abort installation");
            println!("OK");

            // rust-analysis install
            println!("$ rustup component add rust-analysis --toolchain {}", v);
            Command::new("rustup")
                .arg("component")
                .arg("add")
                .arg("rust-analysis")
                .arg("--toolchain")
                .arg(v)
                .status()
                .expect("Abort installation");
            println!("OK");

            // rust-src install
            println!("$ rustup component add rust-src --toolchain {}", v);
            Command::new("rustup")
                .arg("component")
                .arg("add")
                .arg("rust-src")
                .arg("--toolchain")
                .arg(v)
                .status()
                .expect("Abort installation");
            println!("OK");
        } else {
            println!("Cancel");
            exit(0);
        }
    }
    Ok(())
}

fn rust_set_default(v: &str, yes: bool) -> Result<()> {
    println!(
        r"
    3. Set default:

        $ rustup default {}
    ",
        v
    );

    if yes {
        println!("$ rustup default {}", v);
        Command::new("rustup")
            .arg("default")
            .arg(v)
            .status()
            .expect("Abort installation");
        println!("OK");
    } else {
        println!("Command execution (y/n)");
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            println!("$ rustup default {}", v);
            Command::new("rustup")
                .arg("default")
                .arg(v)
                .status()
                .expect("Abort installation");
            println!("OK");
        } else {
            println!("Cancel");
            exit(0);
        }
    }
    Ok(())
}
