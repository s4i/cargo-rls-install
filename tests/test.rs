#[cfg(test)]
mod tests {
    extern crate regex;
    use regex::Regex;
    #[test]

    fn all_missing() {
        let mut build_status: Vec<String> = vec![];
        build_status.push("missing".to_string());
        build_status.push("missing".to_string());
        build_status.push("missing".to_string());
        assert_eq!(build_status.iter().all(|x| x == "missing"), true);
    }

    #[test]
    fn path_divide() {
        let path = r#"C:\Users\sai\.rustup\toolchains\stable-x86_64-pc-windows-msvc\n"#
            .trim_end_matches(r#"\n"#)
            .replace("\\", "/");
        #[allow(unused_variables)]
        let re_stable = Regex::new(r"\b+stable-").unwrap();
        #[allow(unused_variables)]
        let re_beta = Regex::new(r"\b+beta-").unwrap();
        let re_nightly = Regex::new(r"\b+nightly-").unwrap();

        if re_nightly.is_match(&path) {
            let no_head = re_nightly.replace(&path, "");
            let re_date = Regex::new(r"(\d{4}-\d{2}-\d{2})").unwrap();
            #[allow(unused_variables)]
            let old_build_date = re_date
                .find(&no_head)
                .expect("Regex2: Not found")
                .as_str()
                .to_string();
            let re_date_plus_hyphen = Regex::new(r"(\d{4}-\d{2}-\d{2}-)").unwrap();
            if re_date_plus_hyphen.is_match(&no_head) {
                #[allow(unused_variables)]
                let platform_name = re_date_plus_hyphen.replace(&no_head, "").to_string();
            }
        }
    }
}
