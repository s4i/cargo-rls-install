use crate::global::PRESENT_DATE;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::result::Result;

// Trait: RustupCompenentsHistory
// Use URL or String
// HTML(table tag) scraping
pub trait RustupCompenentsHistory {
    fn rustup_components_history(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn scraping(document: Document);
}

impl RustupCompenentsHistory for &str {
    fn rustup_components_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = self as &str;
        let resp = reqwest::get(url)?;
        let document = Document::from_read(resp).expect("Data read failed");
        Self::scraping(document);
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
            .unwrap()
            .parent()
            .unwrap()
            .find(Name("td"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        let mut map = PRESENT_DATE.lock().unwrap();
        if build_status.iter().all(|x| x == "missing") {
            map.insert("seven days".to_owned(), "missing".to_owned());
        } else {
            for (dt, status) in date.iter().zip(build_status.iter()) {
                map.insert(dt.to_owned(), status.to_owned());
            }
        }
    }
}
