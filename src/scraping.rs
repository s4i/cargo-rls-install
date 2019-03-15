#[cfg(test)]
#[path = "scraping_test.rs"]
mod scraping_test;

use crate::global::PRESENT_DATE;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::io::ErrorKind;
// use std::io::ErrorKind::InvalidData;
use std::result;

// Trait: RustupCompenentsHistory
// Use URL or String
// HTML(table tag) scraping
pub trait RustupCompenentsHistory {
    fn rustup_components_history(&self) -> result::Result<(), ErrorKind>;
    fn scraping(document: Document);
    // fn switch_url_or_text(url: &str) -> result::Result<Document, ErrorKind>;
}

impl RustupCompenentsHistory for &str {
    fn rustup_components_history(&self) -> result::Result<(), ErrorKind> {
        let url = self as &str;
        let resp = reqwest::get(url).expect("Can't get response.");
        let document = Document::from_read(resp).expect("Data read failed.");
        Self::scraping(document);
        // match Self::switch_url_or_text(url) {
        // Ok(document) => Self::scraping(document),
        // Err(_e) => (), // "InvalidData" not print
        // }
        Ok(())
    }

    fn scraping(document: Document) {
        let date = document
            .find(Attr("scope", "col"))
            .skip(1)
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        let build_status = document
            .find(Attr("scope", "row"))
            .find(|x| x.text() == "rls")
            .expect("iter to string failed or not found web page.")
            .parent()
            .unwrap()
            .find(Name("td"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

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

// fn switch_url_or_text(url_or_text: &str) -> result::Result<Document, ErrorKind> {
// if url_or_text.starts_with("http") {
// // Get HTML from internet
//     let resp = reqwest::get(url_or_text).expect("Can't get response.");
//     let document = Document::from_read(resp).expect("Data read failed.");
//     Ok(document)
// } else if url_or_text.starts_with("<!DOCTYPE") {
// // Get HTML local file
//         let document = Document::from(url_or_text);
//         Ok(document)
//     } else {
//         Err(InvalidData)
//     }
// }
