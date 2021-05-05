extern crate lexical_core;

pub fn main() {
    let res = lexical_core::parse::<f64>("1.2345".as_bytes());
    println!("res={:?}", res.unwrap());
}
