use super::{exit, fs, PathBuf, Regex};

#[test]
fn github_folder() {
    let mut dir_paths: Vec<PathBuf> = Vec::new();
    dir_paths.push(PathBuf::from(
        r"C:\Users\uname\.cargo\registry\src\github.com-19ccb2329d69ej323\",
    ));
    dir_paths.push(PathBuf::from(
        r"C:\Users\uname\.cargo\registry\src\Star_Platinum\",
    ));
    dir_paths.push(PathBuf::from(
        r"/mnt/c/Bash/home/uname/.cargo/registry/src/github.com-19ccb2329d69ej324/",
    ));
    dir_paths.push(PathBuf::from(
        r"/mnt/c/Bash/home/uname/.cargo/registry/src/The_World/",
    ));

    let mut result: Vec<String> = Vec::new();
    for dir in dir_paths {
        let mut dirs: Vec<String> = Vec::new();

        let paths = if dir.is_dir() {
            fs::read_dir(dir).unwrap()
        } else {
            println!("Not found github.com-* directory");
            exit(1);
        };

        for p in paths {
            dirs.push(p.unwrap().path().display().to_string());
        }

        for d in dirs {
            let re_get_github = Regex::new(r"github.com-\b.+").unwrap();
            if re_get_github.is_match(&d) {
                result.push(re_get_github.find(&d).unwrap().as_str().to_owned());
            }
        }
    }
    assert_eq!("github.com-19ccb2329d69ej323", result[0]);
    assert_eq!("github.com-19ccb2329d69ej324", result[1]);
}

#[test]
fn app_src_dir() {
    format!(
        "{}{}{}",
        env!("CARGO_PKG_NAME"),
        '-',
        env!("CARGO_PKG_VERSION")
    );
}
