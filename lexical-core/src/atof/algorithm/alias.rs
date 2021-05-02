//! Aliases and traits to simplify float-parsing.

use crate::float::*;
use crate::table::*;
use crate::traits::*;

use super::bignum::ToBigfloat;
use super::errors::FloatErrors;

// MAX CORRECT DIGITS
// ------------------

/// Calculate the maximum number of digits possible in the mantissa.
///
/// Returns the maximum number of digits plus one.
///
/// We can exactly represent a float in radix `b` from radix 2 if
/// `b` is divisible by 2. This function calculates the exact number of
/// digits required to exactly represent that float.
///
/// According to the "Handbook of Floating Point Arithmetic",
/// for IEEE754, with emin being the min exponent, p2 being the
/// precision, and b being the radix, the number of digits follows as:
///
/// `−emin + p2 + ⌊(emin + 1) log(2, b) − log(1 − 2^(−p2), b)⌋`
///
/// For f16, this follows as:
///     emin = -14
///     p2 = 11
///
/// For bfloat16 , this follows as:
///     emin = -126
///     p2 = 8
///
/// For f32, this follows as:
///     emin = -126
///     p2 = 24
///
/// For f64, this follows as:
///     emin = -1022
///     p2 = 53
///
/// For f128, this follows as:
///     emin = -16382
///     p2 = 113
///
/// In Python:
///     `-emin + p2 + math.floor((emin+ 1)*math.log(2, b)-math.log(1-2**(-p2), b))`
///
/// This was used to calculate the maximum number of digits for [2, 36].
///
/// The minimum, denormal exponent can be calculated as follows: given
/// the number of exponent bits `exp_bits`, and the number of bits
/// in the mantissa `mantissa_bits`, we have an exponent bias
/// `exp_bias` equal to `2^(exp_bits-1) - 1 + mantissa_bits`. We
/// therefore have a denormal exponent `denormal_exp` equal to
/// `1 - exp_bias` and the minimum, denormal float `min_float` is
/// therefore `2^denormal_exp`.
///
/// For f16, this follows as:
///     exp_bits = 5
///     mantissa_bits = 10
///     exp_bias = 25
///     denormal_exp = -24
///     min_float = 5.96 * 10^−8
///
/// For bfloat16, this follows as:
///     exp_bits = 8
///     mantissa_bits = 7
///     exp_bias = 134
///     denormal_exp = -133
///     min_float = 9.18 * 10^−41
///
/// For f32, this follows as:
///     exp_bits = 8
///     mantissa_bits = 23
///     exp_bias = 150
///     denormal_exp = -149
///     min_float = 1.40 * 10^−45
///
/// For f64, this follows as:
///     exp_bits = 11
///     mantissa_bits = 52
///     exp_bias = 1075
///     denormal_exp = -1074
///     min_float = 5.00 * 10^−324
///
/// For f128, this follows as:
///     exp_bits = 15
///     mantissa_bits = 112
///     exp_bias = 16495
///     denormal_exp = -16494
///     min_float = 6.48 * 10^−4966
///
/// These match statements can be generated with the following Python
/// code:
/// ```python
/// import math
///
/// def digits(emin, p2, b):
///     return -emin + p2 + math.floor((emin+ 1)*math.log(2, b)-math.log(1-2**(-p2), b))
///
/// def max_digits(mantissa_size):
///     radices = [6, 10, 12, 14, 18, 20, 22, 24, 26, 28, 30, 34, 36]
///     print('match radix {')
///     for radix in radices:
///         value = digits(emin, p2, radix)
///         print(f'    {radix} => Some({value + 2}),')
///     print('    // Powers of two should be unreachable.')
///     print('    // Odd numbers will have infinite digits.')
///     print('    _ => None,')
///     print('}')
/// ```
pub trait MaxCorrectDigits: Float {
    fn max_correct_digits(radix: u32) -> Option<usize>;
}

/// emin = -14
/// p2 = 11
#[cfg(feature = "f16")]
impl MaxCorrectDigits for f16 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_correct_digits(_: u32) -> Option<usize> {
        Some(23)
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_correct_digits(radix: u32) -> Option<usize> {
        match radix {
            6 => Some(21),
            10 => Some(23),
            12 => Some(23),
            14 => Some(23),
            18 => Some(23),
            20 => Some(23),
            22 => Some(24),
            24 => Some(24),
            26 => Some(24),
            28 => Some(24),
            30 => Some(24),
            34 => Some(24),
            36 => Some(24),
            // Powers of two should be unreachable.
            // Odd numbers will have infinite digits.
            _ => None,
        }
    }
}

