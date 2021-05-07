//! Shared operations for arbitrary-precision integers.

use crate::lib::cmp;
use crate::util::config::*;
use crate::util::traits::*;

use super::from_uint::*;
use super::hi::*;
use super::large;
use super::small;

// SHARED OPS
// ----------

/// Traits for shared operations for big integers.
///
/// None of these are implemented using normal traits, since these
/// are very expensive operations, and we want to deliberately
/// and explicitly use these functions.
#[allow(dead_code)]
pub(crate) trait SharedOps: Clone + Sized + Default {
    /// Underlying storage type for a SmallOps.
    type StorageType: CloneableVecLike<Limb>;

    // DATA

    /// Get access to the underlying data
    fn data<'a>(&'a self) -> &'a Self::StorageType;

    /// Get access to the underlying data
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType;

    // ZERO

    /// Check if the value is a normalized 0.
    #[inline]
    fn is_zero(&self) -> bool {
        self.limb_length() == 0
    }

    // RELATIVE OPERATIONS

    /// Compare self to y.
    #[inline]
    fn compare(&self, y: &Self) -> cmp::Ordering {
        large::compare(self.data(), y.data())
    }

    /// Check if self is greater than y.
    #[inline]
    fn greater(&self, y: &Self) -> bool {
        large::greater(self.data(), y.data())
    }

    /// Check if self is greater than or equal to y.
    #[inline]
    fn greater_equal(&self, y: &Self) -> bool {
        large::greater_equal(self.data(), y.data())
    }

    /// Check if self is less than y.
    #[inline]
    fn less(&self, y: &Self) -> bool {
        large::less(self.data(), y.data())
    }

    /// Check if self is less than or equal to y.
    #[inline]
    fn less_equal(&self, y: &Self) -> bool {
        large::less_equal(self.data(), y.data())
    }

    /// Check if self is equal to y.
    #[inline]
    fn equal(&self, y: &Self) -> bool {
        large::equal(self.data(), y.data())
    }

    // PROPERTIES

    /// Get the number of leading zero digits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn leading_zero_limbs(&self) -> usize {
        small::leading_zero_limbs(self.data())
    }

    /// Get the number of trailing zero digits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn trailing_zero_limbs(&self) -> usize {
        small::trailing_zero_limbs(self.data())
    }

    /// Get number of leading zero bits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn leading_zeros(&self) -> usize {
        small::leading_zeros(self.data())
    }

    /// Get number of trailing zero bits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn trailing_zeros(&self) -> usize {
        small::trailing_zeros(self.data())
    }

    /// Calculate the bit-length of the big-integer.
    /// Returns usize::max_value() if the value overflows,
    /// IE, if `self.data().len() > usize::max_value() / 8`.
    #[inline]
    fn bit_length(&self) -> usize {
        small::bit_length(self.data())
    }

    /// Calculate the digit-length of the big-integer.
    #[inline]
    fn limb_length(&self) -> usize {
        small::limb_length(self.data())
    }

    /// Get the high 16-bits from the bigint and if there are remaining bits.
    #[inline]
    fn hi16(&self) -> (u16, bool) {
        self.data().as_slice().hi16()
    }

    /// Get the high 32-bits from the bigint and if there are remaining bits.
    #[inline]
    fn hi32(&self) -> (u32, bool) {
        self.data().as_slice().hi32()
    }

    /// Get the high 64-bits from the bigint and if there are remaining bits.
    #[inline]
    fn hi64(&self) -> (u64, bool) {
        self.data().as_slice().hi64()
    }

    /// Get the high 128-bits from the bigint and if there are remaining bits.
    #[inline]
    fn hi128(&self) -> (u128, bool) {
        self.data().as_slice().hi128()
    }

    /// Pad the buffer with zeros to the least-significant bits.
    #[inline]
    fn pad_zero_digits(&mut self, n: usize) -> usize {
        small::ishl_limbs(self.data_mut(), n);
        n
    }

    // INTEGER CONVERSIONS

    // CREATION

    /// Create new big integer from u16.
    #[inline(always)]
    fn from_u16(x: u16) -> Self {
        x.from_uint()
    }

    /// Create new big integer from u32.
    #[inline(always)]
    fn from_u32(x: u32) -> Self {
        x.from_uint()
    }

    /// Create new big integer from u64.
    #[inline(always)]
    fn from_u64(x: u64) -> Self {
        x.from_uint()
    }

    /// Create new big integer from u128.
    #[inline(always)]
    fn from_u128(x: u128) -> Self {
        x.from_uint()
    }

    /// Create new big integer from generic integer type.
    #[inline(always)]
    fn from_uint<T: FromUint>(x: T) -> Self {
        x.from_uint()
    }

    // NORMALIZE

    /// Normalize the integer, so any leading zero values are removed.
    #[inline]
    fn normalize(&mut self) {
        small::normalize(self.data_mut());
    }

    /// Get if the big integer is normalized.
    #[inline]
    fn is_normalized(&self) -> bool {
        self.data().is_empty() || !self.data().rindex(0).is_zero()
    }

    // SHIFTS

    /// Shift-left the entire buffer n bits, where bits is less than the limb size.
    #[inline]
    fn ishl_bits(&mut self, n: usize) {
        small::ishl_bits(self.data_mut(), n);
    }

    /// Shift-left the entire buffer n bits, where bits is less than the limb size.
    #[inline]
    fn shl_bits(&self, n: usize) -> Self {
        let mut x = self.clone();
        x.ishl_bits(n);
        x
    }

    /// Shift-left the entire buffer n bits.
    #[inline]
    fn ishl(&mut self, n: usize) {
        small::ishl(self.data_mut(), n);
    }

    /// Shift-left the entire buffer n bits.
    #[inline]
    fn shl(&self, n: usize) -> Self {
        let mut x = self.clone();
        x.ishl(n);
        x
    }

    /// Shift-right the entire buffer n bits.
    #[inline]
    fn ishr(&mut self, n: usize, mut roundup: bool) {
        roundup &= small::ishr(self.data_mut(), n);

        // Round-up the least significant bit.
        if roundup {
            if self.data().is_empty() {
                self.data_mut().push(1);
            } else {
                self.data_mut()[0] += 1;
            }
        }
    }

    /// Shift-right the entire buffer n bits.
    #[inline]
    fn shr(&self, n: usize, roundup: bool) -> Self {
        let mut x = self.clone();
        x.ishr(n, roundup);
        x
    }
}
