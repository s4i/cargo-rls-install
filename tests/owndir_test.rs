use regex::Regex;

#[test]
fn github_folder() {
    let mut dir_paths: Vec<&str> = Vec::new();
    dir_paths.push(r"C:\Users\uname\.cargo\registry\src\github.com-19ccb2329d69ej323");
    dir_paths.push(r"C:\Users\uname\.cargo\registry\src\Star_Platinum");
    dir_paths.push(r"/mnt/c/Bash/home/uname/.cargo/registry/src/github.com-19ccb2329d69ej324");
    dir_paths.push(r"/mnt/c/Bash/home/uname/.cargo/registry/src/The_World");

    let mut result: Vec<String> = Vec::new();

    for d in dir_paths {
        let re_get_github = Regex::new(r"github.com-\b.+").unwrap();
        if re_get_github.is_match(&d) {
            result.push(re_get_github.find(&d).unwrap().as_str().to_owned());
        }
    }

    assert_eq!("github.com-19ccb2329d69ej323", result[0]);
    assert_eq!("github.com-19ccb2329d69ej324", result[1]);
}
