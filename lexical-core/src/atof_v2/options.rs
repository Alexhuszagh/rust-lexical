use crate::util::*;

// Config options
// TODO(ahuszagh) Change to use ParseFloatOptions
// TODO(ahuszagh) This can all be determined at compile time.
//      Wayyyy better than storing a struct of it.
pub struct Options {
    // TODO(ahuszagh) Comment with what these values replace.
    // Either needs to have values or names in fast-float-rust
    pub digit_separator: u8,
    pub decimal_point: u8,
    pub exponent: u8,
    pub mantissa_radix: u8,
    pub exponent_base: u8,
    pub exponent_radix: u8,
    pub nan_string: &'static [u8],
    pub inf_string: &'static [u8],
    pub infinity_string: &'static [u8],

    // Bitflags, these will be removed or reworked.
    // TODO(ahuszagh) Shit... I might need the signs too....
    // TODO(ahuszagh) Start adding validators for this....
    pub required_integer_digits: bool,
    pub required_fraction_digits: bool,
    pub required_exponent_digits: bool,
    pub no_positive_mantissa_sign: bool,
    pub required_mantissa_sign: bool,
    pub no_exponent_notation: bool,
    pub no_positive_exponent_sign: bool,
    pub required_exponent_sign: bool,
    pub no_exponent_without_fraction: bool,
    pub no_special: bool,
    pub case_sensitive_special: bool,
    pub case_sensitive_exponent: bool,
    pub no_integer_leading_zeros: bool,
    pub no_float_leading_zeros: bool,
    pub required_exponent_notation: bool,
    pub integer_internal_digit_separator: bool,
    pub fraction_internal_digit_separator: bool,
    pub exponent_internal_digit_separator: bool,
    pub internal_digit_separator: bool,
    pub integer_leading_digit_separator: bool,
    pub fraction_leading_digit_separator: bool,
    pub exponent_leading_digit_separator: bool,
    pub leading_digit_separator: bool,
    pub integer_trailing_digit_separator: bool,
    pub fraction_trailing_digit_separator: bool,
    pub exponent_trailing_digit_separator: bool,
    pub trailing_digit_separator: bool,
    pub integer_consecutive_digit_separator: bool,
    pub fraction_consecutive_digit_separator: bool,
    pub exponent_consecutive_digit_separator: bool,
    pub consecutive_digit_separator: bool,
    // This is a temporary value for the flag for all digit separators.
    pub digit_digit_separator: bool,
    pub special_digit_separator: bool,

    // Mantissa radix.
    // Maximum digits for the base, and the max mantissa during parsing.
    pub max_digits_mantissa: usize, // 19   // TODO(ahuszagh) Size-dependent
    pub f32_min_digit_int: u64,         // MIN_19DIGIT_INT
    pub f64_min_digit_int: u64,         // MIN_19DIGIT_INT
    pub f32_max_mantissa_fast: <f32 as Float>::Mantissa, // MAX_MANTISSA_FAST_PATH
    pub f64_max_mantissa_fast: <f64 as Float>::Mantissa,

