//! Traits that provide format-dependent data for floating parsing algorithms.

use crate::util::*;
use super::result::*;

#[cfg(feature = "correct")]
use super::exponent::*;

/// Private data interface for local utilities.
pub(crate) trait FastDataInterfaceImpl<'a>: Sized {
    /// Get integer component of float.
    fn integer(&self) -> &'a [u8];

    /// Set integer component of float.
    fn set_integer(&mut self, integer: &'a [u8]);

    /// Get fraction component of float.
    fn fraction(&self) -> &'a [u8];

    /// Set fraction component of float.
    fn set_fraction(&mut self, fraction: &'a [u8]);

    /// Get exponent component of float.
    fn exponent(&self) -> &'a [u8];

    /// Set exponent component of float.
    fn set_exponent(&mut self, exponent: &'a [u8]);

    /// Get raw exponent component of float.
    fn raw_exponent(&self) -> i32;

    /// Set raw exponent component of float.
    fn set_raw_exponent(&mut self, raw_exponent: i32);
}

/// Private data interface for local utilities.
#[cfg(feature = "correct")]
pub(crate) trait SlowDataInterfaceImpl<'a>: Sized {
    /// Get integer component of float.
    fn integer(&self) -> &'a [u8];

    /// Set integer component of float.
    fn set_integer(&mut self, integer: &'a [u8]);

    /// Get fraction component of float.
    fn fraction(&self) -> &'a [u8];

    /// Set fraction component of float.
    fn set_fraction(&mut self, fraction: &'a [u8]);

    /// Get raw exponent component of float.
    fn raw_exponent(&self) -> i32;

    /// Set raw exponent component of float.
    fn set_raw_exponent(&mut self, raw_exponent: i32);
}

// Implement FastDataInterfaceImpl for a default structure.
macro_rules! fast_data_interface_impl {
    ($name:ident) => (
        impl<'a> FastDataInterfaceImpl<'a> for $name<'a> {
            perftools_inline!{
            fn integer(&self) -> &'a [u8] {
                self.integer
            }}

            perftools_inline!{
            fn set_integer(&mut self, integer: &'a [u8]) {
                self.integer = integer
            }}

            perftools_inline!{
            fn fraction(&self) -> &'a [u8] {
                self.fraction
            }}

            perftools_inline!{
            fn set_fraction(&mut self, fraction: &'a [u8]) {
                self.fraction = fraction
            }}

            perftools_inline!{
            fn exponent(&self) -> &'a [u8] {
                self.exponent
            }}

            perftools_inline!{
            fn set_exponent(&mut self, exponent: &'a [u8]) {
                self.exponent = exponent
            }}

            perftools_inline!{
            fn raw_exponent(&self) -> i32 {
                self.raw_exponent
            }}

            perftools_inline!{
            fn set_raw_exponent(&mut self, raw_exponent: i32) {
                self.raw_exponent = raw_exponent
            }}
        }
    );
}

// Implement SlowDataInterfaceImpl for a default structure.
#[cfg(feature = "correct")]
macro_rules! slow_data_interface_impl {
    ($name:ident) => (
        impl<'a> SlowDataInterfaceImpl<'a> for $name<'a> {
            perftools_inline!{
            fn integer(&self) -> &'a [u8] {
                self.integer
            }}

            perftools_inline!{
            fn set_integer(&mut self, integer: &'a [u8]) {
                self.integer = integer
            }}

            perftools_inline!{
            fn fraction(&self) -> &'a [u8] {
                self.fraction
            }}

            perftools_inline!{
            fn set_fraction(&mut self, fraction: &'a [u8]) {
                self.fraction = fraction
            }}

            perftools_inline!{
            fn raw_exponent(&self) -> i32 {
                self.raw_exponent
            }}

            perftools_inline!{
            fn set_raw_exponent(&mut self, raw_exponent: i32) {
                self.raw_exponent = raw_exponent
            }}
        }
    );
}

// PUBLIC

