//! Generic float-parsing data interfaces.

use crate::util::*;
use super::exponent::*;
use super::iterator::*;
use super::traits::*;
use super::trim::*;
use super::validate::*;

// The following interfaces are named:
//      Generic*Interface, where * represents any combination of the following:
//          1). [I]nteger.
//          2). [F]raction.
//          3). [E]xponent.

/// Shared definition for all generic fast data interfaces.
macro_rules! generic_data_interface {
    (
        struct $fast:ident,
        struct $slow:ident,
        integer_iter => ( $integer_iter:tt, $integer_iter_fn:ident ),
        fraction_iter => ( $fraction_iter:tt, $fraction_iter_fn:ident ),
        exponent_iter => ( $exponent_iter:tt, $exponent_iter_fn:ident ),
        consume_integer_digits => $consume_integer_digits:expr,
        consume_fraction_digits => $consume_fraction_digits:expr,
        extract_exponent => $extract_exponent:expr,
        ltrim_zero => $ltrim_zero:ident,
        ltrim_separator => $ltrim_separator:ident,
        rtrim_zero => $rtrim_zero:ident,
        rtrim_separator => $rtrim_separator:ident
    ) => {
        data_interface!(
            struct $fast,
            struct $slow,
            fields => {
                format: NumberFormat,
            },
            integer_iter => ($integer_iter, $integer_iter_fn),
            fraction_iter => ($fraction_iter, $fraction_iter_fn),
            exponent_iter => ($exponent_iter, $exponent_iter_fn),
            format => |this: &Self| this.format,
            consume_integer_digits => $consume_integer_digits,
            consume_fraction_digits => $consume_fraction_digits,
            extract_exponent => $extract_exponent,
            validate_mantissa => |this: &Self| validate_mantissa(this, this.format),
            validate_exponent => |this: &Self| validate_exponent(this, this.format),
            validate_exponent_fraction => |this: &Self| validate_exponent_fraction(this, this.format),
            validate_exponent_sign => |this: &Self| validate_exponent_sign(this, this.format),
            ltrim_zero => $ltrim_zero,
            ltrim_separator => $ltrim_separator,
            rtrim_zero => $rtrim_zero,
            rtrim_separator => $rtrim_separator,
            new => fn new(format: NumberFormat) -> Self {
                Self {
                    format: format,
                    integer: &[],
                    fraction: &[],
                    exponent: &[],
                    raw_exponent: 0
                }
            }
        );
    };
}

// Generic data interface without digit separators.
generic_data_interface!(
    struct GenericFastDataInterface,
    struct GenericSlowDataInterface,
    integer_iter => (IteratorNoSeparator, iterate_no_separator),
    fraction_iter => (IteratorNoSeparator, iterate_no_separator),
    exponent_iter => (IteratorNoSeparator, iterate_no_separator),
    consume_integer_digits => consume_digits_no_separator,
    consume_fraction_digits => consume_digits_no_separator,
    extract_exponent => extract_exponent_no_separator,
    ltrim_zero => ltrim_zero_no_separator,
    ltrim_separator => ltrim_separator_no_separator,
    rtrim_zero => rtrim_zero_no_separator,
    rtrim_separator => rtrim_separator_no_separator
);

// Generic data interface with integer digit separators.
generic_data_interface!(
    struct GenericIFastDataInterface,
    struct GenericISlowDataInterface,
    integer_iter => (IteratorSeparator, iterate_separator),
    fraction_iter => (IteratorNoSeparator, iterate_no_separator),
    exponent_iter => (IteratorNoSeparator, iterate_no_separator),
    consume_integer_digits => consume_integer_digits_separator,
    consume_fraction_digits => consume_digits_no_separator,
    extract_exponent => extract_exponent_no_separator,
    ltrim_zero => ltrim_zero_separator,
    ltrim_separator => ltrim_separator_separator,
    rtrim_zero => rtrim_zero_separator,
    rtrim_separator => rtrim_separator_separator
);

