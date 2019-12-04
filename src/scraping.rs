use select::document::Document;
use select::predicate::{Attr, Name};
use std::collections::BTreeMap;
use std::io::Read;
use std::result::Result;
use ureq;

// Trait: RustupCompenentsHistory
// Arg: URL
// HTML(table tag) scraping
pub trait RustupCompenentsHistory {
    fn rustup_components_history(&self) -> Result<Vec<BTreeMap<String, String>>, std::io::Error>;
    fn scraping(document: &Document) -> Vec<BTreeMap<String, String>>;
}

impl RustupCompenentsHistory for &str {
    fn rustup_components_history(&self) -> Result<Vec<BTreeMap<String, String>>, std::io::Error> {
        let url = self as &str;
        let resp = ureq::get(url).call();
        let mut buf = vec![];
        if resp.ok() {
            resp.into_reader().read_to_end(&mut buf)?;
        }

        // Get rls, clippy status row
        Ok(Self::scraping(&Document::from(
            String::from_utf8(buf).unwrap().as_str(),
        )))
    }

    fn scraping(document: &Document) -> Vec<BTreeMap<String, String>> {
        let mut rls_map = BTreeMap::new();
        let mut clippy_map = BTreeMap::new();

        let date = document
            .find(Attr("scope", "col"))
            .skip(1)
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        let clippy_build_status = document
            .find(Attr("scope", "row"))
            .find(|x| x.text() == "clippy")
            .unwrap()
            .parent()
            .unwrap()
            .find(Name("td"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        let rls_build_status = document
            .find(Attr("scope", "row"))
            .find(|x| x.text() == "rls")
            .unwrap()
            .parent()
            .unwrap()
            .find(Name("td"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

        for (date, clippy) in date.iter().zip(clippy_build_status.iter()) {
            clippy_map.insert(date.to_owned(), clippy.to_owned());
        }

        for (date, rls) in date.iter().zip(rls_build_status.iter()) {
            rls_map.insert(date.to_owned(), rls.to_owned());
        }

        vec![clippy_map, rls_map]
    }
}
