//! Simple, shared algorithm utilities.

use lib::convert::AsRef;
use lib::{mem, ptr, slice};

// ALGORITHMS

/// Calculate the difference between two pointers.
#[inline(always)]
pub fn distance<T>(first: *const T, last: *const T)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

/// Check if two slices are equal to each other.
#[inline(always)]
pub fn equal_to_slice(l: &[u8], r: &[u8])
    -> bool
{
    l == r
}

/// Check if two slices are equal to each other without case-sensitivity.
#[inline(always)]
pub fn case_insensitive_equal_to_slice(l: &[u8], r: &[u8])
    -> bool
{
    let liter = l.iter().map(|li| li.to_ascii_lowercase());
    let riter = r.iter().map(|ri| ri.to_ascii_lowercase());
    l.len() == r.len() && liter.eq(riter)
}

/// Check if left slice starts with right slice without case-sensitivity.
#[inline(always)]
pub fn case_insensitive_starts_with_slice(l: &[u8], r: &[u8])
    -> bool
{
    l.len() >= r.len() && case_insensitive_equal_to_slice(&l[..r.len()], r)
}

/// Check if left slice ends with right slice.
#[cfg(feature = "trim_floats")]
#[inline(always)]
pub fn ends_with_slice(l: &[u8], r: &[u8])
    -> bool
{
    l.len() >= r.len() && equal_to_slice(&l[l.len()-r.len()..], r)
}

/// Trim character from the left-side of a slice.
#[inline(always)]
pub fn ltrim_char_slice<'a>(slc: &'a [u8], c: u8)
    -> &'a [u8]
{
    let count = slc.iter().take_while(|&&si| si == c).count();
    &slc[count..]
}

/// Trim character from the right-side of a slice.
#[cfg(any(feature = "correct", feature = "radix"))]
#[inline]
pub fn rtrim_char_slice<'a>(slc: &'a [u8], c: u8)
    -> &'a [u8]
{
    let count = slc.iter().rev().take_while(|&&si| si == c).count();
    &slc[..slc.len()-count]
}

/// Copy from source-to-dst.
#[inline]
pub fn copy_to_dst<'a, Bytes: AsRef<[u8]>>(dst: &'a mut [u8], src: Bytes)
    -> &'a mut [u8]
{
    let src = src.as_ref();
    {
        let dst = &mut dst[..src.len()];
        dst.copy_from_slice(src);
    }
    &mut dst[src.len()..]
}

/// Length-check variant of ptr::write_bytes for a slice.
#[cfg(not(any(feature = "grisu3", feature = "ryu")))]
#[inline]
pub fn write_bytes(dst: &mut [u8], byte: u8)
{
    unsafe {
        ptr::write_bytes(dst.as_mut_ptr(), byte, dst.len());
    }
}

/// Explicitly uninitialize, a wrapper for mem::uninitialize without unsafe.
/// This is mostly to clean up internal code, but should not be used lightly.
#[inline]
pub fn explicit_uninitialized<T>() -> T {
    unsafe {
        mem::uninitialized()
    }
}

/// Create slice from pointer range.
#[cfg(feature = "correct")]
#[inline]
pub fn slice_from_range<'a, T>(first: *const T, last: *const T)
    -> &'a [T]
{
    slice_from_span(first, distance(first, last))
}

/// Create slice from pointer and size.
#[cfg(feature = "correct")]
#[inline]
pub fn slice_from_span<'a, T>(first: *const T, length: usize)
    -> &'a [T]
{
    unsafe {
        slice::from_raw_parts(first, length)
    }
}

