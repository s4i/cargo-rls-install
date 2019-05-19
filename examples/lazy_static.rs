mod global {
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        pub static ref G_VEC: Mutex<Vec<String>> = {
            let v: Vec<String> = vec![];
            Mutex::new(v)
        };
    }
}

fn main() {
    let mut vec = global::G_VEC.lock().unwrap();
    vec.push("abc".to_owned());
    vec.push("def".to_owned());
    vec.push("ghi".to_owned());
    for val in vec.iter() {
        println!("{}", val);
    }
}