/// emin = -126
/// p2 = 8
#[cfg(feature = "f16")]
impl MaxCorrectDigits for bf16 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_correct_digits(_: u32) -> Option<usize> {
        Some(98)
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_correct_digits(radix: u32) -> Option<usize> {
        match radix {
            6 => Some(87),
            10 => Some(98),
            12 => Some(101),
            14 => Some(103),
            18 => Some(106),
            20 => Some(107),
            22 => Some(107),
            24 => Some(108),
            26 => Some(109),
            28 => Some(109),
            30 => Some(110),
            34 => Some(111),
            36 => Some(111),
            // Powers of two should be unreachable.
            // Odd numbers will have infinite digits.
            _ => None,
        }
    }
}

/// emin = -126
/// p2 = 24
impl MaxCorrectDigits for f32 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_correct_digits(_: u32) -> Option<usize> {
        Some(114)
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_correct_digits(radix: u32) -> Option<usize> {
        match radix {
            6 => Some(103),
            10 => Some(114),
            12 => Some(117),
            14 => Some(119),
            18 => Some(122),
            20 => Some(123),
            22 => Some(123),
            24 => Some(124),
            26 => Some(125),
            28 => Some(125),
            30 => Some(126),
            34 => Some(127),
            36 => Some(127),
            // Powers of two should be unreachable.
            // Odd numbers will have infinite digits.
            _ => None,
        }
    }
}

/// emin = -1022
/// p2 = 53
impl MaxCorrectDigits for f64 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_correct_digits(_: u32) -> Option<usize> {
        Some(769)
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_correct_digits(radix: u32) -> Option<usize> {
        match radix {
            6 => Some(682),
            10 => Some(769),
            12 => Some(792),
            14 => Some(808),
            18 => Some(832),
            20 => Some(840),
            22 => Some(848),
            24 => Some(854),
            26 => Some(859),
            28 => Some(864),
            30 => Some(868),
            34 => Some(876),
            36 => Some(879),
            // Powers of two should be unreachable.
            // Odd numbers will have infinite digits.
            _ => None,
        }
    }
}

/// emin = -16382
/// p2 = 113
#[cfg(feature = "f128")]
impl MaxCorrectDigits for f128 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_correct_digits(_: u32) -> Option<usize> {
        Some(11565)
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_correct_digits(radix: u32) -> Option<usize> {
        match radix {
            6 => Some(10159),
            10 => Some(11565),
            12 => Some(11927),
            14 => Some(12194),
            18 => Some(12568),
            20 => Some(12706),
            22 => Some(12823),
            24 => Some(12924),
            26 => Some(13012),
            28 => Some(13089),
            30 => Some(13158),
            34 => Some(13277),
            36 => Some(13328),
            // Powers of two should be unreachable.
            // Odd numbers will have infinite digits.
            _ => None,
        }
    }
}

// MAX INCORRECT DIGITS
// --------------------

/// Calculate the maximum number of digits that should affect
/// the mantissa, or significand (with an incorrect parser).
///
/// This value is always is ceil((Self::MANTISSA_SIZE + 1) / log2(radix)).
/// Since we do not care about the 1-bit rounding in the ULP
/// (unit of least precision), we can truncate early and avoid
/// halfway conversion algorithms. The extra bit is due to the
/// hidden bit adding one more bit of precision to a float.
///
/// These match statements can be generated with the following Python
/// code:
/// ```python
/// import math
///
/// def max_digits(mantissa_size):
///     radices = [
///         3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
///         22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
///     ]
///     print('match radix {')
///     for radix in radices:
///         value = math.ceil((mantissa_size + 1) / math.log2(radix))
///         print(f'    {radix} => {value},')
///     print('    // Powers of two should be unreachable here.')
///     print('    _ => unreachable!(),')
///     print('}')
/// ```
pub trait MaxIncorrectDigits: Float {
    fn max_incorrect_digits(radix: u32) -> i32;
}

// `MANTISSA_SIZE == 10`
#[cfg(feature = "f16")]
impl MaxIncorrectDigits for f16 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_incorrect_digits(_: u32) -> i32 {
        4
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_incorrect_digits(radix: u32) -> i32 {
        match radix {
            3 => 7,
            5 => 5,
            6 => 5,
            7 => 4,
            9 => 4,
            10 => 4,
            11 => 4,
            12 => 4,
            13 => 3,
            14 => 3,
            15 => 3,
            17 => 3,
            18 => 3,
            19 => 3,
            20 => 3,
            21 => 3,
            22 => 3,
            23 => 3,
            24 => 3,
            25 => 3,
            26 => 3,
            27 => 3,
            28 => 3,
            29 => 3,
            30 => 3,
            31 => 3,
            33 => 3,
            34 => 3,
            35 => 3,
            36 => 3,
            // Powers of two should be unreachable here.
            _ => unreachable!(),
        }
    }
}

