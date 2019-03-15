#[cfg(test)]
#[path = "global_test.rs"]
mod global_test;

use std::sync::Mutex;

lazy_static! {
    pub static ref PRESENT_DATE: Mutex<Vec<String>> = {
        let v: Vec<String> = Vec::new();
        Mutex::new(v)
    };
}
