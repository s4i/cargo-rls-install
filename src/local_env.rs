use dirs::home_dir;
use failure::err_msg;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::result::Result;

pub fn latest_txt_path(latest_file: &str) -> PathBuf {
    let mut own_path = cargo_home();
    own_path.push(format!(
        "{}{}{}",
        env!("CARGO_PKG_NAME"),
        '-',
        env!("CARGO_PKG_VERSION")
    ));
    own_path.push(latest_file);
    own_path
}

fn cargo_home() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(home_dir().unwrap());
    path.push(".cargo");
    path.push("registry");
    path.push("src");
    match github_folder(&path) {
        // $home/.cargo/registry/src/github.com-*/
        Ok(dir) => path.push(dir),
        Err(e) => {
            println!("{:?}", e);
            exit(0);
        }
    }
    // $home/.cargo/registry/src/github.com-*/cargo-rls-install-{version}/{build-in-text-name}
    path.to_path_buf()
}

fn github_folder(path: &PathBuf) -> Result<String, failure::Error> {
    if path.is_dir() {
        let mut dirs: Vec<String> = Vec::new();
        let paths = fs::read_dir(path).unwrap();
        let re_get_github = Regex::new(r"github.com-\b.+").unwrap();

        for p in paths {
            dirs.push(p.unwrap().path().display().to_string());
        }

        for d in dirs {
            if re_get_github.is_match(&d) {
                return Ok(re_get_github.find(&d).unwrap().as_str().to_owned());
            }
        }
    }
    Err(err_msg("Not found github.com-* directory"))
}
