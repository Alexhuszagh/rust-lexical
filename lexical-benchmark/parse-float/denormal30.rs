#![feature(llvm_asm)]

use lexical_parse_float::FromLexical;

pub fn black_box<T>(mut dummy: T) -> T {
    // SAFETY: the inline assembly is a no-op.
    unsafe { llvm_asm!("" : : "r"(&mut dummy) : "memory" : "volatile") };

    dummy
}

const DENORMAL30: &str = "2.4703282292062327208828439643e-324";

pub fn main() {
    for _ in 0..150000 {
        black_box(f64::from_lexical(DENORMAL30.as_bytes()).unwrap());
    }
}
