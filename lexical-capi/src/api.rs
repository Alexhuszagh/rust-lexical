// C-compatible API for lexical conversion routines.

use lib::slice;
use lexical_core;

// HELPERS

/// Calculate the difference between two pointers.
#[inline]
pub fn distance<T>(first: *const T, last: *const T)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

/// Convert a mutable pointer range to a mutable slice safely.
#[inline]
pub(crate) unsafe fn slice_from_range_mut<'a, T>(first: *mut T, last: *mut T)
    -> &'a mut [T]
{
    assert!(first <= last && !first.is_null() && !last.is_null());
    slice::from_raw_parts_mut(first, distance(first, last))
}

// FROM LEXICAL

/// Macro to generate complete parser from a pointer range.
macro_rules! lexical_from_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        args => $($argname:ident : $argtype:ty ;)*,
        condition => $($condition:tt)*
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($condition)*
        pub unsafe extern fn $name(first: *const u8, last: *const u8 $(,$argname : $argtype)*)
            -> $crate::result::Result<$type>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            lexical_core::$callback(bytes $(,$argname)*).into()
        }
    );
}

// Macro to generate the partial parser from a pointer range.
macro_rules! lexical_partial_from_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        args => $($argname:ident : $argtype:ty ;)*,
        condition => $($condition:tt)*
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($condition)*
        pub unsafe extern fn $name(first: *const u8, last: *const u8 $(,$argname : $argtype)*)
            -> $crate::result::Result<$crate::result::Tuple<$type, usize>>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            match lexical_core::$callback(bytes $(,$argname)*) {
                Ok(v)  => Ok(v.into()),
                Err(e) => Err(e),
            }.into()
        }
    );
}

// Macro to generate parsers implementing the FromLexical trait.
macro_rules! from_lexical {
    (
        type => $type:ty,
        decimal => $decimal_name:ident,
        partial_decimal => $partial_decimal_name:ident,
        radix => $radix_name:ident,
        partial_radix => $partial_radix_name:ident
    ) => (
        // Decimal.
        lexical_from_range!(
            fn $decimal_name,
            callback => parse,
            type => $type,
            args =>,
            condition =>
        );

        // Partial decimal.
        lexical_partial_from_range!(
            fn $partial_decimal_name,
            callback => parse_partial,
            type => $type,
            args =>,
            condition =>
        );

        // Radix.
        lexical_from_range!(
            fn $radix_name,
            callback => parse_radix,
            type => $type,
            args => radix: u8 ;,
            condition => #[cfg(feature = "radix")]
        );

        // Partial radix.
        lexical_partial_from_range!(
            fn $partial_radix_name,
            callback => parse_partial_radix,
            type => $type,
            args => radix: u8 ;,
            condition => #[cfg(feature = "radix")]
        );
    );
}

// Macro to generate parsers implementing the FromLexicalLossy trait.
macro_rules! from_lexical_lossy {
    (
        type => $type:ty,
        decimal => $decimal_name:ident,
        partial_decimal => $partial_decimal_name:ident,
        radix => $radix_name:ident,
        partial_radix => $partial_radix_name:ident
    ) => (
        // Decimal.
        lexical_from_range!(
            fn $decimal_name,
            callback => parse_lossy,
            type => $type,
            args =>,
            condition =>
        );

        // Partial decimal.
        lexical_partial_from_range!(
            fn $partial_decimal_name,
            callback => parse_partial_lossy,
            type => $type,
            args =>,
            condition =>
        );

        // Radix.
        lexical_from_range!(
            fn $radix_name,
            callback => parse_lossy_radix,
            type => $type,
            args => radix: u8 ;,
            condition => #[cfg(feature = "radix")]
        );

        // Partial radix.
        lexical_partial_from_range!(
            fn $partial_radix_name,
            callback => parse_partial_lossy_radix,
            type => $type,
            args => radix: u8 ;,
            condition => #[cfg(feature = "radix")]
        );
    );
}

