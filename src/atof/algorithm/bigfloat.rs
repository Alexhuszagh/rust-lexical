//! Arbitrary-precision decimal to parse a floating-point number.
// We support a lot of routines that are useful to have for testing or
// future development, but may not be in strict use.
// For example, `mul_small` is never used (but `mul_small_assign` is),
// and so we would like to keep this dead code in the program but not
// in the release builds.
#![allow(dead_code)]

use smallvec;
use atoi;
use float::{ExtendedFloat80, FloatRounding};
use float::convert::into_float;
use float::rounding::*;
use lib::{iter, mem};
use util::*;
use super::exponent::*;
use super::math::*;

// MAX EXPONENT

/// Calculate the reasonable maximum exponent for a float for `i^n`.
///
/// This value is `x+8`, where `x` is the maximum binary exponent
/// for the float type. In practice, that exponent for a finite value
/// could never be > `x+7`, since (for base 36) `35 * 2^X > 2^-(x+1) + 2^-(x+2)`,
/// where `2^-(x+1) + 2^-(x+2)` is the halfway point between 0 and the
/// smallest denormal float. Since 35 contains 6 bits, that sets the
/// maximum, realistic exponent is `x+7`.
///
/// We allow 1 extra bit for potential rounding error.
pub trait FloatMaxExponent: Float + FloatRounding<u64> {
    /// Calculate the max exponent for base N.
    fn max_binary_exponent() -> i32;
}

// `x` is 149.
impl FloatMaxExponent for f32 {
    #[inline(always)]
    fn max_binary_exponent() -> i32 {
        149 + 8
    }
}

// `x` is 1074.
impl FloatMaxExponent for f64 {
    #[inline(always)]
    fn max_binary_exponent() -> i32 {
        1074 + 8
    }
}

/// Maximum valid number of padded bytes (based of FloatMaxExponent).
/// Worst case is 3, which is ceil(ceil(2.1 * 0x3F8 + 55) / 8)
/// This really needs to go down.
const MAX_BYTES: usize = 11065;

// PAD DIVISION

/// Calculate the number of bits to pad for `i**n`.
///
/// This function calculates the steepest slope using numerical simulations
/// on primes, calculating a reasonable upper bound on the slope
/// and intercept, allowing an accurate calculation on the number of padded
/// bits required for any division operation without intermediate rounding
/// error.
///
/// The intercepts required at N bits of precision were also calculated
/// using the `bigfloat` Python module. The change in the slope with precision
/// shown to be linear, and the slope <= 0.00676 for bases
/// 3, 5, 7, and 33. This shows merely overestimating the results by ~0.15 gives
/// us 20 extra bits of precision, and so we are accurate to >70+ bits of
/// precision.
///
/// The intercept was calculated by using the following code:
/// ```text
/// PRECISION = 100 # or some value.
/// def is_same_guard(x,b,exp,n):
///     v = (x << n) // b**exp
///     actual = v * 2**(-n)
///     expected = x / (b**exp)
///     return (actual, expected, (expected-actual)/expected)
///     # Bigfloat code
///     #with bigfloat.precision(PRECISION):
///     #    v = (x << n) // (b**exp)
///     #    actual = bigfloat.mul(v, bigfloat.pow(2, -n))
///     #    expected = bigfloat.div(x, (b**exp))
///     #    return (actual, expected, (expected-actual)/expected)
///
/// def find_guard(x, b, n):
///     d = 1
///     while is_same_guard(x, b, n, d)[2] != 0:
///         d += 1
///     return d
///
/// def find_equation(x, b):
///     x = np.array([find_guard(x, b, i) for i in range(1,150)])
///     slope = np.average(x[1:] - x[:-1])
///     x1 = x[0]
///     return (slope, np.ceil(x1 - slope))
///
/// def iterate_equations(b):
///     primes = [
///         2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
///         59, 61, 67, 71, 1009, 4999, 7919, 17077, 62297, 83009, 95747,
///         95773, 104711, 15485867, 32452843, 49979687, 67867967, 86028121,
///         104395301, 122949823, 141650939, 160481183, 179424673
///     ]
///     return [find_equation(x, b) for x in primes]
///
/// def find_max_equation(b):
///     eqs = iterate_equations(b)
///     max_slope = max(enumerate(eqs), key=lambda x: x[1][0])
///     assert(max_slope[0] < len(eqs)//2)
///     max_intercept = max(eqs, key=lambda x: x[1])
///     return (max_slope[1][0], max_intercept[1])
/// ```
///
/// This showed at maximum 55 bits of extra precision were required, IE,
/// that was the y-intercept.
#[inline]
fn padded_bits(i: u32, n: u32) -> u32 {
    debug_assert!(i >= 2 && i <= 36, "padded_bits() numerical base must be from 2-36.");

    // 53-bit mantissa, all values can be **exactly** represented.
    const U32_MAX: f64 = u32::max_value() as f64;

    // Get slope and intercept.
    let (m, b) = match i {
        // Implement powers of 2 multipliers as the prime.
        3 | 6 | 12 | 24 => (2.1, 55.0),     // (1.9459, 54.0)
        5 | 10 | 20     => (2.8, 54.0),     // (2.6689, 53.0)
        7 | 14 | 28     => (3.3, 54.0),     // (3.1622, 53.0)
        9 | 18 | 36     => (3.4, 53.0),     // (3.2162, 52.0)
        11 | 22         => (4.0, 54.0),     // (3.8108, 53.0)
        13 | 26         => (4.2, 56.0),     // (4.0608, 55.0)
        15 | 30         => (4.2, 55.0),     // (3.9527, 54.0)
        17 | 34         => (4.6, 56.0),     // (4.4324, 55.0)
        19              => (4.8, 54.0),     // (4.6081, 53.0)
        21              => (4.6, 52.0),     // (4.4122, 51.0)
        23              => (5.0, 53.0),     // (4.8851, 52.0)
        25              => (4.8, 55.0),     // (4.6824, 54.0)
        27              => (4.9, 55.0),     // (4.7973, 54.0)
        29              => (5.4, 56.0),     // (5.2095, 55.0)
        31              => (5.5, 54.0),     // (5.3108, 53.0)
        33              => (5.2, 54.0),     // (5.0811, 53.0)
        35              => (5.4, 56.0),     // (5.1689, 55.0)

        // Other bases (powers of 2)
        _               => unreachable!(),
    };

    // Get our bit count using the linear equation.
    let n = n as f64;
    let v = (n*m + b).ceil();

    // Cannot overflow, max value is 33908.44, which is representable
    // by a 16-bit integer.
    v as u32
}

