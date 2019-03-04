use dirs::home_dir;
use std::path::PathBuf;

pub fn cargo_home(file: &str) -> String {
    let mut path = PathBuf::new();
    path.push(home_dir().expect("Not found home directory."));
    path.push(".cargo");
    path.push(file);
    let path_str = path
        .to_str()
        .expect("Not found home directory.")
        .to_string();
    path_str
}