macro_rules! from_lexical_format {
    (
        type => $type:ty,
        decimal => $decimal_name:ident,
        partial_decimal => $partial_decimal_name:ident,
        radix => $radix_name:ident,
        partial_radix => $partial_radix_name:ident
    ) => (
        // Decimal.
        lexical_from_range!(
            fn $decimal_name,
            callback => parse_format,
            type => $type,
            args => format: lexical_core::NumberFormat ; ,
            condition => #[cfg(feature = "format")]
        );

        // Partial decimal.
        lexical_partial_from_range!(
            fn $partial_decimal_name,
            callback => parse_partial_format,
            type => $type,
            args => format: lexical_core::NumberFormat ; ,
            condition => #[cfg(feature = "format")]
        );

        // Radix.
        lexical_from_range!(
            fn $radix_name,
            callback => parse_format_radix,
            type => $type,
            args => radix: u8 ; format: lexical_core::NumberFormat ; ,
            condition => #[cfg(all(feature = "radix", feature = "format"))]
        );

        // Partial radix.
        lexical_partial_from_range!(
            fn $partial_radix_name,
            callback => parse_partial_format_radix,
            type => $type,
            args => radix: u8 ; format: lexical_core::NumberFormat ; ,
            condition => #[cfg(all(feature = "radix", feature = "format"))]
        );
    );
}

macro_rules! from_lexical_lossy_format {
    (
        type => $type:ty,
        decimal => $decimal_name:ident,
        partial_decimal => $partial_decimal_name:ident,
        radix => $radix_name:ident,
        partial_radix => $partial_radix_name:ident
    ) => (
        // Decimal.
        lexical_from_range!(
            fn $decimal_name,
            callback => parse_lossy_format,
            type => $type,
            args => format: lexical_core::NumberFormat ; ,
            condition => #[cfg(feature = "format")]
        );

        // Partial decimal.
        lexical_partial_from_range!(
            fn $partial_decimal_name,
            callback => parse_partial_lossy_format,
            type => $type,
            args => format: lexical_core::NumberFormat ; ,
            condition => #[cfg(feature = "format")]
        );

        // Radix.
        lexical_from_range!(
            fn $radix_name,
            callback => parse_lossy_format_radix,
            type => $type,
            args => radix: u8 ; format: lexical_core::NumberFormat ; ,
            condition => #[cfg(all(feature = "radix", feature = "format"))]
        );

        // Partial radix.
        lexical_partial_from_range!(
            fn $partial_radix_name,
            callback => parse_partial_lossy_format_radix,
            type => $type,
            args => radix: u8 ; format: lexical_core::NumberFormat ; ,
            condition => #[cfg(all(feature = "radix", feature = "format"))]
        );
    );
}

// TO LEXICAL

/// Macro to generate the lexical to_string API using a range.
macro_rules! lexical_to_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        args => $($argname:ident : $argtype:ty)*,
        condition => $($condition:tt)*
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($condition)*
        pub unsafe extern fn $name(value: $type $(,$argname : $argtype)* , first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            let bytes = $crate::api::slice_from_range_mut(first, last);
            let slc = lexical_core::$callback(value $(,$argname)* , bytes);
            let len = slc.len();
            slc[len..].as_mut_ptr()
        }
    );
}

// Macro to generate serializers implementing the ToLexical trait.
macro_rules! to_lexical {
    (
        type => $type:ty,
        decimal => $decimal_name:ident,
        radix => $radix_name:ident
    ) => (
        // Decimal
        lexical_to_range!(
            fn $decimal_name,
            callback => write,
            type => $type,
            args =>,
            condition =>
        );

        // Radix
        lexical_to_range!(
            fn $radix_name,
            callback => write_radix,
            type => $type,
            args => radix: u8,
            condition => #[cfg(feature = "radix")]
        );
    );
}

// API

// ATOF
from_lexical!(
    type => f32,
    decimal => lexical_atof32,
    partial_decimal => lexical_atof32_partial,
    radix => lexical_atof32_radix,
    partial_radix => lexical_atof32_partial_radix
);
from_lexical!(
    type => f64,
    decimal => lexical_atof64,
    partial_decimal => lexical_atof64_partial,
    radix => lexical_atof64_radix,
    partial_radix => lexical_atof64_partial_radix
);
from_lexical_lossy!(
    type => f32,
    decimal => lexical_atof32_lossy,
    partial_decimal => lexical_atof32_partial_lossy,
    radix => lexical_atof32_lossy_radix,
    partial_radix => lexical_atof32_partial_lossy_radix
);
from_lexical_lossy!(
    type => f64,
    decimal => lexical_atof64_lossy,
    partial_decimal => lexical_atof64_partial_lossy,
    radix => lexical_atof64_lossy_radix,
    partial_radix => lexical_atof64_partial_lossy_radix
);

