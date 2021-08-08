#![feature(llvm_asm)]

mod black_box;

use black_box::black_box;
#[cfg(feature = "lexical")]
use core::mem;
#[cfg(feature = "lexical")]
use lexical_util::format::STANDARD;
#[cfg(feature = "lexical")]
use lexical_write_float::{compact, Options};
#[cfg(not(feature = "lexical"))]
use std::io::Write;

pub fn main() {
    #[cfg(feature = "lexical")] {
        let buffer: mem::MaybeUninit<[u8; 128]> = mem::MaybeUninit::uninit();
        let mut buffer = unsafe { buffer.assume_init() };
        let options = Options::builder().build().unwrap();
        let count = unsafe { compact::write_float::<_, STANDARD>(1.2f64, &mut buffer, &options) };
        let _ = black_box(unsafe { buffer.get_unchecked(..count) });
    }

    #[cfg(not(feature = "lexical"))] {
        let mut buffer = Vec::with_capacity(128);
        black_box(buffer.write_fmt(format_args!("{}", 1.2f64)).unwrap());
    }
}
