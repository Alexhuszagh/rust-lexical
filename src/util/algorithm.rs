//! Simple, shared algorithm utilities.

// ALGORITHMS

/// Reverse a range of pointers.
#[inline(always)]
#[allow(dead_code)]
pub unsafe extern "C" fn reverse(first: *mut u8, last: *mut u8) {
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
pub unsafe extern "C" fn equal_to(l: *const u8, r: *const u8, n: usize)
    -> bool
{
    memcmp(l, r, n) == 0
}

/// Check if left range starts with right range.
#[inline(always)]
pub unsafe extern "C" fn starts_with(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to(l, r, rn)
}

/// Check if left range ends with right range.
#[inline(always)]
#[allow(dead_code)]
pub unsafe extern "C" fn ends_with(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to(l.add(ln - rn), r, rn)
}

/// Trim character from the left-side of a range.
///
/// Returns a pointer to the new start of the range.
#[inline(always)]
pub unsafe extern "C" fn ltrim_char(mut first: *const u8, last: *const u8, char: u8)
    -> *const u8
{
    while first < last && *first == char {
        first = first.add(1);
    }
    first
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_test() {
        unsafe {
            let mut x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let y: [u8; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
            let first: *mut u8 = x.as_mut_ptr();
            let last = first.add(x.len());
            reverse(first, last);
            assert_eq!(x, y);
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
        unsafe {
            let x = "Hello";
            let y = "Hello";
            let z = "hello";
            assert!(equal_to(x.as_ptr(), y.as_ptr(), x.len()));
            assert!(!equal_to(x.as_ptr(), z.as_ptr(), x.len()));
            assert!(!equal_to(y.as_ptr(), z.as_ptr(), x.len()));
        }
    }

    #[test]
    fn starts_with_test() {
        unsafe {
            let x = "Hello";
            let y = "H";
            let z = "h";

            // forward
            assert!(starts_with(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!starts_with(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(!starts_with(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!starts_with(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!starts_with(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
        }
    }

    #[test]
    fn ends_with_test() {
        unsafe {
            let w = "Hello";
            let x = "lO";
            let y = "lo";
            let z = "o";

            // forward
            assert!(!ends_with(w.as_ptr(), w.len(), x.as_ptr(), x.len()));
            assert!(ends_with(w.as_ptr(), w.len(), y.as_ptr(), y.len()));
            assert!(ends_with(w.as_ptr(), w.len(), z.as_ptr(), z.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(ends_with(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!ends_with(z.as_ptr(), z.len(), y.as_ptr(), y.len()));
            assert!(!ends_with(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
            assert!(!ends_with(z.as_ptr(), z.len(), w.as_ptr(), w.len()));
            assert!(!ends_with(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!ends_with(y.as_ptr(), y.len(), w.as_ptr(), w.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), w.as_ptr(), w.len()));
        }
    }

    #[test]
    fn ltrim_char_test() {
        unsafe {
            let w = "0001";
            let x = "1010";
            let y = "1.00";
            let z = "1e05";

            let ltrim_char_wrapper = |w: &str, c: u8| {
                let first = w.as_ptr();
                let last = first.add(w.len());
                distance(first, ltrim_char(first, last, c))
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