// ATOF FORMAT
from_lexical_format!(
    type => f32,
    decimal => lexical_atof32_format,
    partial_decimal => lexical_atof32_partial_format,
    radix => lexical_atof32_format_radix,
    partial_radix => lexical_atof32_partial_format_radix
);
from_lexical_format!(
    type => f64,
    decimal => lexical_atof64_format,
    partial_decimal => lexical_atof64_partial_format,
    radix => lexical_atof64_format_radix,
    partial_radix => lexical_atof64_partial_format_radix
);
from_lexical_lossy_format!(
    type => f32,
    decimal => lexical_atof32_lossy_format,
    partial_decimal => lexical_atof32_partial_lossy_format,
    radix => lexical_atof32_lossy_format_radix,
    partial_radix => lexical_atof32_partial_lossy_format_radix
);
from_lexical_lossy_format!(
    type => f64,
    decimal => lexical_atof64_lossy_format,
    partial_decimal => lexical_atof64_partial_lossy_format,
    radix => lexical_atof64_lossy_format_radix,
    partial_radix => lexical_atof64_partial_lossy_format_radix
);

// ATOI
from_lexical!(
    type => u8,
    decimal => lexical_atou8,
    partial_decimal => lexical_atou8_partial,
    radix => lexical_atou8_radix,
    partial_radix => lexical_atou8_partial_radix
);
from_lexical!(
    type => u16,
    decimal => lexical_atou16,
    partial_decimal => lexical_atou16_partial,
    radix => lexical_atou16_radix,
    partial_radix => lexical_atou16_partial_radix
);
from_lexical!(
    type => u32,
    decimal => lexical_atou32,
    partial_decimal => lexical_atou32_partial,
    radix => lexical_atou32_radix,
    partial_radix => lexical_atou32_partial_radix
);
from_lexical!(
    type => u64,
    decimal => lexical_atou64,
    partial_decimal => lexical_atou64_partial,
    radix => lexical_atou64_radix,
    partial_radix => lexical_atou64_partial_radix
);
from_lexical!(
    type => usize,
    decimal => lexical_atousize,
    partial_decimal => lexical_atousize_partial,
    radix => lexical_atousize_radix,
    partial_radix => lexical_atousize_partial_radix
);
#[cfg(has_i128)]
from_lexical!(
    type => u128,
    decimal => lexical_atou128,
    partial_decimal => lexical_atou128_partial,
    radix => lexical_atou128_radix,
    partial_radix => lexical_atou128_partial_radix
);

from_lexical!(
    type => i8,
    decimal => lexical_atoi8,
    partial_decimal => lexical_atoi8_partial,
    radix => lexical_atoi8_radix,
    partial_radix => lexical_atoi8_partial_radix
);
from_lexical!(
    type => i16,
    decimal => lexical_atoi16,
    partial_decimal => lexical_atoi16_partial,
    radix => lexical_atoi16_radix,
    partial_radix => lexical_atoi16_partial_radix
);
from_lexical!(
    type => i32,
    decimal => lexical_atoi32,
    partial_decimal => lexical_atoi32_partial,
    radix => lexical_atoi32_radix,
    partial_radix => lexical_atoi32_partial_radix
);
from_lexical!(
    type => i64,
    decimal => lexical_atoi64,
    partial_decimal => lexical_atoi64_partial,
    radix => lexical_atoi64_radix,
    partial_radix => lexical_atoi64_partial_radix
);
from_lexical!(
    type => isize,
    decimal => lexical_atoisize,
    partial_decimal => lexical_atoisize_partial,
    radix => lexical_atoisize_radix,
    partial_radix => lexical_atoisize_partial_radix
);
#[cfg(has_i128)]
from_lexical!(
    type => i128,
    decimal => lexical_atoi128,
    partial_decimal => lexical_atoi128_partial,
    radix => lexical_atoi128_radix,
    partial_radix => lexical_atoi128_partial_radix
);

// ATOI FORMAT
from_lexical_format!(
    type => u8,
    decimal => lexical_atou8_format,
    partial_decimal => lexical_atou8_partial_format,
    radix => lexical_atou8_format_radix,
    partial_radix => lexical_atou8_partial_format_radix
);

