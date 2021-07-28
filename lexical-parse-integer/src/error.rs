//! Utilities to simplify returning errors.

/// Simple short-circuit to an error.
macro_rules! into_error {
    ($code:ident, $iter:ident $(- $shift:expr)?) => {
        Err((lexical_util::error::ErrorCode::$code, $iter.cursor() $(- $shift)?).into())
    };
}