// PARSE MANTISSA

/// Parse digits into a bigfloat.
/// Returns a pointer to the current buffer position.
///
/// Use small powers steps to extract the proper digit and minimize the number
/// of big integer operations. None of the strings should have leading zeros.
///
/// * `bigfloat`      - Mutable bigfloat to store results in.
/// * `integer`       - Integer component of float.
/// * `fraction`      - Fraction component of float (used to modify the exponent).
/// * `base`          - Radix for the number encoding.
/// * `small_powers`  - Pre-calculated small powers
#[inline]
unsafe fn parse_digits(bigfloat: &mut Bigfloat, state: &mut ParseState, base: u32, last: *const u8)
{
    // We need to consider step - 2, since we guarantee up to the largest
    // small power being <= u32::max_value(). Any large digit in the
    // first position might lead to a larger value, especially for higher bases.
    let small_powers = Bigfloat::small_powers(base);
    let step = small_powers.len() - 2;
    loop {
        // Cannot overflow, choosing the maximum number of digits to avoid
        // overflow.
        let mut value: u32 = 0;
        let f = state.curr;
        let l = last.min(f.add(step));
        atoi::unchecked(&mut value, state, base, l);

        // Find the number of digits parsed, multiply by the small power,
        // and add the calculated value.
        let digits = distance(f, state.curr);
        bigfloat.imul_small(*small_powers.get_unchecked(digits));
        if value != 0 {
            bigfloat.iadd_small(value);
        }

        // Break condition, either we've reached last or we've reached a
        // non-digit character
        if digits != step {
            break;
        }
    }
}

/// Parse the mantissa into a bigfloat.
///
/// Returns the number of digits parsed after the period.
///
/// Use small powers steps to extract the proper digit and minimize the number
/// of big integer operations. None of the strings should have leading zeros.
///
/// * `bigfloat`      - Mutable bigfloat to store results in.
/// * `integer`       - Integer component of float.
/// * `fraction`      - Fraction component of float (used to modify the exponent).
/// * `base`          - Radix for the number encoding.
/// * `small_powers`  - Pre-calculated small powers
#[inline]
unsafe fn parse_mantissa(bigfloat: &mut Bigfloat, state: &mut ParseState, base: u32, last: *const u8)
    -> usize
{
    // Trim the leading 0s.
    state.ltrim_char(last, b'0');

    // Parse the integer component.
    parse_digits(bigfloat, state, base, last);

    // Parse the fraction component if present.
    // We need to store the number of parsed digits after the dot.
    if state.curr != last && *state.curr == b'.' {
        state.increment();
        let first = state.curr;
        if bigfloat.data.is_empty() {
            // Can ignore the leading digits while the mantissa is 0.
            // Simplifies the computational expense of this.
            state.ltrim_char(last, b'0');
        }

        parse_digits(bigfloat, state, base, last);
        distance(first, state.curr)
    } else {
        0
    }
}

/// Parse the mantissa and exponent from a string.
unsafe fn parse_float(base: u32, first: *const u8, last: *const u8)
    -> (Bigfloat, i32, ParseState)
{
    let mut bigfloat = Bigfloat::new();
    let mut state = ParseState::new(first);

    let fraction_digits = parse_mantissa(&mut bigfloat, &mut state, base, last);
    let raw_exponent = parse_exponent(&mut state, base, last);
    let exponent = mantissa_exponent(raw_exponent, fraction_digits, 0);

    (bigfloat, exponent, state)
}

// BIGFLOAT

/// Large, arbitrary-precision float.
///
/// This float aims to solve the half-way problem. If we have a mantissa,
/// with the following representation:
///
/// Mantissa          | Trailing | Truncated
/// 101010010110101010|1000000000|0000000001
///
/// We are supposed to round this up, since the truncated bits are above
/// halfway, however, we have no way to determine this. Any lossy
/// multiplication can push the trailing bits up or below the halfway point,
/// leading to incorrect rounding and incorrect results.
///
/// This large float assumes normalized values: that is, the most-significant
/// 32-bit integer must be non-zero. All operations assume normality, and will
/// return normalized values. It also assumes a non-negative result
/// (the set `[0-+Infinity)`).
///
/// # General Rules
///
/// This code is meant to be used internally, and debug assertions confirm
/// this, however, it can be used safely as long as the following rules
/// are adhered to:
///     1. The value must be "normalized" before any functions are called,
///         that is, the most-significant integer must be non-zero
///         (self.data.back()) or the container empty.
///     2. Addition of a small-value (via `iadd` or `add`)
///         must only occur when the exponent is 0. This is because Bigfloat
///         cheats for performance by avoiding any operation for
///         multiplication by a power of 2,  and rather just increments
///         the exponent.
///     3. Only call `div_pow*` or `mul_pow*` once for performance reasons,
///         and never add after either.
///     4. It's ok to add after `mul_small`, however, it is not ok to add
///         after `div_small` or any other division operation.
///     4. In general, division requires  padding of the underlying buffer
///         to keep numerical precision, and chaining division operations
///         can vastly deteriorate performance. Both multiplication and
///         division change the exponent whenever multiplied by a power
///         of 2, and therefore cannot
///     4. Do not directly access any underlying data or any of the implied
///         methods directly. None of the private
///
/// # Internal Algorithms
///
/// We break the data down into a vector of machine scalars, to simplify
/// the algorithms involved internally and for decent performance.
///
/// **Addition**
/// Addition of a machine scalar to the data can be done by adding the scalar
/// to the least-significant scalar in the vector, and carrying as needed.
/// Addition of two Bigfloats can be done by adding all scalars in `y`
/// to `x`, extending `x` as needed, and adding the carry from the previous
/// operation.
///
/// **Multiplication**
/// Multiplication of a machine scalar to the data can be done by
/// using a native-type twice the size of the original scalar. For each
/// element in in the vector, multiply the two values in a wide type, and
/// afterwards, adding any carry from previous operations. Then, extract
/// the lower half bits as the result, and the upper half bits as the carry.
///
/// To multiply by a power `b**n`, we can precalculate all small powers of
/// `b` that fit into the native scalar type. We then multiply by the largest
/// small power until `n` is less than or equal to the exponent of the
/// largest small power. We then multiply by the small power of the remainder,
/// reducing the constant for multiplication dramatically (still requires
/// N multiplications)
///
/// **Division**
/// To avoid rounding-error due to truncation, we must first pad the vector
/// prior to division by the estimated number of bits in the resulting
/// power.
///
/// Afterwards, division by a machine scalar (`y`) can be done by first
/// subtracting (with unsigned integer overflow) the remainder of the
/// previous operation from each element in the vector, creating `x_rem`.
/// The result is then `x_rem / y`, and the remainder is `x_rem % y`.
///
/// **Division Padding**
/// Calculation of the number of bits required to pad for division
/// by a base of a certain power was done using the follow Python code:
/// ```text
/// tobin = lambda x: "{0:b}".format(x)
/// bitlen = lambda x: len(tobin(x))
/// bitlens = lambda x: [bitlen(x**i) for i in range(0, 350)]
/// def bitdiff(b):
///     x = np.array(bitlens(b))
///     return x[1:] - x[:-1]
/// ```
///
/// An over-estimating linear regression was fit to the data, to estimate
/// exactly the number of bits required. The sharpest slope was calculated,
/// and a sharp upper bound on the data was calculated accordingly.
/// The y-intercept was always 1.
///
/// # Note
///
/// We might be able to speed multiplication and division up by using
/// exponentiation by squaring and Toom Cook or Schönhage–Strassen algorithm,
/// with optimizations for low exponents using precalculated powers.
/// Currently, when `M` is the length of the vector, and `N` is the power,
/// and `C` is the exponent of the largest small power, we do `M * ((N/C)+1)`
/// multiplications and additions, and `(N/C)+1` subtractions, comprising
/// both the exponentiation and multiplication. This could likely be optimized,
/// but would dramatically increase library complexity, or depend on GMP.
/// Consider with caution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bigfloat {
    /// Raw data for the underlying buffer (exactly 32**2 for the largest float).
    /// Don't store more bytes for small floats, since the denormal floats
    /// have almost no bytes of precision.
    /// These numbers are stored in little-endian format, so index 0 is
    /// the least-significant item, and index 31 is the most-significant digit.
    /// On little-endian systems, allows us to use the raw buffer left-to-right
    /// as an extended integer
    data: smallvec::SmallVec<[u32; 32]>,
    /// Exponent in base32.
    exp: i32,
}