from_lexical_format!(
    type => u16,
    decimal => lexical_atou16_format,
    partial_decimal => lexical_atou16_partial_format,
    radix => lexical_atou16_format_radix,
    partial_radix => lexical_atou16_partial_format_radix
);
from_lexical_format!(
    type => u32,
    decimal => lexical_atou32_format,
    partial_decimal => lexical_atou32_partial_format,
    radix => lexical_atou32_format_radix,
    partial_radix => lexical_atou32_partial_format_radix
);
from_lexical_format!(
    type => u64,
    decimal => lexical_atou64_format,
    partial_decimal => lexical_atou64_partial_format,
    radix => lexical_atou64_format_radix,
    partial_radix => lexical_atou64_partial_format_radix
);
from_lexical_format!(
    type => usize,
    decimal => lexical_atousize_format,
    partial_decimal => lexical_atousize_partial_format,
    radix => lexical_atousize_format_radix,
    partial_radix => lexical_atousize_partial_format_radix
);
#[cfg(has_i128)]
from_lexical_format!(
    type => u128,
    decimal => lexical_atou128_format,
    partial_decimal => lexical_atou128_partial_format,
    radix => lexical_atou128_format_radix,
    partial_radix => lexical_atou128_partial_format_radix
);

from_lexical_format!(
    type => i8,
    decimal => lexical_atoi8_format,
    partial_decimal => lexical_atoi8_partial_format,
    radix => lexical_atoi8_format_radix,
    partial_radix => lexical_atoi8_partial_format_radix
);
from_lexical_format!(
    type => i16,
    decimal => lexical_atoi16_format,
    partial_decimal => lexical_atoi16_partial_format,
    radix => lexical_atoi16_format_radix,
    partial_radix => lexical_atoi16_partial_format_radix
);
from_lexical_format!(
    type => i32,
    decimal => lexical_atoi32_format,
    partial_decimal => lexical_atoi32_partial_format,
    radix => lexical_atoi32_format_radix,
    partial_radix => lexical_atoi32_partial_format_radix
);
from_lexical_format!(
    type => i64,
    decimal => lexical_atoi64_format,
    partial_decimal => lexical_atoi64_partial_format,
    radix => lexical_atoi64_format_radix,
    partial_radix => lexical_atoi64_partial_format_radix
);
from_lexical_format!(
    type => isize,
    decimal => lexical_atoisize_format,
    partial_decimal => lexical_atoisize_partial_format,
    radix => lexical_atoisize_format_radix,
    partial_radix => lexical_atoisize_partial_format_radix
);
#[cfg(has_i128)]
from_lexical_format!(
    type => i128,
    decimal => lexical_atoi128_format,
    partial_decimal => lexical_atoi128_partial_format,
    radix => lexical_atoi128_format_radix,
    partial_radix => lexical_atoi128_partial_format_radix
);

// FTOA
to_lexical!(
    type => f32,
    decimal => lexical_f32toa,
    radix => lexical_f32toa_radix
);
to_lexical!(
    type => f64,
    decimal => lexical_f64toa,
    radix => lexical_f64toa_radix
);

// ITOA
to_lexical!(
    type => u8,
    decimal => lexical_u8toa,
    radix => lexical_u8toa_radix
);
to_lexical!(
    type => u16,
    decimal => lexical_u16toa,
    radix => lexical_u16toa_radix
);
to_lexical!(
    type => u32,
    decimal => lexical_u32toa,
    radix => lexical_u32toa_radix
);
to_lexical!(
    type => u64,
    decimal => lexical_u64toa,
    radix => lexical_u64toa_radix
);
to_lexical!(
    type => usize,
    decimal => lexical_usizetoa,
    radix => lexical_usizetoa_radix
);
#[cfg(has_i128)]
to_lexical!(
    type => u128,
    decimal => lexical_u128toa,
    radix => lexical_u128toa_radix
);

to_lexical!(
    type => i8,
    decimal => lexical_i8toa,
    radix => lexical_i8toa_radix
);
to_lexical!(
    type => i16,
    decimal => lexical_i16toa,
    radix => lexical_i16toa_radix
);
to_lexical!(
    type => i32,
    decimal => lexical_i32toa,
    radix => lexical_i32toa_radix
);
to_lexical!(
    type => i64,
    decimal => lexical_i64toa,
    radix => lexical_i64toa_radix
);
to_lexical!(
    type => isize,
    decimal => lexical_isizetoa,
    radix => lexical_isizetoa_radix
);
#[cfg(has_i128)]
to_lexical!(
    type => i128,
    decimal => lexical_i128toa,
    radix => lexical_i128toa_radix
);
