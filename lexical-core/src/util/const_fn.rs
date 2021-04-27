//! Utilities to use const fn only when supported.
//!
//! This will be removed when we drop support for Rustc < 1.46.0

/// Use our const fn as we normally would.
#[cfg(any(has_const_if, has_const_match))]
macro_rules! const_fn {
    (
        $(#[$meta:meta])*
        $vis:vis const fn $($tt:tt)*
    ) => (
        $(#[$meta])*
        $vis const fn $($tt)*
    );
}

/// Do not use our const fns, since they are not supported.
#[cfg(not(any(has_const_if, has_const_match)))]
macro_rules! const_fn {
    (
        $(#[$meta:meta])*
        $vis:vis const fn $($tt:tt)*
    ) => (
        $(#[$meta])*
        $vis fn $($tt)*
    );
}