impl Bigfloat {
    // CREATION

    /// Create new Bigfloat.
    #[inline]
    pub fn new() -> Bigfloat {
        Self::min_value()
    }

    /// Create new Bigfloat from u32.
    #[inline]
    pub fn from_u32(x: u32) -> Bigfloat {
        let mut bigfloat = Bigfloat {
            data: smallvec![x],
            exp: 0,
        };
        bigfloat.normalize();
        bigfloat
    }

    /// Create new Bigfloat from u64.
    #[inline]
    pub fn from_u64(x: u64) -> Bigfloat {
        let (d1, d0) = Bigfloat::split_u64(x);
        let mut bigfloat = Bigfloat {
            data: smallvec![d1, d0],
            exp: 0,
        };
        bigfloat.normalize();
        bigfloat
    }

    /// Create new Bigfloat from u128.
    #[inline]
    pub fn from_u128(x: u128) -> Bigfloat {
        let (d3, d2, d1, d0) = Bigfloat::split_u128(x);
        let mut bigfloat = Bigfloat {
            data: smallvec![d3, d2, d1, d0],
            exp: 0,
        };
        bigfloat.normalize();
        bigfloat
    }

    /// Create new Bigfloat with the minimal value.
    #[inline]
    pub fn min_value() -> Bigfloat {
        Bigfloat {
            data: smallvec![],
            exp: 0,
        }
    }

    /// Create new Bigfloat with the maximal value on stack.
    #[inline]
    pub fn max_value() -> Bigfloat {
        Bigfloat {
            data: smallvec![u32::max_value(); 32],
            exp: i32::max_value(),
        }
    }

    // NORMALIZATION

    /// Get if the float is normalized.
    #[inline]
    pub fn is_normalized(&self) -> bool {
        unsafe {
            self.data.is_empty() || !self.data.back_unchecked().is_zero()
        }
    }

    /// Set the most-significant int to be non-zero.
    #[inline]
    pub fn normalize(&mut self) {
        unsafe {
            while !self.data.is_empty() && self.data.back_unchecked().is_zero() {
                self.data.pop();
            }
        }
    }

    // DIVISION

    /// Pad the buffer with zeros to the least-significant digits.
    fn pad_zeros(&mut self, n: usize) {
        // Pad buffer, and get number of n padded.
        let n = <Bigfloat as SharedOps>::pad_zeros(self, as_cast(n));
        let bits = n * u32::BITS;
        self.exp -= bits as i32;
    }

    /// Pad ints for division, based off the base and exponent.
    fn pad_division(&mut self, n: u32, base: u32) {
        // Calculate total number of bytes to pad.
        let bits = padded_bits(base, n);
        let div = bits / 32;
        let rem = bits % 32;
        let n = div + (rem != 0) as u32;
        self.pad_zeros(as_cast(n));
    }

    /// Initialize Bigfloat from bytes with custom base.
    ///
    /// WARNING: No exponent check occurs here, it is up to the caller of
    /// from_bytes to ensure the number of digits is <= MAX_DIGITS and
    /// MAX_DIGITS is a suitable value to avoid i32 overflow
    pub unsafe fn from_bytes<F: FloatMaxExponent>(base: u32, first: *const u8, last: *const u8)
        -> (Bigfloat, ParseState)
    {
        // Rust guarantees 8-bits to a byte.
        let (mut bigfloat, exponent, state) = parse_float(base, first, last);
        let max_binary_exponent = F::max_binary_exponent();
        let binary_exponent = bigfloat.binary_exponent(base, exponent);
        if exponent == 0 {
            // Do nothing
        } else if binary_exponent > max_binary_exponent {
            // Normalized exponent overflows practical max, or the
            // number of digits is too large to accurately parse.
            bigfloat.exp = i32::max_value();
        } else if binary_exponent < -max_binary_exponent {
            // Normalized exponent overflows practical min, or the
            // number of digits is too large to accurately parse.
            bigfloat.exp = i32::min_value();
        } else if exponent > 0 {
            // Get exact representation via multiplication.
            bigfloat.imul_power(exponent.as_u32(), base);
        } else {
            // Get exact representation via division.
            let exponent = (-exponent).as_u32();
            bigfloat.pad_division(exponent, base);
            bigfloat.idiv_power(exponent, base, true);
        }
        (bigfloat, state)
    }

