//! Pre-computed tables for writing decimal strings.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

// RADIX^2 TABLES
// --------------

// Conditionally compile the precompiled radix**2 tables.
// These tables take `2 * (value % (radix^2))`, and return
// two consecutive values corresponding to both digits.
//
// Total array storage:
//  Without radix: ~430 B:
//      200 u8
//      11 f32
//      23 f64
//  With radix: ~55 KB.
//      32210 u8
//      518 f32
//      2610 f64
// Provides ~5x performance enhancement.
//
// These arrays are cache-friendly, for each BASE[2-36] table,
// elements can be access sequentially 2-at-a-time, preventing as many
// cache misses inside inner loops. For example, accessing the two elements
// for a remainder of `3` for the radix^2 in radix 2 will give you `1` and `1`,
// at indexes 6 and 7.
pub const DIGIT_TO_BASE10_SQUARED: [u8; 200] = [
    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7',
    b'0', b'8', b'0', b'9', b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5',
    b'1', b'6', b'1', b'7', b'1', b'8', b'1', b'9', b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3',
    b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7', b'2', b'8', b'2', b'9', b'3', b'0', b'3', b'1',
    b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7', b'3', b'8', b'3', b'9',
    b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4', b'7',
    b'4', b'8', b'4', b'9', b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5',
    b'5', b'6', b'5', b'7', b'5', b'8', b'5', b'9', b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3',
    b'6', b'4', b'6', b'5', b'6', b'6', b'6', b'7', b'6', b'8', b'6', b'9', b'7', b'0', b'7', b'1',
    b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5', b'7', b'6', b'7', b'7', b'7', b'8', b'7', b'9',
    b'8', b'0', b'8', b'1', b'8', b'2', b'8', b'3', b'8', b'4', b'8', b'5', b'8', b'6', b'8', b'7',
    b'8', b'8', b'8', b'9', b'9', b'0', b'9', b'1', b'9', b'2', b'9', b'3', b'9', b'4', b'9', b'5',
    b'9', b'6', b'9', b'7', b'9', b'8', b'9', b'9',
];
