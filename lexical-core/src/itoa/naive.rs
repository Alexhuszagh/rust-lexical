//! Slow, simple lexical integer-to-string conversion routine.

use util::*;

// Naive implementation for radix-N numbers.
//
// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
//
// `value` must be non-negative and mutable.
perftools_inline!{
fn naive<T>(mut value: T, radix: T, buffer: &mut [u8])
    -> usize
    where T: UnsignedInteger
{
    // Decode all but last digit, 1 at a time.
    let mut index = buffer.len();
    //let mut iter = buffer.iter_mut().rev();
    while value >= radix {
        let r = (value % radix).as_usize();
        value /= radix;

        // This is always safe, since r must be [0, radix).
        index -= 1;
        unchecked_index_mut!(buffer[index] = digit_to_char(r));
    }

    // Decode last digit.
    let r = (value % radix).as_usize();
    // This is always safe, since r must be [0, radix).
    index -= 1;
    unchecked_index_mut!(buffer[index] = digit_to_char(r));

    index
}}

pub(crate) trait Naive {
    // Export integer to string.
    fn naive(self, radix: u32, buffer: &mut [u8]) -> usize;
}

// Implement naive for type.
macro_rules! naive_impl {
    ($($t:ty)*) => ($(
        impl Naive for $t {
            perftools_inline_always!{
            fn naive(self, radix: u32, buffer: &mut [u8]) -> usize {
                naive(self, radix as $t, buffer)
            }}
        }
    )*);
}

naive_impl! { u8 u16 u32 u64 usize }
#[cfg(has_i128)]
naive_impl! { u128 }
