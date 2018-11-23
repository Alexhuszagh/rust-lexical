//! Simple, shared algorithm utilities.

use lib::slice;

// ALGORITHMS

/// Reverse a range of pointers.
#[inline(always)]
#[allow(dead_code)]
pub unsafe extern "C" fn reverse_range(first: *mut u8, last: *mut u8) {
    let mut f = first;
    let mut l = last;
    let mut x: u8;
    let mut li = l.sub(1);

    while f != l && f != li {
        l = li;
        x = *f;
        *f = *l;
        *l = x;
        li = l.sub(1);
        f = f.add(1);
    }
}

/// Reverse slice of bytes.
#[inline(always)]
#[allow(dead_code)]
pub fn reverse_slice<'a>(slc: &'a mut [u8])
{
    unsafe {
        let first = slc.as_mut_ptr();
        let last = first.add(slc.len());
        reverse_range(first, last);
    }
}

/// Calculate the difference between two pointers.
#[inline(always)]
pub unsafe extern "C" fn distance(first: *const u8, last: *const u8)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

extern {
    /// Need memcmp for efficient range comparisons.
    fn memcmp(l: *const u8, r: *const u8, n: usize) -> i32;
}

/// Check if two ranges are equal to each other.
#[inline(always)]
#[allow(dead_code)]
pub unsafe extern "C" fn equal_to_range(l: *const u8, r: *const u8, n: usize)
    -> bool
{
    memcmp(l, r, n) == 0
}

/// Check if two slices are equal to each other.
#[inline(always)]
#[allow(dead_code)]
pub fn equal_to_slice<'a>(l: &'a [u8], r: &'a [u8])
    -> bool
{
    unsafe {
        l.len() == r.len() && equal_to_range(l.as_ptr(), r.as_ptr(), l.len())
    }
}

/// Check if left range starts with right range.
#[inline(always)]
#[allow(dead_code)]
pub unsafe extern "C" fn starts_with_range(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to_range(l, r, rn)
}

/// Check if left slice starts with right slice.
#[inline(always)]
#[allow(dead_code)]
pub fn starts_with_slice<'a>(l: &'a [u8], r: &'a [u8])
    -> bool
{
    unsafe {
        starts_with_range(l.as_ptr(), l.len(), r.as_ptr(), r.len())
    }
}

/// Check if left range ends with right range.
#[inline(always)]
#[allow(dead_code)]
pub unsafe extern "C" fn ends_with_range(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to_range(l.add(ln - rn), r, rn)
}

/// Check if left slice ends with right slice.
#[inline(always)]
#[allow(dead_code)]
pub fn ends_with_slice<'a>(l: &'a [u8], r: &'a [u8])
    -> bool
{
    unsafe {
        ends_with_range(l.as_ptr(), l.len(), r.as_ptr(), r.len())
    }
}

/// Trim character from the left-side of a range.
///
/// Returns a pointer to the new start of the range.
#[inline(always)]
pub unsafe extern "C" fn ltrim_char_range(mut first: *const u8, last: *const u8, c: u8)
    -> *const u8
{
    while first < last && *first == c {
        first = first.add(1);
    }
    first
}

/// Trim character from the left-side of a slice.
#[inline(always)]
#[allow(dead_code)]
pub fn ltrim_char_slice<'a>(slc: &'a [u8], c: u8)
    -> &'a [u8]
{
    unsafe {
        let first = slc.as_ptr();
        let last = first.add(slc.len());
        let first = ltrim_char_range(first, last, c);
        slice::from_raw_parts(first, distance(first, last))
    }
}

// TODO(ahuszagh) Wrap these into slice versions???

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_test() {
        let input: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let reversed: [u8; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        let mut x = input;
        reverse_slice(&mut x);
        assert_eq!(x, reversed);

        unsafe {
            let mut x = input;
            let first: *mut u8 = x.as_mut_ptr();
            let last = first.add(x.len());
            reverse_range(first, last);
            assert_eq!(x, reversed);
        }
    }

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

        unsafe {

            assert!(equal_to_range(x.as_ptr(), y.as_ptr(), x.len()));
            assert!(!equal_to_range(x.as_ptr(), z.as_ptr(), x.len()));
            assert!(!equal_to_range(y.as_ptr(), z.as_ptr(), x.len()));
        }
    }

    #[test]
    fn starts_with_test() {
        let x = "Hello";
        let y = "H";
        let z = "h";

        // forward
        assert!(starts_with_slice(x.as_bytes(), y.as_bytes()));
        assert!(!starts_with_slice(x.as_bytes(), z.as_bytes()));
        assert!(!starts_with_slice(y.as_bytes(), z.as_bytes()));

        // back
        assert!(!starts_with_slice(y.as_bytes(), x.as_bytes()));
        assert!(!starts_with_slice(z.as_bytes(), x.as_bytes()));

        unsafe {
            // forward
            assert!(starts_with_range(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!starts_with_range(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(!starts_with_range(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!starts_with_range(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!starts_with_range(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
        }
    }

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

        unsafe {
            // forward
            assert!(!ends_with_range(w.as_ptr(), w.len(), x.as_ptr(), x.len()));
            assert!(ends_with_range(w.as_ptr(), w.len(), y.as_ptr(), y.len()));
            assert!(ends_with_range(w.as_ptr(), w.len(), z.as_ptr(), z.len()));
            assert!(!ends_with_range(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!ends_with_range(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(ends_with_range(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!ends_with_range(z.as_ptr(), z.len(), y.as_ptr(), y.len()));
            assert!(!ends_with_range(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
            assert!(!ends_with_range(z.as_ptr(), z.len(), w.as_ptr(), w.len()));
            assert!(!ends_with_range(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!ends_with_range(y.as_ptr(), y.len(), w.as_ptr(), w.len()));
            assert!(!ends_with_range(x.as_ptr(), x.len(), w.as_ptr(), w.len()));
        }
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

        unsafe {
            let ltrim_char_wrapper = |w: &str, c: u8| {
                let first = w.as_ptr();
                let last = first.add(w.len());
                distance(first, ltrim_char_range(first, last, c))
            };

            assert_eq!(ltrim_char_wrapper(w, b'0'), 3);
            assert_eq!(ltrim_char_wrapper(x, b'0'), 0);
            assert_eq!(ltrim_char_wrapper(x, b'1'), 1);
            assert_eq!(ltrim_char_wrapper(y, b'0'), 0);
            assert_eq!(ltrim_char_wrapper(y, b'1'), 1);
            assert_eq!(ltrim_char_wrapper(z, b'0'), 0);
            assert_eq!(ltrim_char_wrapper(z, b'1'), 1);
        }
    }
}
