use crate::global::PRESENT_DATE;
use chrono::NaiveDate;
use select::document::Document;
use select::predicate::{Attr, Name};

// Trait: RustupCompenentsHistory
// Use URL or String
// HTML(table tag) scraping
pub trait RustupCompenentsHistory {
    fn rustup_components_history(&self);
    fn scraping(document: Document);
}

impl RustupCompenentsHistory for &str {
    fn rustup_components_history(&self) {
        let url = self as &str;
        let resp = reqwest::get(url).expect("Can't get response.");
        let document = Document::from_read(resp).expect("Data read failed.");
        Self::scraping(document);
    }

    fn scraping(document: Document) {
        // Vec<chrono::NaiveDate>
        let date = document
            .find(Attr("scope", "col"))
            .skip(1)
            .map(|tag| {
                NaiveDate::parse_from_str(&tag.text(), "%Y-%m-%d").expect("date type parse error")
            })
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
            let mut map = PRESENT_DATE.lock().unwrap();
            for (dt, status) in date.iter().zip(build_status.iter()) {
                map.insert(*dt, status.to_owned());
            }
        }
    }
}