// `MANTISSA_SIZE == 7`
#[cfg(feature = "f16")]
impl MaxIncorrectDigits for bf16 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_incorrect_digits(_: u32) -> i32 {
        3
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_incorrect_digits(radix: u32) -> i32 {
        match radix {
            3 => 6,
            5 => 4,
            6 => 4,
            7 => 3,
            9 => 3,
            10 => 3,
            11 => 3,
            12 => 3,
            13 => 3,
            14 => 3,
            15 => 3,
            17 => 2,
            18 => 2,
            19 => 2,
            20 => 2,
            21 => 2,
            22 => 2,
            23 => 2,
            24 => 2,
            25 => 2,
            26 => 2,
            27 => 2,
            28 => 2,
            29 => 2,
            30 => 2,
            31 => 2,
            33 => 2,
            34 => 2,
            35 => 2,
            36 => 2,
            // Powers of two should be unreachable here.
            _ => unreachable!(),
        }
    }
}

// `MANTISSA_SIZE == 23`
impl MaxIncorrectDigits for f32 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_incorrect_digits(_: u32) -> i32 {
        8
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_incorrect_digits(radix: u32) -> i32 {
        match radix {
            3 => 16,
            5 => 11,
            6 => 10,
            7 => 9,
            9 => 8,
            10 => 8,
            11 => 7,
            12 => 7,
            13 => 7,
            14 => 7,
            15 => 7,
            17 => 6,
            18 => 6,
            19 => 6,
            20 => 6,
            21 => 6,
            22 => 6,
            23 => 6,
            24 => 6,
            25 => 6,
            26 => 6,
            27 => 6,
            28 => 5,
            29 => 5,
            30 => 5,
            31 => 5,
            33 => 5,
            34 => 5,
            35 => 5,
            36 => 5,
            // Powers of two should be unreachable here.
            _ => unreachable!(),
        }
    }
}

// `MANTISSA_SIZE == 52`
impl MaxIncorrectDigits for f64 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_incorrect_digits(_: u32) -> i32 {
        16
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_incorrect_digits(radix: u32) -> i32 {
        match radix {
            3 => 34,
            5 => 23,
            6 => 21,
            7 => 19,
            9 => 17,
            10 => 16,
            11 => 16,
            12 => 15,
            13 => 15,
            14 => 14,
            15 => 14,
            17 => 13,
            18 => 13,
            19 => 13,
            20 => 13,
            21 => 13,
            22 => 12,
            23 => 12,
            24 => 12,
            25 => 12,
            26 => 12,
            27 => 12,
            28 => 12,
            29 => 11,
            30 => 11,
            31 => 11,
            33 => 11,
            34 => 11,
            35 => 11,
            36 => 11,
            // Powers of two should be unreachable here.
            _ => unreachable!(),
        }
    }
}

// `MANTISSA_SIZE == 112`
#[cfg(feature = "f128")]
impl MaxIncorrectDigits for f128 {
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    fn max_incorrect_digits(_: u32) -> i32 {
        35
    }

    #[inline(always)]
    #[cfg(feature = "radix")]
    fn max_incorrect_digits(radix: u32) -> i32 {
        match radix {
            3 => 72,
            5 => 49,
            6 => 44,
            7 => 41,
            9 => 36,
            10 => 35,
            11 => 33,
            12 => 32,
            13 => 31,
            14 => 30,
            15 => 29,
            17 => 28,
            18 => 28,
            19 => 27,
            20 => 27,
            21 => 26,
            22 => 26,
            23 => 25,
            24 => 25,
            25 => 25,
            26 => 25,
            27 => 24,
            28 => 24,
            29 => 24,
            30 => 24,
            31 => 23,
            33 => 23,
            34 => 23,
            35 => 23,
            36 => 22,
            // Powers of two should be unreachable here.
            _ => unreachable!(),
        }
    }
}

// FLOAT TYPE
// ----------

/// Trait to simplify type signatures for atof.
pub trait FloatType:
    FloatRounding<u64> + FloatRounding<u128> + StablePower + MaxCorrectDigits + MaxIncorrectDigits
{
    type Mantissa: Mantissa;
    type ExtendedFloat: ExtendedFloatType<Self>;
}

#[cfg(feature = "f116")]
impl FloatType for f116 {
    type Mantissa = Self::Unsigned;
    type ExtendedFloat = ExtendedFloat<Self::Mantissa>;
}

