#![cfg(all(feature = "format", feature = "parse"))]

use core::num;

use lexical_util::format::{NumberFormat, NumberFormatBuilder};
use lexical_util::iterator::AsBytes;
use static_assertions::const_assert;

fn skip_iter_eq<const FORMAT: u128>(input: &[u8], output: &[u8]) {
    // next is done in terms of peek, so we're safe here.
    let mut input = input.bytes::<{ FORMAT }>();
    let mut output = output.bytes::<{ FORMAT }>();
    assert!(input.integer_iter().eq(output.integer_iter()));
}

#[test]
fn test_skip_iter_i() {
    // Test iterators that skip single, internal-only digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b"_.45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4_");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4_.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"_455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b"_.455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"_45_");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"_45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"_45_");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"_45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5__.56");
}

#[test]
fn test_skip_iter_l() {
    // Test iterators that skip single, leading-only digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_leading_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4_");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4_.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"4_5_");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"4_5_.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"4_5_");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"4_5_.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5__.56");
}

#[test]
fn test_skip_iter_t() {
    // Test iterators that skip single, trailing-only digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_trailing_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"_45_5");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"_4_5");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"_4_5.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5__.56");
}

#[test]
fn test_skip_iter_il() {
    // Test iterators that skip single, internal or leading-only digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_leading_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4_");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4_.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5__.56");
}

#[test]
fn test_skip_iter_it() {
    // Test iterators that skip single, internal or trailing-only digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_trailing_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"_455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"_45.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5__.56");
}

#[test]
fn test_skip_iter_lt() {
    // Test iterators that skip single, leading or trailing-only digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_leading_digit_separator(true)
        .integer_trailing_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"4_5.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5__.56");
}

#[test]
fn test_skip_iter_ilt() {
    // Test iterators that skip single digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_leading_digit_separator(true)
        .integer_trailing_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5__.56");
}

#[test]
fn test_skip_iter_ic() {
    // Test iterators that skip multiple, internal digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b"_.45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b"__.45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4_");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4_.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"_455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__455");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b"_.455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b"__.455");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"45__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"45__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"_45_");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"_45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"_45_");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__45__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"_45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__45__.56");
}

#[test]
fn test_skip_iter_lc() {
    // Test iterators that skip multiple, leading digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_leading_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4_");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4_.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b".45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"4_5_");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"4_5_.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"4_5_");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"4__5__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"4_5_.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"4__5__.56");
}

#[test]
fn test_skip_iter_tc() {
    // Test iterators that skip multiple, trailing digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_trailing_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"_45_5");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b".45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"_4_5");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__4__5");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"_4_5.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__4__5.56");
}

#[test]
fn test_skip_iter_ilc() {
    // Test iterators that skip multiple, internal or leading digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_leading_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4_");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4__");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4_.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4__.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"455");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"45__");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"45__.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"45__");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"45__.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"45_");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"45__");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"45_.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"45__.56");
}

#[test]
fn test_skip_iter_itc() {
    // Test iterators that skip multiple, internal or trailing digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_trailing_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"_455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"__455");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"45.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"__45.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"_45");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"__45");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"_45.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"__45.56");
}

#[test]
fn test_skip_iter_ltc() {
    // Test iterators that skip multiple, leading or trailing digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_leading_digit_separator(true)
        .integer_trailing_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"45__5");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b".45__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"4__5.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"45");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"4_5");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"4__5");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"4_5.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"4__5.56");
}

#[test]
fn test_skip_iter_iltc() {
    // Test iterators that skip multiple digit separators.
    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_leading_digit_separator(true)
        .integer_trailing_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    skip_iter_eq::<{ FORMAT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ FORMAT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ FORMAT }>(b"1e", b"1e");
    skip_iter_eq::<{ FORMAT }>(b"1", b"1");
    skip_iter_eq::<{ FORMAT }>(b"_45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45", b"45");
    skip_iter_eq::<{ FORMAT }>(b"_.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"__.45", b".45");
    skip_iter_eq::<{ FORMAT }>(b"4_5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4_", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4__", b"4");
    skip_iter_eq::<{ FORMAT }>(b"4_.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"4__.", b"4.");
    skip_iter_eq::<{ FORMAT }>(b"_45_5", b"455");
    skip_iter_eq::<{ FORMAT }>(b"__45__5", b"455");
    skip_iter_eq::<{ FORMAT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"__.45__5", b".455");
    skip_iter_eq::<{ FORMAT }>(b"4_5_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4__5__", b"45");
    skip_iter_eq::<{ FORMAT }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ FORMAT }>(b"4__5__.5", b"45.5");
    skip_iter_eq::<{ FORMAT }>(b"_45_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__45__", b"45");
    skip_iter_eq::<{ FORMAT }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"__45__.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_", b"45");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__", b"45");
    skip_iter_eq::<{ FORMAT }>(b"_4_5_.56", b"45.56");
    skip_iter_eq::<{ FORMAT }>(b"__4__5__.56", b"45.56");
}
