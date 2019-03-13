use crate::global::PRESENT_DATE;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::io::{ErrorKind, ErrorKind::InvalidData};
use std::result;

// Trait: RustupCompenentsHistory
// Use URL or String
// HTML(table tag) scraping
pub trait RustupCompenentsHistory {
    fn rustup_components_history(&self) -> result::Result<(), ErrorKind>;
    fn read_html_document(url: &str) -> result::Result<Document, ErrorKind>;
    fn scraping(document: Document);
}

impl RustupCompenentsHistory for &str {
    fn rustup_components_history(&self) -> result::Result<(), ErrorKind> {
        let url = self as &str;
        match Self::read_html_document(url) {
            Ok(document) => Self::scraping(document),
            Err(_e) => (), // "InvalidData" not print
        }
        Ok(())
    }

    fn read_html_document(url: &str) -> result::Result<Document, ErrorKind> {
        if url.starts_with("http") {
            // Get HTML from internet
            let resp = reqwest::get(url).expect("Can't get response.");
            let document = Document::from_read(resp).expect("Data read failed.");
            Ok(document)
        } else if url.starts_with("<!DOCTYPE") {
            // Get HTML local file
            let document = Document::from(url);
            Ok(document)
        } else {
            Err(InvalidData)
        }
    }

    fn scraping(document: Document) {
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
            .expect("iter to string failed.")
            .parent()
            .unwrap()
            .find(Name("td"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        // println!("{:?}", build_status);

        if build_status.iter().all(|x| x == "missing") {
            println!("For RLS, unfortunate 8 days.");
            println!("It is impossible to find the latest version.");
            println!("The following version is written in the built-in text.");
        } else {
            for (i, s) in build_status.iter().enumerate() {
                if s == "present" {
                    let mut vec = PRESENT_DATE.lock().unwrap();
                    vec.push(date[i].clone());
                }
            }
        }
    }
}
