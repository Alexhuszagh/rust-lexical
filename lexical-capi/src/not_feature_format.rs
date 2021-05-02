//! C-compatible NumberFormat functions.

#![cfg(not(feature = "format"))]

use super::option::Option;

/// Builder for `NumberFormat`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NumberFormatBuilder {
    decimal_point: u8,
    exponent_decimal: u8,
    exponent_backup: u8,
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::NumberFormatBuilder> for NumberFormatBuilder {
    #[inline(always)]
    fn from(builder: lexical_core::NumberFormatBuilder) -> NumberFormatBuilder {
        NumberFormatBuilder {
            decimal_point: builder.get_decimal_point(),
            exponent_decimal: builder.get_exponent_decimal(),
            exponent_backup: builder.get_exponent_backup(),
        }
    }
}

impl Into<lexical_core::NumberFormatBuilder> for NumberFormatBuilder {
    #[inline(always)]
    fn into(self) -> lexical_core::NumberFormatBuilder {
        lexical_core::NumberFormatBuilder::new()
            .decimal_point(self.decimal_point)
            .exponent_decimal(self.exponent_decimal)
            .exponent_backup(self.exponent_backup)
    }
}

impl Default for NumberFormatBuilder {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::NumberFormatBuilder::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn lexical_number_format_rebuild(
    format: lexical_core::NumberFormat,
) -> NumberFormatBuilder {
    format.rebuild().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn lexical_number_format_builder_new() -> NumberFormatBuilder {
    lexical_core::NumberFormatBuilder::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn lexical_number_format_builder_build(
    builder: NumberFormatBuilder,
) -> Option<lexical_core::NumberFormat> {
    let builder: lexical_core::NumberFormatBuilder = builder.into();
    builder.build().map(|opts| opts.into()).into()
}
