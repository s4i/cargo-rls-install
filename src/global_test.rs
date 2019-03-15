use super::Mutex;

#[cfg(test)]
lazy_static! {
    static ref PRESENT_DATE: Mutex<Vec<String>> = {
        let v: Vec<String> = Vec::new();
        Mutex::new(v)
    };
}
