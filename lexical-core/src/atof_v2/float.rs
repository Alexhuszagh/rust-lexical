use crate::util::*;
use super::options::*;


pub trait FloatType: Float + Default
{
    // TODO(ahuszagh) Document all of this...
    fn min_exponent_round_to_even(options: &Options) -> i32;
    fn max_exponent_round_to_even(options: &Options) -> i32;
    fn min_exp(options: &Options) -> i32;
    fn max_exp(options: &Options) -> i32;
    fn exp_limit_min(options: &Options) -> i64;
    fn exp_limit_max(options: &Options) -> i64;
    fn mantissa_limit(options: &Options) -> i64;
    fn max_mantissa_fast(options: &Options) -> Self::Mantissa;
    fn fast_pow(index: usize, options: &Options) -> Self;
    fn from_mantissa_bits(bits: Self::Mantissa) -> Self;
    fn int_pow(index: usize, options: &Options) -> Self::Mantissa;
    fn min_digit_int(options: &Options) -> Self::Mantissa;

    fn power(q: i32, options: &Options) -> i32 {
        let mul = options.log2_power + (1 << options.log2_power_shift);
        (q.wrapping_mul(mul) >> options.log2_power_shift) + 63
    }
}

impl FloatType for f32
{
    #[inline]
    fn min_exponent_round_to_even(options: &Options) -> i32 {
        options.f32_min_exponent_round_to_even
    }

    #[inline]
    fn max_exponent_round_to_even(options: &Options) -> i32 {
        options.f32_max_exponent_round_to_even
    }

    #[inline]
    fn min_exp(options: &Options) -> i32 {
        options.f32_min_exp
    }

    #[inline]
    fn max_exp(options: &Options) -> i32 {
        options.f32_max_exp
    }

    #[inline]
    fn exp_limit_min(options: &Options) -> i64 {
        options.f32_exp_limit_min
    }

    #[inline]
    fn exp_limit_max(options: &Options) -> i64 {
        options.f32_exp_limit_max
    }

    #[inline]
    fn mantissa_limit(options: &Options) -> i64 {
        options.f32_mantissa_limit
    }

    #[inline]
    fn max_mantissa_fast(options: &Options) -> u64 {
        options.f32_max_mantissa_fast
    }

    #[inline]
    fn fast_pow(index: usize, options: &Options) -> f32 {
        options.f32_fast_pow[index]
    }

    #[inline]
    fn from_mantissa_bits(bits: u64) -> f32 {
        f32::from_bits((bits & 0xFFFFFFFF) as u32)
    }

    #[inline]
    fn int_pow(index: usize, options: &Options) -> u64 {
        options.f32_int_pow[index]
    }

    #[inline]
    fn min_digit_int(options: &Options) -> u64 {
        options.f32_min_digit_int
    }

    // TODO(ahuszagh) Need int_pow
}

impl FloatType for f64
{
    #[inline]
    fn min_exponent_round_to_even(options: &Options) -> i32 {
        options.f64_min_exponent_round_to_even
    }

    #[inline]
    fn max_exponent_round_to_even(options: &Options) -> i32 {
        options.f64_max_exponent_round_to_even
    }

    #[inline]
    fn min_exp(options: &Options) -> i32 {
        options.f64_min_exp
    }

    #[inline]
    fn max_exp(options: &Options) -> i32 {
        options.f64_max_exp
    }

    #[inline]
    fn exp_limit_min(options: &Options) -> i64 {
        options.f64_exp_limit_min
    }

    #[inline]
    fn exp_limit_max(options: &Options) -> i64 {
        options.f64_exp_limit_max
    }

    #[inline]
    fn mantissa_limit(options: &Options) -> i64 {
        options.f64_mantissa_limit
    }

    #[inline]
    fn max_mantissa_fast(options: &Options) -> u64 {
        options.f64_max_mantissa_fast
    }

    #[inline]
    fn fast_pow(index: usize, options: &Options) -> f64 {
        options.f64_fast_pow[index]
    }

    #[inline]
    fn from_mantissa_bits(bits: u64) -> f64 {
        f64::from_bits(bits)
    }

    #[inline]
    fn int_pow(index: usize, options: &Options) -> u64 {
        options.f64_int_pow[index]
    }

    #[inline]
    fn min_digit_int(options: &Options) -> u64 {
        options.f64_min_digit_int
    }
}
