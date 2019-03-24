use regex::Regex;
use std::process::Command;

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
