extern crate lexical_core;

pub fn main() {
    let _ = lexical_core::parse::<f64>("1.2345".as_bytes());
}
