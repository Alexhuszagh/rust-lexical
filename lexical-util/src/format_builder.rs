//! Builder for the number format.

use core::num;

use crate::error::Error;
use crate::format_flags as flags;

// NOTE: The size of `Option<NonZero>` is guaranteed to be the same.
//  https://doc.rust-lang.org/std/num/type.NonZeroUsize.html
/// Type with the exact same size as a `u8`.
#[doc(hidden)]
pub type OptionU8 = Option<num::NonZeroU8>;

/// Add single flag to `SyntaxFormat`.
macro_rules! add_flag {
    ($format:ident, $bool:expr, $flag:ident) => {
        if $bool {
            $format |= flags::$flag;
        }
    };
}

/// Add multiple flags to `SyntaxFormat`.
macro_rules! add_flags {
    ($format:ident ; $($bool:expr, $flag:ident ;)*) => {{
        $(add_flag!($format, $bool, $flag);)*
    }};
}

/// Determine if a flag is set in the format.
macro_rules! has_flag {
    ($format:ident, $flag:ident) => {
        $format & flags::$flag != 0
    };
}

/// Unwrap `Option` as a const fn.
#[inline(always)]
const fn unwrap_or_zero(option: OptionU8) -> u8 {
    match option {
        Some(x) => x.get(),
        None => 0,
    }
}

/// Validating builder for [`NumberFormat`] from the provided specifications.
///
/// Some of the core functionality includes support for:
/// - Digit separators: ignored characters used to make numbers more readable,
///   such as `100,000`.
/// - Non-decimal radixes: writing or parsing numbers written in binary,
///   hexadecimal, or other bases.
/// - Special numbers: disabling support for special floating-point, such as
///   [`NaN`][f64::NAN].
/// - Number components: require signs, significant digits, and more.
///
/// Returns [`NumberFormat`] on calling [`build_strict`] if it was able to
/// compile the format, otherwise, returns [`None`].
///
/// [`NumberFormat`]: crate::NumberFormat
/// [`build_strict`]: Self::build_strict
///
/// # Examples
///
/// To create a format valid for Rust number literals, we can use the builder
/// API:
///
/// ```rust
/// # #[cfg(feature = "format")] {
/// use core::num;
///
/// use lexical_util::{NumberFormat, NumberFormatBuilder};
///
/// // create the format for literal Rust floats
/// const RUST: u128 = NumberFormatBuilder::new()
///    .digit_separator(num::NonZeroU8::new(b'_'))
///    .required_digits(true)
///    .no_positive_mantissa_sign(true)
///    .no_special(true)
///    .internal_digit_separator(true)
///    .trailing_digit_separator(true)
///    .consecutive_digit_separator(true)
///    .build_strict();
///
/// // then, access the formats's properties
/// let format = NumberFormat::<{ RUST }> {};
/// assert!(format.no_positive_mantissa_sign());
/// assert!(format.no_special());
/// assert!(format.internal_digit_separator());
/// assert!(format.trailing_digit_separator());
/// assert!(format.consecutive_digit_separator());
/// assert!(!format.no_exponent_notation());
/// # }
/// ```
///
/// # Fields
///
/// - [`digit_separator`]: Character to separate digits.
/// - [`mantissa_radix`]: Radix for mantissa digits.
/// - [`exponent_base`]: Base for the exponent.
/// - [`exponent_radix`]: Radix for the exponent digits.
/// - [`base_prefix`]: Optional character for the base prefix.
/// - [`base_suffix`]: Optional character for the base suffix.
/// - [`required_integer_digits`]: If digits are required before the decimal
///   point.
/// - [`required_fraction_digits`]: If digits are required after the decimal
///   point.
/// - [`required_exponent_digits`]: If digits are required after the exponent
///   character.
/// - [`required_mantissa_digits`]: If at least 1 significant digit is required.
/// - [`no_positive_mantissa_sign`]: If positive sign before the mantissa is not
///   allowed.
/// - [`required_mantissa_sign`]: If positive sign before the mantissa is
///   required.
/// - [`no_exponent_notation`]: If exponent notation is not allowed.
/// - [`no_positive_exponent_sign`]: If positive sign before the exponent is not
///   allowed.
/// - [`required_exponent_sign`]: If sign before the exponent is required.
/// - [`no_exponent_without_fraction`]: If exponent without fraction is not
///   allowed.
/// - [`no_special`]: If special (non-finite) values are not allowed.
/// - [`case_sensitive_special`]: If special (non-finite) values are
///   case-sensitive.
/// - [`no_integer_leading_zeros`]: If leading zeros before an integer are not
///   allowed.
/// - [`no_float_leading_zeros`]: If leading zeros before a float are not
///   allowed.
/// - [`required_exponent_notation`]: If exponent notation is required.
/// - [`required_integer_digits_with_exponent`]: If digits are required before
///   the decimal point with exponent notation.
/// - [`required_fraction_digits_with_exponent`]: If digits are required after
///   the decimal point with exponent notation, if the decimal point is present.
/// - [`required_mantissa_digits_with_exponent`]: If any significant digits are
///   required with exponent notation.
/// - [`case_sensitive_exponent`]: If exponent characters are case-sensitive.
/// - [`case_sensitive_base_prefix`]: If base prefixes are case-sensitive.
/// - [`case_sensitive_base_suffix`]: If base suffixes are case-sensitive.
/// - [`no_unsigned_negative_sign`]: If a negative sign before an unsigned
///   integer is not allowed.
/// - [`no_mantissa_sign`]: If positive or negative signs before an exponent are
///   not allowed.
/// - [`no_exponent_sign`]: If positive or negative signs before an exponent are
///   not allowed.
/// - [`start_digit_separator`]: If digit separators are allowed at the absolute
///   start of the number.
/// - [`integer_sign_digit_separator`]: If digit separators are allowed before
///   the sign of the integer.
/// - [`integer_consecutive_sign_digit_separator`]: If consecutive digit
///   separators are allowed before the sign of the integer.
/// - [`exponent_sign_digit_separator`]: If digit separators are allowed before
///   the sign of the exponent.
/// - [`exponent_consecutive_sign_digit_separator`]: If consecutive digit
///   separators are allowed before the sign of the exponent.
/// - [`base_prefix_internal_digit_separator`]: If a digit separator is allowed
///   between the base prefix.
/// - [`base_prefix_leading_digit_separator`]: If a digit separator is allowed
///   before the base prefix.
/// - [`base_prefix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base prefix.
/// - [`base_prefix_consecutive_digit_separator`]: If multiple consecutive base
///   prefix digit separators are allowed.
/// - [`base_suffix_internal_digit_separator`]: If a digit separator is allowed
///   between the base suffix.
/// - [`base_suffix_leading_digit_separator`]: If a digit separator is allowed
///   before the base suffix.
/// - [`base_suffix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base suffix.
/// - [`base_suffix_consecutive_digit_separator`]: If multiple consecutive base
///   suffix digit separators are allowed.
/// - [`integer_internal_digit_separator`]: If digit separators are allowed
///   between integer digits.
/// - [`fraction_internal_digit_separator`]: If digit separators are allowed
///   between fraction digits.
/// - [`exponent_internal_digit_separator`]: If digit separators are allowed
///   between exponent digits.
/// - [`integer_leading_digit_separator`]: If a digit separator is allowed
///   before any integer digits.
/// - [`fraction_leading_digit_separator`]: If a digit separator is allowed
///   before any fraction digits.
/// - [`exponent_leading_digit_separator`]: If a digit separator is allowed
///   before any exponent digits.
/// - [`integer_trailing_digit_separator`]: If a digit separator is allowed
///   after any integer digits.
/// - [`fraction_trailing_digit_separator`]: If a digit separator is allowed
///   after any fraction digits.
/// - [`exponent_trailing_digit_separator`]: If a digit separator is allowed
///   after any exponent digits.
/// - [`integer_consecutive_digit_separator`]: If multiple consecutive integer
///   digit separators are allowed.
/// - [`fraction_consecutive_digit_separator`]: If multiple consecutive fraction
///   digit separators are allowed.
/// - [`special_digit_separator`]: If any digit separators are allowed in
///   special (non-finite) values.
///
/// # Write Integer Fields
///
/// No fields are used for writing integers.
///
/// # Parse Integer Fields
///
/// These fields are used for parsing integers:
///
/// - [`digit_separator`]: Character to separate digits.
/// - [`mantissa_radix`]: Radix for mantissa digits.
/// - [`base_prefix`]: Optional character for the base prefix.
/// - [`base_suffix`]: Optional character for the base suffix.
/// - [`no_positive_mantissa_sign`]: If positive sign before the mantissa is not
///   allowed.
/// - [`required_mantissa_sign`]: If positive sign before the mantissa is
///   required.
/// - [`no_integer_leading_zeros`]: If leading zeros before an integer are not
///   allowed.
/// - [`start_digit_separator`]: If digit separators are allowed at the absolute
///   start of the number.
/// - [`integer_sign_digit_separator`]: If digit separators are allowed before
///   the sign of the integer.
/// - [`integer_consecutive_sign_digit_separator`]: If consecutive digit
///   separators are allowed before the sign of the integer.
/// - [`base_prefix_internal_digit_separator`]: If a digit separator is allowed
///   between the base prefix.
/// - [`base_prefix_leading_digit_separator`]: If a digit separator is allowed
///   before the base prefix.
/// - [`base_prefix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base prefix.
/// - [`base_prefix_consecutive_digit_separator`]: If multiple consecutive base
///   prefix digit separators are allowed.
/// - [`base_suffix_internal_digit_separator`]: If a digit separator is allowed
///   between the base suffix.
/// - [`base_suffix_leading_digit_separator`]: If a digit separator is allowed
///   before the base suffix.
/// - [`base_suffix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base suffix.
/// - [`base_suffix_consecutive_digit_separator`]: If multiple consecutive base
///   suffix digit separators are allowed.
/// - [`integer_internal_digit_separator`]: If digit separators are allowed
///   between integer digits.
/// - [`integer_leading_digit_separator`]:  If a digit separator is allowed
///   before any integer digits.
/// - [`integer_trailing_digit_separator`]: If a digit separator is allowed
///   after any integer digits.
/// - [`integer_consecutive_digit_separator`]: If multiple consecutive integer
///   digit separators are allowed.
///
/// # Write Float Fields
///
/// These fields are used for writing floats:
///
/// - [`mantissa_radix`]: Radix for mantissa digits.
/// - [`exponent_base`]: Base for the exponent.
/// - [`exponent_radix`]: Radix for the exponent digits.
/// - [`no_positive_mantissa_sign`]: If positive sign before the mantissa is not
///   allowed.
/// - [`required_mantissa_sign`]: If positive sign before the mantissa is
///   required.
/// - [`no_exponent_notation`]: If exponent notation is not allowed.
/// - [`no_positive_exponent_sign`]: If positive sign before the exponent is not
///   allowed.
/// - [`required_exponent_sign`]: If sign before the exponent is required.
/// - [`required_exponent_notation`]: If exponent notation is required.
/// - [`case_sensitive_base_prefix`]: If base prefixes are case-sensitive.
/// - [`case_sensitive_base_suffix`]: If base suffixes are case-sensitive.
/// - [`no_unsigned_negative_sign`]: If a negative sign before an unsigned
///   integer is not allowed.
/// - [`no_mantissa_sign`]: If positive or negative signs before an exponent are
///   not allowed.
/// - [`no_exponent_sign`]: If positive or negative signs before an exponent are
///   not allowed.
/// - [`start_digit_separator`]: If digit separators are allowed at the absolute
///   start of the number.
/// - [`integer_sign_digit_separator`]: If digit separators are allowed before
///   the sign of the integer.
/// - [`integer_consecutive_sign_digit_separator`]: If consecutive digit
///   separators are allowed before the sign of the integer.
/// - [`base_prefix_internal_digit_separator`]: If a digit separator is allowed
///   between the base prefix.
/// - [`base_prefix_leading_digit_separator`]: If a digit separator is allowed
///   before the base prefix.
/// - [`base_prefix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base prefix.
/// - [`base_prefix_consecutive_digit_separator`]: If multiple consecutive base
///   prefix digit separators are allowed.
/// - [`base_suffix_internal_digit_separator`]: If a digit separator is allowed
///   between the base suffix.
/// - [`base_suffix_leading_digit_separator`]: If a digit separator is allowed
///   before the base suffix.
/// - [`base_suffix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base suffix.
/// - [`base_suffix_consecutive_digit_separator`]: If multiple consecutive base
///   suffix digit separators are allowed.
/// - [`integer_internal_digit_separator`]: If digit separators are allowed
///   between integer digits.
/// - [`integer_leading_digit_separator`]: If a digit separator is allowed
///   before any integer digits.
/// - [`integer_trailing_digit_separator`]: If a digit separator is allowed
///   after any integer digits.
/// - [`integer_consecutive_digit_separator`]: If multiple consecutive integer
///   digit separators are allowed.
///
/// # Parse Float Fields
///
/// These fields are used for parsing floats:
///
/// - [`digit_separator`]: Character to separate digits.
/// - [`mantissa_radix`]: Radix for mantissa digits.
/// - [`exponent_base`]: Base for the exponent.
/// - [`exponent_radix`]: Radix for the exponent digits.
/// - [`base_prefix`]: Optional character for the base prefix.
/// - [`base_suffix`]: Optional character for the base suffix.
/// - [`required_mantissa_digits`]: If at least 1 significant digit is required.
/// - [`required_integer_digits`]: If digits are required before the decimal
///   point.
/// - [`required_fraction_digits`]: If digits are required after the decimal
///   point.
/// - [`required_exponent_digits`]: If digits are required after the exponent
///   character.
/// - [`no_positive_mantissa_sign`]: If positive sign before the mantissa is not
///   allowed.
/// - [`required_mantissa_sign`]: If positive sign before the mantissa is
///   required.
/// - [`no_exponent_notation`]: If exponent notation is not allowed.
/// - [`no_positive_exponent_sign`]: If positive sign before the exponent is not
///   allowed.
/// - [`required_exponent_sign`]: If sign before the exponent is required.
/// - [`no_exponent_without_fraction`]: If exponent without fraction is not
///   allowed.
/// - [`no_special`]: If special (non-finite) values are not allowed.
/// - [`case_sensitive_special`]: If special (non-finite) values are
///   case-sensitive.
/// - [`no_integer_leading_zeros`]: If leading zeros before an integer are not
///   allowed.
/// - [`no_float_leading_zeros`]: If leading zeros before a float are not
///   allowed.
/// - [`required_exponent_notation`]: If exponent notation is required.
/// - [`required_integer_digits_with_exponent`]: If digits are required before
///   the decimal point with exponent notation.
/// - [`required_fraction_digits_with_exponent`]: If digits are required after
///   the decimal point with exponent notation, if the decimal point is present.
/// - [`required_mantissa_digits_with_exponent`]: If any significant digits are
///   required with exponent notation.
/// - [`case_sensitive_exponent`]: If exponent characters are case-sensitive.
/// - [`case_sensitive_base_prefix`]: If base prefixes are case-sensitive.
/// - [`case_sensitive_base_suffix`]: If base suffixes are case-sensitive.
/// - [`no_unsigned_negative_sign`]: If a negative sign before an unsigned
///   integer is not allowed.
/// - [`no_mantissa_sign`]: If positive or negative signs before an exponent are
///   not allowed.
/// - [`no_exponent_sign`]: If positive or negative signs before an exponent are
///   not allowed.
/// - [`start_digit_separator`]: If digit separators are allowed at the absolute
///   start of the number.
/// - [`integer_sign_digit_separator`]: If digit separators are allowed before
///   the sign of the integer.
/// - [`integer_consecutive_sign_digit_separator`]: If consecutive digit
///   separators are allowed before the sign of the integer.
/// - [`exponent_sign_digit_separator`]: If digit separators are allowed before
///   the sign of the exponent.
/// - [`exponent_consecutive_sign_digit_separator`]: If consecutive digit
///   separators are allowed before the sign of the exponent.
/// - [`base_prefix_internal_digit_separator`]: If a digit separator is allowed
///   between the base prefix.
/// - [`base_prefix_leading_digit_separator`]: If a digit separator is allowed
///   before the base prefix.
/// - [`base_prefix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base prefix.
/// - [`base_prefix_consecutive_digit_separator`]: If multiple consecutive base
///   prefix digit separators are allowed.
/// - [`base_suffix_internal_digit_separator`]: If a digit separator is allowed
///   between the base suffix.
/// - [`base_suffix_leading_digit_separator`]: If a digit separator is allowed
///   before the base suffix.
/// - [`base_suffix_trailing_digit_separator`]: If a digit separator is allowed
///   after the base suffix.
/// - [`base_suffix_consecutive_digit_separator`]: If multiple consecutive base
///   suffix digit separators are allowed.
/// - [`integer_internal_digit_separator`]: If digit separators are allowed
///   between integer digits.
/// - [`fraction_internal_digit_separator`]: If digit separators are allowed
///   between fraction digits.
/// - [`exponent_internal_digit_separator`]: If digit separators are allowed
///   between exponent digits.
/// - [`integer_leading_digit_separator`]: If a digit separator is allowed
///   before any integer digits.
/// - [`fraction_leading_digit_separator`]: If a digit separator is allowed
///   before any fraction digits.
/// - [`exponent_leading_digit_separator`]: If a digit separator is allowed
///   before any exponent digits.
/// - [`integer_trailing_digit_separator`]: If a digit separator is allowed
///   after any integer digits.
/// - [`fraction_trailing_digit_separator`]: If a digit separator is allowed
///   after any fraction digits.
/// - [`exponent_trailing_digit_separator`]: If a digit separator is allowed
///   after any exponent digits.
/// - [`integer_consecutive_digit_separator`]: If multiple consecutive integer
///   digit separators are allowed.
/// - [`fraction_consecutive_digit_separator`]: If multiple consecutive fraction
///   digit separators are allowed.
/// - [`special_digit_separator`]: If any digit separators are allowed in
///   special (non-finite) values.
#[cfg_attr(
    feature = "power-of-two",
    doc = "\n