// Generic data interface with fraction digit separators.
generic_data_interface!(
    struct GenericFFastDataInterface,
    struct GenericFSlowDataInterface,
    integer_iter => (IteratorNoSeparator, iterate_no_separator),
    fraction_iter => (IteratorSeparator, iterate_separator),
    exponent_iter => (IteratorNoSeparator, iterate_no_separator),
    consume_integer_digits => consume_digits_no_separator,
    consume_fraction_digits => consume_fraction_digits_separator,
    extract_exponent => extract_exponent_no_separator,
    ltrim_zero => ltrim_zero_separator,
    ltrim_separator => ltrim_separator_separator,
    rtrim_zero => rtrim_zero_separator,
    rtrim_separator => rtrim_separator_separator
);

// Generic data interface with exponent digit separators.
generic_data_interface!(
    struct GenericEFastDataInterface,
    struct GenericESlowDataInterface,
    integer_iter => (IteratorNoSeparator, iterate_no_separator),
    fraction_iter => (IteratorNoSeparator, iterate_no_separator),
    exponent_iter => (IteratorSeparator, iterate_separator),
    consume_integer_digits => consume_digits_no_separator,
    consume_fraction_digits => consume_digits_no_separator,
    extract_exponent => extract_exponent_separator,
    ltrim_zero => ltrim_zero_no_separator,
    ltrim_separator => ltrim_separator_no_separator,
    rtrim_zero => rtrim_zero_no_separator,
    rtrim_separator => rtrim_separator_no_separator
);

// Generic data interface with integer and fraction digit separators.
generic_data_interface!(
    struct GenericIFFastDataInterface,
    struct GenericIFSlowDataInterface,
    integer_iter => (IteratorSeparator, iterate_separator),
    fraction_iter => (IteratorSeparator, iterate_separator),
    exponent_iter => (IteratorNoSeparator, iterate_no_separator),
    consume_integer_digits => consume_integer_digits_separator,
    consume_fraction_digits => consume_fraction_digits_separator,
    extract_exponent => extract_exponent_no_separator,
    ltrim_zero => ltrim_zero_separator,
    ltrim_separator => ltrim_separator_separator,
    rtrim_zero => rtrim_zero_separator,
    rtrim_separator => rtrim_separator_separator
);

// Generic data interface with integer and exponent digit separators.
generic_data_interface!(
    struct GenericIEFastDataInterface,
    struct GenericIESlowDataInterface,
    integer_iter => (IteratorSeparator, iterate_separator),
    fraction_iter => (IteratorNoSeparator, iterate_no_separator),
    exponent_iter => (IteratorSeparator, iterate_separator),
    consume_integer_digits => consume_integer_digits_separator,
    consume_fraction_digits => consume_digits_no_separator,
    extract_exponent => extract_exponent_separator,
    ltrim_zero => ltrim_zero_separator,
    ltrim_separator => ltrim_separator_separator,
    rtrim_zero => rtrim_zero_separator,
    rtrim_separator => rtrim_separator_separator
);

// Generic data interface with fraction and exponent digit separators.
generic_data_interface!(
    struct GenericFEFastDataInterface,
    struct GenericFESlowDataInterface,
    integer_iter => (IteratorNoSeparator, iterate_no_separator),
    fraction_iter => (IteratorSeparator, iterate_separator),
    exponent_iter => (IteratorSeparator, iterate_separator),
    consume_integer_digits => consume_digits_no_separator,
    consume_fraction_digits => consume_fraction_digits_separator,
    extract_exponent => extract_exponent_separator,
    ltrim_zero => ltrim_zero_separator,
    ltrim_separator => ltrim_separator_separator,
    rtrim_zero => rtrim_zero_separator,
    rtrim_separator => rtrim_separator_separator
);

// Generic data interface with integer, fraction, and exponent digit separators.
generic_data_interface!(
    struct GenericIFEFastDataInterface,
    struct GenericIFESlowDataInterface,
    integer_iter => (IteratorSeparator, iterate_separator),
    fraction_iter => (IteratorSeparator, iterate_separator),
    exponent_iter => (IteratorSeparator, iterate_separator),
    consume_integer_digits => consume_integer_digits_separator,
    consume_fraction_digits => consume_fraction_digits_separator,
    extract_exponent => extract_exponent_separator,
    ltrim_zero => ltrim_zero_separator,
    ltrim_separator => ltrim_separator_separator,
    rtrim_zero => rtrim_zero_separator,
    rtrim_separator => rtrim_separator_separator
);

// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh) Implement...
}