/// Data interface for fast float parsers.
pub(crate) trait FastDataInterface<'a>: FastDataInterfaceImpl<'a> {
    /// Integer digits iterator type.
    type IntegerIter: ConsumedIterator<Item=&'a u8> + AsPtrIterator<'a, u8>;

    /// Float digits iterator type.
    type FractionIter: ConsumedIterator<Item=&'a u8> + AsPtrIterator<'a, u8>;

    /// Associated slow data type.
    #[cfg(feature = "correct")]
    type SlowInterface: SlowDataInterface<'a>;

    /// Create new float data from format specification.
    fn new(format: u32) -> Self;

    // DATA

    /// Iterate over all integer digits.
    fn integer_iter(&self) -> Self::IntegerIter;

    /// Iterate over all fraction digits
    fn fraction_iter(&self) -> Self::FractionIter;

    /// Get the mantissa exponent from the raw exponent.
    perftools_inline!{
    #[cfg(feature = "correct")]
    fn mantissa_exponent(&self, truncated_digits: usize) -> i32 {
        mantissa_exponent(self.raw_exponent(), self.fraction_iter().count(), truncated_digits)
    }}

    // EXTRACT

    // Consume until a non-digit character is found.
    fn consume_digits(&self, bytes: &'a [u8], radix: u32) -> (&'a [u8], &'a [u8]);

    // Extract the integer substring from the float.
    perftools_inline!{
    fn extract_integer(&mut self, bytes: &'a [u8], radix: u32)
        -> &'a [u8]
    {
        let result = self.consume_digits(bytes, radix);
        self.set_integer(result.0);
        result.1
    }}

    // Extract the fraction substring from the float.
    //
    //  Preconditions:
    //      `bytes.len()` >= 1 and `bytes[0] == b'.'`.
    perftools_inline!{
    fn extract_fraction(&mut self, bytes: &'a [u8], radix: u32)
        -> &'a [u8]
    {
        let digits = &index!(bytes[1..]);
        let result = self.consume_digits(digits, radix);
        self.set_fraction(result.0);
        result.1
    }}

    // Extract and parse the exponent substring from the float.
    fn extract_exponent(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8];

    // Validate the extracted mantissa components.
    fn validate_mantissa(&self) -> ParseResult<()>;

    // Validate the extracted exponent component.
    fn validate_exponent(&self) -> ParseResult<()>;

    // Trim leading 0s and digit separators.
    fn ltrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Trim leading digit separators.
    fn ltrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Trim trailing 0s and digit separators.
    fn rtrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Trim trailing digit separators.
    fn rtrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize);

    // Post-process float to trim leading and trailing 0s and digit separators.
    // This is required for accurate results in the slow-path algorithm,
    // otherwise, we may incorrect guess the mantissa or scientific exponent.
    perftools_inline!{
    fn trim(&mut self) {
        self.set_integer(self.ltrim_zero(self.integer()).0);
        self.set_integer(self.rtrim_separator(self.integer()).0);
        self.set_fraction(self.rtrim_zero(self.fraction()).0);
    }}

    /// Extract float subcomponents from input bytes.
    perftools_inline!{
    fn extract(&mut self, bytes: &'a [u8], radix: u32) -> ParseResult<*const u8> {
        // Parse the integer, aka, the digits preceding any control characters.
        let mut digits = bytes;
        digits = self.extract_integer(digits, radix);

        // Parse and validate a fraction, if present.
        let exp_char = exponent_notation_char(radix).to_ascii_lowercase();
        if let Some(&b'.') = digits.first() {
            digits = self.extract_fraction(digits, radix);
        }
        self.validate_mantissa()?;

        // Parse and validate an exponent, if present.
        if let Some(&c) = digits.first() {
            if c.to_ascii_lowercase() == exp_char {
                digits = self.extract_exponent(digits, radix);
            }
        }
        self.validate_exponent()?;

        // Trim the remaining digits.
        self.trim();

        Ok(digits.as_ptr())
    }}

    // TO SLOW DATA

    // Calculate the digit start from the integer and fraction slices.
    perftools_inline!{
    #[cfg(feature = "correct")]
    fn digits_start(&self) -> usize {
        // Since integer is trimmed, any digit separators have been removed.
        // Just need to check if it's empty.
        match self.integer().is_empty() {
            true  => self.ltrim_zero(self.fraction()).1,
            false => 0,
        }
    }}

    /// Process float data for moderate/slow float parsers.
    #[cfg(feature = "correct")]
    fn to_slow(self, truncated_digits: usize) -> Self::SlowInterface;

    // TESTS

    #[cfg(test)]
    fn clear(&mut self) {
        self.set_integer(&[]);
        self.set_fraction(&[]);
        self.set_exponent(&[]);
        self.set_raw_exponent(0);
    }

    /// Check the float state parses the desired data.
    #[cfg(test)]
    fn check_extract(&mut self, digits: &'a [u8], expected: &TestResult<Self>) {
        let expected = expected.as_ref();
        match self.extract(digits, 10) {
            Ok(_)       => {
                let expected = expected.unwrap();
                assert_eq!(self.integer(), expected.integer());
                assert_eq!(self.fraction(), expected.fraction());
                assert_eq!(self.exponent(), expected.exponent());
            },
            Err((c, _))  => assert_eq!(c, *expected.err().unwrap()),
        }
    }

    // Run series of tests.
    #[cfg(test)]
    fn run_tests<Iter>(&mut self, tests: Iter)
        where Iter: Iterator<Item=&'a (&'a str, TestResult<Self>)>,
              Self: 'a
    {
        for value in tests {
            self.check_extract(value.0.as_bytes(), &value.1);
            self.clear();
        }
    }
}

/// Data interface for moderate/slow float parsers.
#[cfg(feature = "correct")]
pub(crate) trait SlowDataInterface<'a>: SlowDataInterfaceImpl<'a> {
    /// Integer digits iterator type.
    type IntegerIter: ConsumedIterator<Item=&'a u8> + AsPtrIterator<'a, u8>;