    // TO FLOAT

    /// Estimate the binary exponent for the normalized value.
    ///
    /// Produces exact values for for exponents <= 1<<50, which is sufficient
    /// for all realistic values.
    ///
    /// This converts the `base^exponent` to base2, as if by
    /// `ceil(log(base^exponent, 2))`.
    #[inline]
    fn binary_exponent(&self, base: u32, exponent: i32) -> i32 {
        debug_assert!(self.exp == 0, "binary_exponent() can only be called when self.exp == 0");

        // Check the binary exponent won't overflow. These values are
        // comically large, IE, i32::max_value() / log(36, 2).
        // Avoid using i32::min_value(), we don't want weird issues overflowing
        // on negation.
        if exponent > 0x18c23246 {
            i32::max_value()
        } else if exponent < -0x18c23246 {
            -i32::max_value()
        } else {
            // Cannot overflow.
            // Convert the basen exponent to binary. This will give an exact
            // value of the binary exponent. Need to roundaway from zero.
            let binary_exp = binary_exponent(base, exponent);

            // Get the data contribution to the binary exponent
            let storage_exp = unwrap_or_max(
                self.data.len().as_i32().checked_mul(32)
                .and_then(|v| Some(v - self.leading_zeros() as i32))
            );

            // Get the estimated binary exponent, if it overflows, who cares,
            // we'll assign infinity.
            unwrap_or_max(binary_exp.checked_add(storage_exp))
        }
    }

    /// Get the most 64-bits of the mantissa and if non-zero bits are truncated.
    #[inline]
    pub fn mantissa(&self) -> (u64, bool) {
        // We need to extract the following bit patterns. Say, for example,
        // we have the following bit pattern for the least-significant int.
        // Least-significant int: index 0.
        //      00000000000000001010101010101010
        //
        // We need to set the result bits to be:
        //      1010101010101010XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
        //
        // To do so, we calculate the bitshift required to put the value
        // so the most-significant bit is 1 in the int using `leading_zeros()`
        // (`u32_shift`), and then add a the number of bits required to
        // shift it to into the u64 representation (`u64_shift`). We then
        // widen the value, and shift left `u32_shift + u64_shift`.
        //
        // We then need to extract all the bits of the next int, and shift
        // them into the start of the `X`s in the result.
        //
        // Next int: index 1:
        //      10010010010010010010010010010010
        //
        // To do so, we just shift the int `u32_shift` to the left.
        //      101010101010101010010010010010010010010010010010XXXXXXXXXXXXXXXX
        //
        // Finally, we need to extract the upper `32-u32_shift` bits from
        // the value, right-shifted to the least-significant position.
        //
        // Next int: index 2:
        //      11011011011011011011011011011011
        //
        // To do so, we just shift the int `u64_shift - u32_shift` to the right.
        //      1010101010101010100100100100100100100100100100101101101101101101
        let u32_shift = self.leading_zeros();
        let u64_shift = u32::BITS as u32;

        let len = self.data.len();
        let rget = | index: usize | unsafe { *self.data.get_unchecked(len - index - 1) };

        // Ensure the resulting fraction is properly normalized.
        // We want consistency.
        match len {
            // No bytes, return a literal o-mantissa.
            // Can never have truncated bits.
            0 => (0, false),
            // One int, can only add 1-32-bits.
            // Can never have truncated bits.
            1 => {
                let v = rget(0).as_u64() << (u32_shift + u64_shift);
                (v, false)
            },
            // Two ints, can only add 33-64-bits.
            // Can never have truncated bits.
            2 => {
                let v0 = rget(0).as_u64() << (u32_shift + u64_shift);
                let v1 = rget(1).as_u64() << u32_shift;
                let v = v0 | v1;
                (v, false)
            },
            // Three ints, can always add all 64+ bits.
            // Can have truncated bits.
            _ => {
                // Get our value.
                let v0 = rget(0).as_u64() << (u32_shift + u64_shift);
                let v1 = rget(1).as_u64() << u32_shift;
                // Get the upper `(u64_shift-u32_shift)` bits, right-u32_shift
                // to zero out lower `u32_shift` bits.
                let v2 = rget(2).as_u64() >> (u64_shift - u32_shift);
                let v = v0 | v1 | v2;

                // Check if all the truncated elements are 0.
                if (rget(2) << u32_shift) != 0 {
                    (v, true)
                } else {
                    let mut iter = self.data.iter().rev().skip(3);
                    let nonzero = iter.any(|&x| x != 0);
                    (v, nonzero)
                }
            },
        }
    }

    /// Calculate the real exponent, binary for the float.
    /// Same as `self.exp + (u32::BITS * self.data.len()) - 64 - leading_zeros()`.
    /// Need to subtract an extra `64 + leading_zeros()` since that's the effective
    /// bitshift we're adding to the mantissa.
    #[inline]
    pub fn exponent(&self) -> i32 {
        const U64_BYTES: i32 = mem::size_of::<u64>() as i32;
        const U64_BITS: i32 = 8 * U64_BYTES;

        // Don't subtract U64_BITS immediately, for small integers, can cause underflow.
        let bitshift = self.leading_zeros() as usize;
        let bits = (u32::BITS * self.data.len()) - bitshift;
        let bits: i32 = unwrap_or_max(try_cast(bits));
        let shift = bits - U64_BITS;

        // Can overflow or underflow, just return maximum value in that case.
        match self.exp.checked_add(shift) {
            Some(v) => v,
            None    => if shift < 0 { i32::min_value() } else { i32::max_value() },
        }
    }

