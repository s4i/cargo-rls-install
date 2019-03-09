// use dirs::home_dir;
use failure::err_msg;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::result::Result;

pub fn latest_txt_path(app_name: &str, latest_file: &str) -> PathBuf {
    let mut own_path = cargo_home();
    // println!("{}", name_hyphen_version(&own_path));
    own_path.push(app_name);
    own_path.push(latest_file);
    own_path
}

fn cargo_home() -> PathBuf {
    let mut path = PathBuf::new();
    // path.push(home_dir().expect("Not found home directory."));
    // path.push(".cargo"); // $home/.cargo
    path.push(env!("CARGO_HOME"));
    path.push("registry");
    path.push("src");
    if path.is_dir() {
        match github_folder(&path) {
            // $home/.cargo/registry/src/github.com-*/
            Ok(dir) => path.push(dir),
            Err(e) => {
                println!("{:?}", e);
                exit(0);
            }
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
                let item = re_get_github.find(&d).unwrap().as_str().to_owned();
                return Ok(item);
            }
        }
    }
    Err(err_msg("Not found github.com-* directory"))
}

// fn name_hyphen_version(own_path: &PathBuf) -> String {
//     use cargo_toml::Manifest;
//     let mut root = PathBuf::new();
//     root.push(own_path);
// root.push("Cargo.toml");
// println!("{:?}", root);
//     let m = Manifest::from_path(root).unwrap();
//     let package = m.package.as_ref().unwrap();
//     [
//         package.name.clone(),
//         "-".to_owned(),
//         package.version.clone(),
//     ]
//     .concat()
// }