[`exponent_base`]: Self::exponent_base
[`exponent_radix`]: Self::exponent_radix
[`mantissa_radix`]: Self::mantissa_radix
"
)]
#[cfg_attr(
    not(feature = "power-of-two"),
    doc = "\n
[`exponent_base`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L602\n
[`exponent_radix`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L610\n
[`mantissa_radix`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L594\n
"
)]
#[cfg_attr(
    feature = "format",
    doc = "\n
[`digit_separator`]: Self::digit_separator\n
[`required_integer_digits`]: Self::required_integer_digits\n
[`required_fraction_digits`]: Self::required_fraction_digits\n
[`required_exponent_digits`]: Self::required_exponent_digits\n
[`required_mantissa_digits`]: Self::required_mantissa_digits\n
[`no_positive_mantissa_sign`]: Self::no_positive_mantissa_sign\n
[`required_mantissa_sign`]: Self::required_mantissa_sign\n
[`no_exponent_notation`]: Self::no_exponent_notation\n
[`no_positive_exponent_sign`]: Self::no_positive_exponent_sign\n
[`required_exponent_sign`]: Self::required_exponent_sign\n
[`no_exponent_without_fraction`]: Self::no_exponent_without_fraction\n
[`no_special`]: Self::no_special\n
[`case_sensitive_special`]: Self::case_sensitive_special\n
[`no_integer_leading_zeros`]: Self::no_integer_leading_zeros\n
[`no_float_leading_zeros`]: Self::no_float_leading_zeros\n
[`required_exponent_notation`]: Self::required_exponent_notation\n
[`required_integer_digits_with_exponent`]: Self::required_integer_digits_with_exponent\n
[`required_fraction_digits_with_exponent`]: Self::required_fraction_digits_with_exponent\n
[`required_mantissa_digits_with_exponent`]: Self::required_mantissa_digits_with_exponent\n
[`case_sensitive_exponent`]: Self::case_sensitive_exponent\n
[`no_unsigned_negative_sign`]: Self::no_unsigned_negative_sign\n
[`no_mantissa_sign`]: Self::no_mantissa_sign\n
[`no_exponent_sign`]: Self::no_exponent_sign\n
[`start_digit_separator`]: Self::start_digit_separator\n
[`integer_sign_digit_separator`]: Self::integer_sign_digit_separator\n
[`integer_consecutive_sign_digit_separator`]: Self::integer_consecutive_sign_digit_separator\n
[`exponent_sign_digit_separator`]: Self::exponent_sign_digit_separator\n
[`exponent_consecutive_sign_digit_separator`]: Self::exponent_consecutive_sign_digit_separator\n
[`integer_internal_digit_separator`]: Self::integer_internal_digit_separator\n
[`fraction_internal_digit_separator`]: Self::fraction_internal_digit_separator\n
[`exponent_internal_digit_separator`]: Self::exponent_internal_digit_separator\n
[`integer_leading_digit_separator`]: Self::integer_leading_digit_separator\n
[`fraction_leading_digit_separator`]: Self::fraction_leading_digit_separator\n
[`exponent_leading_digit_separator`]: Self::exponent_leading_digit_separator\n
[`integer_trailing_digit_separator`]: Self::integer_trailing_digit_separator\n
[`fraction_trailing_digit_separator`]: Self::fraction_trailing_digit_separator\n
[`exponent_trailing_digit_separator`]: Self::exponent_trailing_digit_separator\n
[`integer_consecutive_digit_separator`]: Self::integer_consecutive_digit_separator\n
[`fraction_consecutive_digit_separator`]: Self::fraction_consecutive_digit_separator\n
[`special_digit_separator`]: Self::special_digit_separator\n
"
)]
#[cfg_attr(
    not(feature = "format"),
    doc = "\n