    /// Export native float from bigfloat.
    /// Use the rounding machinery for the extended-precision float, since
    /// we have total accuracy here, with a different callback that
    /// forces us to be above when we were originally halfway and bit
    /// truncation for the representation occurred, allowing accurate rounding
    /// for all float representations.
    #[inline]
    pub fn as_float<F>(&self)
        -> F
        where F: FloatRounding<u64>
    {
        // Get our initial values and create our floating point
        let (mant, is_truncated) = self.mantissa();
        let exp = self.exponent();
        let mut fp = ExtendedFloat80 { mant: mant, exp: exp };

        // Create our wrapper for round_nearest_tie_even.
        // If there are truncated bits, and we are exactly halfway,
        // then we need to set above to true and halfway to false.
        let rounding = move | f: &mut ExtendedFloat80, shift: i32 | {
            let (mut is_above, mut is_halfway) = round_nearest(f, shift);
            if is_halfway && is_truncated {
                is_above = true;
                is_halfway = false;
            }
            tie_even(f, is_above, is_halfway);
        };

        // Export to float. We can ignore normalization, since the value
        // is already normalized.
        round_to_float::<F, u64, _>(&mut fp, rounding);
        avoid_overflow::<F, u64>(&mut fp);
        into_float(fp)
    }

    /// Export 32-bit native float from bigfloat.
    #[inline]
    pub fn as_f32(&self) -> f32
        where f32: FloatRounding<u64>
    {
        self.as_float()
    }

    /// Export 64-bit native float from bigfloat.
    #[inline]
    pub fn as_f64(&self) -> f64
        where f64: FloatRounding<u64>
    {
        self.as_float()
    }
}

impl SharedOps for Bigfloat {
    type StorageType = smallvec::SmallVec<[u32; 32]>;

    #[inline]
    fn data<'a>(&'a self) -> &'a Self::StorageType {
        &self.data
    }

    #[inline]
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
        &mut self.data
    }

    // We want to conditionally pad zeros, since we only use it for the
    // pad division operation.
    fn pad_zeros(&mut self, n: usize) -> usize {
        // Assume **no** overflow for the usize, since this would lead to
        // other memory errors. Add `bytes` 0s to the left of the current
        // buffer, and decrease the exponent accordingly.

        // Remove the number of trailing zeros values for the padding.
        // If we don't need to pad the resulting buffer, return early.
        let n = n.checked_sub(self.trailing_zero_values() as usize).unwrap_or(0);
        if n.is_zero() || self.data().is_empty() {
            return n;
        }

        // Insert n `0`s at the start of the iterator.
        self.data_mut().insert_many(0, iter::repeat(0).take(n));

        n
    }
}

impl SmallOps for Bigfloat {
    #[inline]
    fn imul_pow2(&mut self, n: u32) {
        // Increment exponent to simulate actual multiplication.
        self.exp += n.as_i32();
    }

