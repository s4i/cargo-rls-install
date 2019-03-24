use regex::Regex;
use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        String::from_utf8(
            Command::new("cmd")
                .args(&["/C", "rustup show"])
                .output()
                .expect("failed to execute process")
                .stdout,
        )
        .unwrap()
    } else {
        String::from_utf8(
            Command::new("rustup")
                .arg("show")
                .output()
                .expect("rustup show failed")
                .stdout,
        )
        .unwrap()
    };
    println!("{:?}", output);

    let lines: Vec<&str> = output.split('\n').collect();

    let re_nightly = Regex::new(r"^nightly-\d{4}-\d{2}-\d{2}-").unwrap();
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    let mut installed_toolchain = vec![];

    for line in &lines {
        // End loop: Find "active toolchain" line
        if line.starts_with("active") {
            break;
        }

        if re_nightly.is_match(line) {
            installed_toolchain.push(re_date.find(line).unwrap().as_str().to_owned());
        }
    }

    println!("{:?}", installed_toolchain);
}
