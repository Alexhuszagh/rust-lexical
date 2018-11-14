//! Utilities to generate the low-level API.

// TODO(ahuszagh) Rename this crap...
// Make sure these are mostly functions!!!!

// LOW LEVEL WRAPPERS

/// Generate the low-level bytes API.
///
/// Wraps unsafe functions to generate the low-level, unchecked, bytes parsers.
#[doc(hidden)]
macro_rules! bytes_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level bytes to number parser.
        #[inline]
        pub fn $func(bytes: &[u8], base: u8)
            -> $t
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                let (value, _, _) = $callback(first, last, base);
                value
            }
        }
    )
}

/// Error-checking version of `bytes_impl`.
///
/// Wraps unsafe functions to generate the low-level, checked, bytes parsers.
// TODO(ahuszagh) Should be a function or some shit... Make better...
//      We should have a generic type was can use as the base...
#[doc(hidden)]
macro_rules! try_bytes_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level bytes to number parser.
        /// On error, returns position of invalid char.
        #[inline]
        pub fn $func(bytes: &[u8], base: u8)
            -> Result<$t, $crate::Error>
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                let (value, p, overflow) = $callback(first, last, base);
                if overflow {
                    Err(From::from($crate::ErrorKind::Overflow))
                } else if p == last {
                    Ok(value)
                } else {
                    let dist = if p == ptr::null() { 0 } else { distance(first, p) };
                    Err(From::from($crate::ErrorKind::InvalidDigit(dist)))
                }
            }
        }
    )
}

/// Generate the low-level string API using wrappers around the unsafe function.
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! string_impl {
    ($func:ident, $t:ty, $callback:ident, $capacity:expr) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $func(value: $t, base: u8)
            -> lib::String
        {
            let mut string = lib::String::with_capacity($capacity);
            unsafe {
                let buf = string.as_mut_vec();
                let first: *mut u8 = buf.as_mut_ptr();
                let last = first.add(buf.capacity());
                let end = $callback(value, first, last, base);
                let size = distance(first, end);
                buf.set_len(size);

            }
            string
        }
    )
}