    #[inline]
    fn idiv_pow2(&mut self, n: u32, _: bool) {
        // Decrement exponent to simulate actual division.
        self.exp -= n.as_i32();
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use test::*;
    use super::*;

    // CREATION

    #[test]
    fn new_test() {
        let bigfloat = Bigfloat::new();
        assert_eq!(bigfloat, Bigfloat { data: smallvec![], exp: 0 });
    }

    #[test]
    fn from_u32_test() {
        let bigfloat = Bigfloat::from_u32(255);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255], exp: 0 });
    }

    #[test]
    fn from_u64_test() {
        assert_eq!(Bigfloat::from_u32(255), Bigfloat::from_u64(255));

        let bigfloat = Bigfloat::from_u64(1152921504606847231);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255, 1 << 28], exp: 0 });
    }

    #[test]
    fn from_u128_test() {
        assert_eq!(Bigfloat::from_u32(255), Bigfloat::from_u128(255));
        assert_eq!(Bigfloat::from_u64(255), Bigfloat::from_u128(255));
        assert_eq!(Bigfloat::from_u64(1152921504606847231), Bigfloat::from_u128(1152921504606847231));

        let bigfloat = Bigfloat::from_u128(1329227997022855913342108839786316031);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255, 1 << 28, 1 << 26, 1<< 24], exp: 0 });
    }

    // DIVISION

    #[test]
    fn padded_bits_test() {
        // Ensure it works for all bases.
        for base in BASE_POWN.iter().cloned() {
            padded_bits(base, 1);
        }

        // Check compared to known values.
        assert_eq!(padded_bits(3, 10), 76);
        assert_eq!(padded_bits(6, 10), 76);
        assert_eq!(padded_bits(12, 10), 76);
        assert_eq!(padded_bits(24, 10), 76);
        assert_eq!(padded_bits(5, 10), 82);
        assert_eq!(padded_bits(10, 10), 82);
        assert_eq!(padded_bits(20, 10), 82);
        assert_eq!(padded_bits(7, 10), 87);
        assert_eq!(padded_bits(14, 10), 87);
        assert_eq!(padded_bits(28, 10), 87);
        assert_eq!(padded_bits(11, 10), 94);
        assert_eq!(padded_bits(22, 10), 94);
        assert_eq!(padded_bits(13, 10), 98);
        assert_eq!(padded_bits(26, 10), 98);
        assert_eq!(padded_bits(17, 10), 102);
        assert_eq!(padded_bits(34, 10), 102);
        assert_eq!(padded_bits(19, 10), 102);
        assert_eq!(padded_bits(23, 10), 103);
        assert_eq!(padded_bits(29, 10), 110);
        assert_eq!(padded_bits(31, 10), 109);
        assert_eq!(padded_bits(9, 10), 87);
        assert_eq!(padded_bits(18, 10), 87);
        assert_eq!(padded_bits(36, 10), 87);
        assert_eq!(padded_bits(15, 10), 97);
        assert_eq!(padded_bits(30, 10), 97);
        assert_eq!(padded_bits(21, 10), 98);
        assert_eq!(padded_bits(27, 10), 104);
        assert_eq!(padded_bits(33, 10), 106);
        assert_eq!(padded_bits(25, 10), 103);
        assert_eq!(padded_bits(35, 10), 110);
    }

    #[test]
    fn pad_zeros_test() {
        // Pad 0
        let mut x = Bigfloat { data: smallvec![0, 0, 0, 1], exp: 0 };
        x.pad_zeros(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exp: 0 });

        // Pad 1
        let mut x = Bigfloat { data: smallvec![0, 0, 1], exp: 0 };
        x.pad_zeros(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exp: -32 });

        // Pad 2
        let mut x = Bigfloat { data: smallvec![0, 1], exp: 0 };
        x.pad_zeros(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exp: -64 });

        // Pad 3
        let mut x = Bigfloat::from_u32(1);
        x.pad_zeros(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exp: -96 });

        let mut x = Bigfloat::from_u64(0x100000001);
        x.pad_zeros(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1, 1], exp: -96 });

        let mut x = Bigfloat { data: smallvec![1, 1, 1], exp: 0 };
        x.pad_zeros(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1, 1, 1], exp: -96 });

        // Pad 4
        let mut x = Bigfloat::from_u128(0x1000000010000000100000001);
        x.pad_zeros(4);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 0, 1, 1, 1, 1], exp: -128 });
    }

    #[test]
    fn pad_division_test() {
        // Pad 0
        let mut x = Bigfloat { data: smallvec![1], exp: 0 };
        x.pad_division(10, 3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exp: -96 });
    }

    #[test]
    fn idiv_small_test() {
        // 1-int.
        let mut x = Bigfloat::from_u32(5);
        x.pad_zeros(2);
        assert_eq!(x.idiv_small(7), 3);
        assert_eq!(x, Bigfloat { data: smallvec![0xDB6DB6DB, 0xB6DB6DB6], exp: -64 });

        // 2-ints.
        let mut x = Bigfloat::from_u64(0x4000000040000);
        x.pad_zeros(2);
        assert_eq!(x.idiv_small(5), 3);
        assert_eq!(x, Bigfloat { data: smallvec![0x99999999, 0x99999999, 0xCCCD9999, 0xCCCC], exp: -64 });

        // 1-int.
        let mut x = Bigfloat::from_u32(0x33333334);
        x.pad_zeros(2);
        assert_eq!(x.idiv_small(5), 0);
        assert_eq!(x, Bigfloat { data: smallvec![0x0, 0x0, 0xA3D70A4], exp: -64 });

        // 2-ints.
        let mut x = Bigfloat::from_u64(0x133333334);
        x.pad_zeros(2);
        assert_eq!(x.idiv_small(5), 1);
        assert_eq!(x, Bigfloat { data: smallvec![0x33333333, 0x33333333, 0x3D70A3D7], exp: -64 });

        // 2-ints.
        let mut x = Bigfloat::from_u64(0x3333333333333334);
        x.pad_zeros(2);
        assert_eq!(x.idiv_small(5), 4);
        assert_eq!(x, Bigfloat { data: smallvec![0xCCCCCCCC, 0xCCCCCCCC, 0xD70A3D70, 0xA3D70A3], exp: -64 });
    }

    // AS FLOAT

    #[test]
    fn binary_exponent_test() {
        // Empty
        let x = Bigfloat::new();
        assert_eq!(x.binary_exponent(10, 0), 0);

        // Denormal float, extreme halfway case
        let x = Bigfloat { data: smallvec![1727738441, 330069557, 3509095598, 686205316, 156923684, 750687444, 2688855918, 28211928, 1887482096, 3222998811, 913348873, 1652282845, 1600735541, 1664240266, 84454144, 1487769792, 1855966778, 2832488299, 507030148, 1410055467, 2513359584, 3453963205, 779237894, 3456088326, 3671009895, 3094451696, 1250165638, 2682979794, 357925323, 1713890438, 3271046672, 3485897285, 3934710962, 1813530592, 199705026, 976390839, 2805488572, 2194288220, 2094065006, 2592523639, 3798974617, 586957244, 1409218821, 3442050171, 3789534764, 1380190380, 2055222457, 3535299831, 429482276, 389342206, 133558576, 721875297, 3013586570, 540178306, 2389746866, 2313334501, 422440635, 1288499129, 864978311, 842263325, 3016323856, 2282442263, 1440906063, 3931458696, 3511314276, 1884879882, 946366824, 4260548261, 1073379659, 1732329252, 3828972211, 1915607049, 3665440937, 1844358779, 3735281178, 2646335050, 1457460927, 2940016422, 1051], exp: 0 };
        assert_eq!(x.binary_exponent(10, -1078), -1075);
    }

    #[test]
    fn mantissa_test() {
        // Empty
        let x = Bigfloat::new();
        assert_eq!(x.mantissa(), (0, false));

        // 1-int
        let x = Bigfloat::from_u32(1);
        assert_eq!(x.mantissa(), (1<<63, false));

        // 2-ints
        let x = Bigfloat::from_u64(0x1000000000000000);
        assert_eq!(x.mantissa(), (1<<63, false));

        // 3-ints
        let x = Bigfloat::from_u128(0x40000000000000000000000);
        assert_eq!(x.mantissa(), (1<<63, false));

        // 4-ints
        let x = Bigfloat::from_u128(0x1000000000000000000000000000000);
        assert_eq!(x.mantissa(), (1<<63, false));

        // 2-ints + halfway (round-down)
        let x = Bigfloat::from_u64(0x20000000000001);
        assert_eq!(x.mantissa(), (0x8000000000000400, false));

        // 2-ints + halfway (round-up)
        let x = Bigfloat::from_u64(0x20000000000003);
        assert_eq!(x.mantissa(), (0x8000000000000C00, false));

        // 2-ints + below halfway (not truncated)
        let x = Bigfloat::from_u64(0x80000000000003FF);
        assert_eq!(x.mantissa(), (0x80000000000003FF, false));

        // 2-ints + above halfway (not truncated)
        let x = Bigfloat::from_u64(0x8000000000000401);
        assert_eq!(x.mantissa(), (0x8000000000000401, false));

        // 3-ints + halfway (round-down) (not truncated)
        let x = Bigfloat::from_u128(0x100000000000008000);
        assert_eq!(x.mantissa(), (0x8000000000000400, false));

        // 3-ints + halfway (round-up) (not truncated)
        let x = Bigfloat::from_u128(0x100000000000018000);
        assert_eq!(x.mantissa(), (0x8000000000000C00, false));

        // 3-ints + below halfway (truncated)
        let x = Bigfloat::from_u128(0x100000000000007FFF);
        assert_eq!(x.mantissa(), (0x80000000000003FF, true));

        // 3-ints + above halfway (truncated)
        let x = Bigfloat::from_u128(0x100000000000008001);
        assert_eq!(x.mantissa(), (0x8000000000000400, true));

        // Custom (bug fix), 0x20000000000001 << 100
        let x = Bigfloat { data: smallvec![0, 0, 0, 16, 33554432], exp: 0 };
        assert_eq!(x.mantissa(), (0x8000000000000400, false));

        // Underflow
        // Adapted from failures in strtod.
        let x = Bigfloat { data: smallvec![635365730, 2971208776, 32671145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8], exp: -3670 };
        assert_eq!(x.mantissa(), (0x8000000000000000, true));
    }

    #[test]
    fn exponent_test() {
        // Empty
        let x = Bigfloat::new();
        assert_eq!(x.exponent(), -64);

        // 1-int
        let x = Bigfloat::from_u32(1);
        assert_eq!(x.exponent(), -63);

        // 2-ints
        let x = Bigfloat::from_u64(0x1000000000000000);
        assert_eq!(x.exponent(), -3);

        // 3-ints
        let x = Bigfloat::from_u128(0x40000000000000000000000);
        assert_eq!(x.exponent(), 27);

        // 4-ints
        let x = Bigfloat::from_u128(0x1000000000000000000000000000000);
        assert_eq!(x.exponent(), 57);

        // Multiply by a power-of-two
        let mut x = Bigfloat::from_u32(1);
        x.imul_pow2(1);
        assert_eq!(x.exponent(), -62);

        // Divide by a power-of-two
        let mut x = Bigfloat::from_u32(1);
        x.idiv_pow2(1, true);
        assert_eq!(x.exponent(), -64);

        // Custom (bug fix), 0x20000000000001 << 100
        let x = Bigfloat { data: smallvec![0, 0, 0, 16, 33554432], exp: 0 };
        assert_eq!(x.exponent(), 90);

        // Underflow
        // Adapted from failures in strtod.
        let x = Bigfloat { data: smallvec![635365730, 2971208776, 32671145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8], exp: -3670 };
        assert_eq!(x.exponent(), -1138);
    }

    #[test]
    fn as_float_test() {
        // Empty
        let x = Bigfloat::new();
        assert_eq!(x.as_f64(), 0.0);

        // 1-int
        let x = Bigfloat::from_u32(1);
        assert_eq!(x.as_f64(), 1 as f64);

        // 2-ints
        let x = Bigfloat::from_u64(0x1000000000000000);
        assert_eq!(x.as_f64(), 0x1000000000000000u64 as f64);

        // 3-ints
        let x = Bigfloat::from_u128(0x40000000000000000000000);
        assert_eq!(x.as_f64(), 0x40000000000000000000000u128 as f64);

        // 4-ints
        let x = Bigfloat::from_u128(0x1000000000000000000000000000000);
        assert_eq!(x.as_f64(), 0x1000000000000000000000000000000u128 as f64);

        // 2-ints + halfway (round-down)
        let x = Bigfloat::from_u64(0x20000000000001);
        assert_eq!(x.as_f64(), 0x20000000000000u64 as f64);

        // 2-ints + halfway (round-up)
        let x = Bigfloat::from_u64(0x20000000000003);
        assert_eq!(x.as_f64(), 0x20000000000004u64 as f64);

        // 2-ints + below halfway (not truncated)
        let x = Bigfloat::from_u64(0x80000000000003FF);
        assert_eq!(x.as_f64(), 0x8000000000000000u64 as f64);

        // 2-ints + above halfway (not truncated)
        let x = Bigfloat::from_u64(0x8000000000000401);
        assert_eq!(x.as_f64(), 0x8000000000000800u64 as f64);

        // 3-ints + halfway (round-down) (not truncated)
        let x = Bigfloat::from_u128(0x100000000000008000);
        assert_eq!(x.as_f64(), 0x100000000000000000u128 as f64);

        // 3-ints + halfway (round-up) (not truncated)
        let x = Bigfloat::from_u128(0x100000000000018000);
        assert_eq!(x.as_f64(), 0x100000000000020000u128 as f64);

        // 3-ints + below halfway (truncated)
        let x = Bigfloat::from_u128(0x100000000000007FFF);
        assert_eq!(x.as_f64(), 0x100000000000000000u128 as f64);

        // 3-ints + above halfway (truncated)
        let x = Bigfloat::from_u128(0x100000000000008001);
        assert_eq!(x.as_f64(), 0x100000000000010000u128 as f64);

        // Custom (bug fix), 0x20000000000001 << 100
        let x = Bigfloat { data: smallvec![0, 0, 0, 16, 33554432], exp: 0 };
        assert_eq!(x.as_f64(), 11417981541647679048466287755595961091061972992.0);

        // Underflow
        // Adapted from failures in strtod.
        let x = Bigfloat { data: smallvec![635365730, 2971208776, 32671145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8], exp: -3670 };
        assert_eq!(x.as_f64(), 5e-324);
    }

    // PARSING

    unsafe fn check_parse_mantissa(base: u32, s: &str, tup: (Bigfloat, usize, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let mut value = Bigfloat::new();
        let mut state = ParseState::new(first);
        let fraction_digits = parse_mantissa(&mut value, &mut state, base, last);
        assert_eq!(value, tup.0);
        assert_eq!(fraction_digits, tup.1);
        assert_eq!(distance(first, state.curr), tup.2);
    }

    #[test]
    fn parse_mantissa_test() {
        unsafe {
            check_parse_mantissa(10, "1.2345", (Bigfloat::from_u32(12345), 4, 6));
            check_parse_mantissa(10, "12.345", (Bigfloat::from_u32(12345), 3, 6));
            check_parse_mantissa(10, "1.234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890", (Bigfloat { data: smallvec![3460238034, 2899308950, 4017877323, 1199904321, 1198752190, 2818107006, 390189029, 1795052211, 2368297274, 4229382910, 577], exp: 0 }, 99, 101));
            check_parse_mantissa(10, "0", (Bigfloat::new(), 0, 1));
            check_parse_mantissa(10, "12", (Bigfloat::from_u32(12), 0, 2));

            // Check the bigfloat parser works with extremely difficult, close-to-halfway cases.
            // Round-down, halfway
            check_parse_mantissa(10, "11417981541647679048466287755595961091061972992", (Bigfloat { data: smallvec![0, 0, 0, 0, 33554432], exp: 0 }, 0, 47));
            check_parse_mantissa(10, "11417981541647680316116887983825362587765178368", (Bigfloat { data: smallvec![0, 0, 0, 16, 33554432], exp: 0 }, 0, 47));
            check_parse_mantissa(10, "11417981541647681583767488212054764084468383744", (Bigfloat { data: smallvec![0, 0, 0, 32, 33554432], exp: 0 }, 0, 47));

            // Round-up, halfway
            check_parse_mantissa(10, "11417981541647681583767488212054764084468383744", (Bigfloat { data: smallvec![0, 0, 0, 32, 33554432], exp: 0 }, 0, 47));
            check_parse_mantissa(10, "11417981541647682851418088440284165581171589120", (Bigfloat { data: smallvec![0, 0, 0, 48, 33554432], exp: 0 }, 0, 47));
            check_parse_mantissa(10, "11417981541647684119068688668513567077874794496", (Bigfloat { data: smallvec![0, 0, 0, 64, 33554432], exp: 0 }, 0, 47));

            // Round-up, above halfway
            check_parse_mantissa(10, "11417981541647680316116887983825362587765178369", (Bigfloat { data: smallvec![1, 0, 0, 16, 33554432], exp: 0 }, 0, 47));
        }
    }

    unsafe fn check_parse_float(base: u32, s: &str, tup: (Bigfloat, i32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (value, exp, state) = parse_float(base, first, last);
        assert_eq!(value, tup.0);
        assert_eq!(exp, tup.1);
        assert_eq!(distance(first, state.curr), tup.2);
    }

    #[test]
    fn parse_float_test() {
        unsafe {
            check_parse_float(10, "1.2345", (Bigfloat::from_u32(12345), -4, 6));
            check_parse_float(10, "12.345", (Bigfloat::from_u32(12345), -3, 6));
            check_parse_float(10, "1.234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890", (Bigfloat { data: smallvec![3460238034, 2899308950, 4017877323, 1199904321, 1198752190, 2818107006, 390189029, 1795052211, 2368297274, 4229382910, 577], exp: 0 }, -99, 101));
            check_parse_float(10, "0", (Bigfloat::new(), 0, 1));
            check_parse_float(10, "12", (Bigfloat::from_u32(12), 0, 2));
            check_parse_float(10, "1.234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890e308", (Bigfloat { data: smallvec![3460238034, 2899308950, 4017877323, 1199904321, 1198752190, 2818107006, 390189029, 1795052211, 2368297274, 4229382910, 577], exp: 0 }, 209, 105));

            // Check the bigfloat parser works with extremely difficult, close-to-halfway cases.
            // Round-down, halfway
            check_parse_float(10, "11417981541647679048466287755595961091061972992", (Bigfloat { data: smallvec![0, 0, 0, 0, 33554432], exp: 0 }, 0, 47));
            check_parse_float(10, "11417981541647680316116887983825362587765178368", (Bigfloat { data: smallvec![0, 0, 0, 16, 33554432], exp: 0 }, 0, 47));
            check_parse_float(10, "11417981541647681583767488212054764084468383744", (Bigfloat { data: smallvec![0, 0, 0, 32, 33554432], exp: 0 }, 0, 47));

            // Round-up, halfway
            check_parse_float(10, "11417981541647681583767488212054764084468383744", (Bigfloat { data: smallvec![0, 0, 0, 32, 33554432], exp: 0 }, 0, 47));
            check_parse_float(10, "11417981541647682851418088440284165581171589120", (Bigfloat { data: smallvec![0, 0, 0, 48, 33554432], exp: 0 }, 0, 47));
            check_parse_float(10, "11417981541647684119068688668513567077874794496", (Bigfloat { data: smallvec![0, 0, 0, 64, 33554432], exp: 0 }, 0, 47));

            // Round-up, above halfway
            check_parse_float(10, "11417981541647680316116887983825362587765178369", (Bigfloat { data: smallvec![1, 0, 0, 16, 33554432], exp: 0 }, 0, 47));

            // Underflow
            // Adapted from failures in strtod.
            check_parse_float(10, "2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001e-324", (Bigfloat { data: smallvec![1727738441, 330069557, 3509095598, 686205316, 156923684, 750687444, 2688855918, 28211928, 1887482096, 3222998811, 913348873, 1652282845, 1600735541, 1664240266, 84454144, 1487769792, 1855966778, 2832488299, 507030148, 1410055467, 2513359584, 3453963205, 779237894, 3456088326, 3671009895, 3094451696, 1250165638, 2682979794, 357925323, 1713890438, 3271046672, 3485897285, 3934710962, 1813530592, 199705026, 976390839, 2805488572, 2194288220, 2094065006, 2592523639, 3798974617, 586957244, 1409218821, 3442050171, 3789534764, 1380190380, 2055222457, 3535299831, 429482276, 389342206, 133558576, 721875297, 3013586570, 540178306, 2389746866, 2313334501, 422440635, 1288499129, 864978311, 842263325, 3016323856, 2282442263, 1440906063, 3931458696, 3511314276, 1884879882, 946366824, 4260548261, 1073379659, 1732329252, 3828972211, 1915607049, 3665440937, 1844358779, 3735281178, 2646335050, 1457460927, 2940016422, 1051], exp: 0 }, -1078, 761));

            // Rounding error
            // Adapted from test-float-parse failures.
            check_parse_float(10, "1009e-31", (Bigfloat { data: smallvec![1009], exp: 0 }, -31, 8));
        }
    }

    unsafe fn check_from_bytes<F: FloatMaxExponent>(base: u32, s: &str, tup: (Bigfloat, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, state) = Bigfloat::from_bytes::<F>(base, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, state.curr), tup.1);
    }

    #[test]
    fn from_bytes_test() {
        unsafe {
            // Underflow
            // Adapted from failures in strtod.
            check_from_bytes::<f64>(10, "2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001e-324", (Bigfloat { data: smallvec![642017486, 3921539298, 3824343719, 91359114, 1738187133, 1383153214, 3150573688, 2249385240, 2573401083, 3095825845, 3660217666, 1733774432, 4281766689, 4040834041, 3939311820, 1480925659, 635365729, 2971208776, 32671145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8], exp: -4182 }, 761));

            // Rounding error
            // Adapted from test-float-parse failures.
            check_from_bytes::<f64>(5, "0.00000000000000000000000000000000000000004243233340111410410443", (Bigfloat { data: smallvec![880090781, 186210280, 869146737, 1385950651, 4269719750, 7], exp: -256 }, 64));
            check_from_bytes::<f64>(10, "1009e-31", (Bigfloat { data: smallvec![1614477393, 692973567, 4282343523, 3], exp: -191 }, 8));
        }
    }
}
