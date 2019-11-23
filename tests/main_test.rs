use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

#[test]
fn left_ge_right_year_and_anyone() {
    let left = "2020-03-13".to_owned();
    let right = "2019-03-13".to_owned();

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

    for (cnt, item) in compare_date1.iter().zip(compare_date2.iter()).enumerate() {
        if item.0 >= item.1 {
            match cnt {
                0 => decision.0 = true,
                1 => decision.1 = true,
                2 => decision.2 = true,
                _ => {}
            }
        }
    }

    match decision {
        (true, true, true) => (),
        (true, true, false) => (),
        (true, false, true) => (),
        (true, false, false) => {
            // year compare
            let ge = if compare_date1[0] > compare_date2[0] {
                compare_date1[0]
            } else {
                compare_date2[1]
            };
            assert_eq!(2020, ge);
        }
        _ => (),
    }
}

#[test]
fn local_system_rust_version() {
    let sysroot = "C:\\Users\\uname\\.cargo\\registry\\src\\cargo-rls-install\\".to_owned();

    // forced linux path format
    let path = sysroot
        .trim_start_matches('/')
        .trim_end()
        .replace("\\", "/");

    assert_eq!(
        "C:/Users/uname/.cargo/registry/src/cargo-rls-install/".to_owned(),
        path
    );
}

#[test]
fn sysroot_regex() {
    let stable = r"C:\Users\uname\.rustup\toolchains\stable-x86_64-pc-windows-msvc";
    let beta = r"C:\Users\uname\.rustup\toolchains\beta-x86_64-pc-windows-msvc";
    let nightly = r"C:\Users\uname\.rustup\toolchains\nightly-2019-03-10-x86_64-pc-windows-msvc";
    let channel = [stable, beta, nightly];

    let re_stable = Regex::new(r"\b.+stable-").unwrap();
    let re_beta = Regex::new(r"\b.+beta-").unwrap();
    let re_nightly = Regex::new(r"\b.+nightly-").unwrap();

    for ch in channel.iter() {
        // Get platform name
        match (
            re_nightly.is_match(ch),
            re_beta.is_match(ch),
            re_stable.is_match(ch),
        ) {
            (true, false, false) => {
                // Forward path string delete
                let no_head = re_nightly.replace(ch, "");

                let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

                let now_build_date = if re_date.is_match(&no_head) {
                    re_date.find(&no_head).unwrap().as_str().to_owned()
                } else {
                    String::new()
                };

                let re_date_plus_hyphen = Regex::new(r"\d{4}-\d{2}-\d{2}-").unwrap();

                let platform_name = if re_date_plus_hyphen.is_match(&no_head) {
                    re_date_plus_hyphen.replace(&no_head, "").to_string()
                } else {
                    String::new()
                };

                assert_eq!(
                    ("2019-03-10".to_owned(), "x86_64-pc-windows-msvc".to_owned()),
                    (now_build_date, platform_name)
                );
            }
            (false, true, false) => {
                let no_head = re_beta.replace(ch, "");

                println!("Default use Rust channel: Beta");
                let re_date_plus_hyphen = Regex::new(r"\d{4}-\d{2}-\d{2}-").unwrap();

                let platform_name = if re_date_plus_hyphen.is_match(&no_head) {
                    re_date_plus_hyphen.replace(&no_head, "").to_string()
                } else {
                    no_head.to_string()
                };
                assert_eq!("x86_64-pc-windows-msvc".to_owned(), platform_name);
            }
            (false, false, true) => {
                let no_head = re_stable.replace(ch, "");

                println!("Default use Rust channel: Stable");
                let re_date_plus_hyphen = Regex::new(r"\d{4}-\d{2}-\d{2}-").unwrap();

                let platform_name = if re_date_plus_hyphen.is_match(&no_head) {
                    re_date_plus_hyphen.replace(&no_head, "").to_string()
                } else {
                    no_head.to_string()
                };
                assert_eq!("x86_64-pc-windows-msvc".to_owned(), platform_name);
            }
            _ => {
                eprintln!("Other Error");
            }
        }
    }
}

#[test]
fn latest_text_last_line() {
    let mut path = std::path::PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("read_test");
    path.push("text");
    path.push("latest.txt");
    let reader_opt = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .open(path)
        .expect("Can't open file.");

    let reader = BufReader::new(reader_opt);

    let mut text_vector = reader
        .lines()
        .filter_map(std::result::Result::ok) // None case validate
        .collect::<Vec<_>>();

    match text_vector.len() {
        0 => (),
        _ => assert_eq!(
            "2019-03-10".to_owned(),
            text_vector.pop().expect("vector pop failed.")
        ),
    }
}

#[test]
fn print_rust_and_rls_install() {
    let v = "nightly".to_owned();
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
    // Operation 2
    // Operation 3
}

#[test]
fn rust_install() {
    let v = "nightly".to_owned();
    let yes = false;
    println!(
        r"
    1. Rust installation:

        $ rustup install {}
    ",
        v
    );
    if yes {
        println!("$ rustup install {}", v);
        println!("OK");
    } else {
        println!("Command execution (y/n)");
        let buf = "YES".to_owned();
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            println!("$ rustup install {}", v);
            println!("OK");
        } else {
            println!("Cancel");
            exit(0);
        }
    }
}

#[test]
fn rls_install() {
    let v = "nightly".to_owned();
    let yes = false;

    println!(
        r"
    2. RLS installation:

        $ rustup component add rls --toolchain {}
        $ rustup component add rust-analysis --toolchain {}
        $ rustup component add rust-src --toolchain {}
",
        &v, &v, &v
    );
    if yes {
        // rls install
        println!("$ rustup component add rls --toolchain {}", &v);
        println!("OK");

        // rust-analysis install
        println!("$ rustup component add rust-analysis --toolchain {}", &v);
        println!("OK");

        // rust-src install
        println!("$ rustup component add rust-src --toolchain {}", &v);
        println!("OK");
    } else {
        println!("Command execution (y/n)");
        let buf = "YES".to_owned();
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            // rls install
            println!("$ rustup component add rls --toolchain {}", &v);
            println!("OK");

            // rust-analysis install
            println!("$ rustup component add rust-analysis --toolchain {}", &v);
            println!("OK");

            // rust-src install
            println!("$ rustup component add rust-src --toolchain {}", &v);
            println!("OK");
        } else {
            println!("Cancel");
            exit(0);
        }
    }
}

#[test]
fn rust_set_default() {
    let v = "nightly".to_owned();
    let yes = false;

    println!(
        r"
    3. Set default:

        $ rustup default {}
    ",
        v
    );

    if yes {
        println!("$ rustup default {}", &v);
        println!("OK");
    } else {
        println!("Command execution (y/n)");
        let buf = "YES".to_owned();
        if yes
            || buf.trim() == ""
            || buf.to_lowercase().trim() == "y"
            || buf.to_lowercase().trim() == "yes"
        {
            println!("$ rustup default {}", &v);
            println!("OK");
        } else {
            println!("Cancel");
            exit(0);
        }
    }
}
