#![feature(llvm_asm)]

mod black_box;

use black_box::black_box;
#[cfg(feature = "lexical")]
use core::mem;
#[cfg(feature = "lexical")]
use lexical_write_integer::ToLexical;
#[cfg(not(feature = "lexical"))]
use std::io::Write;

pub fn main() {
    #[cfg(feature = "lexical")] {
        let buffer: mem::MaybeUninit<[u8; 128]> = mem::MaybeUninit::uninit();
        let mut buffer = unsafe { buffer.assume_init() };
        let _ = black_box(12u8.to_lexical(&mut buffer));
    }

    #[cfg(not(feature = "lexical"))] {
        let mut buffer = Vec::with_capacity(128);
        black_box(buffer.write_fmt(format_args!("{}", 12u8)).unwrap());
    }
}
