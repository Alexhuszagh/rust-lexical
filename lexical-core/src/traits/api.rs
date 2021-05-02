//! Wrap the low-level API into idiomatic serializers.
//!
//! These traits define the high-level API for lexical-core:
//! in order to use lexical in generic code, you must use the
//! type-bounds defined in this module.

#[cfg(any(
    feature = "atof",
    feature = "atoi",
    feature = "ftoa",
    feature = "itoa"
))]
use super::num::Number;

cfg_if! {
if #[cfg(any(feature = "atof", feature = "atoi"))] {
use crate::result::Result;

// HELPERS

/// Map partial result to complete result.
macro_rules! to_complete {
    ($cb:expr, $bytes:expr $(,$args:expr)*) => {
        match $cb($bytes $(,$args)*) {
            Err(e)                  => Err(e),
            Ok((value, processed))  => if processed == $bytes.len() {
                Ok(value)
            } else{
                Err((crate::ErrorCode::InvalidDigit, processed).into())
            }
        }
    };
}

// FROM LEXICAL

/// Trait for numerical types that can be parsed from bytes.
pub trait FromLexical: Number {
    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit, and therefore may mask an invalid digit error.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    fn from_lexical(bytes: &[u8]) -> Result<Self>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    fn from_lexical_partial(bytes: &[u8]) -> Result<(Self, usize)>;
}

// Implement FromLexical for numeric type.
#[doc(hidden)]
#[macro_export]
macro_rules! from_lexical {
    ($cb:expr, $t:ty $(, #[$meta:meta])?) => (
        impl FromLexical for $t {
            $(#[$meta:meta])?
            fn from_lexical(bytes: &[u8]) -> Result<$t>
            {
                to_complete!($cb, bytes)
            }

            $(#[$meta:meta])?
            fn from_lexical_partial(bytes: &[u8]) -> Result<($t, usize)>
            {
                $cb(bytes)
            }
        }
    )
}

// FROM LEXICAL WITH OPTIONS

/// Trait for number that can be parsed using a custom options specification.
pub trait FromLexicalOptions: FromLexical {
    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses the entire string, returning an error if
    /// any invalid digits are found during parsing. The parsing
    /// is dictated by the options, which specifies special
    /// float strings, required float components, digit separators,
    /// exponent characters, and more.
    ///
    /// Returns a `Result` containing either the parsed value,
    /// or an error containing any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `options` - Options to dictate number parsing.
    fn from_lexical_with_options(bytes: &[u8], options: &Self::ParseOptions) -> Result<Self>;

    /// Checked parser for a string-to-number conversion.
    ///
    /// This method parses until an invalid digit is found (or the end
    /// of the string), returning the number of processed digits
    /// and the parsed value until that point.
    ///
    /// Returns a `Result` containing either the parsed value
    /// and the number of processed digits, or an error containing
    /// any errors that occurred during parsing.
    ///
    /// * `bytes`   - Slice containing a numeric string.
    /// * `options` - Options to dictate number parsing.
    fn from_lexical_partial_with_options(bytes: &[u8], options: &Self::ParseOptions) -> Result<(Self, usize)>;
}

// Implement FromLexicalOptions for numeric type.
#[doc(hidden)]
#[macro_export]
macro_rules! from_lexical_with_options {
    ($cb:expr, $t:ty $(, #[$meta:meta])?) => (
        impl FromLexicalOptions for $t {
            $(#[$meta:meta])?
            fn from_lexical_with_options(bytes: &[u8], options: &Self::ParseOptions)
                -> Result<Self>
            {
                to_complete!($cb, bytes, options)
            }

            $(#[$meta:meta])?
            fn from_lexical_partial_with_options(bytes: &[u8], options: &Self::ParseOptions)
                -> Result<($t, usize)>
            {
                $cb(bytes, options)
            }
        }
    )
}
}}   // cfg_if

cfg_if! {
if #[cfg(any(feature = "ftoa", feature = "itoa"))] {
// TO LEXICAL

/// Trait for numerical types that can be serialized to bytes.
///
/// To determine the number of bytes required to serialize a value to
/// string, check the associated constants from a required trait:
/// - [`FORMATTED_SIZE`]
/// - [`FORMATTED_SIZE_DECIMAL`]
///
/// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
/// [`FORMATTED_SIZE_DECIMAL`]: trait.Number.html#associatedconstant.FORMATTED_SIZE_DECIMAL
pub trait ToLexical: Number {
    /// Serializer for a number-to-string conversion.
    ///
    /// Returns a subslice of the input buffer containing the written bytes,
    /// starting from the same address in memory as the input slice.
    ///
    /// * `value`   - Number to serialize.
    /// * `bytes`   - Buffer to write number to.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not of sufficient size. The caller
    /// must provide a slice of sufficient size. In order to ensure
    /// the function will not panic, ensure the buffer has at least
    /// [`FORMATTED_SIZE_DECIMAL`] elements.
    ///
    /// [`FORMATTED_SIZE_DECIMAL`]: trait.Number.html#associatedconstant.FORMATTED_SIZE_DECIMAL
    fn to_lexical<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8];
}

// Implement ToLexical for numeric type.
#[doc(hidden)]
#[macro_export]
macro_rules! to_lexical {
    ($cb:expr, $t:ty $(, #[$meta:meta])?) => (
        impl ToLexical for $t {
            $(#[$meta:meta])?
            fn to_lexical<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                assert_buffer!(10, bytes, $t);
                let len = $cb(self, 10, bytes);
                &mut bytes[..len]
            }
        }
    )
}

// TO LEXICAL WITH OPTIONS

/// Trait for numerical types that can be serialized to bytes with custom options.
pub trait ToLexicalOptions: ToLexical {
    /// Serializer for a number-to-string conversion.
    ///
    /// Returns a subslice of the input buffer containing the written bytes,
    /// starting from the same address in memory as the input slice.
    ///
    /// * `value`   - Number to serialize.
    /// * `options` - Options for number formatting.
    /// * `bytes`   - Buffer to write number to.
    ///
    /// # Panics
    ///
    /// Also panics if the buffer is not of sufficient size. The caller
    /// must provide a slice of sufficient size. In order to ensure
    /// the function will not panic, ensure the buffer has at least
    /// [`FORMATTED_SIZE`] elements.
    ///
    /// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
    fn to_lexical_with_options<'a>(self, bytes: &'a mut [u8], options: &Self::WriteOptions) -> &'a mut [u8];
}

// Implement ToLexicalOptions for numeric type.
#[doc(hidden)]
#[macro_export]
macro_rules! to_lexical_with_options {
    ($cb:expr, $t:ty $(, #[$meta:meta])?) => (
        impl ToLexicalOptions for $t {
            $(#[$meta:meta])?
            fn to_lexical_with_options<'a>(self, bytes: &'a mut [u8], options: &Self::WriteOptions)
                -> &'a mut [u8]
            {
                assert_buffer!(options.radix(), bytes, $t);
                let len = $cb(self, bytes, options);
                &mut bytes[..len]
            }
        }
    )
}
}}  // cfg_if
