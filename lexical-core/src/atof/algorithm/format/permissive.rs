//! Permissive float-parsing data.

use crate::util::*;
use super::consume::*;
use super::exponent::*;
use super::iterator::*;
use super::traits::*;
use super::trim::*;
use super::validate::*;

// Permissive data interface for fast float parsers.
//
// Guaranteed to parse `FloatFormat::default()`.
//
// The requirements:
//     1). Must contain significant digits.
//     2). Does not contain any digit separators.
fast_data_interface!(
    struct PermissiveFastDataInterface,
    fields => {},
    integer_iter => (IteratorNoSeparator, iterate_no_separator),
    fraction_iter => (IteratorNoSeparator, iterate_no_separator),
    slow_interface => PermissiveSlowDataInterface,
    consume_digits => consume_digits_no_separator,
    extract_exponent => extract_exponent_no_separator,
    validate_mantissa => validate_mantissa_no_separator,
    validate_exponent => validate_optional_exponent_no_separator,
    ltrim_zero => ltrim_zero_no_separator,
    ltrim_separator => ltrim_separator_no_separator,
    rtrim_zero => rtrim_zero_no_separator,
    rtrim_separator => rtrim_separator_no_separator,
    new => fn new(format: FloatFormat) -> Self {
        Self {
            integer: &[],
            fraction: &[],
            exponent: &[],
            raw_exponent: 0
        }
    }
);

// Permissive data interface for moderate/slow float parsers.
//
// Guaranteed to parse `FloatFormat::default()`.
//
// The requirements:
//     1). Must contain significant digits.
//     2). Does not contain any digit separators.
slow_data_interface!(
    struct PermissiveSlowDataInterface,
    fields => {},
    integer_iter => (IteratorNoSeparator, iterate_no_separator),
    fraction_iter => (IteratorNoSeparator, iterate_no_separator)
);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! permissive {
        ($integer:expr, $fraction:expr, $exponent:expr, $raw_exponent:expr) => {
            PermissiveFastDataInterface {
                integer: $integer,
                fraction: $fraction,
                exponent: $exponent,
                raw_exponent: $raw_exponent
            }
        };
    }

    #[test]
    fn extract_test() {
        PermissiveFastDataInterface::new(FloatFormat::default()).run_tests([
            // Valid
            ("1.2345", Ok(permissive!(b"1", b"2345", b"", 0))),
            ("12.345", Ok(permissive!(b"12", b"345", b"", 0))),
            ("12345.6789", Ok(permissive!(b"12345", b"6789", b"", 0))),
            ("1.2345e10", Ok(permissive!(b"1", b"2345", b"e10", 10))),
            ("1.2345e+10", Ok(permissive!(b"1", b"2345", b"e+10", 10))),
            ("1.2345e-10", Ok(permissive!(b"1", b"2345", b"e-10", -10))),
            ("100000000000000000000", Ok(permissive!(b"100000000000000000000", b"", b"", 0))),
            ("100000000000000000001", Ok(permissive!(b"100000000000000000001", b"", b"", 0))),
            ("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", Ok(permissive!(b"179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791", b"9999999999999999999999999999999999999999999999999999999999999999999999", b"", 0))),
            ("1009e-31", Ok(permissive!(b"1009", b"", b"e-31", -31))),
            ("001.0", Ok(permissive!(b"1", b"", b"", 0))),
            ("1.", Ok(permissive!(b"1", b"", b"", 0))),
            ("12.", Ok(permissive!(b"12", b"", b"", 0))),
            ("1234567.", Ok(permissive!(b"1234567", b"", b"", 0))),
            (".1", Ok(permissive!(b"", b"1", b"", 0))),
            (".12", Ok(permissive!(b"", b"12", b"", 0))),
            (".1234567", Ok(permissive!(b"", b"1234567", b"", 0))),
            ("1.2345e", Ok(permissive!(b"1", b"2345", b"e", 0))),
            (".3e", Ok(permissive!(b"", b"3", b"e", 0))),

            // Invalid
            ("", Err(ErrorCode::EmptyFraction)),
            ("+", Err(ErrorCode::EmptyFraction)),
            ("-", Err(ErrorCode::EmptyFraction)),
            (".", Err(ErrorCode::EmptyFraction)),
            ("+.", Err(ErrorCode::EmptyFraction)),
            ("-.", Err(ErrorCode::EmptyFraction)),
            ("e", Err(ErrorCode::EmptyFraction)),
            ("E", Err(ErrorCode::EmptyFraction)),
            ("e1", Err(ErrorCode::EmptyFraction)),
            ("e+1", Err(ErrorCode::EmptyFraction)),
            ("e-1", Err(ErrorCode::EmptyFraction)),
            (".e", Err(ErrorCode::EmptyFraction)),
            (".E", Err(ErrorCode::EmptyFraction)),
            (".e1", Err(ErrorCode::EmptyFraction)),
            (".e+1", Err(ErrorCode::EmptyFraction)),
            (".e-1", Err(ErrorCode::EmptyFraction)),
        ].iter());
    }
}
