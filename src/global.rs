use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref PRESENT_DATE: Mutex<BTreeMap<String, String>> = {
        let m: BTreeMap<String, String> = BTreeMap::new();
        Mutex::new(m)
    };
}