[`digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L579\n
[`required_integer_digits`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L634\n
[`required_fraction_digits`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L642\n
[`required_exponent_digits`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L650\n
[`required_mantissa_digits`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L658\n
[`no_positive_mantissa_sign`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L677\n
[`required_mantissa_sign`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L685\n
[`no_exponent_notation`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L693\n
[`no_positive_exponent_sign`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L701\n
[`required_exponent_sign`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L709\n
[`no_exponent_without_fraction`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L717\n
[`no_special`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L725\n
[`case_sensitive_special`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L733\n
[`no_integer_leading_zeros`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L741\n
[`no_float_leading_zeros`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L749\n
[`required_exponent_notation`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L757\n
[`required_integer_digits_with_exponent`]: https://github.com/Alexhuszagh/rust-lexical/blob/0cad692/lexical-util/src/format_builder.rs#L1129\n
[`required_fraction_digits_with_exponent`]: https://github.com/Alexhuszagh/rust-lexical/blob/0cad692/lexical-util/src/format_builder.rs#L1149\n
[`required_mantissa_digits_with_exponent`]: https://github.com/Alexhuszagh/rust-lexical/blob/47a090d/lexical-util/src/format_builder.rs#L1233\n
[`case_sensitive_exponent`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L765\n
[`no_unsigned_negative_sign`]: https://github.com/Alexhuszagh/rust-lexical/blob/TODO/lexical-util/src/format_builder.rs#LTODO\n
[`no_mantissa_sign`]: https://github.com/Alexhuszagh/rust-lexical/blob/TODO/lexical-util/src/format_builder.rs#LTODO\n
[`no_exponent_sign`]: https://github.com/Alexhuszagh/rust-lexical/blob/TODO/lexical-util/src/format_builder.rs#LTODO\n
[`start_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1650\n
[`integer_sign_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1678\n
[`integer_consecutive_sign_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1706\n
[`exponent_sign_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1728\n
[`exponent_consecutive_sign_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1754\n
[`integer_internal_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L793\n
[`fraction_internal_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L805\n
[`exponent_internal_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L817\n
[`integer_leading_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L842\n
[`fraction_leading_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L853\n
[`exponent_leading_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L864\n
[`integer_trailing_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L888\n
[`fraction_trailing_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L899\n
[`exponent_trailing_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L910\n
[`integer_consecutive_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L931\n
[`fraction_consecutive_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L939\n
[`special_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L965\n
"
)]
#[cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = "\n
[`base_prefix`]: Self::base_prefix
[`base_suffix`]: Self::base_suffix
[`case_sensitive_base_prefix`]: Self::case_sensitive_base_prefix
[`case_sensitive_base_suffix`]: Self::case_sensitive_base_suffix
[`required_base_prefix`]: Self::required_base_prefix
[`required_base_suffix`]: Self::required_base_suffix
[`base_prefix_internal_digit_separator`]: Self::base_prefix_internal_digit_separator\n
[`base_prefix_leading_digit_separator`]: Self::base_prefix_leading_digit_separator\n
[`base_prefix_trailing_digit_separator`]: Self::base_prefix_trailing_digit_separator\n
[`base_prefix_consecutive_digit_separator`]: Self::base_prefix_consecutive_digit_separator\n
[`base_suffix_internal_digit_separator`]: Self::base_suffix_internal_digit_separator\n
[`base_suffix_leading_digit_separator`]: Self::base_suffix_leading_digit_separator\n
[`base_suffix_trailing_digit_separator`]: Self::base_suffix_trailing_digit_separator\n
[`base_suffix_consecutive_digit_separator`]: Self::base_suffix_consecutive_digit_separator\n
"
)]
#[cfg_attr(
    not(all(feature = "format", feature = "power-of-two")),
    doc = "\n
[`base_prefix`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L618\n
[`base_suffix`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L626\n
[`case_sensitive_base_prefix`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L773\n
[`case_sensitive_base_suffix`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-util/src/format_builder.rs#L781\n
[`required_base_prefix`]: https://github.com/Alexhuszagh/rust-lexical/blob/63f9adf/lexical-util/src/format_builder.rs#L1267\n
[`required_base_suffix`]: https://github.com/Alexhuszagh/rust-lexical/blob/63f9adf/lexical-util/src/format_builder.rs#L1291\n
[`base_prefix_internal_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1777\n
[`base_prefix_leading_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1809\n
[`base_prefix_trailing_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1841\n
[`base_prefix_consecutive_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1870\n
[`base_suffix_internal_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1883\n
[`base_suffix_leading_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1916\n
[`base_suffix_trailing_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1939\n
[`base_suffix_consecutive_digit_separator`]: https://github.com/Alexhuszagh/rust-lexical/blob/27ca418/lexical-util/src/format_builder.rs#L1967\n
"
)]
///
/// # Number Details
///
/// This assumes a number that follows the following conventions. You should
/// design custom number formats using these assumptions.
///
/// #### Integers
///
/// ```text
/// +--1--+--2--+--3--+--4--+--5--+--6--+--7--+--8--+--9--+--10-+--11-+
/// | __  | +/- | __  | 0x  | __  | 12  | __  | 34  | __  |  h  | __  |
/// +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
/// ```
///
/// Where the components are:
/// 1. Sign digit separators (rarely, if ever, used)
/// 2. Sign
/// 3. Leading base prefix digit separators
/// 4. Base prefix (always `0` + a character)
/// 5. Leading integer digit separators
/// 6. Integer digits
/// 7. Internal integer digit separators
/// 8. Integer digits
/// 9. Trailing integer digit separators
/// 10. Base suffix (such as `h` for X86 assembly)
/// 11. Trailing base suffix digit separators
///
/// #### Floats
///
/// This has all the same components as integers, with many additional
/// ones.
///
/// ```text
/// +--1--+--2--+--3--+--4--+--5--+--6--+--7--+--8--+--9--+-10--+-11--+-12--+-13--+-14--+-15--+
/// | __  | +/- | __  | 0x  | __  | 12  | __  | 34  | __  |  .  | __  | 56  | __  | 78  | __  |
/// +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
///
/// +-16--+-17--+-18--+-19--+-20--+-21--+-22--+-23--+-24--+-25--+-----+-----+-----+-----+-----+
/// |  e  | __  | +/- |  __ | 90  | __  | 12  | __  |  h  | __  |     |     |     |     |     |
/// +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
/// ```
///
/// Where the components are:
/// 1. Mantissa sign digit separators (rarely, if ever, used)
/// 2. Mantissa sign
/// 3. Leading base prefix digit separators
/// 4. Base prefix (always `0` + a character)
/// 5. Leading integer digit separators
/// 6. Integer digits
/// 7. Internal integer digit separators
/// 8. Integer digits
/// 9. Trailing integer digit separators
/// 10. Decimal point
/// 11. Leading fraction digit separators
/// 12. Fraction digits
/// 13. Internal fraction digit separators
/// 14. Fraction digits
/// 15. Trailing fraction digit separators
/// 16. Exponent symbol
/// 17. Exponent sign digit separators (rarely, if ever, used)
/// 18. Exponent sign
/// 19. Leading exponent digit separators
/// 20. Exponent digits
/// 21. Internal exponent digit separators
/// 22. Exponent digits
/// 23. Trailing exponent digit separators
/// 24. Base suffix (such as `h` for X86 assembly)
/// 25. Trailing base suffix digit separators
pub struct NumberFormatBuilder {
    digit_separator: OptionU8,
    base_prefix: OptionU8,
    base_suffix: OptionU8,
    mantissa_radix: u8,
    exponent_base: OptionU8,
    exponent_radix: OptionU8,
    required_integer_digits: bool,
    required_fraction_digits: bool,
    required_exponent_digits: bool,
    required_mantissa_digits: bool,
    no_positive_mantissa_sign: bool,
    required_mantissa_sign: bool,
    no_exponent_notation: bool,
    no_positive_exponent_sign: bool,
    required_exponent_sign: bool,
    no_exponent_without_fraction: bool,
    no_special: bool,
    case_sensitive_special: bool,
    no_integer_leading_zeros: bool,
    no_float_leading_zeros: bool,
    required_exponent_notation: bool,
    case_sensitive_exponent: bool,
    case_sensitive_base_prefix: bool,
    case_sensitive_base_suffix: bool,
    required_integer_digits_with_exponent: bool,
    required_fraction_digits_with_exponent: bool,
    required_mantissa_digits_with_exponent: bool,
    required_base_prefix: bool,
    required_base_suffix: bool,
    no_unsigned_negative_sign: bool,
    no_mantissa_sign: bool,
    no_exponent_sign: bool,
    start_digit_separator: bool,
    integer_sign_digit_separator: bool,
    integer_consecutive_sign_digit_separator: bool,
    exponent_sign_digit_separator: bool,
    exponent_consecutive_sign_digit_separator: bool,
    base_prefix_internal_digit_separator: bool,
    base_prefix_leading_digit_separator: bool,
    base_prefix_trailing_digit_separator: bool,
    base_prefix_consecutive_digit_separator: bool,
    base_suffix_internal_digit_separator: bool,
    base_suffix_leading_digit_separator: bool,
    base_suffix_trailing_digit_separator: bool,
    base_suffix_consecutive_digit_separator: bool,
    integer_internal_digit_separator: bool,
    fraction_internal_digit_separator: bool,
    exponent_internal_digit_separator: bool,
    integer_leading_digit_separator: bool,
    fraction_leading_digit_separator: bool,
    exponent_leading_digit_separator: bool,
    integer_trailing_digit_separator: bool,
    fraction_trailing_digit_separator: bool,
    exponent_trailing_digit_separator: bool,
    integer_consecutive_digit_separator: bool,
    fraction_consecutive_digit_separator: bool,
    exponent_consecutive_digit_separator: bool,
    special_digit_separator: bool,
}

impl NumberFormatBuilder {
    // CONSTRUCTORS

    /// Create new [`NumberFormatBuilder`] with default arguments.
    ///
    /// The default values are:
    /// - [`digit_separator`][Self::get_digit_separator] - `None`
    /// - [`base_prefix`][Self::get_base_prefix] - `None`
    /// - [`base_suffix`][Self::get_base_suffix] - `None`
    /// - [`mantissa_radix`][Self::get_mantissa_radix] - `10`
    /// - [`exponent_base`][Self::get_exponent_base] - `None`
    /// - [`exponent_radix`][Self::get_exponent_radix] - `None`
    /// - [`required_integer_digits`][Self::get_required_integer_digits] -
    ///   `false`
    /// - [`required_fraction_digits`][Self::get_required_fraction_digits] -
    ///   `false`
    /// - [`required_exponent_digits`][Self::get_required_exponent_digits] -
    ///   `true`
    /// - [`required_mantissa_digits`][Self::get_required_mantissa_digits] -
    ///   `true`
    /// - [`no_positive_mantissa_sign`][Self::get_no_positive_mantissa_sign] -
    ///   `false`
    /// - [`required_mantissa_sign`][Self::get_required_mantissa_sign] - `false`
    /// - [`no_exponent_notation`][Self::get_no_exponent_notation] - `false`
    /// - [`no_positive_exponent_sign`][Self::get_no_positive_exponent_sign] -
    ///   `false`
    /// - [`required_exponent_sign`][Self::get_required_exponent_sign] - `false`
    /// - [`no_exponent_without_fraction`][Self::get_no_exponent_without_fraction] -
    ///   `false`
    /// - [`no_special`][Self::get_no_special] - `false`
    /// - [`case_sensitive_special`][Self::get_case_sensitive_special] - `false`
    /// - [`no_integer_leading_zeros`][Self::get_no_integer_leading_zeros] -
    ///   `false`
    /// - [`no_float_leading_zeros`][Self::get_no_float_leading_zeros] - `false`
    /// - [`required_exponent_notation`][Self::get_required_exponent_notation] -
    ///   `false`
    /// - [`required_integer_digits_with_exponent`][Self::get_required_integer_digits_with_exponent] -`false`
    /// - [`required_fraction_digits_with_exponent`][Self::get_required_fraction_digits_with_exponent] -`false`
    /// - [`required_mantissa_digits_with_exponent`][Self::get_required_mantissa_digits_with_exponent] -`true`
    /// - [`case_sensitive_exponent`][Self::get_case_sensitive_exponent] -
    ///   `false`
    /// - [`case_sensitive_base_prefix`][Self::get_case_sensitive_base_prefix] -
    ///   `false`
    /// - [`case_sensitive_base_suffix`][Self::get_case_sensitive_base_suffix] -
    ///   `false`
    /// - [`required_base_prefix`][Self::get_required_base_prefix] - `false`
    /// - [`required_base_suffix`][Self::get_required_base_suffix] - `false`
    /// - [`no_unsigned_negative_sign`][Self::get_no_unsigned_negative_sign] -
    ///   `true`
    /// - [`no_mantissa_sign`][Self::get_no_mantissa_sign] - `false`
    /// - [`no_exponent_sign`][Self::get_no_exponent_sign] - `false`
    /// - [`start_digit_separator`][Self::start_digit_separator] - `true`
    /// - [`integer_sign_digit_separator`][Self::integer_sign_digit_separator] -
    ///   `false`
    /// - [`integer_consecutive_sign_digit_separator`][Self::integer_consecutive_sign_digit_separator] - `false`
    /// - [`exponent_sign_digit_separator`][Self::exponent_sign_digit_separator]
    ///   - `false`
    /// - [`exponent_consecutive_sign_digit_separator`][Self::exponent_consecutive_sign_digit_separator] - `false`
    /// - [`base_prefix_internal_digit_separator`][Self::base_prefix_internal_digit_separator] - `false`
    /// - [`base_prefix_leading_digit_separator`][Self::base_prefix_leading_digit_separator] - `false`
    /// - [`base_prefix_trailing_digit_separator`][Self::base_prefix_trailing_digit_separator] - `false`
    /// - [`base_prefix_consecutive_digit_separator`][Self::base_prefix_consecutive_digit_separator] - `false`
    /// - [`base_suffix_internal_digit_separator`][Self::base_suffix_internal_digit_separator] - `false`
    /// - [`base_suffix_leading_digit_separator`][Self::base_suffix_leading_digit_separator] - `false`
    /// - [`base_suffix_trailing_digit_separator`][Self::base_suffix_trailing_digit_separator] - `false`
    /// - [`base_suffix_consecutive_digit_separator`][Self::base_suffix_consecutive_digit_separator] - `false`
    /// - [`integer_internal_digit_separator`][Self::get_integer_internal_digit_separator] - `false`
    /// - [`fraction_internal_digit_separator`][Self::get_fraction_internal_digit_separator] - `false`
    /// - [`exponent_internal_digit_separator`][Self::get_exponent_internal_digit_separator] - `false`
    /// - [`integer_leading_digit_separator`][Self::get_integer_leading_digit_separator] - `false`
    /// - [`fraction_leading_digit_separator`][Self::get_fraction_leading_digit_separator] - `false`
    /// - [`exponent_leading_digit_separator`][Self::get_exponent_leading_digit_separator] - `false`
    /// - [`integer_trailing_digit_separator`][Self::get_integer_trailing_digit_separator] - `false`
    /// - [`fraction_trailing_digit_separator`][Self::get_fraction_trailing_digit_separator] - `false`
    /// - [`exponent_trailing_digit_separator`][Self::get_exponent_trailing_digit_separator] - `false`
    /// - [`integer_consecutive_digit_separator`][Self::get_integer_consecutive_digit_separator] - `false`
    /// - [`fraction_consecutive_digit_separator`][Self::get_fraction_consecutive_digit_separator] - `false`
    /// - [`exponent_consecutive_digit_separator`][Self::get_exponent_consecutive_digit_separator] - `false`
    /// - [`special_digit_separator`][Self::get_special_digit_separator] -
    ///   `false`
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            digit_separator: None,
            base_prefix: None,
            base_suffix: None,
            mantissa_radix: 10,
            exponent_base: None,
            exponent_radix: None,
            required_integer_digits: false,
            required_fraction_digits: false,
            required_exponent_digits: true,
            required_mantissa_digits: true,
            no_positive_mantissa_sign: false,
            required_mantissa_sign: false,
            no_exponent_notation: false,
            no_positive_exponent_sign: false,
            required_exponent_sign: false,
            no_exponent_without_fraction: false,
            no_special: false,
            case_sensitive_special: false,
            no_integer_leading_zeros: false,
            no_float_leading_zeros: false,
            required_exponent_notation: false,
            case_sensitive_exponent: false,
            case_sensitive_base_prefix: false,
            case_sensitive_base_suffix: false,
            required_integer_digits_with_exponent: false,
            required_fraction_digits_with_exponent: false,
            required_mantissa_digits_with_exponent: true,
            required_base_prefix: false,
            required_base_suffix: false,
            no_unsigned_negative_sign: true,
            no_mantissa_sign: false,
            no_exponent_sign: false,
            start_digit_separator: cfg!(feature = "format"),
            integer_sign_digit_separator: false,
            integer_consecutive_sign_digit_separator: false,
            exponent_sign_digit_separator: false,
            exponent_consecutive_sign_digit_separator: false,
            base_prefix_internal_digit_separator: false,
            base_prefix_leading_digit_separator: false,
            base_prefix_trailing_digit_separator: false,
            base_prefix_consecutive_digit_separator: false,
            base_suffix_internal_digit_separator: false,
            base_suffix_leading_digit_separator: false,
            base_suffix_trailing_digit_separator: false,
            base_suffix_consecutive_digit_separator: false,
            integer_internal_digit_separator: false,
            fraction_internal_digit_separator: false,
            exponent_internal_digit_separator: false,
            integer_leading_digit_separator: false,
            fraction_leading_digit_separator: false,
            exponent_leading_digit_separator: false,
            integer_trailing_digit_separator: false,
            fraction_trailing_digit_separator: false,
            exponent_trailing_digit_separator: false,
            integer_consecutive_digit_separator: false,
            fraction_consecutive_digit_separator: false,
            exponent_consecutive_digit_separator: false,
            special_digit_separator: false,
        }
    }

    /// Create new [`NumberFormatBuilder`] without any flags set.
    ///
    /// This only sets the default radix to 10.
    #[inline(always)]
    pub const fn none() -> Self {
        Self {
            digit_separator: None,
            base_prefix: None,
            base_suffix: None,
            mantissa_radix: 10,
            exponent_base: None,
            exponent_radix: None,
            required_integer_digits: false,
            required_fraction_digits: false,
            required_exponent_digits: false,
            required_mantissa_digits: false,
            no_positive_mantissa_sign: false,
            required_mantissa_sign: false,
            no_exponent_notation: false,
            no_positive_exponent_sign: false,
            required_exponent_sign: false,
            no_exponent_without_fraction: false,
            no_special: false,
            case_sensitive_special: false,
            no_integer_leading_zeros: false,
            no_float_leading_zeros: false,
            required_exponent_notation: false,
            case_sensitive_exponent: false,
            case_sensitive_base_prefix: false,
            case_sensitive_base_suffix: false,
            required_integer_digits_with_exponent: false,
            required_fraction_digits_with_exponent: false,
            required_mantissa_digits_with_exponent: false,
            required_base_prefix: false,
            required_base_suffix: false,
            no_unsigned_negative_sign: true,
            no_mantissa_sign: false,
            no_exponent_sign: false,
            start_digit_separator: true,
            integer_sign_digit_separator: false,
            integer_consecutive_sign_digit_separator: false,
            exponent_sign_digit_separator: false,
            exponent_consecutive_sign_digit_separator: false,
            base_prefix_internal_digit_separator: false,
            base_prefix_leading_digit_separator: false,
            base_prefix_trailing_digit_separator: false,
            base_prefix_consecutive_digit_separator: false,
            base_suffix_internal_digit_separator: false,
            base_suffix_leading_digit_separator: false,
            base_suffix_trailing_digit_separator: false,
            base_suffix_consecutive_digit_separator: false,
            integer_internal_digit_separator: false,
            fraction_internal_digit_separator: false,
            exponent_internal_digit_separator: false,
            integer_leading_digit_separator: false,
            fraction_leading_digit_separator: false,
            exponent_leading_digit_separator: false,
            integer_trailing_digit_separator: false,
            fraction_trailing_digit_separator: false,
            exponent_trailing_digit_separator: false,
            integer_consecutive_digit_separator: false,
            fraction_consecutive_digit_separator: false,
            exponent_consecutive_digit_separator: false,
            special_digit_separator: false,
        }
    }

    /// Create number format for standard, binary number.
    #[cfg(feature = "power-of-two")]
    pub const fn binary() -> u128 {
        Self::from_radix(2)
    }

    /// Create number format for standard, octal number.
    #[cfg(feature = "power-of-two")]
    pub const fn octal() -> u128 {
        Self::from_radix(8)
    }

    /// Create number format for standard, decimal number.
    pub const fn decimal() -> u128 {
        let mut builder = Self::new();
        builder.mantissa_radix = 10;
        builder.exponent_base = num::NonZeroU8::new(10);
        builder.exponent_radix = num::NonZeroU8::new(10);
        builder.build_strict()
    }

    /// Create number format for standard, hexadecimal number.
    #[cfg(feature = "power-of-two")]
    pub const fn hexadecimal() -> u128 {
        Self::from_radix(16)
    }

    /// Create number format from radix.
    ///
    /// <div class="warning">
    ///
    /// This function will never fail even if the radix is invalid. It is up to
    /// the caller to ensure the format is valid using
    /// [`NumberFormat::is_valid`]. Only radixes from `2` to `36` should be
    /// used.
    ///
    /// </div>
    ///
    /// [`NumberFormat::is_valid`]: crate::NumberFormat::is_valid
    // FIXME: Use `build_strict` when we can have a breaking change.
    #[allow(deprecated)]
    #[cfg(feature = "power-of-two")]
    pub const fn from_radix(radix: u8) -> u128 {
        Self::new()
            .radix(radix)
            .exponent_base(num::NonZeroU8::new(radix))
            .exponent_radix(num::NonZeroU8::new(radix))
            .build()
    }

    // GETTERS

    // NOTE: This contains a lot of tests for our tables that would spam our
    // documentation, so we hide them internally. See `scripts/docs.py` for
    // how the tests are generated and run. This assumes the `format` and
    // `radix` features are enabled.

    /// Get the digit separator for the number format.
    ///
    /// Digit separators are frequently used in number literals to group
    /// digits: `1,000,000` is a lot more readable than `1000000`, but
    /// the `,` characters should be ignored in the parsing of the number.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`None`], or no digit separators allowed.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` (note that the validity
    /// oh where a digit separator can appear depends on the other digit
    /// separator flags).
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1_4` | ✔️ |
    /// | `+_14` | ✔️ |
    /// | `+14e3_5` | ✔️ |
    /// | `1_d` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_digit_separator(&self) -> OptionU8 {
        self.digit_separator
    }

    /// Get the radix for mantissa digits.
    ///
    /// This is only used for the significant digits, that is, the integral and
    /// fractional components. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix`. Defaults
    /// to `10`.
    ///
    /// | Radix | String | Number |
    /// |:-:|:-:|:-:|
    /// | 2 | "10011010010" | 1234 |
    /// | 3 | "1200201" | 1234 |
    /// | 8 | "2322" | 1234 |
    /// | 10 | "1234" | 1234 |
    /// | 16 | "4d2" | 1234 |
    /// | 31 | "18p" | 1234 |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    #[inline(always)]
    pub const fn get_mantissa_radix(&self) -> u8 {
        self.mantissa_radix
    }

    /// Get the radix for the exponent.
    ///
    /// For example, in `1.234e3`, it means `1.234 * 10^3`, and the exponent
    /// base here is 10. Some programming languages, like C, support hex floats
    /// with an exponent base of 2, for example `0x1.8p3`, or `1.5 * 2^3`.
    /// Defaults to `10`. Can only be modified with [`feature`][crate#features]
    /// `power-of-two` or `radix`. Defaults to `10`.
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_exponent_base(&self) -> OptionU8 {
        self.exponent_base
    }

    /// Get the radix for exponent digits.
    ///
    /// This is only used for the exponent digits. We assume the radix for the
    /// significant digits ([`get_mantissa_radix`][Self::get_mantissa_radix]) is
    /// 10 as is the exponent base. Defaults to `10`. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix`. Defaults to `10`.
    ///
    /// | Radix | String | Number |
    /// |:-:|:-:|:-:|
    /// | 2 | "1.234^1100" | 1.234e9 |
    /// | 3 | "1.234^110" | 1.234e9 |
    /// | 8 | "1.234^14" | 1.234e9 |
    /// | 10 | "1.234^12" | 1.234e9 |
    /// | 16 | "1.234^c" | 1.234e9 |
    /// | 31 | "1.234^c" | 1.234e9 |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_exponent_radix(&self) -> OptionU8 {
        self.exponent_radix
    }

    /// Get the optional character for the base prefix.
    ///
    /// This character will come after a leading zero, so for example
    /// setting the base prefix to `x` means that a leading `0x` will
    /// be ignore, if present. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to [`None`], or no base prefix allowed. This is only
    /// used for writing numbers if [`required_base_prefix`] is [`true`].
    /// This is ignored for special floating-point numbers.
    ///
    /// Note that base prefixes are not allowed for the exponent digits
    /// of a float: the prefix (such as `0x`) must come before any other
    /// significant digits, optionally with digit separators present.
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `x1` | ❌ |
    /// | `1` | ✔️ |
    /// | `1x` | ❌ |
    /// | `1x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_prefix`]: Self::get_required_base_prefix
    #[inline(always)]
    pub const fn get_base_prefix(&self) -> OptionU8 {
        self.base_prefix
    }

    /// Get the optional character for the base suffix.
    ///
    /// This character will at the end of the buffer, so for example
    /// setting the base prefix to `x` means that a trailing `x` will
    /// be ignored, if present.  Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to [`None`], or no base suffix allowed. This is only
    /// used for writing numbers if [`required_base_suffix`] is [`true`].
    /// This is ignored for special floating-point numbers.
    ///
    /// Note that base suffixes are not allowed for the exponent digits
    /// of a float: the suffix, if it's present, will always refer to the
    /// float as a whole, optionally with digit separators present. For
    /// example, `4d2p0101x` would have the `x` refer to the full float,
    /// not the exponent of `0101`.
    ///
    /// # Examples
    ///
    /// Using a base suffix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1x` | ✔️ |
    /// | `1d` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_suffix`]: Self::get_required_base_suffix
    #[inline(always)]
    pub const fn get_base_suffix(&self) -> OptionU8 {
        self.base_suffix
    }

    /// Get if digits are required before the decimal point.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `0.1` | ✔️ |
    /// | `1` | ✔️ |
    /// | `.1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_required_integer_digits(&self) -> bool {
        self.required_integer_digits
    }

    /// Get if digits are required after the decimal point, if the decimal point
    /// is present.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1` | ✔️ |
    /// | `1.` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_required_fraction_digits(&self) -> bool {
        self.required_fraction_digits
    }

    /// Get if digits are required after the exponent character, if the exponent
    /// is present.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`true`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e+3` | ✔️ |
    /// | `1.1e3` | ✔️ |
    /// | `1.1e+` | ❌ |
    /// | `1.1e` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_required_exponent_digits(&self) -> bool {
        self.required_exponent_digits
    }

    /// Get if at least 1 significant digit is required.
    ///
    /// If not required, then values like `.` (`0`) are valid, but empty strings
    /// are still invalid. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `.` | ✔️ |
    /// | `e10` | ✔️ |
    /// | `.e10` | ✔️ |
    /// |  | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_required_mantissa_digits(&self) -> bool {
        self.required_mantissa_digits
    }

    /// Get if a positive sign before the mantissa is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `-1.1` | ✔️ |
    /// | `+1.1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    #[inline(always)]
    pub const fn get_no_positive_mantissa_sign(&self) -> bool {
        self.no_positive_mantissa_sign
    }

    /// Get if a sign symbol before the mantissa is required.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ❌ |
    /// | `-1.1` | ✔️ |
    /// | `+1.1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    #[inline(always)]
    pub const fn get_required_mantissa_sign(&self) -> bool {
        self.required_mantissa_sign
    }

    /// Get if exponent notation is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1.1` | ✔️ |
    /// | `1.1e` | ❌ |
    /// | `1.1e5` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn get_no_exponent_notation(&self) -> bool {
        self.no_exponent_notation
    }

    /// Get if a positive sign before the exponent is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e3` | ✔️ |
    /// | `1.1e-3` | ✔️ |
    /// | `1.1e+3` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn get_no_positive_exponent_sign(&self) -> bool {
        self.no_positive_exponent_sign
    }

    /// Get if a sign symbol before the exponent is required.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e3` | ❌ |
    /// | `1.1e-3` | ✔️ |
    /// | `1.1e+3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn get_required_exponent_sign(&self) -> bool {
        self.required_exponent_sign
    }

    /// Get if an exponent without fraction is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1e3` | ❌ |
    /// | `1.e3` | ❌ |
    /// | `1.1e` | ✔️ |
    /// | `.1e3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_no_exponent_without_fraction(&self) -> bool {
        self.no_exponent_without_fraction
    }

    /// Get if special (non-finite) values are not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `NaN` | ❌ |
    /// | `inf` | ❌ |
    /// | `-Infinity` | ❌ |
    /// | `1.1e` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_no_special(&self) -> bool {
        self.no_special
    }

    /// Get if special (non-finite) values are case-sensitive.
    ///
    /// If set to [`true`], then `NaN` and `nan` are treated as the same value
    /// ([Not a Number][f64::NAN]). Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_case_sensitive_special(&self) -> bool {
        self.case_sensitive_special
    }

    /// Get if leading zeros before an integer are not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. This
    /// only applies if there is no base prefix: that is, the zeros are
    /// at the absolute start of the number. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// With a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `01` | ❌ |
    /// | `0` | ✔️ |
    /// | `10` | ✔️ |
    /// | `0x01` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_no_integer_leading_zeros(&self) -> bool {
        self.no_integer_leading_zeros
    }

    /// Get if leading zeros before a float are not allowed.
    ///
    /// This is before the significant digits of the float, that is, if there is
    /// 1 or more digits in the integral component and the leading digit is 0,
    /// Can only be modified with [`feature`][crate#features] `format`. This
    /// only applies if there is no base prefix: that is, the zeros are
    /// at the absolute start of the number. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// With a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `01` | ❌ |
    /// | `01.0` | ❌ |
    /// | `0` | ✔️ |
    /// | `10` | ✔️ |
    /// | `0x01.0` | ✔️ |
    /// | `0.1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_no_float_leading_zeros(&self) -> bool {
        self.no_float_leading_zeros
    }

    /// Get if exponent notation is required.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ❌ |
    /// | `1.0` | ❌ |
    /// | `1e3` | ✔️ |
    /// | `1.1e3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn get_required_exponent_notation(&self) -> bool {
        self.required_exponent_notation
    }

    /// Get if exponent characters are case-sensitive.
    ///
    /// If set to [`true`], then the exponent character `e` would be considered
    /// the different from `E`. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_case_sensitive_exponent(&self) -> bool {
        self.case_sensitive_exponent
    }

    /// Get if base prefixes are case-sensitive.
    ///
    /// If set to [`true`], then the base prefix `x` would be considered the
    /// different from `X`. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to [`false`]. This is only used for writing numbers
    /// if [`required_base_prefix`] is [`true`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_prefix`]: Self::get_required_base_prefix
    #[inline(always)]
    pub const fn get_case_sensitive_base_prefix(&self) -> bool {
        self.case_sensitive_base_prefix
    }

    /// Get if base suffixes are case-sensitive.
    ///
    /// If set to [`true`], then the base suffix `x` would be considered the
    /// different from `X`. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to [`false`]. This is only used for writing numbers
    /// if [`required_base_suffix`] is [`true`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_suffix`]: Self::get_required_base_suffix
    #[inline(always)]
    pub const fn get_case_sensitive_base_suffix(&self) -> bool {
        self.case_sensitive_base_suffix
    }

    /// Get if digits are required before the decimal point with exponent
    /// notation.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `.1e5` | ❌ |
    /// | `.e5` | ❌ |
    /// | `1.e5` | ✔️ |
    /// | `1.0e5` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_required_integer_digits_with_exponent(&self) -> bool {
        self.required_integer_digits_with_exponent
    }

    /// Get if digits are required after the decimal point with exponent
    /// notation, if the decimal point is present.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `.1e5` | ✔️ |
    /// | `.e5` | ❌ |
    /// | `1.e5` | ❌ |
    /// | `1.0e5` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_required_fraction_digits_with_exponent(&self) -> bool {
        self.required_fraction_digits_with_exponent
    }

    /// Get if any significant digits are required with exponent notation.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `.1e5` | ✔️ |
    /// | `.e5` | ❌ |
    /// | `1.e5` | ✔️ |
    /// | `1.0e5` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_required_mantissa_digits_with_exponent(&self) -> bool {
        self.required_mantissa_digits_with_exponent
    }

    /// Get if the format requires base prefixes.
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `4d2` | ❌ |
    /// | `x4d2` | ❌ |
    /// | `4d2x` | ❌ |
    /// | `0x4d2` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Write Float
    /// - Write Integer
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_required_base_prefix(&self) -> bool {
        self.required_base_prefix
    }

    /// Get if the format requires base suffixes.
    ///
    /// This will override most errors, including all invalid digit errors.
    ///
    /// # Examples
    ///
    /// Using a base suffix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `4d2` | ❌ |
    /// | `x4d2` | ❌ |
    /// | `4d2x` | ✔️ |
    /// | `0x4d2` | ❌ |
    ///
    /// # Used For
    ///
    /// - Write Float
    /// - Write Integer
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_required_base_suffix(&self) -> bool {
        self.required_base_suffix
    }

    /// Get if a negative sign before an unsigned integer is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. This
    /// does not apply to signed integers or floating point numbers.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `-12` | ❌ |
    /// | `+12` | ✔️ |
    /// | `12` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_no_unsigned_negative_sign(&self) -> bool {
        self.no_unsigned_negative_sign
    }

    /// Get if positive or negative signs before the significant digits are not
    /// allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. if
    /// enabled, then the type cannot represent negative literal or string
    /// values (although they may be computed via mathematical operations).
    ///
    /// If you only want to disable positive signs, see
    /// [`no_positive_mantissa_sign`]. If you wish to disable negative signs
    /// on unsigned integers, see [`no_unsigned_negative_sign`].
    ///
    /// [`no_positive_mantissa_sign`]: Self::get_no_positive_mantissa_sign
    /// [`no_unsigned_negative_sign`]: Self::get_no_unsigned_negative_sign
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `-12` | ❌ |
    /// | `+12` | ❌ |
    /// | `12` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Integer
    /// - Parse Float
    #[inline(always)]
    pub const fn get_no_mantissa_sign(&self) -> bool {
        self.no_mantissa_sign
    }

    /// Get if positive or negative signs before an exponent are not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`.
    ///
    /// If you only want to disable positive signs, see
    /// [`no_positive_exponent_sign`].
    ///
    /// [`no_positive_exponent_sign`]: Self::get_no_positive_exponent_sign
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.0e-12` | ❌ |
    /// | `1.0e+12` | ❌ |
    /// | `1.0e12` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_no_exponent_sign(&self) -> bool {
        self.no_exponent_sign
    }

    /// Get if digit separators are allowed at the absolute start of the number.
    ///
    /// This modifies the behavior of [`integer_sign_digit_separator`] and
    /// [`integer_leading_digit_separator`] so that the digit separators can
    /// occur at the absolute start and not just internal to the number.
    ///
    /// [`integer_sign_digit_separator`]: Self::get_integer_sign_digit_separator
    /// [`integer_leading_digit_separator`]: Self::get_integer_leading_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, integer leading digit separators
    /// enabled, and starting digit separators enabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `_1` | ✔️ |
    /// | `_+1` | ❌ |
    /// | `+_1` | ✔️ |
    /// | `+0x_1` | ✔️ |
    /// | `+_0x1` | ❌ |
    ///
    /// Using a digit separator of `_`, integer leading digit separators
    /// enabled, and starting digit separators disabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `_1` | ❌ |
    /// | `_+1` | ❌ |
    /// | `+_1` | ✔️ |
    /// | `+0x_1` | ✔️ |
    /// | `+_0x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_start_digit_separator(&self) -> bool {
        self.start_digit_separator
    }

    /// Get if digit separators are allowed before the sign of the integer.
    ///
    /// You can disable support for digit separators at the absolute start
    /// using the [`start_digit_separator`] flag.
    ///
    /// [`start_digit_separator`]: Self::get_start_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `_1` | ❌ |
    /// | `_+1` | ✔️ |
    /// | `+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_integer_sign_digit_separator(&self) -> bool {
        self.integer_sign_digit_separator
    }

    /// Get if consecutive digit separators are allowed before the sign of the
    /// integer.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `_1` | ❌ |
    /// | `_+1` | ✔️ |
    /// | `__+1` | ✔️ |
    /// | `+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_integer_consecutive_sign_digit_separator(&self) -> bool {
        self.integer_consecutive_sign_digit_separator
    }

    /// Get if digit separators are allowed before the sign of the exponent.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.0e1` | ✔️ |
    /// | `1.0e_` | ❌ |
    /// | `1.0e_1` | ❌ |
    /// | `1.0e_+1` | ✔️ |
    /// | `1.0e+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_exponent_sign_digit_separator(&self) -> bool {
        self.exponent_sign_digit_separator
    }

    /// Get if consecutive digit separators are allowed before the sign of the
    /// exponent.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.0e1` | ✔️ |
    /// | `1.0e_` | ❌ |
    /// | `1.0e_1` | ❌ |
    /// | `1.0e_+1` | ✔️ |
    /// | `1.0e__+1` | ✔️ |
    /// | `1.0e+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_exponent_consecutive_sign_digit_separator(&self) -> bool {
        self.exponent_consecutive_sign_digit_separator
    }

    /// Get if a digit separator is allowed between the `0` and the base prefix.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, and a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ❌ |
    /// | `0x_1` | ❌ |
    /// | `0_x1` | ✔️ |
    /// | `+0_x1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_prefix_internal_digit_separator(&self) -> bool {
        self.base_prefix_internal_digit_separator
    }

    /// Get if a digit separator is allowed before the base prefix.
    ///
    /// This is taken **after** the sign and therefore  may be at the absolute
    /// start of the number or internal to it. You can disable support for
    /// digit separators at the absolute start using the
    /// [`start_digit_separator`] flag.
    ///
    /// [`start_digit_separator`]: Self::get_start_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, a base prefix of `x`, and starting digit
    /// separators enabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ✔️ |
    /// | `0x_1` | ❌ |
    /// | `0_x1` | ❌ |
    /// | `+_0x1` | ✔️ |
    /// | `_+0x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_prefix_leading_digit_separator(&self) -> bool {
        self.base_prefix_leading_digit_separator
    }

    /// Get if a digit separator is allowed after the base prefix.
    ///
    /// If a base prefix is present, then [`integer_leading_digit_separator`]
    /// overlaps with this functionality, and either being enabled, as well
    /// as consecutive digit separators, will consider it to be valid. If
    /// both are set and neither enables consecutive digit separators, then
    /// `0x__1` is not valid.
    ///
    /// [`integer_leading_digit_separator`]: Self::get_integer_leading_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` and a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ❌ |
    /// | `0_x1` | ❌ |
    /// | `0x_1` | ✔️ |
    /// | `0x__1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_prefix_trailing_digit_separator(&self) -> bool {
        self.base_prefix_trailing_digit_separator
    }

    /// Get if multiple consecutive base prefix digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// base prefix. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, a base prefix of `x`, and trailing base
    /// prefix digit separators.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ❌ |
    /// | `0_x1` | ❌ |
    /// | `0x_1` | ✔️ |
    /// | `0x__1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_prefix_consecutive_digit_separator(&self) -> bool {
        self.base_prefix_consecutive_digit_separator
    }

    /// Get if a digit separator is allowed between the base suffix.
    ///
    /// This is currently unused, since base suffixes are a single character.
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_suffix_internal_digit_separator(&self) -> bool {
        self.base_suffix_internal_digit_separator
    }

    /// Get if a digit separator is allowed before the base suffix.
    ///
    /// If a base suffix is present, then [`integer_trailing_digit_separator`]
    /// or [`exponent_trailing_digit_separator`] may overlap
    /// with this functionality (depending on the number components), and either
    /// being enabled, as well as consecutive digit separators, will
    /// consider it to be valid. If both are set and neither
    /// enables consecutive digit separators, then `1__h` is not valid.
    ///
    /// [`integer_trailing_digit_separator`]: Self::get_integer_trailing_digit_separator
    /// [`exponent_trailing_digit_separator`]: Self::get_exponent_trailing_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` and a base suffix of `h`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `12h` | ✔️ |
    /// | `1_2h` | ❌ |
    /// | `12_h` | ✔️ |
    /// | `12__h` | ✔️ |
    /// | `12h_` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_suffix_leading_digit_separator(&self) -> bool {
        self.base_suffix_leading_digit_separator
    }

    /// Get if a digit separator is allowed after the base suffix.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` and a base suffix of `h`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `12h` | ✔️ |
    /// | `1_2h` | ❌ |
    /// | `12_h` | ❌ |
    /// | `12h_` | ✔️ |
    /// | `12h__` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_suffix_trailing_digit_separator(&self) -> bool {
        self.base_suffix_trailing_digit_separator
    }

    /// Get if multiple consecutive base suffix digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// base suffix. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, a base suffix of `h`, and trailing base
    /// suffix digit separators.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `12h` | ✔️ |
    /// | `1_2h` | ❌ |
    /// | `12_h` | ❌ |
    /// | `12h_` | ✔️ |
    /// | `12h__` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_base_suffix_consecutive_digit_separator(&self) -> bool {
        self.base_suffix_consecutive_digit_separator
    }

    /// Get if digit separators are allowed between integer digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ✔️ |
    /// | `1_` | ❌ |
    /// | `_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_integer_internal_digit_separator(&self) -> bool {
        self.integer_internal_digit_separator
    }

    /// Get if digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ✔️ |
    /// | `1.1_` | ❌ |
    /// | `1._1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_fraction_internal_digit_separator(&self) -> bool {
        self.fraction_internal_digit_separator
    }

    /// Get if digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ✔️ |
    /// | `1.1e1_` | ❌ |
    /// | `1.1e_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_exponent_internal_digit_separator(&self) -> bool {
        self.exponent_internal_digit_separator
    }

    /// Get if a digit separator is allowed before any integer digits.
    ///
    /// This is taken **after** any signs and base prefixes, and therefore
    /// may be at the absolute start of the number or internal to it. You
    /// can disable support for digit separators at the absolute start
    /// using the [`start_digit_separator`] flag. For only those before or after
    /// a base prefix, see [`base_prefix_leading_digit_separator`] and
    /// [`base_prefix_trailing_digit_separator`], respectively. For only those
    /// prior to a sign, see [`integer_sign_digit_separator`].
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Defaults to [`false`].
    ///
    /// [`start_digit_separator`]: Self::get_start_digit_separator
    /// [`base_prefix_leading_digit_separator`]: Self::get_base_prefix_leading_digit_separator
    /// [`base_prefix_trailing_digit_separator`]: Self::get_base_prefix_trailing_digit_separator
    /// [`integer_sign_digit_separator`]: Self::get_integer_sign_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, and an optional base prefix of
    /// `x` and starting digit separators enabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ❌ |
    /// | `1_` | ❌ |
    /// | `_1` | ✔️ |
    /// | `_+1` | ❌ |
    /// | `+_1` | ✔️ |
    /// | `+0x_1` | ✔️ |
    /// | `+_0x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_integer_leading_digit_separator(&self) -> bool {
        self.integer_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ❌ |
    /// | `1.1_` | ❌ |
    /// | `1._1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_fraction_leading_digit_separator(&self) -> bool {
        self.fraction_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ❌ |
    /// | `1.1e1_` | ❌ |
    /// | `1.1e_1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_exponent_leading_digit_separator(&self) -> bool {
        self.exponent_leading_digit_separator
    }

    /// Get if a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ❌ |
    /// | `1_` | ✔️ |
    /// | `_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_integer_trailing_digit_separator(&self) -> bool {
        self.integer_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`]. # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ❌ |
    /// | `1.1_` | ✔️ |
    /// | `1._1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_fraction_trailing_digit_separator(&self) -> bool {
        self.fraction_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ❌ |
    /// | `1.1e1_` | ✔️ |
    /// | `1.1e_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_exponent_trailing_digit_separator(&self) -> bool {
        self.exponent_trailing_digit_separator
    }

    /// Get if multiple consecutive integer digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// integer. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn get_integer_consecutive_digit_separator(&self) -> bool {
        self.integer_consecutive_digit_separator
    }

    /// Get if multiple consecutive fraction digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// fraction. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_fraction_consecutive_digit_separator(&self) -> bool {
        self.fraction_consecutive_digit_separator
    }

    /// Get if multiple consecutive exponent digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// exponent. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_exponent_consecutive_digit_separator(&self) -> bool {
        self.exponent_consecutive_digit_separator
    }

    /// Get if any digit separators are allowed in special (non-finite) values.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for any special floats: for example, `N__a_N_` is considered
    /// the same as `NaN`. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn get_special_digit_separator(&self) -> bool {
        self.special_digit_separator
    }

    // SETTERS

    /// Set the digit separator for the number format.
    ///
    /// Digit separators are frequently used in number literals to group
    /// digits: `1,000,000` is a lot more readable than `1000000`, but
    /// the `,` characters should be ignored in the parsing of the number.
    ///
    /// Defaults to [`None`], or no digit separators allowed.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` (note that the validity
    /// oh where a digit separator can appear depends on the other digit
    /// separator flags).
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1_4` | ✔️ |
    /// | `+_14` | ✔️ |
    /// | `+14e3_5` | ✔️ |
    /// | `1_d` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .leading_digit_separator(true)
    ///     .internal_digit_separator(true)
    ///     .trailing_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_4", &PF_OPTS), Ok(14.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+_14", &PF_OPTS), Ok(14.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+14e3_5", &PF_OPTS), Ok(14e35));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_d", &PF_OPTS), Err(Error::InvalidDigit(2)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_4", &PI_OPTS), Ok(14));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+_14", &PI_OPTS), Ok(14));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_d", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn digit_separator(mut self, character: OptionU8) -> Self {
        self.digit_separator = character;
        self
    }

    /// Alias for [`mantissa radix`][Self::mantissa_radix].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn radix(self, radix: u8) -> Self {
        self.mantissa_radix(radix)
    }

    /// Set the radix for mantissa digits.
    ///
    /// This is only used for the significant digits, that is, the integral and
    /// fractional components. Defaults to `10`.
    ///
    /// | Radix | String | Number |
    /// |:-:|:-:|:-:|
    /// | 2 | "10011010010" | 1234 |
    /// | 3 | "1200201" | 1234 |
    /// | 8 | "2322" | 1234 |
    /// | 10 | "1234" | 1234 |
    /// | 16 | "4d2" | 1234 |
    /// | 31 | "18p" | 1234 |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const BASE2: u128 = NumberFormatBuilder::from_radix(2);
    /// const BASE3: u128 = NumberFormatBuilder::from_radix(3);
    /// const BASE8: u128 = NumberFormatBuilder::from_radix(8);
    /// const BASE10: u128 = NumberFormatBuilder::from_radix(10);
    /// const BASE16: u128 = NumberFormatBuilder::from_radix(16);
    /// const BASE31: u128 = NumberFormatBuilder::from_radix(31);
    /// const PI_RDX: ParseIntegerOptions = ParseIntegerOptions::from_radix(16);
    /// const PF_RDX: ParseFloatOptions = ParseFloatOptions::from_radix(16);
    /// const WI_RDX: WriteIntegerOptions = WriteIntegerOptions::from_radix(16);
    /// const WF_RDX: WriteFloatOptions = WriteFloatOptions::from_radix(16);
    ///
    /// assert_eq!(parse_with_options::<f64, BASE2>(b"10011010010", &PF_RDX), Ok(1234.0));
    /// assert_eq!(parse_with_options::<f64, BASE3>(b"1200201", &PF_RDX), Ok(1234.0));
    /// assert_eq!(parse_with_options::<f64, BASE8>(b"2322", &PF_RDX), Ok(1234.0));
    /// assert_eq!(parse_with_options::<f64, BASE10>(b"1234", &PF_RDX), Ok(1234.0));
    /// assert_eq!(parse_with_options::<f64, BASE16>(b"4d2", &PF_RDX), Ok(1234.0));
    /// assert_eq!(parse_with_options::<f64, BASE31>(b"18p", &PF_RDX), Ok(1234.0));
    ///
    /// assert_eq!(parse_with_options::<i64, BASE2>(b"10011010010", &PI_RDX), Ok(1234));
    /// assert_eq!(parse_with_options::<i64, BASE3>(b"1200201", &PI_RDX), Ok(1234));
    /// assert_eq!(parse_with_options::<i64, BASE8>(b"2322", &PI_RDX), Ok(1234));
    /// assert_eq!(parse_with_options::<i64, BASE10>(b"1234", &PI_RDX), Ok(1234));
    /// assert_eq!(parse_with_options::<i64, BASE16>(b"4d2", &PI_RDX), Ok(1234));
    /// assert_eq!(parse_with_options::<i64, BASE31>(b"18p", &PI_RDX), Ok(1234));
    ///
    /// let mut buffer = [0u8; BUFFER_SIZE];
    /// assert_eq!(write_with_options::<f64, BASE2>(1234.0, &mut buffer, &WF_RDX), b"1.001101001^1010");
    /// assert_eq!(write_with_options::<f64, BASE3>(1234.0, &mut buffer, &WF_RDX), b"1200201.0");
    /// assert_eq!(write_with_options::<f64, BASE8>(1234.0, &mut buffer, &WF_RDX), b"2.322^3");
    /// assert_eq!(write_with_options::<f64, BASE10>(1234.0, &mut buffer, &WF_RDX), b"1234.0");
    /// assert_eq!(write_with_options::<f64, BASE16>(1234.0, &mut buffer, &WF_RDX), b"4.D2^2");
    /// assert_eq!(write_with_options::<f64, BASE31>(1234.0, &mut buffer, &WF_RDX), b"18P.0");
    ///
    /// assert_eq!(write_with_options::<i64, BASE2>(1234, &mut buffer, &WI_RDX), b"10011010010");
    /// assert_eq!(write_with_options::<i64, BASE3>(1234, &mut buffer, &WI_RDX), b"1200201");
    /// assert_eq!(write_with_options::<i64, BASE8>(1234, &mut buffer, &WI_RDX), b"2322");
    /// assert_eq!(write_with_options::<i64, BASE10>(1234, &mut buffer, &WI_RDX), b"1234");
    /// assert_eq!(write_with_options::<i64, BASE16>(1234, &mut buffer, &WI_RDX), b"4D2");
    /// assert_eq!(write_with_options::<i64, BASE31>(1234, &mut buffer, &WI_RDX), b"18P");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn mantissa_radix(mut self, radix: u8) -> Self {
        self.mantissa_radix = radix;
        self
    }

    /// Set the radix for the exponent.
    ///
    /// For example, in `1.234e3`, it means `1.234 * 10^3`, and the exponent
    /// base here is 10. Some programming languages, like C, support hex floats
    /// with an exponent base of 2, for example `0x1.8p3`, or `1.5 * 2^3`.
    /// Defaults to `10`.
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn exponent_base(mut self, base: OptionU8) -> Self {
        self.exponent_base = base;
        self
    }

    /// Set the radix for exponent digits.
    ///
    /// This is only used for the exponent digits. We assume the radix for the
    /// significant digits ([`mantissa_radix`][Self::mantissa_radix]) is 10 as
    /// is the exponent base. Defaults to `10`.
    ///
    /// | Radix | String | Number |
    /// |:-:|:-:|:-:|
    /// | 2 | "1.234^1100" | 1.234e9 |
    /// | 3 | "1.234^110" | 1.234e9 |
    /// | 8 | "1.234^14" | 1.234e9 |
    /// | 10 | "1.234^12" | 1.234e9 |
    /// | 16 | "1.234^c" | 1.234e9 |
    /// | 31 | "1.234^c" | 1.234e9 |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// macro_rules! exp_radix {
    ///     ($exp:literal) => {
    ///         NumberFormatBuilder::new()
    ///             .mantissa_radix(10)
    ///             .exponent_base(num::NonZeroU8::new(10))
    ///             .exponent_radix(num::NonZeroU8::new($exp))
    ///             .build_strict()
    ///     };
    /// }
    /// const BASE2: u128 = exp_radix!(2);
    /// const BASE3: u128 = exp_radix!(3);
    /// const BASE8: u128 = exp_radix!(8);
    /// const BASE10: u128 = exp_radix!(10);
    /// const BASE16: u128 = exp_radix!(16);
    /// const BASE31: u128 = exp_radix!(31);
    /// const PF_RDX: ParseFloatOptions = ParseFloatOptions::from_radix(16);
    /// const WF_RDX: WriteFloatOptions = WriteFloatOptions::from_radix(16);
    ///
    /// assert_eq!(parse_with_options::<f64, BASE2>(b"1.234^1100", &PF_RDX), Ok(1234e9));
    /// assert_eq!(parse_with_options::<f64, BASE3>(b"1.234^110", &PF_RDX), Ok(1234e9));
    /// assert_eq!(parse_with_options::<f64, BASE8>(b"1.234^14", &PF_RDX), Ok(1234e9));
    /// assert_eq!(parse_with_options::<f64, BASE10>(b"1.234^12", &PF_RDX), Ok(1234e9));
    /// assert_eq!(parse_with_options::<f64, BASE16>(b"1.234^c", &PF_RDX), Ok(1234e9));
    /// assert_eq!(parse_with_options::<f64, BASE31>(b"1.234^c", &PF_RDX), Ok(1234e9));
    ///
    /// let mut buffer = [0u8; BUFFER_SIZE];
    /// assert_eq!(write_with_options::<f64, BASE2>(1234e9, &mut buffer, &WF_RDX), b"1.234^1100");
    /// assert_eq!(write_with_options::<f64, BASE3>(1234e9, &mut buffer, &WF_RDX), b"1.234^110");
    /// assert_eq!(write_with_options::<f64, BASE8>(1234e9, &mut buffer, &WF_RDX), b"1.234^14");
    /// assert_eq!(write_with_options::<f64, BASE10>(1234e9, &mut buffer, &WF_RDX), b"1.234^12");
    /// assert_eq!(write_with_options::<f64, BASE16>(1234e9, &mut buffer, &WF_RDX), b"1.234^C");
    /// assert_eq!(write_with_options::<f64, BASE31>(1234e9, &mut buffer, &WF_RDX), b"1.234^C");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn exponent_radix(mut self, radix: OptionU8) -> Self {
        self.exponent_radix = radix;
        self
    }

    /// Set the optional character for the base prefix.
    ///
    /// This character will come after a leading zero, so for example
    /// setting the base prefix to `x` means that a leading `0x` will
    /// be ignore, if present. Defaults to [`None`], or no base prefix
    /// allowed. This is only used for writing numbers if
    /// [`required_base_prefix`] is [`true`]. This is ignored for special
    /// floating-point numbers.
    ///
    /// Note that base prefixes are not allowed for the exponent digits
    /// of a float: the prefix (such as `0x`) must come before any other
    /// significant digits, optionally with digit separators present.
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `x1` | ❌ |
    /// | `1` | ✔️ |
    /// | `1x` | ❌ |
    /// | `1x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_prefix`]: Self::required_base_prefix
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'x'))
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0x1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"x1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1x", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1x1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0x1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"x1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1x", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1x1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn base_prefix(mut self, base_prefix: OptionU8) -> Self {
        self.base_prefix = base_prefix;
        self
    }

    /// Set the optional character for the base suffix.
    ///
    /// This character will at the end of the buffer, so for example
    /// setting the base prefix to `x` means that a trailing `x` will
    /// be ignored, if present. Defaults to [`None`], or no base suffix
    /// allowed. This is only used for writing numbers if
    /// [`required_base_suffix`] is [`true`]. This is ignored for special
    /// floating-point numbers.
    ///
    /// Note that base suffixes are not allowed for the exponent digits
    /// of a float: the suffix, if it's present, will always refer to the
    /// float as a whole, optionally with digit separators present. For
    /// example, `4d2p0101x` would have the `x` refer to the full float,
    /// not the exponent of `0101`.
    ///
    /// # Examples
    ///
    /// Using a base suffix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1x` | ✔️ |
    /// | `1d` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_suffix`]: Self::required_base_suffix
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_suffix(num::NonZeroU8::new(b'x'))
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0x1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"x1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1x", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1x1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    ///
    /// // TODO: FIXME! This is incorrectly getting the location wrong
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0x1", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"x1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1x", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1x1", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn base_suffix(mut self, base_suffix: OptionU8) -> Self {
        self.base_suffix = base_suffix;
        self
    }

    /// Set if digits are required before the decimal point.
    ///
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `0.1` | ✔️ |
    /// | `1` | ✔️ |
    /// | `.1` | ❌ |
    /// | `1.` | ❌ |
    /// |  | ❌ |
    ///
    ///  # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_integer_digits(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0.1", &PF_OPTS), Ok(0.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".1", &PF_OPTS), Err(Error::EmptyInteger(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_integer_digits(mut self, flag: bool) -> Self {
        self.required_integer_digits = flag;
        self
    }

    /// Set if digits are required after the decimal point, if the decimal point
    /// is present.
    ///
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `0.1` | ✔️ |
    /// | `1` | ✔️ |
    /// | `.1` | ✔️ |
    /// | `1.` | ❌ |
    /// | | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_fraction_digits(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0.1", &PF_OPTS), Ok(0.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.", &PF_OPTS), Err(Error::EmptyFraction(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".1", &PF_OPTS), Ok(0.1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_fraction_digits(mut self, flag: bool) -> Self {
        self.required_fraction_digits = flag;
        self
    }

    /// Set if digits are required after the exponent character, if the exponent
    /// is present.
    ///
    /// Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e+3` | ✔️ |
    /// | `1.1e3` | ✔️ |
    /// | `1.1e+` | ❌ |
    /// | `1.1e` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_fraction_digits(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e+3", &PF_OPTS), Ok(1.1e3));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e3", &PF_OPTS), Ok(1.1e3));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e+", &PF_OPTS), Err(Error::EmptyExponent(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_exponent_digits(mut self, flag: bool) -> Self {
        self.required_exponent_digits = flag;
        self
    }

    /// Set if at least 1 significant digit is required.
    ///
    /// If not required, then values like `.` (`0`) are valid, but empty strings
    /// are still invalid. Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `.` | ✔️ |
    /// | `e10` | ❌ |
    /// | `.e10` | ❌ |
    /// | | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_mantissa_digits(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".", &PF_OPTS), Err(Error::EmptyMantissa(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"e10", &PF_OPTS), Err(Error::EmptyMantissa(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".e10", &PF_OPTS), Err(Error::EmptyMantissa(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"", &PF_OPTS), Err(Error::Empty(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_mantissa_digits(mut self, flag: bool) -> Self {
        self.required_mantissa_digits = flag;
        self
    }

    /// Set if digits are required for all float components.
    ///
    /// Note that digits are **always** required for integers. Defaults
    /// to requiring digits only for the mantissa and exponent.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1.1e3` | ✔️ |
    /// | `1.1e` | ✔️ |
    /// | `0.1` | ✔️ |
    /// | `.1` | ❌ |
    /// | `1.` | ❌ |
    /// | `e10` | ❌ |
    /// | `.1e10` | ❌ |
    /// | | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_digits(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e3", &PF_OPTS), Ok(1.1e3));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0.1", &PF_OPTS), Ok(0.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".", &PF_OPTS), Err(Error::EmptyInteger(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"e10", &PF_OPTS), Err(Error::EmptyInteger(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".e10", &PF_OPTS), Err(Error::EmptyInteger(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"", &PF_OPTS), Err(Error::Empty(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_digits(mut self, flag: bool) -> Self {
        self = self.required_integer_digits(flag);
        self = self.required_fraction_digits(flag);
        self = self.required_exponent_digits(flag);
        self = self.required_mantissa_digits(flag);
        self
    }

    /// Set if a positive sign before the mantissa is not allowed.
    ///
    /// Defaults to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `-1.1` | ✔️ |
    /// | `+1.1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .no_positive_mantissa_sign(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"-1.1", &PF_OPTS), Ok(-1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+1.1", &PF_OPTS), Err(Error::InvalidPositiveMantissaSign(0)));
    ///
    /// let mut buffer = [0u8; BUFFER_SIZE];
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1, &mut buffer, &WF_OPTS), b"1.1");
    /// assert_eq!(write_with_options::<f64, FORMAT>(-1.1, &mut buffer, &WF_OPTS), b"-1.1");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_positive_mantissa_sign(mut self, flag: bool) -> Self {
        self.no_positive_mantissa_sign = flag;
        self
    }

    /// Set if a sign symbol before the mantissa is required.
    ///
    /// Defaults to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ❌ |
    /// | `-1.1` | ✔️ |
    /// | `+1.1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_mantissa_sign(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Err(Error::MissingMantissaSign(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"-1.1", &PF_OPTS), Ok(-1.1));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Err(Error::MissingSign(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"-1", &PI_OPTS), Ok(-1));
    ///
    /// let mut buffer = [0u8; BUFFER_SIZE];
    /// assert_eq!(write_with_options::<f64, FORMAT>(-1.0, &mut buffer, &WF_OPTS), b"-1.0");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.0, &mut buffer, &WF_OPTS), b"+1.0");
    ///
    /// assert_eq!(write_with_options::<i64, FORMAT>(-1, &mut buffer, &WI_OPTS), b"-1");
    /// assert_eq!(write_with_options::<i64, FORMAT>(1, &mut buffer, &WI_OPTS), b"+1");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_mantissa_sign(mut self, flag: bool) -> Self {
        self.required_mantissa_sign = flag;
        self
    }

    /// Set if exponent notation is not allowed.
    ///
    /// Defaults to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1.1` | ✔️ |
    /// | `1.1e` | ❌ |
    /// | `1.1e5` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .no_exponent_notation(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e", &PF_OPTS), Err(Error::InvalidExponent(3)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e5", &PF_OPTS), Err(Error::InvalidExponent(3)));
    ///
    /// const SIZE: usize = WF_OPTS.buffer_size_const::<f64, FORMAT>();
    /// let mut buffer = [0u8; SIZE];
    /// assert_eq!(write(1.0e10, &mut buffer), b"1.0e10");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.0, &mut buffer, &WF_OPTS), b"1.0");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1, &mut buffer, &WF_OPTS), b"1.1");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.0e10, &mut buffer, &WF_OPTS), b"10000000000.0");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_exponent_notation(mut self, flag: bool) -> Self {
        self.no_exponent_notation = flag;
        self
    }

    /// Set if a positive sign before the exponent is not allowed.
    ///
    /// Defaults to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e3` | ✔️ |
    /// | `1.1e-3` | ✔️ |
    /// | `1.1e+3` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .no_positive_exponent_sign(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e15", &PF_OPTS), Ok(1.1e15));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e+15", &PF_OPTS), Err(Error::InvalidPositiveExponentSign(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e-15", &PF_OPTS), Ok(1.1e-15));
    ///
    /// let mut buffer = [0u8; BUFFER_SIZE];
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1e15, &mut buffer, &WF_OPTS), b"1.1e15");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1e-15, &mut buffer, &WF_OPTS), b"1.1e-15");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_positive_exponent_sign(mut self, flag: bool) -> Self {
        self.no_positive_exponent_sign = flag;
        self
    }

    /// Set if a sign symbol before the exponent is required.
    ///
    /// Defaults to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e3` | ❌ |
    /// | `1.1e-3` | ✔️ |
    /// | `1.1e+3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_exponent_sign(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e15", &PF_OPTS), Err(Error::MissingExponentSign(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e+15", &PF_OPTS), Ok(1.1e15));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e-15", &PF_OPTS), Ok(1.1e-15));
    ///
    /// let mut buffer = [0u8; BUFFER_SIZE];
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1e15, &mut buffer, &WF_OPTS), b"1.1e+15");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1e-15, &mut buffer, &WF_OPTS), b"1.1e-15");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_exponent_sign(mut self, flag: bool) -> Self {
        self.required_exponent_sign = flag;
        self
    }

    /// Set if an exponent without fraction is not allowed.
    ///
    /// Defaults to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1e3` | ❌ |
    /// | `1.e3` | ✔️ |
    /// | `1.1e` | ✔️ |
    /// | `.1e3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .no_exponent_without_fraction(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1e3", &PF_OPTS), Err(Error::ExponentWithoutFraction(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.e3", &PF_OPTS), Ok(1000.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e3", &PF_OPTS), Ok(1.1e3));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".1e3", &PF_OPTS), Ok(1.0e2));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_exponent_without_fraction(mut self, flag: bool) -> Self {
        self.no_exponent_without_fraction = flag;
        self
    }

    /// Set if special (non-finite) values are not allowed.
    ///
    /// Defaults to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `NaN` | ❌ |
    /// | `inf` | ❌ |
    /// | `-Infinity` | ❌ |
    /// | `1.1e3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .no_special(true)
    ///     .build_strict();
    /// assert_eq!(parse::<f64>(b"NaN").map(|x| x.is_nan()), Ok(true));
    /// assert_eq!(parse::<f64>(b"inf"), Ok(f64::INFINITY));
    /// assert_eq!(parse::<f64>(b"infinity"), Ok(f64::INFINITY));
    /// assert_eq!(parse::<f64>(b"1.1e3"), Ok(1.1e3));
    ///
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"NaN", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"inf", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"infinity", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e3", &PF_OPTS), Ok(1.1e3));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_special(mut self, flag: bool) -> Self {
        self.no_special = flag;
        self
    }

    /// Set if special (non-finite) values are case-sensitive.
    ///
    /// If set to [`true`], then `NaN` and `nan` are treated as the same value
    /// ([Not a Number][f64::NAN]). Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `nan` | ❌ |
    /// | `NaN` | ✔️ |
    /// | `inf` | ✔️ |
    /// | `Inf` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .case_sensitive_special(true)
    ///     .build_strict();
    /// assert_eq!(parse::<f64>(b"nan").map(|x| x.is_nan()), Ok(true));
    /// assert_eq!(parse::<f64>(b"NaN").map(|x| x.is_nan()), Ok(true));
    /// assert_eq!(parse::<f64>(b"inf"), Ok(f64::INFINITY));
    /// assert_eq!(parse::<f64>(b"Inf"), Ok(f64::INFINITY));
    /// assert_eq!(parse::<f64>(b"1.1e3"), Ok(1.1e3));
    ///
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"nan", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"NaN", &PF_OPTS).map(|x| x.is_nan()), Ok(true));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"inf", &PF_OPTS), Ok(f64::INFINITY));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"Inf", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e3", &PF_OPTS), Ok(1.1e3));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn case_sensitive_special(mut self, flag: bool) -> Self {
        self.case_sensitive_special = flag;
        self
    }

    /// Set if leading zeros before an integer are not allowed.
    ///
    /// This only applies if there is no base prefix: that is, the zeros are
    /// at the absolute start of the number. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// With a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `01` | ❌ |
    /// | `0` | ✔️ |
    /// | `10` | ✔️ |
    /// | `0x01` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'd'))
    ///     .no_integer_leading_zeros(true)
    ///     .build_strict();
    /// assert_eq!(parse::<i64>(b"01"), Ok(1));
    /// assert_eq!(parse::<i64>(b"+01"), Ok(1));
    /// assert_eq!(parse::<i64>(b"0"), Ok(0));
    /// assert_eq!(parse::<i64>(b"10"), Ok(10));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"01", &PI_OPTS), Err(Error::InvalidLeadingZeros(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+01", &PI_OPTS), Err(Error::InvalidLeadingZeros(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0", &PI_OPTS), Ok(0));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"10", &PI_OPTS), Ok(10));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d01", &PI_OPTS), Ok(1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_integer_leading_zeros(mut self, flag: bool) -> Self {
        self.no_integer_leading_zeros = flag;
        self
    }

    /// Set if leading zeros before a float are not allowed.
    ///
    /// This is before the significant digits of the float, that is, if there is
    /// 1 or more digits in the integral component and the leading digit is 0,
    /// This only applies if there is no base prefix: that is, the zeros are
    /// at the absolute start of the number. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// With a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `01` | ❌ |
    /// | `01.0` | ❌ |
    /// | `0` | ✔️ |
    /// | `10` | ✔️ |
    /// | `0x01.0` | ✔️ |
    /// | `0.1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'd'))
    ///     .no_float_leading_zeros(true)
    ///     .build_strict();
    /// assert_eq!(parse::<f64>(b"01"), Ok(1.0));
    /// assert_eq!(parse::<f64>(b"+01"), Ok(1.0));
    /// assert_eq!(parse::<f64>(b"0"), Ok(0.0));
    /// assert_eq!(parse::<f64>(b"10"), Ok(10.0));
    /// assert_eq!(parse::<f64>(b"0.1"), Ok(0.1));
    ///
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"01", &PF_OPTS), Err(Error::InvalidLeadingZeros(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+01", &PF_OPTS), Err(Error::InvalidLeadingZeros(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0", &PF_OPTS), Ok(0.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"10", &PF_OPTS), Ok(10.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0.1", &PF_OPTS), Ok(0.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d01", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d01.1", &PF_OPTS), Ok(1.1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_float_leading_zeros(mut self, flag: bool) -> Self {
        self.no_float_leading_zeros = flag;
        self
    }

    /// Set if exponent notation is required.
    ///
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ❌ |
    /// | `1.0` | ❌ |
    /// | `1e3` | ✔️ |
    /// | `1.1e3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_exponent_notation(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Err(Error::MissingExponent(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0", &PF_OPTS), Err(Error::MissingExponent(3)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e3", &PF_OPTS), Ok(1.0e3));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e3", &PF_OPTS), Ok(1.1e3));
    ///
    /// const SIZE: usize = WF_OPTS.buffer_size_const::<f64, FORMAT>();
    /// let mut buffer = [0u8; SIZE];
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.0, &mut buffer, &WF_OPTS), b"1.0e0");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1, &mut buffer, &WF_OPTS), b"1.1e0");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.0e3, &mut buffer, &WF_OPTS), b"1.0e3");
    /// assert_eq!(write_with_options::<f64, FORMAT>(1.1e3, &mut buffer, &WF_OPTS), b"1.1e3");
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_exponent_notation(mut self, flag: bool) -> Self {
        self.required_exponent_notation = flag;
        self
    }

    /// Set if exponent characters are case-sensitive.
    ///
    /// If set to [`true`], then the exponent character `e` would be considered
    /// the different from `E`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1.1e3` | ✔️ |
    /// | `1.1E3` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .case_sensitive_exponent(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e3", &PF_OPTS), Ok(1.0e3));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0E3", &PF_OPTS), Err(Error::InvalidDigit(3)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn case_sensitive_exponent(mut self, flag: bool) -> Self {
        self.case_sensitive_exponent = flag;
        self
    }

    /// Set if base prefixes are case-sensitive.
    ///
    /// If set to [`true`], then the base prefix `x` would be considered the
    /// different from `X`. Defaults to [`false`]. This is only used for writing
    /// numbers if [`required_base_prefix`] is [`true`].
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `0X1` | ❌ |
    /// | `1` | ✔️ |
    /// | `1x` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_prefix`]: Self::required_base_prefix
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'x'))
    ///     .case_sensitive_base_prefix(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0x1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0X1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1x", &PF_OPTS), Err(Error::InvalidDigit(1)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0x1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0X1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1x", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn case_sensitive_base_prefix(mut self, flag: bool) -> Self {
        self.case_sensitive_base_prefix = flag;
        self
    }

    /// Set if base suffixes are case-sensitive.
    ///
    /// If set to [`true`], then the base suffix `x` would be considered the
    /// different from `X`. Defaults to [`false`]. This is only used for writing
    /// numbers if [`required_base_suffix`] is [`true`].
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1x` | ✔️ |
    /// | `1X` | ❌ |
    /// | `1d` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    ///
    /// [`required_base_suffix`]: Self::required_base_suffix
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_suffix(num::NonZeroU8::new(b'x'))
    ///     .case_sensitive_base_suffix(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0x1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1x", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1X", &PF_OPTS), Err(Error::InvalidDigit(1)));
    ///
    /// // TODO: This has the wrong placement
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0x1", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1x", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1X", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn case_sensitive_base_suffix(mut self, flag: bool) -> Self {
        self.case_sensitive_base_suffix = flag;
        self
    }

    /// Set if digits are required before the decimal point with exponent
    /// notation.
    ///
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `.1e5` | ❌ |
    /// | `.e5` | ❌ |
    /// | `1.e5` | ✔️ |
    /// | `1.0e5` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_mantissa_digits(false)
    ///     .required_integer_digits_with_exponent(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1e5", &PF_OPTS), Ok(1e5));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".1e5", &PF_OPTS), Err(Error::ExponentWithoutIntegerDigits(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".e5", &PF_OPTS), Err(Error::ExponentWithoutIntegerDigits(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.e5", &PF_OPTS), Ok(1e5));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e5", &PF_OPTS), Ok(1e5));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_integer_digits_with_exponent(mut self, flag: bool) -> Self {
        self.required_integer_digits_with_exponent = flag;
        self
    }

    /// Set if digits are required after the decimal point with exponent
    /// notation, if the decimal point is present.
    ///
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `.1e5` | ✔️ |
    /// | `.e5` | ❌ |
    /// | `1.e5` | ❌ |
    /// | `1.0e5` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_mantissa_digits(false)
    ///     .required_fraction_digits_with_exponent(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1e5", &PF_OPTS), Ok(1e5));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".1e5", &PF_OPTS), Ok(1e4));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".e5", &PF_OPTS), Err(Error::ExponentWithoutFractionDigits(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.e5", &PF_OPTS), Err(Error::ExponentWithoutFractionDigits(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e5", &PF_OPTS), Ok(1e5));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_fraction_digits_with_exponent(mut self, flag: bool) -> Self {
        self.required_fraction_digits_with_exponent = flag;
        self
    }

    /// Set if any significant digits are required with exponent notation.
    ///
    /// Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `.1e5` | ✔️ |
    /// | `.e5` | ❌ |
    /// | `1.e5` | ✔️ |
    /// | `1.0e5` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .required_mantissa_digits(false)
    ///     .required_mantissa_digits_with_exponent(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1e5", &PF_OPTS), Ok(1e5));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".1e5", &PF_OPTS), Ok(1e4));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b".e5", &PF_OPTS), Err(Error::ExponentWithoutMantissaDigits(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.e5", &PF_OPTS), Ok(1e5));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e5", &PF_OPTS), Ok(1e5));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_mantissa_digits_with_exponent(mut self, flag: bool) -> Self {
        self.required_mantissa_digits_with_exponent = flag;
        self
    }

    /// Set if the format requires base prefixes.
    ///
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `4d2` | ❌ |
    /// | `x4d2` | ❌ |
    /// | `4d2x` | ❌ |
    /// | `0x4d2` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Write Float
    /// - Write Integer
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn required_base_prefix(mut self, flag: bool) -> Self {
        self.required_base_prefix = flag;
        self
    }

    /// Set if the format requires base suffixes.
    ///
    /// This will override most errors, including all invalid digit errors.
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `4d2` | ❌ |
    /// | `x4d2` | ❌ |
    /// | `4d2x` | ✔️ |
    /// | `0x4d2` | ❌ |
    ///
    /// # Used For
    ///
    /// - Write Float
    /// - Write Integer
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn required_base_suffix(mut self, flag: bool) -> Self {
        self.required_base_suffix = flag;
        self
    }

    /// If a negative sign before an unsigned integer is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. This
    /// does not apply to signed integers or floating point numbers.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `-12` | ❌ |
    /// | `+12` | ✔️ |
    /// | `12` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const NO_SIGN: u128 = NumberFormatBuilder::new()
    ///     .no_unsigned_negative_sign(true)
    ///     .build_strict();
    ///
    /// assert_eq!(parse_with_options::<u64, NO_SIGN>(b"-1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<u64, NO_SIGN>(b"1", &PI_OPTS), Ok(1));
    ///
    /// assert_eq!(parse_with_options::<i64, NO_SIGN>(b"-1", &PI_OPTS), Ok(-1));
    /// assert_eq!(parse_with_options::<i64, NO_SIGN>(b"1", &PI_OPTS), Ok(1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_unsigned_negative_sign(mut self, flag: bool) -> Self {
        self.no_unsigned_negative_sign = flag;
        self
    }

    /// If positive or negative signs before the significant digits are not
    /// allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. if
    /// enabled, then the type cannot represent negative literal or string
    /// values (although they may be computed via mathematical operations).
    ///
    /// If you only want to disable positive signs, see
    /// [`no_positive_mantissa_sign`]. If you wish to disable negative signs
    /// on unsigned integers, see [`no_unsigned_negative_sign`].
    ///
    /// [`no_positive_mantissa_sign`]: Self::no_positive_mantissa_sign
    /// [`no_unsigned_negative_sign`]: Self::no_unsigned_negative_sign
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `-12` | ❌ |
    /// | `+12` | ❌ |
    /// | `12` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Integer
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const NO_SIGN: u128 = NumberFormatBuilder::new()
    ///     .no_mantissa_sign(true)
    ///     .build_strict();
    ///
    /// assert_eq!(parse_with_options::<i64, NO_SIGN>(b"-1", &PI_OPTS), Err(Error::InvalidNegativeMantissaSign(0)));
    /// assert_eq!(parse_with_options::<i64, NO_SIGN>(b"+1", &PI_OPTS), Err(Error::InvalidPositiveMantissaSign(0)));
    /// assert_eq!(parse_with_options::<i64, NO_SIGN>(b"1", &PI_OPTS), Ok(1));
    ///
    /// assert_eq!(parse_with_options::<f64, NO_SIGN>(b"-1.0", &PF_OPTS), Err(Error::InvalidNegativeMantissaSign(0)));
    /// assert_eq!(parse_with_options::<f64, NO_SIGN>(b"+1.0", &PF_OPTS), Err(Error::InvalidPositiveMantissaSign(0)));
    /// assert_eq!(parse_with_options::<f64, NO_SIGN>(b"1.0", &PF_OPTS), Ok(1.0));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_mantissa_sign(mut self, flag: bool) -> Self {
        self.no_mantissa_sign = flag;
        self
    }

    /// If positive or negative signs before an exponent are not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`.
    ///
    /// If you only want to disable positive signs, see
    /// [`no_positive_exponent_sign`].
    ///
    /// [`no_positive_exponent_sign`]: Self::no_positive_exponent_sign
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.0e-12` | ❌ |
    /// | `1.0e+12` | ❌ |
    /// | `1.0e12` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const NO_SIGN: u128 = NumberFormatBuilder::new()
    ///     .no_exponent_sign(true)
    ///     .build_strict();
    ///
    /// assert_eq!(parse_with_options::<f64, NO_SIGN>(b"1.0e-12", &PF_OPTS), Err(Error::InvalidNegativeExponentSign(4)));
    /// assert_eq!(parse_with_options::<f64, NO_SIGN>(b"1.0e+12", &PF_OPTS), Err(Error::InvalidPositiveExponentSign(4)));
    /// assert_eq!(parse_with_options::<f64, NO_SIGN>(b"1.0e12", &PF_OPTS), Ok(1.0e12));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_exponent_sign(mut self, flag: bool) -> Self {
        self.no_exponent_sign = flag;
        self
    }

    /// Set if digit separators are allowed at the absolute start of the number.
    ///
    /// This modifies the behavior of [`integer_sign_digit_separator`] and
    /// [`integer_leading_digit_separator`] so that the digit separators can
    /// occur at the absolute start and not just internal to the number.
    ///
    /// Defaults to [`true`].
    ///
    /// [`integer_sign_digit_separator`]: Self::get_integer_sign_digit_separator
    /// [`integer_leading_digit_separator`]: Self::get_integer_leading_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, integer leading digit separators
    /// enabled, and starting digit separators enabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `_1` | ✔️ |
    /// | `_+1` | ❌ |
    /// | `+_1` | ✔️ |
    /// | `+0x_1` | ✔️ |
    /// | `+_0x1` | ❌ |
    ///
    /// Using a digit separator of `_`, integer leading digit separators
    /// enabled, and starting digit separators disabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `_1` | ❌ |
    /// | `_+1` | ❌ |
    /// | `+_1` | ✔️ |
    /// | `+0x_1` | ✔️ |
    /// | `+_0x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const START: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_leading_digit_separator(true)
    ///     .start_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, START>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, START>(b"_1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, START>(b"+_1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, START>(b"_+1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    ///
    /// const NO_START: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_leading_digit_separator(true)
    ///     .start_digit_separator(false)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<i64, NO_START>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, NO_START>(b"_1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, NO_START>(b"+_1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, NO_START>(b"_+1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn start_digit_separator(mut self, flag: bool) -> Self {
        self.start_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed before the sign of the integer.
    ///
    /// You can disable support for digit separators at the absolute start
    /// using the [`start_digit_separator`] flag.
    ///
    /// [`start_digit_separator`]: Self::get_start_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `_1` | ❌ |
    /// | `_+1` | ✔️ |
    /// | `+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_sign_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+_1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_+1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"__+1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_sign_digit_separator(mut self, flag: bool) -> Self {
        self.integer_sign_digit_separator = flag;
        self
    }

    /// Set if consecutive digit separators are allowed before the sign of the
    /// integer.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `_1` | ❌ |
    /// | `_+1` | ✔️ |
    /// | `__+1` | ✔️ |
    /// | `+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_sign_digit_separator(true)
    ///     .integer_consecutive_sign_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+_1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_+1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"__+1", &PF_OPTS), Ok(1.0));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_consecutive_sign_digit_separator(mut self, flag: bool) -> Self {
        self.integer_consecutive_sign_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed before the sign of the exponent.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.0e1` | ✔️ |
    /// | `1.0e_` | ❌ |
    /// | `1.0e_1` | ❌ |
    /// | `1.0e_+1` | ✔️ |
    /// | `1.0e+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .exponent_sign_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e1", &PF_OPTS), Ok(10.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e_", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e_1", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e_+1", &PF_OPTS), Ok(10.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e__+1", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e+_1", &PF_OPTS), Err(Error::EmptyExponent(5)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_sign_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_sign_digit_separator = flag;
        self
    }

    /// Set if consecutive digit separators are allowed before the sign of the
    /// exponent.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.0e1` | ✔️ |
    /// | `1.0e_` | ❌ |
    /// | `1.0e_1` | ❌ |
    /// | `1.0e_+1` | ✔️ |
    /// | `1.0e__+1` | ✔️ |
    /// | `1.0e+_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .exponent_sign_digit_separator(true)
    ///     .exponent_consecutive_sign_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e1", &PF_OPTS), Ok(10.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e_", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e_1", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e_+1", &PF_OPTS), Ok(10.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e__+1", &PF_OPTS), Ok(10.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.0e+_1", &PF_OPTS), Err(Error::EmptyExponent(5)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_consecutive_sign_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_consecutive_sign_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed between the `0` and the base prefix.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, and a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ❌ |
    /// | `0x_1` | ❌ |
    /// | `0_x1` | ✔️ |
    /// | `+0_x1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .base_prefix_internal_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_0d1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d_1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0_d1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+0_d1", &PF_OPTS), Ok(1.0));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_0d1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d_1", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0_d1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+0_d1", &PI_OPTS), Ok(1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_prefix_internal_digit_separator(mut self, flag: bool) -> Self {
        self.base_prefix_internal_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before the base prefix.
    ///
    /// This is taken **after** the sign and therefore  may be at the absolute
    /// start of the number or internal to it. You can disable support for
    /// digit separators at the absolute start using the
    /// [`start_digit_separator`] flag.
    ///
    /// [`start_digit_separator`]: Self::get_start_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, a base prefix of `x`, and starting digit
    /// separators enabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ✔️ |
    /// | `0x_1` | ❌ |
    /// | `0_x1` | ❌ |
    /// | `+_0x1` | ✔️ |
    /// | `_+0x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .base_prefix_leading_digit_separator(true)
    ///     .start_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_+1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_0d1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d_1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+_0d1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_+0d1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_+1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_0d1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d_1", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+_0d1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_+0d1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    ///
    /// const NO_START: u128 = NumberFormatBuilder::rebuild(FORMAT)
    ///     .start_digit_separator(false)
    ///     .build_strict();
    /// // TODO: Start digit separator isn't being properly respected
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_0d1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_0d1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_prefix_leading_digit_separator(mut self, flag: bool) -> Self {
        self.base_prefix_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after the base prefix.
    ///
    /// If a base prefix is present, then [`integer_leading_digit_separator`]
    /// overlaps with this functionality, and either being enabled, as well
    /// as consecutive digit separators, will consider it to be valid. If
    /// both are set and neither enables consecutive digit separators, then
    /// `0x__1` is not valid.
    ///
    /// [`integer_leading_digit_separator`]: Self::get_integer_leading_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` and a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ❌ |
    /// | `0_x1` | ❌ |
    /// | `0x_1` | ✔️ |
    /// | `0x__1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .base_prefix_trailing_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_0d1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0_d1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d_1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d__1", &PF_OPTS), Err(Error::InvalidDigit(3)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_0d1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0_d1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d_1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d__1", &PI_OPTS), Err(Error::InvalidDigit(3)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_prefix_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.base_prefix_trailing_digit_separator = flag;
        self
    }

    /// Set if multiple consecutive base prefix digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// base prefix. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, a base prefix of `x`, and trailing base
    /// prefix digit separators.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `0x1` | ✔️ |
    /// | `_0x1` | ❌ |
    /// | `0_x1` | ❌ |
    /// | `0x_1` | ✔️ |
    /// | `0x__1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .base_prefix_trailing_digit_separator(true)
    ///     .base_prefix_consecutive_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_0d1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0_d1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d_1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"0d__1", &PF_OPTS), Ok(1.0));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_0d1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0_d1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d_1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"0d__1", &PI_OPTS), Ok(1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_prefix_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.base_prefix_consecutive_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed between the base suffix.
    ///
    /// This is currently unused, since base suffixes are a single character.
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_suffix_internal_digit_separator(mut self, flag: bool) -> Self {
        self.base_suffix_internal_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before the base suffix.
    ///
    /// If a base suffix is present, then [`integer_trailing_digit_separator`]
    /// or [`exponent_trailing_digit_separator`] may overlap
    /// with this functionality (depending on the number components), and either
    /// being enabled, as well as consecutive digit separators, will
    /// consider it to be valid. If both are set and neither
    /// enables consecutive digit separators, then `1__h` is not valid.
    ///
    /// [`integer_trailing_digit_separator`]: Self::get_integer_trailing_digit_separator
    /// [`exponent_trailing_digit_separator`]: Self::get_exponent_trailing_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` and a base suffix of `h`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `12h` | ✔️ |
    /// | `1_2h` | ❌ |
    /// | `12_h` | ✔️ |
    /// | `12__h` | ✔️ |
    /// | `12h_` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_suffix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .base_suffix_leading_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d", &PF_OPTS), Ok(12.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_2d", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12_d", &PF_OPTS), Ok(12.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12__d", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d_", &PF_OPTS), Err(Error::InvalidDigit(3)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d", &PI_OPTS), Ok(12));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_2d", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12_d", &PI_OPTS), Ok(12));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12__d", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// // TODO: This is incorrectly using the current placement
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d_", &PI_OPTS), Err(Error::InvalidDigit(3)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_suffix_leading_digit_separator(mut self, flag: bool) -> Self {
        self.base_suffix_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after the base suffix.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` and a base suffix of `h`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `12h` | ✔️ |
    /// | `1_2h` | ❌ |
    /// | `12_h` | ❌ |
    /// | `12h_` | ✔️ |
    /// | `12h__` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_suffix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .base_suffix_trailing_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d", &PF_OPTS), Ok(12.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_2d", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12_d", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d_", &PF_OPTS), Ok(12.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d__", &PF_OPTS), Err(Error::InvalidDigit(4)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d", &PI_OPTS), Ok(12));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_2d", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12_d", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d_", &PI_OPTS), Ok(12));
    /// // TODO: This is getting the location wrong
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d__", &PI_OPTS), Err(Error::InvalidDigit(4)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_suffix_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.base_suffix_trailing_digit_separator = flag;
        self
    }

    /// Set if multiple consecutive base suffix digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// base suffix. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, a base suffix of `h`, and trailing base
    /// suffix digit separators.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `12h` | ✔️ |
    /// | `1_2h` | ❌ |
    /// | `12_h` | ❌ |
    /// | `12h_` | ✔️ |
    /// | `12h__` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_suffix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .base_suffix_trailing_digit_separator(true)
    ///     .base_suffix_consecutive_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d", &PF_OPTS), Ok(12.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_2d", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12_d", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d_", &PF_OPTS), Ok(12.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"12d__", &PF_OPTS), Ok(12.0));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d", &PI_OPTS), Ok(12));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_2d", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12_d", &PI_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d_", &PI_OPTS), Ok(12));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"12d__", &PI_OPTS), Ok(12));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_suffix_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.base_suffix_consecutive_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed between integer digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ✔️ |
    /// | `1_` | ❌ |
    /// | `_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_internal_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_1", &PI_OPTS), Ok(11));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_internal_digit_separator(mut self, flag: bool) -> Self {
        self.integer_internal_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ✔️ |
    /// | `1.1_` | ❌ |
    /// | `1._1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .fraction_internal_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_1", &PF_OPTS), Ok(1.11));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_", &PF_OPTS), Err(Error::InvalidDigit(3)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_internal_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_internal_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ✔️ |
    /// | `1.1e1_` | ❌ |
    /// | `1.1e_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .exponent_internal_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_1", &PF_OPTS), Ok(1.1e11));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_", &PF_OPTS), Err(Error::InvalidDigit(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_1", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_internal_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_internal_digit_separator = flag;
        self
    }

    /// Set all internal digit separator flags.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Sets [`integer_internal_digit_separator`],
    /// [`fraction_internal_digit_separator`], and
    /// [`exponent_internal_digit_separator`].
    ///
    /// [`integer_internal_digit_separator`]: Self::integer_internal_digit_separator
    /// [`fraction_internal_digit_separator`]: Self::fraction_internal_digit_separator
    /// [`exponent_internal_digit_separator`]: Self::exponent_internal_digit_separator
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn internal_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_internal_digit_separator(flag);
        self = self.fraction_internal_digit_separator(flag);
        self = self.exponent_internal_digit_separator(flag);
        self
    }

    /// Set if a digit separator is allowed before any integer digits.
    ///
    /// This is taken **after** any signs and base prefixes, and therefore
    /// may be at the absolute start of the number or internal to it. You
    /// can disable support for digit separators at the absolute start
    /// using the [`start_digit_separator`] flag. For only those before or after
    /// a base prefix, see [`base_prefix_leading_digit_separator`] and
    /// [`base_prefix_trailing_digit_separator`], respectively. For only those
    /// prior to a sign, see [`integer_sign_digit_separator`].
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Defaults to [`false`].
    ///
    /// [`start_digit_separator`]: Self::get_start_digit_separator
    /// [`base_prefix_leading_digit_separator`]: Self::get_base_prefix_leading_digit_separator
    /// [`base_prefix_trailing_digit_separator`]: Self::get_base_prefix_trailing_digit_separator
    /// [`integer_sign_digit_separator`]: Self::get_integer_sign_digit_separator
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`, an optional base prefix of `x` and
    /// starting digit separators enabled.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ❌ |
    /// | `1_` | ❌ |
    /// | `_1` | ✔️ |
    /// | `_+1` | ❌ |
    /// | `+_1` | ✔️ |
    /// | `+0x_1` | ✔️ |
    /// | `+_0x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .base_prefix(num::NonZeroU8::new(b'd'))
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_leading_digit_separator(true)
    ///     .start_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_", &PF_OPTS), Err(Error::Empty(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_+1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+_1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+0d_1", &PF_OPTS), Ok(1.0));
    /// // TODO: This is incorrectly consuming the base prefix
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"+_0d1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_", &PI_OPTS), Err(Error::Empty(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_+1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+_1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+0d_1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"+_0d1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    ///
    /// const NO_START: u128 = NumberFormatBuilder::rebuild(FORMAT)
    ///     .start_digit_separator(false)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Err(Error::Empty(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_1", &PI_OPTS), Err(Error::Empty(1)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_leading_digit_separator(mut self, flag: bool) -> Self {
        self.integer_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ✔️ |
    /// | `1.1_1` | ❌ |
    /// | `1.1_` | ❌ |
    /// | `1._1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .fraction_leading_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_1", &PF_OPTS), Err(Error::InvalidDigit(3)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_", &PF_OPTS), Err(Error::InvalidDigit(3)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._1", &PF_OPTS), Ok(1.1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_leading_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ❌ |
    /// | `1.1e1_` | ❌ |
    /// | `1.1e_1` | ✔️ |
    /// | `1.1e_+1` | ❌ |
    /// | `1.1e+_1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .exponent_leading_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_", &PF_OPTS), Err(Error::EmptyExponent(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_1", &PF_OPTS), Err(Error::InvalidDigit(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_", &PF_OPTS), Err(Error::InvalidDigit(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_+1", &PF_OPTS), Err(Error::EmptyExponent(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e+_1", &PF_OPTS), Ok(11.0));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_leading_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_leading_digit_separator = flag;
        self
    }

    /// Set all leading digit separator flags.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Sets
    /// [`integer_leading_digit_separator`],
    /// [`fraction_leading_digit_separator`], and
    /// [`exponent_leading_digit_separator`].
    ///
    /// [`integer_leading_digit_separator`]: Self::integer_leading_digit_separator
    /// [`fraction_leading_digit_separator`]: Self::fraction_leading_digit_separator
    /// [`exponent_leading_digit_separator`]: Self::exponent_leading_digit_separator
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn leading_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_leading_digit_separator(flag);
        self = self.fraction_leading_digit_separator(flag);
        self = self.exponent_leading_digit_separator(flag);
        self
    }

    /// Set if a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ❌ |
    /// | `1_` | ✔️ |
    /// | `_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_trailing_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_", &PF_OPTS), Err(Error::Empty(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_1", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_", &PI_OPTS), Err(Error::Empty(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_1", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.integer_trailing_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Defaults to [`false`].
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ✔️ |
    /// | `1.1_1` | ❌ |
    /// | `1.1_` | ✔️ |
    /// | `1._1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .fraction_trailing_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_1", &PF_OPTS), Err(Error::InvalidDigit(3)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_", &PF_OPTS), Ok(1.1));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_trailing_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ❌ |
    /// | `1.1e1_` | ✔️ |
    /// | `1.1e_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .exponent_trailing_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_", &PF_OPTS), Err(Error::EmptyExponent(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_1", &PF_OPTS), Err(Error::InvalidDigit(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_1", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_trailing_digit_separator = flag;
        self
    }

    /// Set all trailing digit separator flags.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Sets
    /// [`integer_trailing_digit_separator`],
    /// [`fraction_trailing_digit_separator`], and
    /// [`exponent_trailing_digit_separator`].
    ///
    /// [`integer_trailing_digit_separator`]: Self::integer_trailing_digit_separator
    /// [`fraction_trailing_digit_separator`]: Self::fraction_trailing_digit_separator
    /// [`exponent_trailing_digit_separator`]: Self::exponent_trailing_digit_separator
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn trailing_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_trailing_digit_separator(flag);
        self = self.fraction_trailing_digit_separator(flag);
        self = self.exponent_trailing_digit_separator(flag);
        self
    }

    /// Set if multiple consecutive integer digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// integer. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` with only internal integer digit
    /// separators being valid.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ✔️ |
    /// | `1__1` | ✔️ |
    /// | `1_` | ❌ |
    /// | `_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .integer_internal_digit_separator(true)
    ///     .integer_consecutive_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1", &PF_OPTS), Ok(1.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1__1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1_", &PF_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"_1", &PF_OPTS), Err(Error::InvalidDigit(0)));
    ///
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1", &PI_OPTS), Ok(1));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_1", &PI_OPTS), Ok(11));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1__1", &PI_OPTS), Ok(11));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"1_", &PI_OPTS), Err(Error::InvalidDigit(1)));
    /// assert_eq!(parse_with_options::<i64, FORMAT>(b"_1", &PI_OPTS), Err(Error::InvalidDigit(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.integer_consecutive_digit_separator = flag;
        self
    }

    /// Set if multiple consecutive fraction digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// fraction. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` with only internal fraction digit
    /// separators being valid.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ✔️ |
    /// | `1.1__1` | ✔️ |
    /// | `1.1_` | ❌ |
    /// | `1._1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .fraction_internal_digit_separator(true)
    ///     .fraction_consecutive_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1", &PF_OPTS), Ok(1.1));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_1", &PF_OPTS), Ok(1.11));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1__1", &PF_OPTS), Ok(1.11));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1_", &PF_OPTS), Err(Error::InvalidDigit(3)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1._1", &PF_OPTS), Err(Error::InvalidDigit(2)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_consecutive_digit_separator = flag;
        self
    }

    /// Set if multiple consecutive exponent digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// exponent. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` with only internal exponent digit
    /// separators being valid.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ✔️ |
    /// | `1.1e1__1` | ✔️ |
    /// | `1.1e1_` | ❌ |
    /// | `1.1e_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FORMAT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .exponent_internal_digit_separator(true)
    ///     .exponent_consecutive_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1", &PF_OPTS), Ok(11.0));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_1", &PF_OPTS), Ok(1.1e11));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1__1", &PF_OPTS), Ok(1.1e11));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e1_", &PF_OPTS), Err(Error::InvalidDigit(5)));
    /// assert_eq!(parse_with_options::<f64, FORMAT>(b"1.1e_1", &PF_OPTS), Err(Error::EmptyExponent(4)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_consecutive_digit_separator = flag;
        self
    }

    /// Set all consecutive digit separator flags.
    ///
    ///  Sets [`integer_consecutive_digit_separator`],
    /// [`fraction_consecutive_digit_separator`], and
    /// [`exponent_consecutive_digit_separator`].
    ///
    /// [`integer_consecutive_digit_separator`]: Self::integer_consecutive_digit_separator
    /// [`fraction_consecutive_digit_separator`]: Self::fraction_consecutive_digit_separator
    /// [`exponent_consecutive_digit_separator`]: Self::exponent_consecutive_digit_separator
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn consecutive_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_consecutive_digit_separator(flag);
        self = self.fraction_consecutive_digit_separator(flag);
        self = self.exponent_consecutive_digit_separator(flag);
        self
    }

    /// Set if any digit separators are allowed in special (non-finite) values.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for any special floats: for example, `N__a_N_` is considered
    /// the same as `NaN`. Defaults to [`false`].
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `nan` | ✔️ |
    /// | `na_n` | ✔️ |
    /// | `na_n_` | ✔️ |
    /// | `na_nx` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    ///
    /// <!-- TEST
    /// ```rust
    /// const FMT: u128 = NumberFormatBuilder::new()
    ///     .digit_separator(num::NonZeroU8::new(b'_'))
    ///     .special_digit_separator(true)
    ///     .build_strict();
    /// assert_eq!(parse_with_options::<f64, FMT>(b"nan", &PF_OPTS).map(|x| x.is_nan()), Ok(true));
    /// assert_eq!(parse_with_options::<f64, FMT>(b"na_n", &PF_OPTS).map(|x| x.is_nan()), Ok(true));
    /// assert_eq!(parse_with_options::<f64, FMT>(b"na_n_", &PF_OPTS).map(|x| x.is_nan()), Ok(true));
    /// assert_eq!(parse_with_options::<f64, FMT>(b"na_nx", &PF_OPTS), Err(Error::InvalidDigit(0)));
    /// ```
    /// -->
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn special_digit_separator(mut self, flag: bool) -> Self {
        self.special_digit_separator = flag;
        self
    }

    /// Allow digit separators in all locations for all components.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for the integer, fraction, exponent, special, sign, base
    /// prefix, and base suffix components. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.integer_digit_separator_flags(flag);
        self = self.fraction_digit_separator_flags(flag);
        self = self.exponent_digit_separator_flags(flag);
        self = self.special_digit_separator(flag);
        self = self.start_digit_separator(flag);
        self = self.sign_digit_separator_flags(flag);
        #[cfg(feature = "power-of-two")]
        {
            self = self.base_prefix_digit_separator_flags(flag);
            self = self.base_suffix_digit_separator_flags(flag);
        }
        self
    }

    /// Set all integer digit separator flag masks.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for the integer component. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.integer_internal_digit_separator(flag);
        self = self.integer_leading_digit_separator(flag);
        self = self.integer_trailing_digit_separator(flag);
        self = self.integer_consecutive_digit_separator(flag);
        self
    }

    /// Set all fraction digit separator flag masks.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for the fraction component. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.fraction_internal_digit_separator(flag);
        self = self.fraction_leading_digit_separator(flag);
        self = self.fraction_trailing_digit_separator(flag);
        self = self.fraction_consecutive_digit_separator(flag);
        self
    }

    /// Set all exponent digit separator flag masks.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for the exponent component. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.exponent_internal_digit_separator(flag);
        self = self.exponent_leading_digit_separator(flag);
        self = self.exponent_trailing_digit_separator(flag);
        self = self.exponent_consecutive_digit_separator(flag);
        self
    }

    /// Set all sign digit separator flag masks.
    ///
    /// This enables digit separators, including consecutive ones,
    /// for the integer and exponent formats. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn sign_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.integer_sign_digit_separator(flag);
        self = self.integer_consecutive_sign_digit_separator(flag);
        self = self.exponent_sign_digit_separator(flag);
        self = self.exponent_consecutive_sign_digit_separator(flag);
        self
    }

    /// Set all base prefix digit separator flag masks.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for the base prefix component. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_prefix_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.base_prefix_internal_digit_separator(flag);
        self = self.base_prefix_leading_digit_separator(flag);
        self = self.base_prefix_trailing_digit_separator(flag);
        self = self.base_prefix_consecutive_digit_separator(flag);
        self
    }

    /// Set all base suffix digit separator flag masks.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for the base suffix component. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[cfg(all(feature = "format", feature = "power-of-two"))]
    pub const fn base_suffix_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.base_suffix_internal_digit_separator(flag);
        self = self.base_suffix_leading_digit_separator(flag);
        self = self.base_suffix_trailing_digit_separator(flag);
        self = self.base_suffix_consecutive_digit_separator(flag);
        self
    }

    // BUILDER

    /// Create 128-bit, packed number format struct from builder options.
    ///
    /// <div class="warning">
    ///
    /// This function will never fail. It is up to the caller to ensure the
    /// format is valid using [`NumberFormat::is_valid`].
    ///
    /// </div>
    ///
    /// [`NumberFormat::is_valid`]: crate::NumberFormat::is_valid
    #[inline(always)]
    pub const fn build_unchecked(&self) -> u128 {
        let mut format: u128 = 0;
        add_flags!(
            format ;
            self.required_integer_digits, REQUIRED_INTEGER_DIGITS ;
            self.required_fraction_digits, REQUIRED_FRACTION_DIGITS ;
            self.required_exponent_digits, REQUIRED_EXPONENT_DIGITS ;
            self.required_mantissa_digits, REQUIRED_MANTISSA_DIGITS ;
            self.no_positive_mantissa_sign, NO_POSITIVE_MANTISSA_SIGN ;
            self.required_mantissa_sign, REQUIRED_MANTISSA_SIGN ;
            self.no_exponent_notation, NO_EXPONENT_NOTATION ;
            self.no_positive_exponent_sign, NO_POSITIVE_EXPONENT_SIGN ;
            self.required_exponent_sign, REQUIRED_EXPONENT_SIGN ;
            self.no_exponent_without_fraction, NO_EXPONENT_WITHOUT_FRACTION ;
            self.no_special, NO_SPECIAL ;
            self.case_sensitive_special, CASE_SENSITIVE_SPECIAL ;
            self.no_integer_leading_zeros, NO_INTEGER_LEADING_ZEROS ;
            self.no_float_leading_zeros, NO_FLOAT_LEADING_ZEROS ;
            self.required_exponent_notation, REQUIRED_EXPONENT_NOTATION ;
            self.case_sensitive_exponent, CASE_SENSITIVE_EXPONENT ;
            self.case_sensitive_base_prefix, CASE_SENSITIVE_BASE_PREFIX ;
            self.case_sensitive_base_suffix, CASE_SENSITIVE_BASE_SUFFIX ;
            self.required_integer_digits_with_exponent, REQUIRED_INTEGER_DIGITS_WITH_EXPONENT ;
            self.required_fraction_digits_with_exponent, REQUIRED_FRACTION_DIGITS_WITH_EXPONENT ;
            self.required_mantissa_digits_with_exponent, REQUIRED_MANTISSA_DIGITS_WITH_EXPONENT ;
            self.required_base_prefix, REQUIRED_BASE_PREFIX ;
            self.required_base_suffix, REQUIRED_BASE_SUFFIX ;
            self.no_unsigned_negative_sign, NO_UNSIGNED_NEGATIVE_SIGN ;
            self.no_mantissa_sign, NO_MANTISSA_SIGN ;
            self.no_exponent_sign, NO_EXPONENT_SIGN ;
            self.start_digit_separator, START_DIGIT_SEPARATOR ;
            self.integer_sign_digit_separator, INTEGER_SIGN_DIGIT_SEPARATOR ;
            self.integer_consecutive_sign_digit_separator, INTEGER_CONSECUTIVE_SIGN_DIGIT_SEPARATOR ;
            self.exponent_sign_digit_separator, EXPONENT_SIGN_DIGIT_SEPARATOR ;
            self.exponent_consecutive_sign_digit_separator, EXPONENT_CONSECUTIVE_SIGN_DIGIT_SEPARATOR ;
            self.base_prefix_internal_digit_separator, BASE_PREFIX_INTERNAL_DIGIT_SEPARATOR ;
            self.base_prefix_leading_digit_separator, BASE_PREFIX_LEADING_DIGIT_SEPARATOR ;
            self.base_prefix_trailing_digit_separator, BASE_PREFIX_TRAILING_DIGIT_SEPARATOR ;
            self.base_prefix_consecutive_digit_separator, BASE_PREFIX_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.base_suffix_internal_digit_separator, BASE_SUFFIX_INTERNAL_DIGIT_SEPARATOR ;
            self.base_suffix_leading_digit_separator, BASE_SUFFIX_LEADING_DIGIT_SEPARATOR ;
            self.base_suffix_trailing_digit_separator, BASE_SUFFIX_TRAILING_DIGIT_SEPARATOR ;
            self.base_suffix_consecutive_digit_separator, BASE_SUFFIX_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.integer_internal_digit_separator, INTEGER_INTERNAL_DIGIT_SEPARATOR ;
            self.fraction_internal_digit_separator, FRACTION_INTERNAL_DIGIT_SEPARATOR ;
            self.exponent_internal_digit_separator, EXPONENT_INTERNAL_DIGIT_SEPARATOR ;
            self.integer_leading_digit_separator, INTEGER_LEADING_DIGIT_SEPARATOR ;
            self.fraction_leading_digit_separator, FRACTION_LEADING_DIGIT_SEPARATOR ;
            self.exponent_leading_digit_separator, EXPONENT_LEADING_DIGIT_SEPARATOR ;
            self.integer_trailing_digit_separator, INTEGER_TRAILING_DIGIT_SEPARATOR ;
            self.fraction_trailing_digit_separator, FRACTION_TRAILING_DIGIT_SEPARATOR ;
            self.exponent_trailing_digit_separator, EXPONENT_TRAILING_DIGIT_SEPARATOR ;
            self.integer_consecutive_digit_separator, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.fraction_consecutive_digit_separator, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.exponent_consecutive_digit_separator, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.special_digit_separator, SPECIAL_DIGIT_SEPARATOR ;
        );
        if format & flags::ALL_DIGIT_SEPARATOR_FLAG_MASK != 0 {
            format |=
                (unwrap_or_zero(self.digit_separator) as u128) << flags::DIGIT_SEPARATOR_SHIFT;
        }
        format |= (unwrap_or_zero(self.base_prefix) as u128) << flags::BASE_PREFIX_SHIFT;
        format |= (unwrap_or_zero(self.base_suffix) as u128) << flags::BASE_SUFFIX_SHIFT;
        format |= (self.mantissa_radix as u128) << flags::MANTISSA_RADIX_SHIFT;
        format |= (unwrap_or_zero(self.exponent_base) as u128) << flags::EXPONENT_BASE_SHIFT;
        format |= (unwrap_or_zero(self.exponent_radix) as u128) << flags::EXPONENT_RADIX_SHIFT;

        format
    }

    /// Build the packed number format, panicking if the builder is invalid.
    ///
    /// # Panics
    ///
    /// If the built format is not valid.
    #[inline(always)]
    pub const fn build_strict(&self) -> u128 {
        use crate::format::format_error_impl;

        let packed = self.build_unchecked();
        match format_error_impl(packed) {
            Error::Success => packed,
            error => core::panic!("{}", error.description()),
        }
    }

    /// Create 128-bit, packed number format struct from builder options.
    ///
    /// <div class="warning">
    ///
    /// This function will never fail. It is up to the caller to ensure the
    /// format is valid using [`NumberFormat::is_valid`]. This function is
    /// soft-deprecated and you should prefer [`build_unchecked`] and handle
    /// if the result is invalid instead, or use [`build_strict`] to panic on
    /// any errors. This exists when compatibility with older Rust
    /// versions was required.
    ///
    /// </div>
    ///
    /// [`build_unchecked`]: Self::build_unchecked
    /// [`build_strict`]: Self::build_strict
    /// [`NumberFormat::is_valid`]: crate::NumberFormat::is_valid
    #[inline(always)]
    #[deprecated = "Use `build_strict` or `build_unchecked` instead."]
    pub const fn build(&self) -> u128 {
        self.build_unchecked()
    }

    /// Re-create builder from format.
    #[inline(always)]
    pub const fn rebuild(format: u128) -> Self {
        NumberFormatBuilder {
            digit_separator: num::NonZeroU8::new(flags::digit_separator(format)),
            base_prefix: num::NonZeroU8::new(flags::base_prefix(format)),
            base_suffix: num::NonZeroU8::new(flags::base_suffix(format)),
            mantissa_radix: flags::mantissa_radix(format) as u8,
            exponent_base: num::NonZeroU8::new(flags::exponent_base(format) as u8),
            exponent_radix: num::NonZeroU8::new(flags::exponent_radix(format) as u8),
            required_integer_digits: has_flag!(format, REQUIRED_INTEGER_DIGITS),
            required_fraction_digits: has_flag!(format, REQUIRED_FRACTION_DIGITS),
            required_exponent_digits: has_flag!(format, REQUIRED_EXPONENT_DIGITS),
            required_mantissa_digits: has_flag!(format, REQUIRED_MANTISSA_DIGITS),
            no_positive_mantissa_sign: has_flag!(format, NO_POSITIVE_MANTISSA_SIGN),
            required_mantissa_sign: has_flag!(format, REQUIRED_MANTISSA_SIGN),
            no_exponent_notation: has_flag!(format, NO_EXPONENT_NOTATION),
            no_positive_exponent_sign: has_flag!(format, NO_POSITIVE_EXPONENT_SIGN),
            required_exponent_sign: has_flag!(format, REQUIRED_EXPONENT_SIGN),
            no_exponent_without_fraction: has_flag!(format, NO_EXPONENT_WITHOUT_FRACTION),
            no_special: has_flag!(format, NO_SPECIAL),
            case_sensitive_special: has_flag!(format, CASE_SENSITIVE_SPECIAL),
            no_integer_leading_zeros: has_flag!(format, NO_INTEGER_LEADING_ZEROS),
            no_float_leading_zeros: has_flag!(format, NO_FLOAT_LEADING_ZEROS),
            required_exponent_notation: has_flag!(format, REQUIRED_EXPONENT_NOTATION),
            case_sensitive_exponent: has_flag!(format, CASE_SENSITIVE_EXPONENT),
            case_sensitive_base_prefix: has_flag!(format, CASE_SENSITIVE_BASE_PREFIX),
            case_sensitive_base_suffix: has_flag!(format, CASE_SENSITIVE_BASE_SUFFIX),
            required_integer_digits_with_exponent: has_flag!(
                format,
                REQUIRED_INTEGER_DIGITS_WITH_EXPONENT
            ),
            required_fraction_digits_with_exponent: has_flag!(
                format,
                REQUIRED_FRACTION_DIGITS_WITH_EXPONENT
            ),
            required_mantissa_digits_with_exponent: has_flag!(
                format,
                REQUIRED_MANTISSA_DIGITS_WITH_EXPONENT
            ),
            required_base_prefix: has_flag!(format, REQUIRED_BASE_PREFIX),
            required_base_suffix: has_flag!(format, REQUIRED_BASE_SUFFIX),
            no_unsigned_negative_sign: has_flag!(format, NO_UNSIGNED_NEGATIVE_SIGN),
            no_mantissa_sign: has_flag!(format, NO_MANTISSA_SIGN),
            no_exponent_sign: has_flag!(format, NO_EXPONENT_SIGN),
            start_digit_separator: has_flag!(format, START_DIGIT_SEPARATOR),
            integer_sign_digit_separator: has_flag!(format, INTEGER_SIGN_DIGIT_SEPARATOR),
            integer_consecutive_sign_digit_separator: has_flag!(
                format,
                INTEGER_CONSECUTIVE_SIGN_DIGIT_SEPARATOR
            ),
            exponent_sign_digit_separator: has_flag!(format, EXPONENT_SIGN_DIGIT_SEPARATOR),
            exponent_consecutive_sign_digit_separator: has_flag!(
                format,
                EXPONENT_CONSECUTIVE_SIGN_DIGIT_SEPARATOR
            ),
            base_prefix_internal_digit_separator: has_flag!(
                format,
                BASE_PREFIX_INTERNAL_DIGIT_SEPARATOR
            ),
            base_prefix_leading_digit_separator: has_flag!(
                format,
                BASE_PREFIX_LEADING_DIGIT_SEPARATOR
            ),
            base_prefix_trailing_digit_separator: has_flag!(
                format,
                BASE_PREFIX_TRAILING_DIGIT_SEPARATOR
            ),
            base_prefix_consecutive_digit_separator: has_flag!(
                format,
                BASE_PREFIX_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            base_suffix_internal_digit_separator: has_flag!(
                format,
                BASE_SUFFIX_INTERNAL_DIGIT_SEPARATOR
            ),
            base_suffix_leading_digit_separator: has_flag!(
                format,
                BASE_SUFFIX_LEADING_DIGIT_SEPARATOR
            ),
            base_suffix_trailing_digit_separator: has_flag!(
                format,
                BASE_SUFFIX_TRAILING_DIGIT_SEPARATOR
            ),
            base_suffix_consecutive_digit_separator: has_flag!(
                format,
                BASE_SUFFIX_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            integer_internal_digit_separator: has_flag!(format, INTEGER_INTERNAL_DIGIT_SEPARATOR),
            fraction_internal_digit_separator: has_flag!(format, FRACTION_INTERNAL_DIGIT_SEPARATOR),
            exponent_internal_digit_separator: has_flag!(format, EXPONENT_INTERNAL_DIGIT_SEPARATOR),
            integer_leading_digit_separator: has_flag!(format, INTEGER_LEADING_DIGIT_SEPARATOR),
            fraction_leading_digit_separator: has_flag!(format, FRACTION_LEADING_DIGIT_SEPARATOR),
            exponent_leading_digit_separator: has_flag!(format, EXPONENT_LEADING_DIGIT_SEPARATOR),
            integer_trailing_digit_separator: has_flag!(format, INTEGER_TRAILING_DIGIT_SEPARATOR),
            fraction_trailing_digit_separator: has_flag!(format, FRACTION_TRAILING_DIGIT_SEPARATOR),
            exponent_trailing_digit_separator: has_flag!(format, EXPONENT_TRAILING_DIGIT_SEPARATOR),
            integer_consecutive_digit_separator: has_flag!(
                format,
                INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            fraction_consecutive_digit_separator: has_flag!(
                format,
                FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            exponent_consecutive_digit_separator: has_flag!(
                format,
                EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            special_digit_separator: has_flag!(format, SPECIAL_DIGIT_SEPARATOR),
        }
    }
}

impl Default for NumberFormatBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
