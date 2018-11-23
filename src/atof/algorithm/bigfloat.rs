//! Arbitrary-precision decimal to parse a floating-point number.
// TODO(ahuszagh) Remove this arbitrary warning, we're
// in rapid development, so allow it for now.
#![allow(dead_code)]

// TODO(ahuszagh) Move all the bounds checks outside.
// Only use static checks inside of functions.

use smallvec;
use float::Mantissa;
use lib::{mem, slice};
use util::*;

// CONSTANTS

// TODO(ahuszagh) Need to enforce the MAX_EXPONENT during parsing.

/// Maximum valid exponent internally.
const MAX_EXPONENT: i32 = 0x1400;

/// Maximum valid number of padded bytes (based of MAX_EXPONENT).
const MAX_BYTES: usize = 1060;

/// Small powers (u32) for base3 operations.
const SMALL_POWERS_BASE3: [u32; 21] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147,  531441, 1594323, 4782969, 14348907, 43046721, 129140163, 387420489, 1162261467, 3486784401];

/// Small powers (u32) for base5 operations.
const SMALL_POWERS_BASE5: [u32; 14] = [1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625, 1220703125];

/// Small powers (u32) for base7 operations.
const SMALL_POWERS_BASE7: [u32; 12] = [1, 7, 49, 343, 2401, 16807, 117649, 823543, 5764801, 40353607, 282475249, 1977326743];

/// Small powers (u32) for base11 operations.
const SMALL_POWERS_BASE11: [u32; 10] = [1, 11, 121, 1331, 14641, 161051, 1771561, 19487171, 214358881, 2357947691];

/// Small powers (u32) for base13 operations.
const SMALL_POWERS_BASE13: [u32; 9] = [1, 13, 169, 2197, 28561, 371293, 4826809, 62748517, 815730721];

/// Small powers (u32) for base17 operations.
const SMALL_POWERS_BASE17: [u32; 8] = [1, 17, 289, 4913, 83521, 1419857, 24137569, 410338673];

/// Small powers (u32) for base19 operations.
const SMALL_POWERS_BASE19: [u32; 8] = [1, 19, 361, 6859, 130321, 2476099, 47045881, 893871739];

/// Small powers (u32) for base23 operations.
const SMALL_POWERS_BASE23: [u32; 8] = [1, 23, 529, 12167, 279841, 6436343, 148035889, 3404825447];

/// Small powers (u32) for base29 operations.
const SMALL_POWERS_BASE29: [u32; 7] = [1, 29, 841, 24389, 707281, 20511149, 594823321];

/// Small powers (u32) for base31 operations.
const SMALL_POWERS_BASE31: [u32; 7] = [1, 31, 961, 29791, 923521, 28629151, 887503681];

// HELPERS

/// Try cast value to i32.
#[inline]
fn try_cast_i32<T: Integer>(t: T) -> Option<i32> {
    try_cast(t)
}

// ADD

/// Add two small integers and return the resulting value and if overflow happens.
#[inline(always)]
fn add_small<T: Integer>(x: T, y: T)
    -> (T, bool)
{
    x.overflowing_add(y)
}

/// AddAssign two small integers and return if overflow happens.
#[inline(always)]
fn add_small_assign<T: Integer>(x: &mut T, y: T)
    -> bool
{
    let t = add_small(*x, y);
    *x = t.0;
    t.1
}

// MUL

/// Multiply two small integers (with carry) (and return the overflow contribution).
///
/// Returns the (low, high) components.
#[inline(always)]
fn mul_small<Wide, Narrow>(x: Narrow, y: Narrow, carry: Narrow)
    -> (Narrow, Narrow)
    where Narrow: Integer,
          Wide: Integer
{
    // Assert that wide is 2 times as wide as narrow.
    debug_assert!(mem::size_of::<Narrow>()*2 == mem::size_of::<Wide>());

    // Cannot overflow, as long as wide is 2x as wide. This is because
    // the following is always true:
    // `Wide::max_value() - (Narrow::max_value() * Narrow::max_value()) >= Narrow::max_value()`
    let bits = mem::size_of::<Narrow>() * 8;
    let z: Wide = as_cast::<Wide, _>(x) * as_cast::<Wide, _>(y) + as_cast::<Wide,_>(carry);
    (as_cast::<Narrow, _>(z), as_cast::<Narrow, _>(z >> bits))
}

/// Multiply two small integers (with carry) (and return if overflow happens).
#[inline(always)]
fn mul_small_assign<Wide, Narrow>(x: &mut Narrow, y: Narrow, carry: Narrow)
    -> Narrow
    where Narrow: Integer,
          Wide: Integer
{
    let t = mul_small::<Wide, Narrow>(*x, y, carry);
    *x = t.0;
    t.1
}

// DIVISION

/// Divide two small integers (with remainder) (and return the remainder contribution).
///
/// Returns the (value, remainder) components.
#[inline(always)]
fn div_small<Wide, Narrow>(x: Narrow, y: Narrow, rem: Narrow)
    -> (Narrow, Narrow)
    where Narrow: Integer,
          Wide: Integer
{
    // Assert that wide is 2 times as wide as narrow.
    debug_assert!(mem::size_of::<Narrow>()*2 == mem::size_of::<Wide>());

    // Cannot overflow, as long as wide is 2x as wide.
    let bits = mem::size_of::<Narrow>() * 8;
    let x = as_cast::<Wide, _>(x) | (as_cast::<Wide, _>(rem) << bits);
    let y = as_cast::<Wide, _>(y);
    (as_cast::<Narrow, _>(x / y), as_cast::<Narrow, _>(x % y))
}

/// DivAssign two small integers and return the remainder.
#[inline(always)]
fn div_small_assign<Wide, Narrow>(x: &mut Narrow, y: Narrow, rem: Narrow)
    -> Narrow
    where Narrow: Integer,
          Wide: Integer
{
    let t = div_small::<Wide, Narrow>(*x, y, rem);
    *x = t.0;
    t.1
}

// PAD DIVISION

