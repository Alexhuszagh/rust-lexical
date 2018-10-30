//! Helper utilities for extern-ed C code,

/// Reverse a range of pointers.
#[inline(always)]
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
}
