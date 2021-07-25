//! Implement string conversion routines in a single trait.

/// Map partial result to complete result.
#[macro_export]
#[cfg(feature = "parse")]
macro_rules! lexical_partial_to_complete {
    ($cb:expr, $bytes:expr $(,$args:expr)*) => {
        match $cb($bytes $(,$args)*) {
            Err(e)                  => Err(e),
            Ok((value, processed))  => if processed == $bytes.len() {
                Ok(value)
            } else{
                Err((lexical_util::error::ErrorCode::InvalidDigit, processed).into())
            }
        }
    };
}

// FROM LEXICAL

// Define FromLexical trait.
// We use this since we can't define external traits for types
// defined outside the current crates.
#[macro_export]
#[cfg(feature = "parse")]
macro_rules! from_lexical_trait {
    () => {
        /// Trait for numerical types that can be parsed from bytes.
        pub trait FromLexical: lexical_util::num::Number {
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
            fn from_lexical(bytes: &[u8]) -> lexical_util::result::ParseResult<Self>;

            /// Checked parser for a string-to-number conversion.
            ///
            /// This method parses until an invalid digit is found (or the end
            /// of the string), returning the number of processed digits
            /// and the parsed value until that point.
            ///
            /// Returns a `ParseResult` containing either the parsed value
            /// and the number of processed digits, or an error containing
            /// any errors that occurred during parsing.
            ///
            /// * `bytes`   - Slice containing a numeric string.
            fn from_lexical_partial(
                bytes: &[u8],
            ) -> lexical_util::result::ParseResult<(Self, usize)>;
        }
    };
}

// TODO(ahuszagh) Need FromLexicalOptions

// TO LEXICAL

// Define ToLexical trait.
// We use this since we can't define external traits for types
// defined outside the current crates.
#[macro_export]
#[cfg(feature = "write")]
macro_rules! to_lexical {
    () => {
        /// Trait for numerical types that can be serialized to bytes.
        ///
        /// To determine the number of bytes required to serialize a value to
        /// string, check the associated constants from a required trait:
        /// - [`FORMATTED_SIZE`]
        /// - [`FORMATTED_SIZE_DECIMAL`]
        ///
        /// [`FORMATTED_SIZE`]: trait.FormattedSize.html#associatedconstant.FORMATTED_SIZE
        /// [`FORMATTED_SIZE_DECIMAL`]: trait.FormattedSize.html#associatedconstant.FORMATTED_SIZE_DECIMAL
        pub trait ToLexical:
            lexical_util::constants::FormattedSize + lexical_util::num::Number
        {
            /// Serializer for a number-to-string conversion.
            ///
            /// Returns a subslice of the input buffer containing the written bytes,
            /// starting from the same address in memory as the input slice.
            ///
            /// * `value`   - Number to serialize.
            /// * `bytes`   - Buffer to write number to.
            ///
            /// # Safety
            ///
            /// Safe as long as the caller has provided a buffer of at least
            /// [`FORMATTED_SIZE_DECIMAL`] elements. If a smaller buffer is
            /// provided, a buffer overflow is very likely.
            ///
            /// [`FORMATTED_SIZE_DECIMAL`]: trait.Number.html#associatedconstant.FORMATTED_SIZE_DECIMAL
            unsafe fn to_lexical_unchecked<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8];

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
    };
}

// Define ToLexicalWithOptions trait.
// We use this since we can't define external traits for types
// defined outside the current crates.
#[macro_export]
#[cfg(feature = "write")]
macro_rules! to_lexical_with_options {
    () => {
        /// Trait for numerical types that can be serialized to bytes.
        ///
        /// To determine the number of bytes required to serialize a value to
        /// string, check the associated constants from a required trait:
        /// - [`FORMATTED_SIZE`]
        /// - [`FORMATTED_SIZE_DECIMAL`]
        ///
        /// The `Options` type specifies the configurable options to provide.
        ///
        /// [`FORMATTED_SIZE`]: trait.FormattedSize.html#associatedconstant.FORMATTED_SIZE
        /// [`FORMATTED_SIZE_DECIMAL`]: trait.FormattedSize.html#associatedconstant.FORMATTED_SIZE_DECIMAL
        pub trait ToLexicalWithOptions:
            lexical_util::constants::FormattedSize + lexical_util::num::Number
        {
            /// Custom formatting options for writing a number.
            type Options: Default;

            /// Serializer for a number-to-string conversion.
            ///
            /// Returns a subslice of the input buffer containing the written bytes,
            /// starting from the same address in memory as the input slice.
            ///
            /// * `RADIX`   - The radix to serialize the string as.
            /// * `value`   - Number to serialize.
            /// * `bytes`   - Buffer to write number to.
            /// * `options` - Options for number formatting.
            ///
            /// # Safety
            ///
            /// Safe as long as the caller has provided a buffer of at least
            /// [`FORMATTED_SIZE`] elements. If a smaller buffer is
            /// provided, a buffer overflow is very likely.
            ///
            /// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
            unsafe fn to_lexical_with_options_unchecked<'a, const RADIX: u32>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8];

            /// Serializer for a number-to-string conversion.
            ///
            /// Returns a subslice of the input buffer containing the written bytes,
            /// starting from the same address in memory as the input slice.
            ///
            /// * `RADIX`   - The radix to serialize the string as.
            /// * `value`   - Number to serialize.
            /// * `bytes`   - Buffer to write number to.
            /// * `options` - Options for number formatting.
            ///
            /// # Panics
            ///
            /// Panics if the buffer is not of sufficient size. The caller
            /// must provide a slice of sufficient size. In order to ensure
            /// the function will not panic, ensure the buffer has at least
            /// [`FORMATTED_SIZE`] elements.
            ///
            /// [`FORMATTED_SIZE`]: trait.Number.html#associatedconstant.FORMATTED_SIZE
            fn to_lexical_with_options<'a, const RADIX: u32>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8];
        }
    };
}