    /// Float digits iterator type.
    type FractionIter: ConsumedIterator<Item=&'a u8> + AsPtrIterator<'a, u8>;

    /// Iterate over all integer digits.
    fn integer_iter(&self) -> Self::IntegerIter;

    /// Get number of all integer digits.
    perftools_inline!{
    fn integer_digits(&self) -> usize {
        self.integer_iter().count()
    }}

    /// Iterate over all fraction digits
    fn fraction_iter(&self) -> Self::FractionIter;

    /// Get number of all fraction digits.
    perftools_inline!{
    fn fraction_digits(&self) -> usize {
        self.fraction_iter().count()
    }}

    /// Iterate over significant fraction digits.
    fn significant_fraction_iter(&self) -> Self::FractionIter;

    /// Get number of significant fraction digits.
    perftools_inline!{
    fn significant_fraction_digits(&self) -> usize {
        self.significant_fraction_iter().count()
    }}

    /// Get the number of digits in the mantissa.
    /// Cannot overflow, since this is based off a single usize input string.
    perftools_inline!{
    fn mantissa_digits(&self) -> usize {
        self.integer_digits() + self.significant_fraction_digits()
    }}

    /// Get index to start of significant digits in the fraction.
    fn digits_start(&self) -> usize;

    /// Get number of truncated digits.
    fn truncated_digits(&self) -> usize;

    /// Get the mantissa exponent from the raw exponent.
    perftools_inline!{
    fn mantissa_exponent(&self) -> i32 {
        mantissa_exponent(self.raw_exponent(), self.fraction_digits(), self.truncated_digits())
    }}

    /// Get the scientific exponent from the raw exponent.
    perftools_inline!{
    fn scientific_exponent(&self) -> i32 {
        scientific_exponent(self.raw_exponent(), self.integer_digits(), self.digits_start())
    }}
}
