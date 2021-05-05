extern crate lexical_core;

pub fn main() {
    let res = "1.2345".parse::<f64>();
    println!("res={:?}", res.unwrap());
}
