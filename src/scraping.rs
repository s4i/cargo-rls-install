use crate::global::PRESENT_DATE;
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
        let resp = reqwest::get(url).expect("Can't get response");
        let document = Document::from_read(resp).expect("Data read failed");
        Self::scraping(document);
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
            .expect("iter to string failed or not found web page")
            .parent()
            .unwrap()
            .find(Name("td"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        let mut map = PRESENT_DATE.lock().unwrap();
        if build_status.iter().all(|x| x == "missing") {
            map.insert("eight days".to_owned(), "missing all".to_owned());
        } else {
            for (dt, status) in date.iter().zip(build_status.iter()) {
                map.insert(dt.to_owned(), status.to_owned());
            }
        }
    }
}
