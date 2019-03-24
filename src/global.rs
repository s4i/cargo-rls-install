use chrono::NaiveDate;
use std::collections::BTreeMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref PRESENT_DATE: Mutex<BTreeMap<NaiveDate, String>> = {
        let m: BTreeMap<NaiveDate, String> = BTreeMap::new();
        Mutex::new(m)
    };
}
