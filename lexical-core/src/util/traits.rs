//! Wrap the low-level API into idiomatic serializers.

use super::format::NumberFormat;
use super::options::ParseFloatOptions;
use super::num::Number;
use super::result::Result;

// HELPERS

/// Map partial result to complete result.
macro_rules! to_complete {
    ($cb:expr, $bytes:expr $(,$args:expr)*) => {
        match $cb($bytes $(,$args)*) {
            Err(e)                  => Err(e),
            Ok((value, processed))  => if processed == $bytes.len() {
                Ok(value)
            } else{
                Err((ErrorCode::InvalidDigit, processed).into())
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
macro_rules! from_lexical {
    ($cb:expr, $t:ty) => (
        impl FromLexical for $t {
            fn from_lexical(bytes: &[u8]) -> Result<$t>
            {
                to_complete!($cb, bytes)
            }

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
macro_rules! from_lexical_with_options {
    ($cb:expr, $t:ty) => (
        impl FromLexicalOptions for $t {
            fn from_lexical_with_options(bytes: &[u8], options: &Self::ParseOptions)
                -> Result<Self>
            {
                to_complete!($cb, bytes, options)
            }

            fn from_lexical_partial_with_options(bytes: &[u8], options: &Self::ParseOptions)
                -> Result<($t, usize)>
            {
                $cb(bytes, options)
            }
        }
    )
}

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
    /// * `bytes`   - Slice containing a numeric string.
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

    /// Serializer for a number-to-string conversion.
    ///
    /// Returns a subslice of the input buffer containing the written bytes,
    /// starting from the same address in memory as the input slice.
    ///
    /// * `value`   - Number to serialize.
    /// * `radix`   - Radix for number encoding.
    /// * `bytes`   - Slice containing a numeric string.
    ///
    /// # Panics
    ///
    /// Panics if the radix is not in the range `[2, 36]`.
    ///
    /// Also panics if the buffer is not of sufficient size. The caller
    /// must provide a slice of sufficient size. In order to ensure
    /// the function will not panic, ensure the buffer has at least
    /// [`FORMATTED_SIZE`] elements.
    ///
    /// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
    #[cfg(feature = "radix")]
    fn to_lexical_radix<'a>(self, radix: u8, bytes: &'a mut [u8]) -> &'a mut [u8];
}

// Implement ToLexical for numeric type.
macro_rules! to_lexical {
    ($cb:expr, $t:ty) => (
        impl ToLexical for $t {
            // TODO(ahuszagh) Need to have a type that accepts a format.
            fn to_lexical<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                assert_buffer!(10, bytes, $t);
                let len = $cb(self, 10, bytes);
                &mut bytes[..len]
            }

            // TODO(ahuszagh) Deprecate this.
            #[cfg(feature = "radix")]
            fn to_lexical_radix<'a>(self, radix: u8, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                assert_radix!(radix);
                assert_buffer!(radix, bytes, $t);
                let len = $cb(self, radix.as_u32(), bytes);
                &mut bytes[..len]
            }
        }
    )
}

// TO LEXICAL WITH OPTIONS

/// Trait for numerical types that can be serialized to bytes with custom options.
pub trait ToLexicalOptions: ToLexical {
    // TODO(ahuszagh) Implement...
}

// TODO(ahuszagh) Add impl trait
