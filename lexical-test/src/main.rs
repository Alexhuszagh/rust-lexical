extern crate lexical;

pub fn main() {
    // Dummy main fn for the package.
    let _: f32 = lexical::parse(b"5.0").unwrap();
}