/// Calculate the number of bits to pad for `i**n`.
///
/// This function calculates the steepest slope for a repeating
/// pattern inside the number of bits require to calculate `i**n` for
/// `n ∀ [0, 350)`, calculating a reasonable upper bound on the slope.
///
/// The intercept was calculated by using the following code:
/// ```text
/// def is_same_guard(x,b,exp,n):
///     v = (x << n) // b**exp
///     actual = v * 2**(-n)
///     expected = x / (b**exp)
///     return (actual, expected, (expected-actual)/expected)
///
/// def find_guard(x, b, n):
///     d = 1
///     while is_same_guard(x, b, n, d)[2] != 0:
///         d += 1
///     return d
///
/// def find_intercept(x, b):
///     x = np.array([find_guard(x, b, i) for i in range(1,150)])
///     slope = np.average(x[1:] - x[:-1])
///     x1 = x[0]
///     return np.ceil(x1 - slope)
/// ```
///
/// This showed at maximum 55 bits of extra precision were required, IE,
/// that was the y-intercept.
#[inline]
fn padded_bits(i: u32, n:i32) -> u32 {
    debug_assert!(i >= 2 && i <= 36, "padded_bits() numerical base must be from 2-36.");
    debug_assert!(n <= MAX_EXPONENT, "padded_bits() internal exponent overflow, n is {}.", n);

    // 53-bit mantissa, all values can be **exactly** represented.
    const U32_MAX: f64 = u32::max_value() as f64;

    // Get slope and intercept.
    let (m, b) = match i {
        // Implement powers of 2 multipliers as the prime.
        3 | 6 | 12 | 24 => (1.600, 55.0),  // [1, 2, 1, 2, 2]
        5 | 10 | 20     => (2.334, 55.0),  // [2, 2, 3]
        7 | 14 | 28     => (2.834, 55.0),   // [2, 3, 3, 3, 3, 3]
        11 | 22         => (3.500, 55.0),   // [3, 4]
        13 | 26         => (3.750, 55.0),   // [3, 4, 4, 4]
        17 | 34         => (4.091, 55.0),   // [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5]
        19              => (4.250, 55.0),   // [4, 4, 4, 5]
        23              => (4.667, 55.0),   // [4, 5, 5]
        29              => (4.875, 55.0),   // [4, 5, 5, 5, 5, 5, 5, 5]
        31              => (4.955, 55.0),   // [4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]

        // Compound bases (multiply m, keep intercept)
        9 | 18 | 36     => (2.560, 55.0),    // (3 * 3)
        15 | 30         => (3.734, 55.0),    // (3 * 5)
        21              => (4.534, 55.0),    // (3 * 7)
        27              => (4.096, 55.0),    // (3 * 9)
        33              => (5.600, 55.0),    // (3 * 11)
        25              => (5.445, 55.0),    // (5 * 5)
        35              => (6.612, 55.0),    // (5 * 7)

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

// MUL POW ASSIGN

/// Wrap mul_pow2 implementation using implied call.
macro_rules! wrap_mul_pow2_assign {
    ($name:ident, $impl:ident, $b:expr) => (
        /// MulAssign by a power of $b (not safe to chain calls).
        #[inline(always)]
        fn $name(&mut self, n: i32) {
            debug_assert!(n >= 0, stringify!(Bigfloat::$name() must multiply by a positive power, n is {}.), n);
            // Don't bounds check, we already check for exponent overflow.
            self.$impl(n);
        }
    );
}

/// Wrap mul_pown implementation using implied call.
macro_rules! wrap_mul_pown_assign {
    ($name:ident, $impl:ident, $b:expr) => (
        /// MulAssign by a power of $b (not safe to chain calls).
        #[inline(always)]
        fn $name(&mut self, n: i32) {
            debug_assert!(n >= 0, stringify!(Bigfloat::$name() must multiply by a positive power, n is {}.), n);
            if n > 0x1400 {
                // Comically large value, always infinity.
                self.exponent = i32::max_value();
            } else {
                self.$impl(n);
            }
        }
    );
}

// DIV POW ASSIGN

/// Wrap div_pow2 implementation using implied call.
macro_rules! wrap_div_pow2_assign {
    ($name:ident, $impl:ident, $b:expr) => (
        /// DivAssign by a power of $b (not safe to chain calls).
        #[inline(always)]
        fn $name(&mut self, n: i32) {
            debug_assert!(n >= 0, stringify!(Bigfloat::$name() must divide by a positive power, n is {}.), n);
            debug_assert!(n <= MAX_EXPONENT, stringify!(Bigfloat::$name() internal exponent overflow, n is {}.), n);
            // Don't pad or do bounds check, we already check for exponent underflow.
            self.$impl(n);
        }
    );
}

/// Wrap div_pown implementation using implied call.
macro_rules! wrap_div_pown_assign {
    ($name:ident, $impl:ident, $b:expr) => (
        /// DivAssign by a power of $b (not safe to chain calls).
        #[inline(always)]
        fn $name(&mut self, n: i32) {
            debug_assert!(n >= 0, stringify!(Bigfloat::$name() must divide by a positive power, n is {}.), n);
            debug_assert!(n <= MAX_EXPONENT, stringify!(Bigfloat::$name() internal exponent overflow, n is {}.), n);

            // Calculate the number of bytes required to pad the vector,
            // and pad the vector. usize may be 16-bit, so use try_cast.
            // Padded bits is representable as a u16, so it cannot overflow
            // for any usize.
            let bits = padded_bits($b, n);
            let div = bits / 32;
            let rem = bits % 32;
            let bytes = div + (rem != 0) as u32;
            self.pad_division(as_cast(bytes));

            // Call the implied method after padding.
            self.$impl(n);
        }
    );
}

// TODO(ahuszagh) Implement...

// FROM BYTES

/// Wrapper for basen from_bytes implementations.
// TODO(ahuszagh) Implement
macro_rules! from_bytes {
    ($name:ident) => (
        /// Initialize Bigfloat from bytes with base3.
        #[allow(unused)] // TODO(ahuszagh) Remove
        fn $name(first: *const u8, last: *const u8) -> (Bigfloat, *const u8) {
            let bigfloat = Bigfloat::new();
            unimplemented!()
        }
    );
}

// BIGFLOAT

/// Large, arbitrary-precision float.
///
/// This float aims to solve the half-way problem. If we have a mantissa,
/// with the following representation:
///
///     Mantissa          | Trailing | Truncated
///     101010010110101010|1000000000|0000000001
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Bigfloat {
    /// Raw data for the underlying buffer (exactly 32**2 for the largest float).
    /// Don't store more bytes for small floats, since the denormal floats
    /// have almost no bytes of precision.
    /// These numbers are stored in little-endian format, so index 0 is
    /// the least-significant item, and index 31 is the most-significant digit.
    /// On little-endian systems, allows us to use the raw buffer left-to-right
    /// as an extended integer
    data: smallvec::SmallVec<[u32; 32]>,
    /// Exponent in base32.
    exponent: i32,
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
        Bigfloat {
            data: smallvec![x],
            exponent: 0,
        }
    }

    /// Create new Bigfloat from u64.
    #[inline]
    pub fn from_u64(x: u64) -> Bigfloat {
        let hi = (x >> 32) as u32;
        let lo = (x & u64::LOMASK) as u32;
        Bigfloat {
            data: smallvec![lo, hi],
            exponent: 0,
        }
    }

    /// Create new Bigfloat from u128.
    #[inline]
    pub fn from_u128(x: u128) -> Bigfloat {
        let hi64 = (x >> 64) as u64;
        let lo64 = (x & u128::LOMASK) as u64;
        let d3 = (lo64 & u64::LOMASK) as u32;
        let d2 = (lo64 >> 32) as u32;
        let d1 = (hi64 & u64::LOMASK) as u32;
        let d0 = (hi64 >> 32) as u32;
        Bigfloat {
            data: smallvec![d3, d2, d1, d0],
            exponent: 0,
        }
    }

    /// Create new Bigfloat with the minimal value.
    #[inline]
    pub fn min_value() -> Bigfloat {
        Bigfloat {
            data: smallvec![],
            exponent: 0,
        }
    }

    /// Create new Bigfloat with the maximal value on stack.
    #[inline]
    pub fn max_value() -> Bigfloat {
        Bigfloat {
            data: smallvec![u32::max_value(); 32],
            exponent: i32::max_value(),
        }
    }

    // PROPERTIES

    /// Number of bits in the underlying storage.
    const BITS: usize = mem::size_of::<u32>() * 8;

    /// Get the number of leading zero values in Bigfloat.
    /// Assumes the value is normalized.
    #[inline]
    pub fn leading_zero_values(&self) -> u32 {
        debug_assert!(!self.is_empty(), "Bigfloat::leading_zero_values() data cannot be empty.");
        debug_assert!(!self.back().is_zero(), "Bigfloat::leading_zero_values() data is not normalized.");
        0
    }

    /// Get the number of trailing zero values in Bigfloat.
    /// Assumes the value is normalized.
    #[inline]
    pub fn trailing_zero_values(&self) -> u32 {
        debug_assert!(self.is_empty() || !self.back().is_zero(), "Bigfloat::trailing_zero_values() data is not normalized.");
        for (i, v) in self.iter().enumerate() {
            if !v.is_zero() {
                return i as u32;
            }
        }
        self.len() as u32
    }

    /// Get number of leading zero bits in Bigfloat.
    /// Assumes the value is normalized.
    #[inline]
    pub fn leading_zeros(&self) -> u32 {
        debug_assert!(!self.is_empty(), "Bigfloat::leading_zeros() data cannot be empty.");
        debug_assert!(!self.back().is_zero(), "Bigfloat::leading_zeros() data is not normalized.");
        self.back().leading_zeros()
    }

    /// Get number of trailing zero bits in Bigfloat.
    /// Assumes the value is normalized.
    #[inline]
    pub fn trailing_zeros(&self) -> u32 {
        debug_assert!(!self.is_empty(), "Bigfloat::leading_zeros() data cannot be empty.");
        debug_assert!(!self.back().is_zero(), "Bigfloat::trailing_zeros() data is not normalized.");

        // Get the index of the last non-zero value
        let index: usize = self.trailing_zero_values() as usize;
        let mut count = (index * Self::BITS) as u32;
        if index != self.len() {
            count += self.get(index).trailing_zeros();
        }
        count
    }

    // ADDITION

    /// Implementation for AssAssign with small integer. Must be non-empty.
    #[inline]
    fn add_small_assign_impl(&mut self, y: u32) {
        // Initial add
        let mut carry = add_small_assign(self.get_mut(0), y);

        // Increment until overflow stops occurring.
        let mut size = 1;
        while carry && size < self.len() {
            carry = add_small_assign(self.get_mut(size), 1);
            size += 1;
        }

        // If we overflowed the buffer entirely, need to add 1 to the end
        // of the buffer.
        if carry {
            self.push(1);
        }
    }

    /// AddAssign small integer to bigfloat.
    #[inline]
    fn add_small_assign(&mut self, y: u32) {
        if self.is_empty() {
            self.push(y)
        } else {
            self.add_small_assign_impl(y)
        }
    }

    /// Add small integer to bigfloat.
    #[inline]
    fn add_small(&self, y: u32) -> Bigfloat {
        let mut x = self.clone();
        x.add_small_assign(y);
        x
    }

    /// AddAssign between two bigfloats.
    #[inline]
    fn add_large_assign(&mut self, y: &Bigfloat) {
        // Logic error, ensure both numbers have the same exponent.
        debug_assert!(self.exponent == y.exponent, "Bigfloat::add_large_assign different exponents");

        // Get the number of values to add_assign between them.
        // Resize the buffer so at least y.data elements are in x.data.
        let size = self.len().max(y.len());
        self.resize(size, 0);

        // Iteratively add elements from y to x.
        let mut carry = false;
        for (l, r) in self.iter_mut().zip(y.iter()).take(size) {
            // Only one op of the two can overflow, since we added at max
            // u32::max_value() + u32::max_value(). Add the previous carry,
            // and store the current carry for the next.
            let mut tmp_carry = add_small_assign(l, *r);
            if carry {
                tmp_carry |= add_small_assign(l, 1);
            }
            carry = tmp_carry;
        }

        // Overflow from the previous bit.
        if carry {
            if size == self.len() {
                // Overflow for the entire container, push 1 to the end.
                self.push(1);
            } else {
                // Internal overflow, just add 1 to the next item.
                *self.get_mut(size) += 1;
            }
        }
    }

    /// Add between two bigfloats.
    #[inline]
    fn add_large(&self, y: &Bigfloat) -> Bigfloat {
        let mut x = self.clone();
        x.add_large_assign(y);
        x
    }

    // MULTIPLICATION

    /// MulAssign small integer to bigfloat.
    fn mul_small_assign(&mut self, y: u32) {
        // Multiply iteratively over all elements, adding the carry each time.
        let mut carry: u32 = 0;
        for x in self.iter_mut() {
            carry = mul_small_assign::<u64, u32>(x, y, carry);
        }

        // Overflow of value, add to end.
        if carry != 0 {
            self.push(carry);
        }
    }

    /// Mul small integer to bigfloat.
    #[inline]
    fn mul_small(&self, y: u32) -> Bigfloat {
        let mut x = self.clone();
        x.mul_small_assign(y);
        x
    }

    /// MulAssign using pre-calculated small powers.
    #[inline]
    fn mul_spowers_assign(&mut self, mut n: i32, small_powers: &[u32]) {
        debug_assert!(n >= 0, "Bigfloat::mul_spowers_assign() must multiply by a positive power.");

        let get_power = | i: usize | unsafe { *small_powers.get_unchecked(i) };

        // Multiply by the largest small power until n < step.
        let step = small_powers.len() - 1;
        let power = get_power(step);
        let step = step as i32;
        while n >= step {
            self.mul_small_assign(power);
            n -= step;
        }

        // Multiply by the remainder.
        self.mul_small_assign(get_power(n as usize));
    }

    /// Implied MulAssign by a power of 2 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow2_assign_impl(&mut self, n: i32) {
        // Increment exponent to simulate actual addition.
        self.exponent += n;
    }

    /// Implied MulAssign by a power of 3 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow3_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE3);
    }

    /// Implied MulAssign by a power of 4 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow4_assign_impl(&mut self, n: i32) {
        // Use 4**n = 2**(2n) to minimize overflow checks.
        self.mul_pow2_assign_impl(n * 2);
    }

    /// Implied MulAssign by a power of 5 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow5_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE5);
    }

    /// Implied MulAssign by a power of 6 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow6_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow3_assign_impl(n);
    }

    /// Implied MulAssign by a power of 7 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow7_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE7);
    }

    /// Implied MulAssign by a power of 8 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow8_assign_impl(&mut self, n: i32) {
        // Use 8**n = 2**(3n) to minimize overflow checks.
        self.mul_pow2_assign_impl(n * 3);
    }

    /// Implied MulAssign by a power of 9 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow9_assign_impl(&mut self, n: i32) {
        self.mul_pow3_assign_impl(n);
        self.mul_pow3_assign_impl(n);
    }

    /// Implied MulAssign by a power of 10 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow10_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow5_assign_impl(n);
    }

    /// Implied MulAssign by a power of 11 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow11_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE11);
    }

    /// Implied MulAssign by a power of 12 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow12_assign_impl(&mut self, n: i32) {
        self.mul_pow3_assign_impl(n);
        self.mul_pow4_assign_impl(n);
    }

    /// Implied MulAssign by a power of 13 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow13_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE13);
    }

    /// Implied MulAssign by a power of 14 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow14_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow7_assign_impl(n);
    }

    /// Implied MulAssign by a power of 15 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow15_assign_impl(&mut self, n: i32) {
        self.mul_pow3_assign_impl(n);
        self.mul_pow5_assign_impl(n);
    }

    /// Implied MulAssign by a power of 16 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow16_assign_impl(&mut self, n: i32) {
        // Use 16**n = 2**(4n) to minimize overflow checks.
        self.mul_pow2_assign_impl(n * 4);
    }

    /// Implied MulAssign by a power of 17 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow17_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE17);
    }

    /// Implied MulAssign by a power of 18 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow18_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow9_assign_impl(n);
    }

    /// Implied MulAssign by a power of 19 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow19_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE19);
    }

    /// Implied MulAssign by a power of 20 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow20_assign_impl(&mut self, n: i32) {
        self.mul_pow4_assign_impl(n);
        self.mul_pow5_assign_impl(n);
    }

    /// Implied MulAssign by a power of 21 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow21_assign_impl(&mut self, n: i32) {
        self.mul_pow3_assign_impl(n);
        self.mul_pow7_assign_impl(n);
    }

    /// Implied MulAssign by a power of 22 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow22_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow11_assign_impl(n);
    }

    /// Implied MulAssign by a power of 23 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow23_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE23);
    }

    /// Implied MulAssign by a power of 24 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow24_assign_impl(&mut self, n: i32) {
        self.mul_pow3_assign_impl(n);
        self.mul_pow8_assign_impl(n);
    }

    /// Implied MulAssign by a power of 25 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow25_assign_impl(&mut self, n: i32) {
        self.mul_pow5_assign_impl(n);
        self.mul_pow5_assign_impl(n);
    }

    /// Implied MulAssign by a power of 26 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow26_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow13_assign_impl(n);
    }

    /// Implied MulAssign by a power of 27 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow27_assign_impl(&mut self, n: i32) {
        self.mul_pow3_assign_impl(n);
        self.mul_pow9_assign_impl(n);
    }

    /// Implied MulAssign by a power of 28 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow28_assign_impl(&mut self, n: i32) {
        self.mul_pow4_assign_impl(n);
        self.mul_pow7_assign_impl(n);
    }

    /// Implied MulAssign by a power of 29 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow29_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE29);
    }

    /// Implied MulAssign by a power of 30 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow30_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow15_assign_impl(n);
    }

    /// Implied MulAssign by a power of 31 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow31_assign_impl(&mut self, n: i32) {
        self.mul_spowers_assign(n, &SMALL_POWERS_BASE31);
    }

    /// Implied MulAssign by a power of 32 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow32_assign_impl(&mut self, n: i32) {
        // Use 32**n = 2**(5n) to minimize overflow checks.
        self.mul_pow2_assign_impl(n * 5);
    }

    /// Implied MulAssign by a power of 33 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow33_assign_impl(&mut self, n: i32) {
        self.mul_pow3_assign_impl(n);
        self.mul_pow11_assign_impl(n);
    }

    /// Implied MulAssign by a power of 34 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow34_assign_impl(&mut self, n: i32) {
        self.mul_pow2_assign_impl(n);
        self.mul_pow17_assign_impl(n);
    }

    /// Implied MulAssign by a power of 35 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow35_assign_impl(&mut self, n: i32) {
        self.mul_pow5_assign_impl(n);
        self.mul_pow7_assign_impl(n);
    }

    /// Implied MulAssign by a power of 36 (safe to chain calls).
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn mul_pow36_assign_impl(&mut self, n: i32) {
        self.mul_pow4_assign_impl(n);
        self.mul_pow9_assign_impl(n);
    }

    wrap_mul_pow2_assign!(mul_pow2_assign, mul_pow2_assign_impl, 2);
    wrap_mul_pown_assign!(mul_pow3_assign, mul_pow3_assign_impl, 3);
    wrap_mul_pow2_assign!(mul_pow4_assign, mul_pow4_assign_impl, 4);
    wrap_mul_pown_assign!(mul_pow5_assign, mul_pow5_assign_impl, 5);
    wrap_mul_pown_assign!(mul_pow6_assign, mul_pow6_assign_impl, 6);
    wrap_mul_pown_assign!(mul_pow7_assign, mul_pow7_assign_impl, 7);
    wrap_mul_pow2_assign!(mul_pow8_assign, mul_pow8_assign_impl, 8);
    wrap_mul_pown_assign!(mul_pow9_assign, mul_pow9_assign_impl, 9);
    wrap_mul_pown_assign!(mul_pow10_assign, mul_pow10_assign_impl, 10);
    wrap_mul_pown_assign!(mul_pow11_assign, mul_pow11_assign_impl, 11);
    wrap_mul_pown_assign!(mul_pow12_assign, mul_pow12_assign_impl, 12);
    wrap_mul_pown_assign!(mul_pow13_assign, mul_pow13_assign_impl, 13);
    wrap_mul_pown_assign!(mul_pow14_assign, mul_pow14_assign_impl, 14);
    wrap_mul_pown_assign!(mul_pow15_assign, mul_pow15_assign_impl, 15);
    wrap_mul_pow2_assign!(mul_pow16_assign, mul_pow16_assign_impl, 16);
    wrap_mul_pown_assign!(mul_pow17_assign, mul_pow17_assign_impl, 17);
    wrap_mul_pown_assign!(mul_pow18_assign, mul_pow18_assign_impl, 18);
    wrap_mul_pown_assign!(mul_pow19_assign, mul_pow19_assign_impl, 19);
    wrap_mul_pown_assign!(mul_pow20_assign, mul_pow20_assign_impl, 20);
    wrap_mul_pown_assign!(mul_pow21_assign, mul_pow21_assign_impl, 21);
    wrap_mul_pown_assign!(mul_pow22_assign, mul_pow22_assign_impl, 22);
    wrap_mul_pown_assign!(mul_pow23_assign, mul_pow23_assign_impl, 23);
    wrap_mul_pown_assign!(mul_pow24_assign, mul_pow24_assign_impl, 24);
    wrap_mul_pown_assign!(mul_pow25_assign, mul_pow25_assign_impl, 25);
    wrap_mul_pown_assign!(mul_pow26_assign, mul_pow26_assign_impl, 26);
    wrap_mul_pown_assign!(mul_pow27_assign, mul_pow27_assign_impl, 27);
    wrap_mul_pown_assign!(mul_pow28_assign, mul_pow28_assign_impl, 28);
    wrap_mul_pown_assign!(mul_pow29_assign, mul_pow29_assign_impl, 29);
    wrap_mul_pown_assign!(mul_pow30_assign, mul_pow30_assign_impl, 30);
    wrap_mul_pown_assign!(mul_pow31_assign, mul_pow31_assign_impl, 31);
    wrap_mul_pow2_assign!(mul_pow32_assign, mul_pow32_assign_impl, 32);
    wrap_mul_pown_assign!(mul_pow33_assign, mul_pow33_assign_impl, 33);
    wrap_mul_pown_assign!(mul_pow34_assign, mul_pow34_assign_impl, 34);
    wrap_mul_pown_assign!(mul_pow35_assign, mul_pow35_assign_impl, 35);
    wrap_mul_pown_assign!(mul_pow36_assign, mul_pow36_assign_impl, 36);

    // DIVISION

    /// Pad ints for division. Called internally during `div_pow*_assign`.
    #[inline]
    fn pad_division(&mut self, bytes: usize) {
        // Logic error, checked with max exponent.
        debug_assert!(bytes <= MAX_BYTES, "Bigfloat::pad_division() internal bytes overflow, bytes is {}.", bytes);

        // Assume **no** overflow for the usize, since this would lead to
        // other memory errors. Add `bytes` 0s to the left of the current
        // buffer, and decrease the exponent accordingly.

        // Remove the number of trailing zeros values for the padding.
        // If we don't need to pad the resulting buffer, return early.
        let bytes = bytes.checked_sub(self.trailing_zero_values() as usize).unwrap_or(0);
        if bytes.is_zero() || self.is_empty() {
            return;
        }

        // Decrease the exponent component.
        let bits = bytes * Self::BITS;
        self.exponent -= bits as i32;

        // Move data to new buffer, prepend `bytes` 0s, and then append
        // current data.
        let mut data = smallvec::SmallVec::with_capacity(self.len() + bytes);
        data.resize(bytes, 0);
        data.extend_from_slice(self.as_slice());

        // Swap the buffers.
        mem::swap(&mut data, &mut self.data);
    }

    /// DivAssign small integer to bigfloat.
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    fn div_small_assign(&mut self, y: u32) {
        // Divide iteratively over all elements, adding the carry each time.
        let mut rem: u32 = 0;
        for x in self.iter_mut().rev() {
            rem = div_small_assign::<u64, u32>(x, y, rem);
        }

        // Round-up if there's truncation in least-significant bit.
        // This only occurs if rem < 0x80000000, which is the midway
        // point for when we should round.
        // The container **cannot** be empty, since rem is not 0.
        // If the vector is not padded prior to use, this rounding error
        // is **very** significant.
        if rem > 0 && rem < 0x80000000 {
            *self.front_mut() += 1;
        }

        // Remove leading zero if we cause underflow. Since we're dividing
        // by a small power, we have at max 1 int removed.
        if !self.is_empty() && self.back().is_zero() {
            self.pop();
        }
    }

    /// Div small integer to bigfloat.
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    #[inline]
    fn div_small(&self, y: u32) -> Bigfloat {
        let mut x = self.clone();
        x.div_small_assign(y);
        x
    }

    /// DivAssign using pre-calculated small powers.
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    #[inline]
    fn div_spowers_assign(&mut self, mut n: i32, small_powers: &[u32]) {
        debug_assert!(n >= 0, "Bigfloat::div_spowers_assign() must multiply by a positive power.");

        let get_power = | i: usize | unsafe { *small_powers.get_unchecked(i) };

        // Divide by the largest small power until n < step.
        let step = small_powers.len() - 1;
        let power = get_power(step);
        let step = step as i32;
        while n >= step {
            self.div_small_assign(power);
            n -= step;
        }

        // Multiply by the remainder.
        self.div_small_assign(get_power(n as usize));
    }

    /// Implied DivAssign by a power of 2 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    #[inline]
    fn div_pow2_assign_impl(&mut self, n: i32) {
        // Decrement exponent to simulate actual addition.
        self.exponent -= n;
    }

    /// Implied DivAssign by a power of 3 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow3_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE3);
    }

    /// Implied DivAssign by a power of 4 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow4_assign_impl(&mut self, n: i32) {
        // Use 4**n = 2**(2n) to minimize overflow checks.
        self.div_pow2_assign_impl(n * 2);
    }

    /// Implied DivAssign by a power of 5 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow5_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE5);
    }

    /// Implied DivAssign by a power of 6 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow6_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow3_assign_impl(n);
    }

    /// Implied DivAssign by a power of 7 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow7_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE7);
    }

    /// Implied DivAssign by a power of 8 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow8_assign_impl(&mut self, n: i32) {
        // Use 8**n = 2**(3n) to minimize overflow checks.
        self.div_pow2_assign_impl(n * 3);
    }

    /// Implied DivAssign by a power of 9 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow9_assign_impl(&mut self, n: i32) {
        self.div_pow3_assign_impl(n);
        self.div_pow3_assign_impl(n);
    }

    /// Implied DivAssign by a power of 10 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow10_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow5_assign_impl(n);
    }

    /// Implied DivAssign by a power of 11 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow11_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE11);
    }

    /// Implied DivAssign by a power of 12 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow12_assign_impl(&mut self, n: i32) {
        self.div_pow3_assign_impl(n);
        self.div_pow4_assign_impl(n);
    }

    /// Implied DivAssign by a power of 13 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow13_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE13);
    }

    /// Implied DivAssign by a power of 14 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow14_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow7_assign_impl(n);
    }

    /// Implied DivAssign by a power of 15 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow15_assign_impl(&mut self, n: i32) {
        self.div_pow3_assign_impl(n);
        self.div_pow5_assign_impl(n);
    }

    /// Implied DivAssign by a power of 16 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow16_assign_impl(&mut self, n: i32) {
        // Use 16**n = 2**(4n) to minimize overflow checks.
        self.div_pow2_assign_impl(n * 4);
    }

    /// Implied DivAssign by a power of 17 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow17_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE17);
    }

    /// Implied DivAssign by a power of 18 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow18_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow9_assign_impl(n);
    }

    /// Implied DivAssign by a power of 19 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow19_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE19);
    }

    /// Implied DivAssign by a power of 20 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow20_assign_impl(&mut self, n: i32) {
        self.div_pow4_assign_impl(n);
        self.div_pow5_assign_impl(n);
    }

    /// Implied DivAssign by a power of 21 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow21_assign_impl(&mut self, n: i32) {
        self.div_pow3_assign_impl(n);
        self.div_pow7_assign_impl(n);
    }

    /// Implied DivAssign by a power of 22 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow22_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow11_assign_impl(n);
    }

    /// Implied DivAssign by a power of 23 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow23_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE23);
    }

    /// Implied DivAssign by a power of 24 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow24_assign_impl(&mut self, n: i32) {
        self.div_pow3_assign_impl(n);
        self.div_pow8_assign_impl(n);
    }

    /// Implied DivAssign by a power of 25 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow25_assign_impl(&mut self, n: i32) {
        self.div_pow5_assign_impl(n);
        self.div_pow5_assign_impl(n);
    }

    /// Implied DivAssign by a power of 26 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow26_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow13_assign_impl(n);
    }

    /// Implied DivAssign by a power of 27 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow27_assign_impl(&mut self, n: i32) {
        self.div_pow3_assign_impl(n);
        self.div_pow9_assign_impl(n);
    }

    /// Implied DivAssign by a power of 28 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow28_assign_impl(&mut self, n: i32) {
        self.div_pow4_assign_impl(n);
        self.div_pow7_assign_impl(n);
    }

    /// Implied DivAssign by a power of 29 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow29_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE29);
    }

    /// Implied DivAssign by a power of 30 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow30_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow15_assign_impl(n);
    }

    /// Implied DivAssign by a power of 31 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow31_assign_impl(&mut self, n: i32) {
        self.div_spowers_assign(n, &SMALL_POWERS_BASE31);
    }

    /// Implied DivAssign by a power of 32 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow32_assign_impl(&mut self, n: i32) {
        // Use 32**n = 2**(5n) to minimize overflow checks.
        self.div_pow2_assign_impl(n * 5);
    }

    /// Implied DivAssign by a power of 33 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow33_assign_impl(&mut self, n: i32) {
        self.div_pow3_assign_impl(n);
        self.div_pow11_assign_impl(n);
    }

    /// Implied DivAssign by a power of 34 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow34_assign_impl(&mut self, n: i32) {
        self.div_pow2_assign_impl(n);
        self.div_pow17_assign_impl(n);
    }

    /// Implied DivAssign by a power of 35 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow35_assign_impl(&mut self, n: i32) {
        self.div_pow5_assign_impl(n);
        self.div_pow7_assign_impl(n);
    }

    /// Implied DivAssign by a power of 36 (safe to chain calls).
    /// Warning: Bigfloat must have previously been padded `pad_division`.
    /// Warning: Exponent must be <= MAX_EXPONENT.
    #[inline]
    fn div_pow36_assign_impl(&mut self, n: i32) {
        self.div_pow4_assign_impl(n);
        self.div_pow9_assign_impl(n);
    }

    wrap_div_pow2_assign!(div_pow2_assign, div_pow2_assign_impl, 2);
    wrap_div_pown_assign!(div_pow3_assign, div_pow3_assign_impl, 3);
    wrap_div_pow2_assign!(div_pow4_assign, div_pow4_assign_impl, 4);
    wrap_div_pown_assign!(div_pow5_assign, div_pow5_assign_impl, 5);
    wrap_div_pown_assign!(div_pow6_assign, div_pow6_assign_impl, 6);
    wrap_div_pown_assign!(div_pow7_assign, div_pow7_assign_impl, 7);
    wrap_div_pow2_assign!(div_pow8_assign, div_pow8_assign_impl, 8);
    wrap_div_pown_assign!(div_pow9_assign, div_pow9_assign_impl, 9);
    wrap_div_pown_assign!(div_pow10_assign, div_pow10_assign_impl, 10);
    wrap_div_pown_assign!(div_pow11_assign, div_pow11_assign_impl, 11);
    wrap_div_pown_assign!(div_pow12_assign, div_pow12_assign_impl, 12);
    wrap_div_pown_assign!(div_pow13_assign, div_pow13_assign_impl, 13);
    wrap_div_pown_assign!(div_pow14_assign, div_pow14_assign_impl, 14);
    wrap_div_pown_assign!(div_pow15_assign, div_pow15_assign_impl, 15);
    wrap_div_pow2_assign!(div_pow16_assign, div_pow16_assign_impl, 16);
    wrap_div_pown_assign!(div_pow17_assign, div_pow17_assign_impl, 17);
    wrap_div_pown_assign!(div_pow18_assign, div_pow18_assign_impl, 18);
    wrap_div_pown_assign!(div_pow19_assign, div_pow19_assign_impl, 19);
    wrap_div_pown_assign!(div_pow20_assign, div_pow20_assign_impl, 20);
    wrap_div_pown_assign!(div_pow21_assign, div_pow21_assign_impl, 21);
    wrap_div_pown_assign!(div_pow22_assign, div_pow22_assign_impl, 22);
    wrap_div_pown_assign!(div_pow23_assign, div_pow23_assign_impl, 23);
    wrap_div_pown_assign!(div_pow24_assign, div_pow24_assign_impl, 24);
    wrap_div_pown_assign!(div_pow25_assign, div_pow25_assign_impl, 25);
    wrap_div_pown_assign!(div_pow26_assign, div_pow26_assign_impl, 26);
    wrap_div_pown_assign!(div_pow27_assign, div_pow27_assign_impl, 27);
    wrap_div_pown_assign!(div_pow28_assign, div_pow28_assign_impl, 28);
    wrap_div_pown_assign!(div_pow29_assign, div_pow29_assign_impl, 29);
    wrap_div_pown_assign!(div_pow30_assign, div_pow30_assign_impl, 30);
    wrap_div_pown_assign!(div_pow31_assign, div_pow31_assign_impl, 31);
    wrap_div_pow2_assign!(div_pow32_assign, div_pow32_assign_impl, 32);
    wrap_div_pown_assign!(div_pow33_assign, div_pow33_assign_impl, 33);
    wrap_div_pown_assign!(div_pow34_assign, div_pow34_assign_impl, 34);
    wrap_div_pown_assign!(div_pow35_assign, div_pow35_assign_impl, 35);
    wrap_div_pown_assign!(div_pow36_assign, div_pow36_assign_impl, 36);

    // FROM BYTES
    from_bytes!(from_bytes_3);
    from_bytes!(from_bytes_5);
    from_bytes!(from_bytes_6);
    from_bytes!(from_bytes_7);
    from_bytes!(from_bytes_9);
    from_bytes!(from_bytes_10);
    from_bytes!(from_bytes_11);
    from_bytes!(from_bytes_12);
    from_bytes!(from_bytes_13);
    from_bytes!(from_bytes_14);
    from_bytes!(from_bytes_15);
    from_bytes!(from_bytes_17);
    from_bytes!(from_bytes_18);
    from_bytes!(from_bytes_19);
    from_bytes!(from_bytes_20);
    from_bytes!(from_bytes_21);
    from_bytes!(from_bytes_22);
    from_bytes!(from_bytes_23);
    from_bytes!(from_bytes_24);
    from_bytes!(from_bytes_25);
    from_bytes!(from_bytes_26);
    from_bytes!(from_bytes_27);
    from_bytes!(from_bytes_28);
    from_bytes!(from_bytes_29);
    from_bytes!(from_bytes_30);
    from_bytes!(from_bytes_31);
    from_bytes!(from_bytes_33);
    from_bytes!(from_bytes_34);
    from_bytes!(from_bytes_35);
    from_bytes!(from_bytes_36);

    /// Initialize Bigfloat from bytes with custom base.
    pub fn from_bytes(base: u32, first: *const u8, last: *const u8)
        -> (Bigfloat, *const u8)
    {
        match base {
            3  => Self::from_bytes_3(first, last),
            5  => Self::from_bytes_5(first, last),
            6  => Self::from_bytes_6(first, last),
            7  => Self::from_bytes_7(first, last),
            9  => Self::from_bytes_9(first, last),
            10 => Self::from_bytes_10(first, last),
            11 => Self::from_bytes_11(first, last),
            12 => Self::from_bytes_12(first, last),
            13 => Self::from_bytes_13(first, last),
            14 => Self::from_bytes_14(first, last),
            15 => Self::from_bytes_15(first, last),
            17 => Self::from_bytes_17(first, last),
            18 => Self::from_bytes_18(first, last),
            19 => Self::from_bytes_19(first, last),
            20 => Self::from_bytes_20(first, last),
            21 => Self::from_bytes_21(first, last),
            22 => Self::from_bytes_22(first, last),
            23 => Self::from_bytes_23(first, last),
            24 => Self::from_bytes_24(first, last),
            25 => Self::from_bytes_25(first, last),
            26 => Self::from_bytes_26(first, last),
            27 => Self::from_bytes_27(first, last),
            28 => Self::from_bytes_28(first, last),
            29 => Self::from_bytes_29(first, last),
            30 => Self::from_bytes_30(first, last),
            31 => Self::from_bytes_31(first, last),
            33 => Self::from_bytes_33(first, last),
            34 => Self::from_bytes_34(first, last),
            35 => Self::from_bytes_35(first, last),
            36 => Self::from_bytes_36(first, last),
            // We shouldn't have any powers of 2 here.
            _  => unimplemented!()
        }
    }

    // TO FLOAT

    /// Export native float from bigfloat.
    pub fn as_float<F: Float>(&self) -> F {
        unimplemented!()
    }

    // VEC-LIKE

    /// Get if the integer data is empty.
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get length of integer data.
    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len()
    }

    /// Get vector as slice.
    #[inline(always)]
    fn as_slice(&self) -> &[u32] {
        self.data.as_slice()
    }

    /// Remove last element from integer data and return it.
    #[inline(always)]
    fn pop(&mut self) -> Option<u32> {
        self.data.pop()
    }

    /// Append integer to back of collection.
    #[inline(always)]
    fn push(&mut self, value: u32) {
        self.data.push(value)
    }

    /// Extend integer data from slice.
    #[inline(always)]
    fn extend_from_slice(&mut self, other: &[u32]) {
        self.data.extend_from_slice(other)
    }

    /// Resize container to new_len, appending value as needed to container.
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: u32) {
        self.data.resize(new_len, value)
    }

    /// Get iterator to integer data.
    #[inline(always)]
    fn iter(&self) -> slice::Iter<u32> {
        self.data.iter()
    }

    /// Get mutable iterator to integer data.
    #[inline(always)]
    fn iter_mut(&mut self) -> slice::IterMut<u32> {
        self.data.iter_mut()
    }

    /// Get the front integer.
    #[inline(always)]
    fn front(&self) -> &u32 {
        debug_assert!(self.len() > 0);
        self.get(0)
    }

    /// Get the front integer as mutable.
    #[inline(always)]
    fn front_mut(&mut self) -> &mut u32 {
        debug_assert!(self.len() > 0);
        self.get_mut(0)
    }

    /// Get the back integer.
    #[inline(always)]
    fn back(&self) -> &u32 {
        debug_assert!(self.len() > 0);
        self.get(self.len()-1)
    }

    /// Get the back integer as mutable.
    #[inline(always)]
    fn back_mut(&mut self) -> &mut u32 {
        debug_assert!(self.len() > 0);
        let index = self.len()-1;
        self.get_mut(index)
    }

    /// Unchecked get.
    #[inline(always)]
    fn get<I>(&self, index: I) -> &I::Output
        where I: slice::SliceIndex<[u32]>
    {
        unsafe { self.data.get_unchecked(index) }
    }

    /// Unchecked get_mut.
    #[inline(always)]
    fn get_mut<I>(&mut self, index: I) -> &mut I::Output
        where I: slice::SliceIndex<[u32]>
    {
        unsafe { self.data.get_unchecked_mut(index) }
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
        assert_eq!(bigfloat, Bigfloat { data: smallvec![], exponent: 0 });
    }

    #[test]
    fn from_u32_test() {
        let bigfloat = Bigfloat::from_u32(255);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255], exponent: 0 });
    }

    #[test]
    fn from_u64_test() {
        let bigfloat = Bigfloat::from_u64(1152921504606847231);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255, 1 << 28], exponent: 0 });
    }

    #[test]
    fn from_u128_test() {
        let bigfloat = Bigfloat::from_u128(1329227997022855913342108839786316031);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255, 1 << 28, 1 << 26, 1<< 24], exponent: 0 });
    }

    // PROPERTIES

    #[test]
    fn leading_zero_values_test() {
        assert_eq!(Bigfloat::from_u32(0xFF).leading_zero_values(), 0);
        assert_eq!(Bigfloat::from_u64(0xFF00000000).leading_zero_values(), 0);
        assert_eq!(Bigfloat::from_u128(0xFF000000000000000000000000).leading_zero_values(), 0);

        assert_eq!(Bigfloat::from_u32(0xF).leading_zero_values(), 0);
        assert_eq!(Bigfloat::from_u64(0xF00000000).leading_zero_values(), 0);
        assert_eq!(Bigfloat::from_u128(0xF000000000000000000000000).leading_zero_values(), 0);

        assert_eq!(Bigfloat::from_u32(0xF0).leading_zero_values(), 0);
        assert_eq!(Bigfloat::from_u64(0xF000000000).leading_zero_values(), 0);
        assert_eq!(Bigfloat::from_u128(0xF0000000000000000000000000).leading_zero_values(), 0);
    }

    #[test]
    fn trailing_zero_values_test() {
        assert_eq!(Bigfloat::new().trailing_zero_values(), 0);

        assert_eq!(Bigfloat::from_u32(0xFF).trailing_zero_values(), 0);
        assert_eq!(Bigfloat::from_u64(0xFF00000000).trailing_zero_values(), 1);
        assert_eq!(Bigfloat::from_u128(0xFF000000000000000000000000).trailing_zero_values(), 3);

        assert_eq!(Bigfloat::from_u32(0xF).trailing_zero_values(), 0);
        assert_eq!(Bigfloat::from_u64(0xF00000000).trailing_zero_values(), 1);
        assert_eq!(Bigfloat::from_u128(0xF000000000000000000000000).trailing_zero_values(), 3);

        assert_eq!(Bigfloat::from_u32(0xF0).trailing_zero_values(), 0);
        assert_eq!(Bigfloat::from_u64(0xF000000000).trailing_zero_values(), 1);
        assert_eq!(Bigfloat::from_u128(0xF0000000000000000000000000).trailing_zero_values(), 3);
    }

    #[test]
    fn leading_zeros_test() {
        assert_eq!(Bigfloat::from_u32(0xFF).leading_zeros(), 24);
        assert_eq!(Bigfloat::from_u64(0xFF00000000).leading_zeros(), 24);
        assert_eq!(Bigfloat::from_u128(0xFF000000000000000000000000).leading_zeros(), 24);

        assert_eq!(Bigfloat::from_u32(0xF).leading_zeros(), 28);
        assert_eq!(Bigfloat::from_u64(0xF00000000).leading_zeros(), 28);
        assert_eq!(Bigfloat::from_u128(0xF000000000000000000000000).leading_zeros(), 28);

        assert_eq!(Bigfloat::from_u32(0xF0).leading_zeros(), 24);
        assert_eq!(Bigfloat::from_u64(0xF000000000).leading_zeros(), 24);
        assert_eq!(Bigfloat::from_u128(0xF0000000000000000000000000).leading_zeros(), 24);
    }

    #[test]
    fn trailing_zeros_test() {
        assert_eq!(Bigfloat::from_u32(0xFF).trailing_zeros(), 0);
        assert_eq!(Bigfloat::from_u64(0xFF00000000).trailing_zeros(), 32);
        assert_eq!(Bigfloat::from_u128(0xFF000000000000000000000000).trailing_zeros(), 96);

        assert_eq!(Bigfloat::from_u32(0xF).trailing_zeros(), 0);
        assert_eq!(Bigfloat::from_u64(0xF00000000).trailing_zeros(), 32);
        assert_eq!(Bigfloat::from_u128(0xF000000000000000000000000).trailing_zeros(), 96);

        assert_eq!(Bigfloat::from_u32(0xF0).trailing_zeros(), 4);
        assert_eq!(Bigfloat::from_u64(0xF000000000).trailing_zeros(), 36);
        assert_eq!(Bigfloat::from_u128(0xF0000000000000000000000000).trailing_zeros(), 100);
    }

    // ADDITION

    #[test]
    fn add_small_test() {
        // Overflow check (single)
        // This should set all the internal data values to 0, the top
        // value to (1<<31), and the bottom value to (4>>1).
        // This is because the max_value + 1 leads to all 0s, we set the
        // topmost bit to 1.
        let mut x = Bigfloat::from_u32(4294967295);
        x.add_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 1], exponent: 0 });

        // No overflow, single value
        let mut x = Bigfloat::from_u32(5);
        x.add_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![12], exponent: 0 });

        // Single carry, internal overflow
        let mut x = Bigfloat::from_u64(0x80000000FFFFFFFF);
        x.add_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![6, 0x80000001], exponent: 0 });

        // Double carry, overflow
        let mut x = Bigfloat::from_u64(0xFFFFFFFFFFFFFFFF);
        x.add_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![6, 0, 1], exponent: 0 });
    }

    #[test]
    fn add_large_test() {
        // No overflow check, add symmetric (1-int each).
        let mut x = Bigfloat::from_u32(5);
        let y = Bigfloat::from_u32(7);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![12], exponent: 0 });

        // No overflow, symmetric (2- and 2-ints).
        let mut x = Bigfloat::from_u64(1125899906842624);
        let y = Bigfloat::from_u64(35184372088832);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![0, 270336], exponent: 0 });

        // No overflow, asymmetric (1- and 2-ints).
        let mut x = Bigfloat::from_u32(5);
        let y = Bigfloat::from_u64(35184372088832);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![5, 8192], exponent: 0 });

        // Internal overflow check.
        let mut x = Bigfloat::from_u32(0xF1111111);
        let y = Bigfloat::from_u64(0x12345678);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![0x3456789, 1], exponent: 0 });

        // Complete overflow check
        let mut x = Bigfloat::from_u32(4294967295);
        let y = Bigfloat::from_u32(4294967295);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![4294967294, 1], exponent: 0 });
    }

    // MULTIPLICATION

    #[test]
    fn mul_small_test() {
        // No overflow check, 1-int.
        let mut x = Bigfloat::from_u32(5);
        x.mul_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![35], exponent: 0 });

        // No overflow check, 2-ints.
        let mut x = Bigfloat::from_u64(0x4000000040000);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![0x00140000, 0x140000], exponent: 0 });

        // Overflow, 1 carry.
        let mut x = Bigfloat::from_u32(0x33333334);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 1], exponent: 0 });

        // Overflow, 1 carry, internal.
        let mut x = Bigfloat::from_u64(0x133333334);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 6], exponent: 0 });

        // Overflow, 2 carries.
        let mut x = Bigfloat::from_u64(0x3333333333333334);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 0, 1], exponent: 0 });
    }

    /// Checker for the pow tests.
    macro_rules! check_pow {
        ($input_data:expr, $input_exp:expr, $result_data:expr, $result_exp:expr, $n:expr, $func:ident)
        => ({
            let mut i = Bigfloat { data: $input_data, exponent: $input_exp };
            i.$func($n);
            assert_eq!(Bigfloat {data: $result_data, exponent: $result_exp }, i);
        });
    }

    /// Checker for the pow2 tests.
    macro_rules! check_pow2 {
        (@neg $func:ident, $n:expr) => ({
            check_pow!(smallvec![], 0, smallvec![], 0, 0, $func);
            check_pow!(smallvec![1], 0, smallvec![1], 0, 0, $func);
            check_pow!(smallvec![1], 0, smallvec![1], -$n*1, 1, $func);
            check_pow!(smallvec![1], 0, smallvec![1], -$n*4, 4, $func);
            check_pow!(smallvec![2], 0, smallvec![2], -$n*4, 4, $func);
        });
        ($func:ident, $n:expr) => ({
            check_pow!(smallvec![], 0, smallvec![], 0, 0, $func);
            check_pow!(smallvec![1], 0, smallvec![1], 0, 0, $func);
            check_pow!(smallvec![1], 0, smallvec![1], $n*1, 1, $func);
            check_pow!(smallvec![2], 0, smallvec![2], $n*4, 4, $func);
        })
    }

    /// Checker for the pow2n tests.
    macro_rules! check_pown {
        ($input_data:expr, $input_exp:expr, $n:expr ; $($result_data:expr, $result_exp:expr, $func:ident ; )+)
        => ($(
            check_pow!($input_data, $input_exp, $result_data, $result_exp, $n, $func);
        )*)
    }

    #[test]
    fn mul_pow2_test() {
        check_pow2!(mul_pow2_assign, 1);
        check_pow2!(mul_pow4_assign, 2);
        check_pow2!(mul_pow8_assign, 3);
        check_pow2!(mul_pow16_assign, 4);
        check_pow2!(mul_pow32_assign, 5);
    }

    #[test]
    fn mul_pown_test() {
        // Zero case
        check_pown!(
            smallvec![], 0, 0 ;
            smallvec![], 0, mul_pow3_assign ;
            smallvec![], 0, mul_pow5_assign ;
            smallvec![], 0, mul_pow6_assign ;
            smallvec![], 0, mul_pow7_assign ;
            smallvec![], 0, mul_pow9_assign ;
            smallvec![], 0, mul_pow10_assign ;
            smallvec![], 0, mul_pow11_assign ;
            smallvec![], 0, mul_pow12_assign ;
            smallvec![], 0, mul_pow13_assign ;
            smallvec![], 0, mul_pow14_assign ;
            smallvec![], 0, mul_pow15_assign ;
            smallvec![], 0, mul_pow17_assign ;
            smallvec![], 0, mul_pow18_assign ;
            smallvec![], 0, mul_pow19_assign ;
            smallvec![], 0, mul_pow20_assign ;
            smallvec![], 0, mul_pow21_assign ;
            smallvec![], 0, mul_pow22_assign ;
            smallvec![], 0, mul_pow23_assign ;
            smallvec![], 0, mul_pow24_assign ;
            smallvec![], 0, mul_pow25_assign ;
            smallvec![], 0, mul_pow26_assign ;
            smallvec![], 0, mul_pow27_assign ;
            smallvec![], 0, mul_pow28_assign ;
            smallvec![], 0, mul_pow29_assign ;
            smallvec![], 0, mul_pow30_assign ;
            smallvec![], 0, mul_pow31_assign ;
            smallvec![], 0, mul_pow33_assign ;
            smallvec![], 0, mul_pow34_assign ;
            smallvec![], 0, mul_pow35_assign ;
            smallvec![], 0, mul_pow36_assign ;
        );

        // 1 case ** pow2
        check_pown!(
            smallvec![1], 0, 2 ;
            smallvec![9], 0, mul_pow3_assign ;
            smallvec![25], 0, mul_pow5_assign ;
            smallvec![9], 2, mul_pow6_assign ;
            smallvec![49], 0, mul_pow7_assign ;
            smallvec![81], 0, mul_pow9_assign ;
            smallvec![25], 2, mul_pow10_assign ;
            smallvec![121], 0, mul_pow11_assign ;
            smallvec![9], 4, mul_pow12_assign ;
            smallvec![169], 0, mul_pow13_assign ;
            smallvec![49], 2, mul_pow14_assign ;
            smallvec![225], 0, mul_pow15_assign ;
            smallvec![289], 0, mul_pow17_assign ;
            smallvec![81], 2, mul_pow18_assign ;
            smallvec![361], 0, mul_pow19_assign ;
            smallvec![25], 4, mul_pow20_assign ;
            smallvec![441], 0, mul_pow21_assign ;
            smallvec![121], 2, mul_pow22_assign ;
            smallvec![529], 0, mul_pow23_assign ;
            smallvec![9], 6, mul_pow24_assign ;
            smallvec![625], 0, mul_pow25_assign ;
            smallvec![169], 2, mul_pow26_assign ;
            smallvec![729], 0, mul_pow27_assign ;
            smallvec![49], 4, mul_pow28_assign ;
            smallvec![841], 0, mul_pow29_assign ;
            smallvec![225], 2, mul_pow30_assign ;
            smallvec![961], 0, mul_pow31_assign ;
            smallvec![1089], 0, mul_pow33_assign ;
            smallvec![289], 2, mul_pow34_assign ;
            smallvec![1225], 0, mul_pow35_assign ;
            smallvec![81], 4, mul_pow36_assign ;
        );

        // Non-1 case * pow2
        check_pown!(
            smallvec![7], 0, 2 ;
            smallvec![63], 0, mul_pow3_assign ;
            smallvec![175], 0, mul_pow5_assign ;
            smallvec![63], 2, mul_pow6_assign ;
            smallvec![343], 0, mul_pow7_assign ;
            smallvec![567], 0, mul_pow9_assign ;
            smallvec![175], 2, mul_pow10_assign ;
            smallvec![847], 0, mul_pow11_assign ;
            smallvec![63], 4, mul_pow12_assign ;
            smallvec![1183], 0, mul_pow13_assign ;
            smallvec![343], 2, mul_pow14_assign ;
            smallvec![1575], 0, mul_pow15_assign ;
            smallvec![2023], 0, mul_pow17_assign ;
            smallvec![567], 2, mul_pow18_assign ;
            smallvec![2527], 0, mul_pow19_assign ;
            smallvec![175], 4, mul_pow20_assign ;
            smallvec![3087], 0, mul_pow21_assign ;
            smallvec![847], 2, mul_pow22_assign ;
            smallvec![3703], 0, mul_pow23_assign ;
            smallvec![63], 6, mul_pow24_assign ;
            smallvec![4375], 0, mul_pow25_assign ;
            smallvec![1183], 2, mul_pow26_assign ;
            smallvec![5103], 0, mul_pow27_assign ;
            smallvec![343], 4, mul_pow28_assign ;
            smallvec![5887], 0, mul_pow29_assign ;
            smallvec![1575], 2, mul_pow30_assign ;
            smallvec![6727], 0, mul_pow31_assign ;
            smallvec![7623], 0, mul_pow33_assign ;
            smallvec![2023], 2, mul_pow34_assign ;
            smallvec![8575], 0, mul_pow35_assign ;
            smallvec![567], 4, mul_pow36_assign ;
        );

        // Overflow case
        check_pown!(
            smallvec![7], 0, 22 ;
            smallvec![624085167, 51], 0, mul_pow3_assign ;
            smallvec![2517658495, 3885780], 0, mul_pow5_assign ;
            smallvec![624085167, 51], 22, mul_pow6_assign ;
            smallvec![821077879, 2077315763, 1], 0, mul_pow7_assign ;
            smallvec![363536663, 2971099641, 373], 0, mul_pow9_assign ;
            smallvec![2517658495, 3885780], 22, mul_pow10_assign ;
            smallvec![3435804255, 4136938383, 30889], 0, mul_pow11_assign ;
            smallvec![624085167, 51], 44, mul_pow12_assign ;
            smallvec![1461939919, 4042437051, 1218798], 0, mul_pow13_assign ;
            smallvec![821077879, 2077315763, 1], 22, mul_pow14_assign ;
            smallvec![4148791143, 1053307084, 28391348], 0, mul_pow15_assign ;
            smallvec![4274854567, 3675497104, 445712267], 0, mul_pow17_assign ;
            smallvec![363536663, 2971099641, 373], 22, mul_pow18_assign ;
            smallvec![442098831, 2102541774, 854443491, 1], 0, mul_pow19_assign ;
            smallvec![2517658495, 3885780], 44, mul_pow20_assign ;
            smallvec![229089951, 1212071740, 3609236746, 10], 0, mul_pow21_assign ;
            smallvec![3435804255, 4136938383, 30889], 22, mul_pow22_assign ;
            smallvec![1478922199, 2466168986, 903793223, 80], 0, mul_pow23_assign ;
            smallvec![624085167, 51], 66, mul_pow24_assign ;
            smallvec![3338697911, 3024324511, 967955121, 502], 0, mul_pow25_assign ;
            smallvec![1461939919, 4042437051, 1218798], 22, mul_pow26_assign ;
            smallvec![3861939007, 3545742225, 1582773326, 2730], 0, mul_pow27_assign ;
            smallvec![821077879, 2077315763, 1], 44, mul_pow28_assign ;
            smallvec![2186041071, 2503332440, 2033127165, 13151], 0, mul_pow29_assign ;
            smallvec![4148791143, 1053307084, 28391348], 22, mul_pow30_assign ;
            smallvec![123416775, 3495261177, 2153535316, 57039], 0, mul_pow31_assign ;
            smallvec![2037864263, 1104016441, 2837850123, 225696], 0, mul_pow33_assign ;
            smallvec![4274854567, 3675497104, 445712267], 22, mul_pow34_assign ;
            smallvec![649085551, 1084312505, 1210820426, 823598], 0, mul_pow35_assign ;
            smallvec![363536663, 2971099641, 373], 44, mul_pow36_assign ;
        );
    }

    // DIVISION

    #[test]
    fn padded_bits_test() {
        // Ensure it works for all bases.
        for base in BASE_POWN.iter().cloned() {
            padded_bits(base, 1);
        }

        // Check compared to known values.
        // TODO(ahuszagh) Restore
//        assert_eq!(padded_bits(3, 10), 17);
//        assert_eq!(padded_bits(6, 10), 17);
//        assert_eq!(padded_bits(12, 10), 17);
//        assert_eq!(padded_bits(24, 10), 17);
//        assert_eq!(padded_bits(5, 10), 25);
//        assert_eq!(padded_bits(10, 10), 25);
//        assert_eq!(padded_bits(20, 10), 25);
//        assert_eq!(padded_bits(7, 10), 30);
//        assert_eq!(padded_bits(14, 10), 30);
//        assert_eq!(padded_bits(28, 10), 30);
//        assert_eq!(padded_bits(11, 10), 36);
//        assert_eq!(padded_bits(22, 10), 36);
//        assert_eq!(padded_bits(13, 10), 39);
//        assert_eq!(padded_bits(26, 10), 39);
//        assert_eq!(padded_bits(17, 10), 42);
//        assert_eq!(padded_bits(34, 10), 42);
//        assert_eq!(padded_bits(19, 10), 44);
//        assert_eq!(padded_bits(23, 10), 48);
//        assert_eq!(padded_bits(29, 10), 50);
//        assert_eq!(padded_bits(31, 10), 51);
//        assert_eq!(padded_bits(9, 10), 27);
//        assert_eq!(padded_bits(18, 10), 27);
//        assert_eq!(padded_bits(36, 10), 27);
//        assert_eq!(padded_bits(15, 10), 39);
//        assert_eq!(padded_bits(30, 10), 39);
//        assert_eq!(padded_bits(21, 10), 47);
//        assert_eq!(padded_bits(27, 10), 42);
//        assert_eq!(padded_bits(33, 10), 57);
//        assert_eq!(padded_bits(25, 10), 56);
//        assert_eq!(padded_bits(35, 10), 68);
    }

    #[test]
    fn pad_division_test() {
        // Pad 0
        let mut x = Bigfloat { data: smallvec![0, 0, 0, 1], exponent: 0 };
        x.pad_division(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exponent: 0 });

        // Pad 1
        let mut x = Bigfloat { data: smallvec![0, 0, 1], exponent: 0 };
        x.pad_division(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exponent: -32 });

        // Pad 2
        let mut x = Bigfloat { data: smallvec![0, 1], exponent: 0 };
        x.pad_division(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exponent: -64 });

        // Pad 3
        let mut x = Bigfloat::from_u32(1);
        x.pad_division(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1], exponent: -96 });

        let mut x = Bigfloat::from_u64(0x100000001);
        x.pad_division(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1, 1], exponent: -96 });

        let mut x = Bigfloat { data: smallvec![1, 1, 1], exponent: 0 };
        x.pad_division(3);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 1, 1, 1], exponent: -96 });

        // Pad 4
        let mut x = Bigfloat::from_u128(0x1000000010000000100000001);
        x.pad_division(4);
        assert_eq!(x, Bigfloat { data: smallvec![0, 0, 0, 0, 1, 1, 1, 1], exponent: -128 });
    }

    #[test]
    fn div_small_test() {
        // 1-int.
        let mut x = Bigfloat::from_u32(5);
        x.pad_division(2);
        x.div_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![0xDB6DB6DC, 0xB6DB6DB6], exponent: -64 });

        // 2-ints.
        let mut x = Bigfloat::from_u64(0x4000000040000);
        x.pad_division(2);
        x.div_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![0x9999999A, 0x99999999, 0xCCCD9999, 0xCCCC], exponent: -64 });

        // 1-int.
        let mut x = Bigfloat::from_u32(0x33333334);
        x.pad_division(2);
        x.div_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![0x0, 0x0, 0xA3D70A4], exponent: -64 });

        // 2-ints.
        let mut x = Bigfloat::from_u64(0x133333334);
        x.pad_division(2);
        x.div_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![0x33333334, 0x33333333, 0x3D70A3D7], exponent: -64 });

        // 2-ints.
        let mut x = Bigfloat::from_u64(0x3333333333333334);
        x.pad_division(2);
        x.div_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![0xCCCCCCCD, 0xCCCCCCCC, 0xD70A3D70, 0xA3D70A3], exponent: -64 });
    }

    #[test]
    fn div_pow2_test() {
        check_pow2!(@neg div_pow2_assign, 1);
        check_pow2!(@neg div_pow4_assign, 2);
        check_pow2!(@neg div_pow8_assign, 3);
        check_pow2!(@neg div_pow16_assign, 4);
        check_pow2!(@neg div_pow32_assign, 5);
    }

    #[test]
    fn div_pown_test() {
        // Zero case
        check_pown!(
            smallvec![], 0, 0 ;
            smallvec![], 0, div_pow3_assign ;
            smallvec![], 0, div_pow5_assign ;
            smallvec![], 0, div_pow6_assign ;
            smallvec![], 0, div_pow7_assign ;
            smallvec![], 0, div_pow9_assign ;
            smallvec![], 0, div_pow10_assign ;
            smallvec![], 0, div_pow11_assign ;
            smallvec![], 0, div_pow12_assign ;
            smallvec![], 0, div_pow13_assign ;
            smallvec![], 0, div_pow14_assign ;
            smallvec![], 0, div_pow15_assign ;
            smallvec![], 0, div_pow17_assign ;
            smallvec![], 0, div_pow18_assign ;
            smallvec![], 0, div_pow19_assign ;
            smallvec![], 0, div_pow20_assign ;
            smallvec![], 0, div_pow21_assign ;
            smallvec![], 0, div_pow22_assign ;
            smallvec![], 0, div_pow23_assign ;
            smallvec![], 0, div_pow24_assign ;
            smallvec![], 0, div_pow25_assign ;
            smallvec![], 0, div_pow26_assign ;
            smallvec![], 0, div_pow27_assign ;
            smallvec![], 0, div_pow28_assign ;
            smallvec![], 0, div_pow29_assign ;
            smallvec![], 0, div_pow30_assign ;
            smallvec![], 0, div_pow31_assign ;
            smallvec![], 0, div_pow33_assign ;
            smallvec![], 0, div_pow34_assign ;
            smallvec![], 0, div_pow35_assign ;
            smallvec![], 0, div_pow36_assign ;
        );

        // 1 case ** pow2
        check_pown!(
            smallvec![1], 0, 2 ;
            smallvec![1908874354, 477218588], -64, div_pow3_assign ;
            smallvec![3607772529, 171798691], -64, div_pow5_assign ;
            smallvec![1908874354, 477218588], -66, div_pow6_assign ;
            smallvec![3418443359, 87652393], -64, div_pow7_assign ;
            smallvec![2598190093, 53024287], -64, div_pow9_assign ;
            smallvec![3607772529, 171798691], -66, div_pow10_assign ;
            smallvec![2094240252, 35495597], -64, div_pow11_assign ;
            smallvec![1908874354, 477218588], -68, div_pow12_assign ;
            smallvec![2871782867, 25414007], -64, div_pow13_assign ;
            smallvec![3418443359, 87652393], -66, div_pow14_assign ;
            smallvec![2309737969, 19088743], -64, div_pow15_assign ;
            smallvec![2288667695, 14861478], -64, div_pow17_assign ;
            smallvec![2598190093, 53024287], -66, div_pow18_assign ;
            smallvec![1427689960, 11897416], -64, div_pow19_assign ;
            smallvec![3607772529, 171798691], -68, div_pow20_assign ;
            smallvec![3837227018, 3720357158, 9739154], -96, div_pow21_assign ;
            smallvec![2094240252, 35495597], -66, div_pow22_assign ;
            smallvec![235451894, 3458707123, 8119030], -96, div_pow23_assign ;
            smallvec![1908874354, 477218588], -70, div_pow24_assign ;
            smallvec![2515132849, 2893089970, 6871947], -96, div_pow25_assign ;
            smallvec![2871782867, 25414007], -66, div_pow26_assign ;
            smallvec![2197562142, 5891587], -64, div_pow27_assign ;
            smallvec![3418443359, 87652393], -68, div_pow28_assign ;
            smallvec![4121330093, 2451348753, 5106976], -96, div_pow29_assign ;
            smallvec![2309737969, 19088743], -66, div_pow30_assign ;
            smallvec![902792294, 3343013046, 4469268], -96, div_pow31_assign ;
            smallvec![844006430, 1187130538, 3943955], -96, div_pow33_assign ;
            smallvec![2288667695, 14861478], -66, div_pow34_assign ;
            smallvec![1896797802, 3229114187, 3506095], -96, div_pow35_assign ;
            smallvec![2598190093, 53024287], -68, div_pow36_assign ;
        );

        // Non-1 case * pow2
        check_pown!(
            smallvec![7], 0, 2 ;
            smallvec![477218589, 3340530119], -64, div_pow3_assign ;
            smallvec![3779571221, 1202590842], -64, div_pow5_assign ;
            smallvec![477218589, 3340530119], -66, div_pow6_assign ;
            smallvec![2454267027, 613566756], -64, div_pow7_assign ;
            smallvec![1007461465, 371170013], -64, div_pow9_assign ;
            smallvec![3779571221, 1202590842], -66, div_pow10_assign ;
            smallvec![1774779875, 248469182], -64, div_pow11_assign ;
            smallvec![477218589, 3340530119], -68, div_pow12_assign ;
            smallvec![2922610882, 177898053], -64, div_pow13_assign ;
            smallvec![2454267027, 613566756], -66, div_pow14_assign ;
            smallvec![3283263889, 133621204], -64, div_pow15_assign ;
            smallvec![3135771971, 104030349], -64, div_pow17_assign ;
            smallvec![1007461465, 371170013], -66, div_pow18_assign ;
            smallvec![1403895128, 83281914], -64, div_pow19_assign ;
            smallvec![3779571221, 1202590842], -68, div_pow20_assign ;
            smallvec![1090785346, 272696336, 68174084], -96, div_pow21_assign ;
            smallvec![1774779875, 248469182], -66, div_pow22_assign ;
            smallvec![1648163254, 2736113381, 56833215], -96, div_pow23_assign ;
            smallvec![477218589, 3340530119], -70, div_pow24_assign ;
            smallvec![426060756, 3071760610, 48103633], -96, div_pow25_assign ;
            smallvec![2922610882, 177898053], -66, div_pow26_assign ;
            smallvec![2498033105, 41241112], -64, div_pow27_assign ;
            smallvec![2454267027, 613566756], -68, div_pow28_assign ;
            smallvec![3079506873, 4274539389, 35748835], -96, div_pow29_assign ;
            smallvec![3283263889, 133621204], -66, div_pow30_assign ;
            smallvec![2024578757, 1926254843, 31284881], -96, div_pow31_assign ;
            smallvec![1613077709, 4014946471, 27607686], -96, div_pow33_assign ;
            smallvec![3135771971, 104030349], -66, div_pow34_assign ;
            smallvec![392682725, 1128962832, 24542670], -96, div_pow35_assign ;
            smallvec![1007461465, 371170013], -68, div_pow36_assign ;
        );

        // More than 1 iteration
        check_pown!(
            smallvec![7], 0, 22 ;
            smallvec![1097992198, 4114813525], -96, div_pow3_assign ;
            smallvec![1192962241, 3765478296, 54159], -128, div_pow5_assign ;
            smallvec![1097992198, 4114813525], -118, div_pow6_assign ;
            smallvec![2709929360, 113271394, 33], -128, div_pow7_assign ;
            smallvec![4163638954, 563173765], -128, div_pow9_assign ;
            smallvec![1192962241, 3765478296, 54159], -150, div_pow10_assign ;
            smallvec![1846927277, 2280466835, 6813002], -160, div_pow11_assign ;
            smallvec![1097992198, 4114813525], -140, div_pow12_assign ;
            smallvec![2762324558, 3336209160, 172672], -160, div_pow13_assign ;
            smallvec![2709929360, 113271394, 33], -150, div_pow14_assign ;
            smallvec![2920419711, 2530008966, 7412], -160, div_pow15_assign ;
            smallvec![1865691657, 743981915, 472], -160, div_pow17_assign ;
            smallvec![4163638954, 563173765], -150, div_pow18_assign ;
            smallvec![2380074498, 3734101505, 40], -160, div_pow19_assign ;
            smallvec![1192962241, 3765478296, 54159], -172, div_pow20_assign ;
            smallvec![1444322040, 2234040319, 4], -160, div_pow21_assign ;
            smallvec![1846927277, 2280466835, 6813002], -182, div_pow22_assign ;
            smallvec![2427835437, 2623765955], -160, div_pow23_assign ;
            smallvec![1097992198, 4114813525], -162, div_pow24_assign ;
            smallvec![2607034598, 1956428404, 419041749], -192, div_pow25_assign ;
            smallvec![2762324558, 3336209160, 172672], -182, div_pow26_assign ;
            smallvec![3707063943, 77078751], -160, div_pow27_assign ;
            smallvec![2709929360, 113271394, 33], -172, div_pow28_assign ;
            smallvec![1810784743, 2979999428, 16002267], -192, div_pow29_assign ;
            smallvec![2920419711, 2530008966, 7412], -182, div_pow30_assign ;
            smallvec![3974785410, 4053206930, 3689607], -192, div_pow31_assign ;
            smallvec![2574918056, 1209062500, 932461], -192, div_pow33_assign ;
            smallvec![1865691657, 743981915, 472], -182, div_pow34_assign ;
            smallvec![625750939, 2388256588, 793309967, 255529], -224, div_pow35_assign ;
            smallvec![4163638954, 563173765], -172, div_pow36_assign ;
        );
    }
}