/// Create mutable slice from pointer and size.
#[cfg(feature = "trim_floats")]
#[inline]
pub fn slice_from_span_mut<'a, T>(first: *mut T, length: usize)
    -> &'a mut [T]
{
    unsafe {
        slice::from_raw_parts_mut(first, length)
    }
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_test() {
        unsafe {
            let x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let first: *const u8 = x.as_ptr();
            let last = first.add(x.len());
            assert_eq!(distance(first, last), 10);
        }
    }

    #[test]
    fn equal_to_test() {
        let x = "Hello";
        let y = "Hello";
        let z = "hello";

        assert!(equal_to_slice(x.as_bytes(), y.as_bytes()));
        assert!(!equal_to_slice(x.as_bytes(), z.as_bytes()));
        assert!(!equal_to_slice(y.as_bytes(), z.as_bytes()));
    }

    #[test]
    fn case_insensitive_equal_to_test() {
        let w = "Hello";
        let x = "Hello";
        let y = "hello";
        let z = "Strongbad";

        assert!(case_insensitive_equal_to_slice(w.as_bytes(), x.as_bytes()));
        assert!(case_insensitive_equal_to_slice(w.as_bytes(), y.as_bytes()));
        assert!(case_insensitive_equal_to_slice(x.as_bytes(), y.as_bytes()));
        assert!(!case_insensitive_equal_to_slice(w.as_bytes(), z.as_bytes()));
        assert!(!case_insensitive_equal_to_slice(x.as_bytes(), z.as_bytes()));
        assert!(!case_insensitive_equal_to_slice(y.as_bytes(), z.as_bytes()));
    }

    #[test]
    fn case_insensitive_starts_with_test() {
        let w = "Hello";
        let x = "H";
        let y = "h";
        let z = "a";

        // forward
        assert!(case_insensitive_starts_with_slice(w.as_bytes(), x.as_bytes()));
        assert!(case_insensitive_starts_with_slice(w.as_bytes(), y.as_bytes()));
        assert!(case_insensitive_starts_with_slice(x.as_bytes(), y.as_bytes()));
        assert!(!case_insensitive_starts_with_slice(w.as_bytes(), z.as_bytes()));
        assert!(!case_insensitive_starts_with_slice(x.as_bytes(), z.as_bytes()));
        assert!(!case_insensitive_starts_with_slice(y.as_bytes(), z.as_bytes()));

        // back
        assert!(!case_insensitive_starts_with_slice(x.as_bytes(), w.as_bytes()));
        assert!(!case_insensitive_starts_with_slice(y.as_bytes(), w.as_bytes()));
        assert!(!case_insensitive_starts_with_slice(z.as_bytes(), w.as_bytes()));
    }

    #[cfg(feature = "trim_floats")]
    #[test]
    fn ends_with_test() {
        let w = "Hello";
        let x = "lO";
        let y = "lo";
        let z = "o";

        // forward
        assert!(!ends_with_slice(w.as_bytes(), x.as_bytes()));
        assert!(ends_with_slice(w.as_bytes(), y.as_bytes()));
        assert!(ends_with_slice(w.as_bytes(), z.as_bytes()));
        assert!(!ends_with_slice(x.as_bytes(), y.as_bytes()));
        assert!(!ends_with_slice(x.as_bytes(), z.as_bytes()));
        assert!(ends_with_slice(y.as_bytes(), z.as_bytes()));

        // back
        assert!(!ends_with_slice(z.as_bytes(), y.as_bytes()));
        assert!(!ends_with_slice(z.as_bytes(), x.as_bytes()));
        assert!(!ends_with_slice(z.as_bytes(), w.as_bytes()));
        assert!(!ends_with_slice(y.as_bytes(), x.as_bytes()));
        assert!(!ends_with_slice(y.as_bytes(), w.as_bytes()));
        assert!(!ends_with_slice(x.as_bytes(), w.as_bytes()));
    }

    #[test]
    fn ltrim_char_test() {
        let w = "0001";
        let x = "1010";
        let y = "1.00";
        let z = "1e05";

        assert_eq!(ltrim_char_slice(w.as_bytes(), b'0').len(), 1);
        assert_eq!(ltrim_char_slice(x.as_bytes(), b'0').len(), 4);
        assert_eq!(ltrim_char_slice(x.as_bytes(), b'1').len(), 3);
        assert_eq!(ltrim_char_slice(y.as_bytes(), b'0').len(), 4);
        assert_eq!(ltrim_char_slice(y.as_bytes(), b'1').len(), 3);
        assert_eq!(ltrim_char_slice(z.as_bytes(), b'0').len(), 4);
        assert_eq!(ltrim_char_slice(z.as_bytes(), b'1').len(), 3);
    }

    #[cfg(any(feature = "correct", feature = "radix"))]
    #[test]
    fn rtrim_char_test() {
        let w = "0001";
        let x = "1010";
        let y = "1.00";
        let z = "1e05";

        assert_eq!(rtrim_char_slice(w.as_bytes(), b'0').len(), 4);
        assert_eq!(rtrim_char_slice(x.as_bytes(), b'0').len(), 3);
        assert_eq!(rtrim_char_slice(x.as_bytes(), b'1').len(), 4);
        assert_eq!(rtrim_char_slice(y.as_bytes(), b'0').len(), 2);
        assert_eq!(rtrim_char_slice(y.as_bytes(), b'1').len(), 4);
        assert_eq!(rtrim_char_slice(z.as_bytes(), b'0').len(), 4);
        assert_eq!(rtrim_char_slice(z.as_bytes(), b'5').len(), 3);
    }
}
