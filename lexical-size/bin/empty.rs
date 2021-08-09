use std::io::BufRead;

pub fn main() {
    println!("{}", std::io::stdin().lock().lines().next().unwrap().unwrap().trim().len());
}