#[cfg(feature = "f116")]
impl FloatType for bf116 {
    type Mantissa = Self::Unsigned;
    type ExtendedFloat = ExtendedFloat<Self::Mantissa>;
}

impl FloatType for f32 {
    type Mantissa = Self::Unsigned;
    type ExtendedFloat = ExtendedFloat<Self::Mantissa>;
}

impl FloatType for f64 {
    type Mantissa = Self::Unsigned;
    type ExtendedFloat = ExtendedFloat<Self::Mantissa>;
}

#[cfg(feature = "f128")]
impl FloatType for f128 {
    type Mantissa = Self::Unsigned;
    type ExtendedFloat = ExtendedFloat<Self::Mantissa>;
}

// MANTISSA
// --------

/// Trait for a useable mantissa.
pub(super) trait MantissaType: Mantissa + FloatErrors {}

impl MantissaType for u64 {
}

#[cfg(feature = "f128")]
impl MantissaType for u128 {
}

// EXTENDED FLOAT
// --------------

/// Trait for extended-float types.
pub trait ExtendedFloatType<F: FloatType>: ToBigfloat<F> + From<F> {
    // I really wish I had any other choice **other** than getters and setters,
    // but since we can't specify fields in traits, and we can't use properties...
    // C'est la vie.
    fn mant(&self) -> F::Mantissa;
    fn exp(&self) -> i32;
    fn set_mant(&mut self, mant: F::Mantissa);
    fn set_exp(&mut self, exp: i32);
}

impl ExtendedFloatType<f32> for ExtendedFloat<u32> {
    #[inline(always)]
    fn mant(&self) -> u32 {
        self.mant
    }

    #[inline(always)]
    fn exp(&self) -> i32 {
        self.exp
    }

    #[inline(always)]
    fn set_mant(&mut self, mant: u32) {
        self.mant = mant;
    }

    #[inline(always)]
    fn set_exp(&mut self, exp: i32) {
        self.exp = exp;
    }
}

impl ExtendedFloatType<f64> for ExtendedFloat<u64> {
    #[inline(always)]
    fn mant(&self) -> u64 {
        self.mant
    }

    #[inline(always)]
    fn exp(&self) -> i32 {
        self.exp
    }

    #[inline(always)]
    fn set_mant(&mut self, mant: u64) {
        self.mant = mant;
    }

    #[inline(always)]
    fn set_exp(&mut self, exp: i32) {
        self.exp = exp;
    }
}

// TESTS
// ------

#[cfg(all(test, feature = "radix"))]
mod tests {
    use super::*;

    const CORRECT_RADIX: [u32; 13] = [6, 10, 12, 14, 18, 20, 22, 24, 26, 28, 30, 34, 36];
    const INCORRECT_RADIX: [u32; 30] = [
        3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30, 31, 33, 34, 35, 36,
    ];

    fn max_digits(emin: f64, p2: f64, radix: f64) -> usize {
        let log2 = 2.0f64.log(radix);
        let logp2 = (1.0 - 2.0f64.powf(-p2)).log(radix);
        let floor = ((emin + 1.0) * log2 - logp2).floor();

        (-emin + p2 + floor) as usize
    }

    #[test]
    fn test_max_correct_digits() {
        for &radix in CORRECT_RADIX.iter() {
            // Test f32
            let emin: f64 = -126.0;
            let p2: f64 = 24.0;
            let value = max_digits(emin, p2, radix as f64);
            assert_eq!(f32::max_correct_digits(radix), Some(value + 2));

            // Test f64
            let emin: f64 = -1022.0;
            let p2: f64 = 53.0;
            let value = max_digits(emin, p2, radix as f64);
            assert_eq!(f64::max_correct_digits(radix), Some(value + 2));
        }

        // Test an impossible radix.
        assert_eq!(f32::max_correct_digits(2), None);
        assert_eq!(f64::max_correct_digits(2), None);
    }

    #[test]
    fn test_max_incorrect_digits() {
        for &radix in INCORRECT_RADIX.iter() {
            // Test f32
            let num = (f32::MANTISSA_SIZE + 1) as f64;
            let den = (radix as f64).log2();
            let value = (num / den).ceil() as i32;
            assert_eq!(f32::max_incorrect_digits(radix), value);

            // Test f64
            let num = (f64::MANTISSA_SIZE + 1) as f64;
            let value = (num / den).ceil() as i32;
            assert_eq!(f64::max_incorrect_digits(radix), value);
        }
    }

    #[test]
    #[should_panic]
    fn test_max_incorrect_digits_panic() {
        // Test an impossible value.
        f32::max_incorrect_digits(2);
    }
}
