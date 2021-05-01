//! C-compatible parse and write options types.

#[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))]
use lexical_core;

#[cfg(feature = "atof")]
use crate::lib::slice;

#[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))]
use super::ctypes;
#[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))]
use super::option::Option;

// NOTE:
//  NumberFormat uses bitflags of u64, which is guaranteed to be
//  FFI safe for C APIs. Likewise, RoundingKind uses bitflags
// of u32.

// PARSE INTEGER OPTIONS

cfg_if!{
if #[cfg(feature = "atoi")] {
/// Builder for `ParseIntegerOptions`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParseIntegerOptionsBuilder {
    /// Radix for integer string.
    radix: ctypes::uint8_t,
    /// Number format (this is guaranteed to be u64).
    format: Option<lexical_core::NumberFormat>,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::ParseIntegerOptionsBuilder> for ParseIntegerOptionsBuilder {
    #[inline(always)]
    fn from(builder: lexical_core::ParseIntegerOptionsBuilder) -> ParseIntegerOptionsBuilder {
        ParseIntegerOptionsBuilder {
            radix: builder.get_radix(),
            format: builder.get_format().into()
        }
    }
}

impl Into<lexical_core::ParseIntegerOptionsBuilder> for ParseIntegerOptionsBuilder {
    #[inline(always)]
    fn into(self) -> lexical_core::ParseIntegerOptionsBuilder {
        let builder = lexical_core::ParseIntegerOptionsBuilder::new()
            .format(self.format.into());
        #[cfg(feature = "radix")]
        let builder = builder.radix(self.radix);

        builder
    }
}

impl Default for ParseIntegerOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::ParseIntegerOptionsBuilder::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_integer_options_builder_new()
    -> ParseIntegerOptionsBuilder
{
    lexical_core::ParseIntegerOptionsBuilder::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_integer_options_builder_build(builder: ParseIntegerOptionsBuilder)
    -> Option<ParseIntegerOptions>
{
    let builder: lexical_core::ParseIntegerOptionsBuilder = builder.into();
    builder.build().map(|opts| opts.into()).into()
}

/// Options to customize parsing integers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParseIntegerOptions {
    /// Radix for integer string.
    radix: ctypes::uint32_t,
    /// Number format.
    format: Option<lexical_core::NumberFormat>,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::ParseIntegerOptions> for ParseIntegerOptions {
    #[inline(always)]
    fn from(options: lexical_core::ParseIntegerOptions) -> ParseIntegerOptions {
        ParseIntegerOptions {
            radix: options.radix(),
            format: options.format().into()
        }
    }
}

impl Into<lexical_core::ParseIntegerOptions> for ParseIntegerOptions {
    #[inline(always)]
    fn into(self) -> lexical_core::ParseIntegerOptions {
        unsafe {
            let mut options = lexical_core::ParseIntegerOptions::new();
            options.set_radix(self.radix);
            options.set_format(self.format.into());
            options
        }
    }
}

impl Default for ParseIntegerOptions {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::ParseIntegerOptions::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_integer_options_new()
    -> ParseIntegerOptions
{
    lexical_core::ParseIntegerOptions::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_integer_options_builder() -> ParseIntegerOptionsBuilder
{
    ParseIntegerOptionsBuilder::default()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_integer_options_rebuild(options: ParseIntegerOptions) -> ParseIntegerOptionsBuilder
{
    let options: lexical_core::ParseIntegerOptions = options.into();
    options.rebuild().into()
}
}}  // cfg_if

// PARSE FLOAT OPTIONS

cfg_if!{
if #[cfg(feature = "atof")] {
/// Builder for `ParseFloatOptions`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParseFloatOptionsBuilder {
    /// Radix for float string.
    radix: ctypes::uint8_t,
    /// Numerical base for the exponent in the float string.
    exponent_base: ctypes::uint8_t,
    /// Radix for the exponent digits in the float string.
    exponent_radix: ctypes::uint8_t,
    /// Number format.
    format: lexical_core::NumberFormat,
    /// Rounding kind for float.
    rounding: lexical_core::RoundingKind,
    /// Use the incorrect, fast parser.
    incorrect: bool,
    /// Use the lossy, intermediate parser.
    lossy: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string_ptr: *const u8,
    nan_string_size: ctypes::size_t,
    /// Short string representation of `Infinity`.
    inf_string_ptr: *const u8,
    inf_string_size: ctypes::size_t,
    /// Long string representation of `Infinity`.
    infinity_string_ptr: *const u8,
    infinity_string_size: ctypes::size_t,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::ParseFloatOptionsBuilder> for ParseFloatOptionsBuilder {
    #[inline(always)]
    fn from(builder: lexical_core::ParseFloatOptionsBuilder) -> ParseFloatOptionsBuilder {
        // This is safe because we are sure the byte slice references
        // are static.
        ParseFloatOptionsBuilder {
            radix: builder.get_radix(),
            exponent_base: builder.get_exponent_base(),
            exponent_radix: builder.get_exponent_radix(),
            format: builder.get_format(),
            rounding: builder.get_rounding(),
            incorrect: builder.get_incorrect(),
            lossy: builder.get_lossy(),
            nan_string_ptr: builder.get_nan_string().as_ptr(),
            nan_string_size: builder.get_nan_string().len(),
            inf_string_ptr: builder.get_inf_string().as_ptr(),
            inf_string_size: builder.get_inf_string().len(),
            infinity_string_ptr: builder.get_infinity_string().as_ptr(),
            infinity_string_size: builder.get_infinity_string().len(),
        }
    }
}

impl Into<lexical_core::ParseFloatOptionsBuilder> for ParseFloatOptionsBuilder {
    #[inline(always)]
    fn into(self) -> lexical_core::ParseFloatOptionsBuilder {
        unsafe {
            let builder = lexical_core::ParseFloatOptionsBuilder::new()
                .format(Some(self.format))
                .incorrect(self.incorrect)
                .lossy(self.lossy)
                .nan_string(slice::from_raw_parts(self.nan_string_ptr, self.nan_string_size))
                .inf_string(slice::from_raw_parts(self.inf_string_ptr, self.inf_string_size))
                .infinity_string(slice::from_raw_parts(self.infinity_string_ptr, self.infinity_string_size));

            #[cfg(feature = "radix")]
            let builder = builder.radix(self.radix);

            #[cfg(feature = "radix")]
            let builder = builder.exponent_base(self.exponent_base);

            #[cfg(feature = "radix")]
            let builder = builder.exponent_radix(self.exponent_radix);

            #[cfg(feature = "rounding")]
            let builder = builder.rounding(self.rounding);

            builder
        }
    }
}

impl Default for ParseFloatOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::ParseFloatOptionsBuilder::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_float_options_builder_new()
    -> ParseFloatOptionsBuilder
{
    lexical_core::ParseFloatOptionsBuilder::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_float_options_builder_build(builder: ParseFloatOptionsBuilder)
    -> Option<ParseFloatOptions>
{
    let builder: lexical_core::ParseFloatOptionsBuilder = builder.into();
    builder.build().map(|opts| opts.into()).into()
}

/// Options to customize parsing floats.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ParseFloatOptions {
    /// Compressed storage of the radix, rounding kind, incorrect, and lossy.
    compressed: ctypes::uint32_t,
    /// Number format.
    format: lexical_core::NumberFormat,
    /// String representation of Not A Number, aka `NaN`.
    nan_string_ptr: *const u8,
    nan_string_size: ctypes::size_t,
    /// Short string representation of `Infinity`.
    inf_string_ptr: *const u8,
    inf_string_size: ctypes::size_t,
    /// Long string representation of `Infinity`.
    infinity_string_ptr: *const u8,
    infinity_string_size: ctypes::size_t,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::ParseFloatOptions> for ParseFloatOptions {
    #[inline(always)]
    fn from(options: lexical_core::ParseFloatOptions) -> ParseFloatOptions {
        let compressed: u32 = options.radix()
            | options.exponent_base() << 8
            | options.exponent_radix() << 16
            | options.rounding().as_u32() << 24
            | (options.incorrect() as u32) << 28
            | (options.lossy() as u32) << 29;

        ParseFloatOptions {
            compressed,
            format: options.format(),
            nan_string_ptr: options.nan_string().as_ptr(),
            nan_string_size: options.nan_string().len(),
            inf_string_ptr: options.inf_string().as_ptr(),
            inf_string_size: options.inf_string().len(),
            infinity_string_ptr: options.infinity_string().as_ptr(),
            infinity_string_size: options.infinity_string().len(),
        }
    }
}

impl Into<lexical_core::ParseFloatOptions> for ParseFloatOptions {
    #[inline(always)]
    fn into(self) -> lexical_core::ParseFloatOptions {
        unsafe {
            let mut options = lexical_core::ParseFloatOptions::new();
            options.set_radix(self.compressed & 0xFF);
            options.set_exponent_base((self.compressed & 0xFF00) >> 8);
            options.set_exponent_radix((self.compressed & 0xFF0000) >> 16);
            options.set_rounding(lexical_core::RoundingKind::from_u32((self.compressed & 0xF000000) >> 24));
            options.set_incorrect(self.compressed & 0x10000000 != 0);
            options.set_lossy(self.compressed & 0x20000000 != 0);
            options.set_format(self.format);
            options.set_nan_string(slice::from_raw_parts(self.nan_string_ptr, self.nan_string_size));
            options.set_inf_string(slice::from_raw_parts(self.inf_string_ptr, self.inf_string_size));
            options.set_infinity_string(slice::from_raw_parts(self.infinity_string_ptr, self.infinity_string_size));
            options
        }
    }
}

impl Default for ParseFloatOptions {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::ParseFloatOptions::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_float_options_new() -> ParseFloatOptions
{
    lexical_core::ParseFloatOptions::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_float_options_builder() -> ParseFloatOptionsBuilder
{
    ParseFloatOptionsBuilder::default()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_parse_float_options_rebuild(options: ParseFloatOptions) -> ParseFloatOptionsBuilder
{
    let options: lexical_core::ParseFloatOptions = options.into();
    options.rebuild().into()
}
}}  // cfg_if

// WRITE INTEGER OPTIONS

cfg_if! {
if #[cfg(feature = "itoa")] {
/// Builder for `WriteIntegerOptions`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteIntegerOptionsBuilder {
    /// Radix for integer string.
    radix: ctypes::uint8_t,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::WriteIntegerOptionsBuilder> for WriteIntegerOptionsBuilder {
    #[inline(always)]
    fn from(builder: lexical_core::WriteIntegerOptionsBuilder) -> WriteIntegerOptionsBuilder {
        WriteIntegerOptionsBuilder {
            radix: builder.get_radix(),
        }
    }
}

impl Into<lexical_core::WriteIntegerOptionsBuilder> for WriteIntegerOptionsBuilder {
    #[inline(always)]
    fn into(self) -> lexical_core::WriteIntegerOptionsBuilder {
        let builder = lexical_core::WriteIntegerOptionsBuilder::new();
        #[cfg(feature = "radix")]
        let builder = builder.radix(self.radix);

        builder
    }
}

impl Default for WriteIntegerOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::WriteIntegerOptionsBuilder::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_integer_options_builder_new() -> WriteIntegerOptionsBuilder
{
    lexical_core::WriteIntegerOptionsBuilder::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_integer_options_builder_build(builder: WriteIntegerOptionsBuilder)
    -> Option<WriteIntegerOptions>
{
    let builder: lexical_core::WriteIntegerOptionsBuilder = builder.into();
    builder.build().map(|opts| opts.into()).into()
}

/// Options to customize writing integers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteIntegerOptions {
    /// Radix for integer string.
    radix: ctypes::uint32_t,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::WriteIntegerOptions> for WriteIntegerOptions {
    #[inline(always)]
    fn from(options: lexical_core::WriteIntegerOptions) -> WriteIntegerOptions {
        WriteIntegerOptions {
            radix: options.radix(),
        }
    }
}

impl Into<lexical_core::WriteIntegerOptions> for WriteIntegerOptions {
    #[inline(always)]
    fn into(self) -> lexical_core::WriteIntegerOptions {
        unsafe {
            let mut options = lexical_core::WriteIntegerOptions::new();
            options.set_radix(self.radix);
            options
        }
    }
}

impl Default for WriteIntegerOptions {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::WriteIntegerOptions::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_integer_options_new() -> WriteIntegerOptions
{
    lexical_core::WriteIntegerOptions::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_integer_options_builder() -> WriteIntegerOptionsBuilder
{
    WriteIntegerOptionsBuilder::default()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_integer_options_rebuild(options: WriteIntegerOptions) -> WriteIntegerOptionsBuilder
{
    let options: lexical_core::WriteIntegerOptions = options.into();
    options.rebuild().into()
}
}}

// WRITE FLOAT OPTIONS

cfg_if! {
if #[cfg(feature = "ftoa")] {
/// Builder for `WriteFloatOptions`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteFloatOptionsBuilder {
    /// Radix for float string.
    radix: ctypes::uint8_t,
    /// Number format.
    format: Option<lexical_core::NumberFormat>,
    /// Trim the trailing ".0" from integral float strings.
    trim_floats: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string_ptr: *const u8,
    nan_string_size: ctypes::size_t,
    /// Short string representation of `Infinity`.
    inf_string_ptr: *const u8,
    inf_string_size: ctypes::size_t,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::WriteFloatOptionsBuilder> for WriteFloatOptionsBuilder {
    #[inline(always)]
    fn from(builder: lexical_core::WriteFloatOptionsBuilder) -> WriteFloatOptionsBuilder {
        // This is safe because we are sure the byte slice references
        // are static.
        WriteFloatOptionsBuilder {
            radix: builder.get_radix(),
            format: builder.get_format().into(),
            trim_floats: builder.get_trim_floats(),
            nan_string_ptr: builder.get_nan_string().as_ptr(),
            nan_string_size: builder.get_nan_string().len(),
            inf_string_ptr: builder.get_inf_string().as_ptr(),
            inf_string_size: builder.get_inf_string().len(),
        }
    }
}

impl Into<lexical_core::WriteFloatOptionsBuilder> for WriteFloatOptionsBuilder {
    #[inline(always)]
    fn into(self) -> lexical_core::WriteFloatOptionsBuilder {
        unsafe {
            let builder = lexical_core::WriteFloatOptionsBuilder::new()
                .format(self.format.into())
                .trim_floats(self.trim_floats)
                .nan_string(slice::from_raw_parts(self.nan_string_ptr, self.nan_string_size))
                .inf_string(slice::from_raw_parts(self.inf_string_ptr, self.inf_string_size));

            #[cfg(feature = "radix")]
            let builder = builder.radix(self.radix);

            builder
        }
    }
}

impl Default for WriteFloatOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::WriteFloatOptionsBuilder::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_float_options_builder_new() -> WriteFloatOptionsBuilder
{
    lexical_core::WriteFloatOptionsBuilder::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_float_options_builder_build(builder: WriteFloatOptionsBuilder)
    -> Option<WriteFloatOptions>
{
    let builder: lexical_core::WriteFloatOptionsBuilder = builder.into();
    builder.build().map(|opts| opts.into()).into()
}

/// Options to customize writing floats.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteFloatOptions {
    /// Compressed storage of radix and trim floats.
    compressed: ctypes::uint32_t,
    /// Number format.
    format: Option<lexical_core::NumberFormat>,
    /// String representation of Not A Number, aka `NaN`.
    nan_string_ptr: *const u8,
    nan_string_size: ctypes::size_t,
    /// Short string representation of `Infinity`.
    inf_string_ptr: *const u8,
    inf_string_size: ctypes::size_t,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::WriteFloatOptions> for WriteFloatOptions {
    #[inline(always)]
    fn from(options: lexical_core::WriteFloatOptions) -> WriteFloatOptions {
        let compressed: u32 = options.radix()
            | (options.trim_floats() as u32) << 8;

        WriteFloatOptions {
            compressed,
            format: options.format().into(),
            nan_string_ptr: options.nan_string().as_ptr(),
            nan_string_size: options.nan_string().len(),
            inf_string_ptr: options.inf_string().as_ptr(),
            inf_string_size: options.inf_string().len(),
        }
    }
}

impl Into<lexical_core::WriteFloatOptions> for WriteFloatOptions {
    #[inline(always)]
    fn into(self) -> lexical_core::WriteFloatOptions {
        unsafe {
            let mut options = lexical_core::WriteFloatOptions::new();
            options.set_radix(self.compressed & 0xFF);
            options.set_trim_floats(self.compressed & 0x100 != 0);
            options.set_format(self.format.into());
            options.set_nan_string(slice::from_raw_parts(self.nan_string_ptr, self.nan_string_size));
            options.set_inf_string(slice::from_raw_parts(self.inf_string_ptr, self.inf_string_size));
            options
        }
    }
}

impl Default for WriteFloatOptions {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::WriteFloatOptions::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_float_options_new() -> WriteFloatOptions
{
    lexical_core::WriteFloatOptions::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_float_options_builder() -> WriteFloatOptionsBuilder
{
    WriteFloatOptionsBuilder::default()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_write_float_options_rebuild(options: WriteFloatOptions) -> WriteFloatOptionsBuilder
{
    let options: lexical_core::WriteFloatOptions = options.into();
    options.rebuild().into()
}
}}  // cfg_if
