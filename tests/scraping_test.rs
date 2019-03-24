use select::document::Document;
use select::predicate::{Attr, Name};

#[test]
fn scraping_html_text() {
    use std::fs::read;
    use std::path::PathBuf;
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("read_test");
    path.push("html");
    path.push("x86_64-pc-windows-msvc.html");

    let byte_to_string =
        String::from_utf8(read(path.as_path().to_str().unwrap()).unwrap()).unwrap();

    // if cfg!(target_os = "windows") {
    //     byte_to_string = byte_to_string.replace(r"\\", r"\");
    //     println!("{}", byte_to_string);
    // }

    let document = Document::from(byte_to_string.as_str());

    let date = document
        .find(Attr("scope", "col"))
        .skip(1)
        .map(|tag| tag.text())
        .collect::<Vec<_>>();

    // println!("{:?}", date);
    // let pkg_name = document.find(Attr("scope", "row"))
    //     .map(|tag| tag.text()).collect::<Vec<_>>();
    // println!("{:?}", pkg_name);
    // let build_status = document.find(Name("td"))
    //     .map(|tag| tag.text()).collect::<Vec<_>>();
    // println!("{:?}", build_status);

    let build_status = document
        .find(Attr("scope", "row"))
        // .filter(|x| x.text() == "rls")
        // .next()
        .find(|x| x.text() == "rls")
        // .map(|tag| tag.text())
        // .collect::<Vec<_>>(); // result -> "rls"
        .expect("iter to string failed or not found web page.")
        .parent()
        .unwrap()
        .find(Name("td"))
        .map(|tag| tag.text())
        .collect::<Vec<_>>();

    // println!("{:?}", build_status);

    if !build_status.iter().all(|x| x == "missing") {
        for (i, s) in build_status.iter().enumerate() {
            if s == "present" {
                let mut vec = vec![];
                vec.push(date[i].clone());
            }
        }
    }
}

#[test]
fn scraping_method_get() {
    let url = "https://rust-lang.github.io/rustup-components-history/";
    let resp = reqwest::get(url).expect("Can't get response.");
    let document = Document::from_read(resp).expect("Data read failed.");

    let date = document
        .find(Attr("scope", "col"))
        .skip(1)
        .map(|tag| tag.text())
        .collect::<Vec<_>>();

    println!("{:?}", date);
    // println!("{:?}", date);
    // let pkg_name = document.find(Attr("scope", "row"))
    //     .map(|tag| tag.text()).collect::<Vec<_>>();
    // println!("{:?}", pkg_name);
    // let build_status = document.find(Name("td"))
    //     .map(|tag| tag.text()).collect::<Vec<_>>();
    // println!("{:?}", build_status);

    let build_status = document
        .find(Attr("scope", "row"))
        // .filter(|x| x.text() == "rls")
        // .next()
        .find(|x| x.text() == "rls")
        // .map(|tag| tag.text())
        // .collect::<Vec<_>>(); // result -> "rls"
        .expect("iter to string failed or not found web page.")
        .parent()
        .unwrap()
        .find(Name("td"))
        .map(|tag| tag.text())
        .collect::<Vec<_>>();

    // println!("{:?}", build_status);

    if !build_status.iter().all(|x| x == "missing") {
        for (i, s) in build_status.iter().enumerate() {
            if s == "present" {
                let mut vec = vec![];
                vec.push(date[i].clone());
            }
        }
    }
}
