fn main() {
    let mut build_status: Vec<String> = vec![];
    build_status.push("missing".to_string());
    build_status.push("missing".to_string());
    build_status.push("missing".to_string());
    assert_eq!(build_status.iter().all(|x| x == "missing"), true);
}
