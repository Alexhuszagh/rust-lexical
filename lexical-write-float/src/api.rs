//! Implements the algorithm in terms of the lexical API.

#![doc(hidden)]

use crate::options::Options;
use crate::write::WriteFloat;
use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::{to_lexical, to_lexical_with_options};

/// Check if a buffer is sufficiently large.
fn check_buffer<const FORMAT: u128>(len: usize, options: &Options) -> bool {
    let format = NumberFormat::<{ FORMAT }> {};

    // At least 2 for the decimal point and sign.
    let mut count: usize = 2;

    // First need to calculate maximum number of digits from leading or
    // trailing zeros, IE, the exponent break.
    if !format.no_exponent_notation() {
        let min_exp = options.negative_exponent_break().map_or(-5, |x| x.get());
        let max_exp = options.positive_exponent_break().map_or(9, |x| x.get());
        let exp = min_exp.abs().max(max_exp) as usize;
        if cfg!(feature = "power-of-two") && exp < 13 {
            // 11 for the exponent digits in binary, 1 for the sign, 1 for the symbol
            count += 13;
        } else if exp < 5 {
            // 3 for the exponent digits in decimal, 1 for the sign, 1 for the symbol
            count += 5;
        } else {
            // More leading or trailing zeros than the exponent digits.
            count += exp;
        }
    } else if cfg!(feature = "power-of-two") {
        // Min is 2^-1075.
        count += 1075;
    } else {
        // Min is 10^-324.
        count += 324;
    }

    // Now add the number of significant digits.
    let radix = format.radix();
    let formatted_digits = if radix == 10 {
        // Really should be 18, but add some extra to be cautious.
        28
    } else {
        //  BINARY:
        //      53 significant mantissa bits for binary, add a few extra.
        //  RADIX:
        //      Our limit is `delta`. The maximum relative delta is 2.22e-16,
        //      around 1. If we have values below 1, our delta is smaller, but
        //      the max fraction is also a lot smaller. Above, and our fraction
        //      must be < 1.0, so our delta is less significant. Therefore,
        //      if our fraction is just less than 1, for a float near 2.0,
        //      we can do at **maximum** 33 digits (for base 3). Let's just
        //      assume it's a lot higher, and go with 64.
        64
    };
    let digits = if let Some(max_digits) = options.max_significant_digits() {
        formatted_digits.min(max_digits.get())
    } else {
        formatted_digits
    };
    let digits = if let Some(min_digits) = options.min_significant_digits() {
        digits.max(min_digits.get())
    } else {
        formatted_digits
    };
    count += digits;

    len > count
}

// API

const DEFAULT_OPTIONS: Options = Options::new();

// Implement ToLexical for numeric type.
macro_rules! float_to_lexical {
    ($($t:tt $(, #[$meta:meta])? ; )*) => ($(
        impl ToLexical for $t {
            $(#[$meta:meta])?
            unsafe fn to_lexical_unchecked<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                debug_assert!(check_buffer::<{ STANDARD }>(bytes.len(), &DEFAULT_OPTIONS));
                // SAFETY: safe if `check_buffer::<STANDARD>(bytes.len(), &options)`.
                unsafe {
                    let len = self.write_float::<{ STANDARD }>(bytes, &DEFAULT_OPTIONS);
                    &mut index_unchecked_mut!(bytes[..len])
                }
            }

            $(#[$meta:meta])?
            fn to_lexical<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                assert!(check_buffer::<{ STANDARD }>(bytes.len(), &DEFAULT_OPTIONS));
                // SAFETY: safe since `check_buffer::<STANDARD>(bytes.len(), &options)`.
                unsafe { self.to_lexical_unchecked(bytes) }
            }
        }

        impl ToLexicalWithOptions for $t {
            type Options = Options;

            $(#[$meta:meta])?
            unsafe fn to_lexical_with_options_unchecked<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                assert!(NumberFormat::<{ FORMAT }> {}.is_valid());
                debug_assert!(check_buffer::<{ FORMAT }>(bytes.len(), &options));
                // SAFETY: safe if `check_buffer::<FORMAT>(bytes.len(), &options)`.
                unsafe {
                    let len = self.write_float::<{ FORMAT }>(bytes, &options);
                    &mut index_unchecked_mut!(bytes[..len])
                }
            }

            $(#[$meta:meta])?
            fn to_lexical_with_options<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                assert!(NumberFormat::<{ FORMAT }> {}.is_valid());
                assert!(check_buffer::<{ FORMAT }>(bytes.len(), &options));
                // SAFETY: safe since `check_buffer::<FORMAT>(bytes.len(), &options)`.
                unsafe { self.to_lexical_with_options_unchecked::<FORMAT>(bytes, options) }
            }
        }
    )*)
}

to_lexical! {}
to_lexical_with_options! {}
float_to_lexical! {
    f32 ;
    f64 ;
}
