#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[cfg(test)]
lazy_static! {
    static ref PRESENT_DATE: Mutex<HashMap<String, String>> = {
        let v: HashMap<String, String> = HashMap::new();
        Mutex::new(v)
    };
}
