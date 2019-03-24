use regex::Regex;

fn main() {
    let stable = r#"C:\Users\sai\.rustup\toolchains\stable-x86_64-pc-windows-msvc\n"#
        .trim_end_matches('\n')
        .replace("\\", "/");
    let beta = r#"C:\Users\sai\.rustup\toolchains\beta-x86_64-pc-windows-msvc\n"#
        .trim_end_matches('\n')
        .replace("\\", "/");
    let nightly = r#"C:\Users\sai\.rustup\toolchains\nightly-2019-03-10-x86_64-pc-windows-msvc\n"#
        .trim_end_matches('\n')
        .replace("\\", "/");
    let channel = [stable, beta, nightly];

    // let re_stable = Regex::new(r"\b+stable-").unwrap();
    // let re_beta = Regex::new(r"\b+beta-").unwrap();
    let re_nightly = Regex::new(r"\b+nightly-").unwrap();

    for ch in channel.iter() {
        if re_nightly.is_match(&ch) {
            let no_head = re_nightly.replace(&ch, "");

            let re_date = Regex::new(r"(\d{4}-\d{2}-\d{2})").unwrap();

            let old_build_date = re_date
                .find(&no_head)
                .expect("Regex2: Not found")
                .as_str()
                .to_string();

            let re_date_plus_hyphen = Regex::new(r"(\d{4}-\d{2}-\d{2}-)").unwrap();

            let platform_name = if re_date_plus_hyphen.is_match(&no_head) {
                re_date_plus_hyphen.replace(&no_head, "").to_string()
            } else {
                "".to_owned()
            };

            println!(
                "Build date: {}\nPlatform name: {}",
                old_build_date, platform_name
            );
        }
    }
}
