//! Optimized float parser for hexadecimal floats.
//!
//! This actually works for any case where we can exactly represent
//! any power of the mantissa radix using the exponent base. For example,
//! given a mantissa radix of `16`, and an exponent base of `8`,
//! `16^2` cannot be exactly represented in octal. In short:
//! ⌊log2(r) / log2(b)⌋ == ⌈log2(r) / log2(b)⌉.
//!
//! This gives us the following mantissa radix/exponent base combinations:
//!
//! - 4, 2
//! - 8, 2
//! - 16, 2
//! - 32, 2
//! - 16, 4

#![cfg(feature = "power-of-two")]
#![doc(hidden)]

// TODO(ahuszagh) Implement...