    // Exponent base.
    pub f32_fast_pow: &'static [f32],
    pub f64_fast_pow: &'static [f64],
    pub f32_int_pow: &'static [<f32 as Float>::Mantissa],
    pub f64_int_pow: &'static [<f64 as Float>::Mantissa],
    // The value added for estimating the binary exponent to the bitshift.
    pub log2_power: i32,
    pub log2_power_shift: i32,
    // Exp limits for fast-path cases.
    pub f32_exp_limit_min: i64,
    pub f64_exp_limit_min: i64,
    pub f32_exp_limit_max: i64,
    pub f64_exp_limit_max: i64,
    pub f32_mantissa_limit: i64,
    pub f64_mantissa_limit: i64,
    // SMALLEST_POWER_OF_TEN and LARGEST_POWER_OF_TEN in fast-float-rust.
    pub smallest_power: i32,
    pub largest_power: i32,
    pub bias: i32,
    // f32: -17 && 10   (27)
    // f64: -4 && 23    (27)
    //
    //      Round-to-even only happens for negative values of q
    //      when q ≥ −4 in the 64-bit case and when q ≥ −17 in
    //      the 32-bitcase
    //
    //      When q ≥ 0,we have that 5^q ≤ 2m+1. In the 64-bit case,we
    //      have 5^q ≤ 2m+1 ≤ 2^54 or q ≤ 23. In the 32-bit case,we have
    //      5^q ≤ 2m+1 ≤ 2^25 or q ≤ 10.
    //
    //      When q < 0, we have w ≥ (2m+1)×5^−q. We must have that w < 2^64
    //      so (2m+1)×5^−q < 2^64. We have that 2m+1 > 2^53 (64-bit case)
    //      or 2m+1 > 2^24 (32-bit case). Hence,we must have 2^53×5^−q < 2^64
    //      (64-bit) and 2^24×5^−q < 2^64 (32-bit). Hence we have 5^−q < 2^11
    //      or q ≥ −4 (64-bit case) and 5^−q < 2^40 or q ≥ −17 (32-bitcase).
    //
    //      Thus we have that we only need to round ties to even when
    //      we have that q ∈ [−4,23](in the 64-bit case) or q∈[−17,10]
    //      (in the 32-bit case). In both cases,the power of five(5^|q|)
    //      fits in a 64-bit word.
    pub f32_min_exponent_round_to_even: i32,
    pub f64_min_exponent_round_to_even: i32,
    pub f32_max_exponent_round_to_even: i32,
    pub f64_max_exponent_round_to_even: i32,
    // Min and max exp, for decimal, -342 and 308.
    pub f32_min_exp: i32,
    pub f64_min_exp: i32,
    pub f32_max_exp: i32,
    pub f64_max_exp: i32,
    // Generating powers (exponent > 0):
    //      5^q
    // Generating reciprocal powers (exponent < 0):
    //      2^(2b)/(5^−q) with b=64 + ceiling(log2(5−q))
    pub power_of_x_128: &'static [(u64, u64)],

    // 7450580596923828125×10−27 is the smallest exact 64-bit number.
    //  q ≤ 55 since 5^q < 2^128, or ⌊128 / log2(5)⌋
    //  q ≥ -27 5^−q < 2^64, or -⌊64 / log2(5)⌋
    pub min_safe_exponent: i64,
    pub max_safe_exponent: i64,

// TODO(ahuszagh) Need to figure out how to do these for any radix
//  <= 10 (maybe more, if we can.)
//#[inline]
//pub fn is_8digits_le(v: u64) -> bool {
//    let a = v.wrapping_add(0x4646_4646_4646_4646);
//    let b = v.wrapping_sub(0x3030_3030_3030_3030);
//    (a | b) & 0x8080_8080_8080_8080 == 0
//}
//  Lmao how... is that a thing?

//fastfloat_really_inline bool is_made_of_eight_digits_fast(uint64_t val)  noexcept  {
//  return (((val & 0xF0F0F0F0F0F0F0F0) |
//           (((val + 0x0606060606060606) & 0xF0F0F0F0F0F0F0F0) >> 4)) ==
//          0x3333333333333333);
//}
//
//// credit: https://johnnylee-sde.github.io/Fast-numeric-string-to-int/
//fastfloat_really_inline uint32_t parse_eight_digits_unrolled(const char *chars)  noexcept  {
//  uint64_t val;
//  ::memcpy(&val, chars, sizeof(uint64_t));
//  val = (val & 0x0F0F0F0F0F0F0F0F) * 2561 >> 8;
//  val = (val & 0x00FF00FF00FF00FF) * 6553601 >> 16;
//  return uint32_t((val & 0x0000FFFF0000FFFF) * 42949672960001 >> 32);
//}
//
//  So, what do we have:
//      We have the following ranges:
//          0 - 9 => 0x30 - 0x39
//              x & 0xF gives digit
//                  Cannot use 0x1F, since it will match
//              0b0110000 - 0b0111001
//          A - Z => 0x41 - 0x5A
//              (x & 0x1F) + 10 gives digit
//              0b1000001 - 0b1011010
//          a - z => 0x61 - 0x7A
//              (x & 0x1F) + 10 gives digit
//              0b1100001 - 0b1111010
//
//      Should be able to match on 0x40 and 0x1F?
//          (0x40 & x) is upper flag.
//          (0x1F & x) - 0x10 is lower flag.
//              let high = (0x40 & x) >> 2;
//              let lo = (0x1F & x);
//              (hi | lo) - 0x10

    //
    // TODO(ahuszagh) Need power
}
