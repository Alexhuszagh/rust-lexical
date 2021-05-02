// C-compatible API for lexical conversion routines.

use crate::lib::slice;
use lexical_core;

use super::options::*;

// HELPERS

/// Calculate the difference between two pointers.
#[inline]
pub fn distance<T>(first: *const T, last: *const T) -> usize {
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

/// Convert a mutable pointer range to a mutable slice safely.
#[inline]
pub(crate) unsafe fn slice_from_range_mut<'a, T>(first: *mut T, last: *mut T) -> &'a mut [T] {
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
        $(meta => $(#[$meta:meta])*)?
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($(#[$meta])*)?
        pub unsafe extern fn $name(first: *const u8, last: *const u8)
            -> $crate::result::Result<$type>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            lexical_core::$callback(bytes).into()
        }
    );
}

// Macro to generate the partial parser from a pointer range.
macro_rules! lexical_partial_from_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        $(meta => $(#[$meta:meta])*)?
    ) => (
        #[no_mangle]
        #[doc(hidden)]
        $($(#[$meta])*)?
        pub unsafe extern fn $name(first: *const u8, last: *const u8)
            -> $crate::result::Result<$crate::result::Tuple<$type, usize>>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            match lexical_core::$callback(bytes) {
                Ok(v)  => Ok(v.into()),
                Err(e) => Err(e),
            }.into()
        }
    );
}

/// Macro to generate complete options parser from a pointer range.
macro_rules! lexical_options_from_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        options => $options_type:ty,
        $(meta => $(#[$meta:meta])*)?
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($(#[$meta])*)?
        pub unsafe extern fn $name(first: *const u8, last: *const u8, options: $options_type)
            -> $crate::result::Result<$type>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            let options = options.into();
            lexical_core::$callback(bytes, &options).into()
        }
    );
}

// Macro to generate the partial parser with options from a pointer range.
macro_rules! lexical_partial_options_from_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        options => $options_type:ty,
        $(meta => $(#[$meta:meta])*)?
    ) => (
        #[no_mangle]
        #[doc(hidden)]
        $($(#[$meta])*)?
        pub unsafe extern fn $name(first: *const u8, last: *const u8, options: $options_type)
            -> $crate::result::Result<$crate::result::Tuple<$type, usize>>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            let options = options.into();
            match lexical_core::$callback(bytes, &options) {
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
        options => $options_type:ty,
        parse => $parse_name:ident,
        partial_parse => $partial_parse_name:ident,
        parse_with_options => $parse_with_options_name:ident,
        partial_parse_with_options => $partial_parse_with_options_name:ident,
        $(meta => $(#[$meta:meta])*)?
    ) => (
        // Parse.
        lexical_from_range!(
            fn $parse_name,
            callback => parse,
            type => $type,
            $(meta => $(#[$meta])*)?
        );

        // Partial parse.
        lexical_partial_from_range!(
            fn $partial_parse_name,
            callback => parse_partial,
            type => $type,
            $(meta => $(#[$meta])*)?
        );

        // Parse with options.
        lexical_options_from_range!(
            fn $parse_with_options_name,
            callback => parse_with_options,
            type => $type,
            options => $options_type,
            $(meta => $(#[$meta])*)?
        );

        // Parse partial with options.
        lexical_partial_options_from_range!(
            fn $partial_parse_with_options_name,
            callback => parse_partial_with_options,
            type => $type,
            options => $options_type,
            $(meta => $(#[$meta])*)?
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
        $(meta => $(#[$meta:meta])*)?
    ) => (
        #[no_mangle]
        #[doc(hidden)]
        $($(#[$meta])*)?
        pub unsafe extern fn $name(value: $type, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            let bytes = $crate::api::slice_from_range_mut(first, last);
            let slc = lexical_core::$callback(value, bytes);
            let len = slc.len();
            slc[len..].as_mut_ptr()
        }
    );
}

/// Macro to generate the lexical to_string_with_options API using a range.
macro_rules! lexical_options_to_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        options => $options_type:ty,
        $(meta => $(#[$meta:meta])*)?
    ) => (
        #[no_mangle]
        #[doc(hidden)]
        $($(#[$meta])*)?
        pub unsafe extern fn $name(value: $type, first: *mut u8, last: *mut u8, options: $options_type)
            -> *mut u8
        {
            let bytes = $crate::api::slice_from_range_mut(first, last);
            let options = options.into();
            let slc = lexical_core::$callback(value, bytes, &options);
            let len = slc.len();
            slc[len..].as_mut_ptr()
        }
    );
}

// Macro to generate serializers implementing the ToLexical trait.
macro_rules! to_lexical {
    (
        type => $type:ty,
        options => $options_type:ty,
        write => $write_name:ident,
        write_with_options => $write_with_options_name:ident,
        $(meta => $(#[$meta:meta])*)?
    ) => (
        // Write
        lexical_to_range!(
            fn $write_name,
            callback => write,
            type => $type,
            $(meta => $(#[$meta])*)?
        );

        // Write with options
        lexical_options_to_range!(
            fn $write_with_options_name,
            callback => write_with_options,
            type => $type,
            options => $options_type,
            $(meta => $(#[$meta])*)?
        );
    );
}

// API

// ATOF
from_lexical!(
    type => f32,
    options => ParseFloatOptions,
    parse => lexical_atof32,
    partial_parse => lexical_atof32_partial,
    parse_with_options => lexical_atof32_with_options,
    partial_parse_with_options => lexical_atof32_partial_with_options,
);
from_lexical!(
    type => f64,
    options => ParseFloatOptions,
    parse => lexical_atof64,
    partial_parse => lexical_atof64_partial,
    parse_with_options => lexical_atof64_with_options,
    partial_parse_with_options => lexical_atof64_partial_with_options,
);

// ATOI
from_lexical!(
    type => u8,
    options => ParseIntegerOptions,
    parse => lexical_atou8,
    partial_parse => lexical_atou8_partial,
    parse_with_options => lexical_atou8_with_options,
    partial_parse_with_options => lexical_atou8_partial_with_options,
);
from_lexical!(
    type => u16,
    options => ParseIntegerOptions,
    parse => lexical_atou16,
    partial_parse => lexical_atou16_partial,
    parse_with_options => lexical_atou16_with_options,
    partial_parse_with_options => lexical_atou16_partial_with_options,
);
from_lexical!(
    type => u32,
    options => ParseIntegerOptions,
    parse => lexical_atou32,
    partial_parse => lexical_atou32_partial,
    parse_with_options => lexical_atou32_with_options,
    partial_parse_with_options => lexical_atou32_partial_with_options,
);
from_lexical!(
    type => u64,
    options => ParseIntegerOptions,
    parse => lexical_atou64,
    partial_parse => lexical_atou64_partial,
    parse_with_options => lexical_atou64_with_options,
    partial_parse_with_options => lexical_atou64_partial_with_options,
);
from_lexical!(
    type => usize,
    options => ParseIntegerOptions,
    parse => lexical_atousize,
    partial_parse => lexical_atousize_partial,
    parse_with_options => lexical_atousize_with_options,
    partial_parse_with_options => lexical_atousize_partial_with_options,
);
#[cfg(feature = "i128")]
from_lexical!(
    type => u128,
    options => ParseIntegerOptions,
    parse => lexical_atou128,
    partial_parse => lexical_atou128_partial,
    parse_with_options => lexical_atou128_with_options,
    partial_parse_with_options => lexical_atou128_partial_with_options,
    meta => #[allow(improper_ctypes_definitions)]
);

from_lexical!(
    type => i8,
    options => ParseIntegerOptions,
    parse => lexical_atoi8,
    partial_parse => lexical_atoi8_partial,
    parse_with_options => lexical_atoi8_with_options,
    partial_parse_with_options => lexical_atoi8_partial_with_options,
);
from_lexical!(
    type => i16,
    options => ParseIntegerOptions,
    parse => lexical_atoi16,
    partial_parse => lexical_atoi16_partial,
    parse_with_options => lexical_atoi16_with_options,
    partial_parse_with_options => lexical_atoi16_partial_with_options,
);
from_lexical!(
    type => i32,
    options => ParseIntegerOptions,
    parse => lexical_atoi32,
    partial_parse => lexical_atoi32_partial,
    parse_with_options => lexical_atoi32_with_options,
    partial_parse_with_options => lexical_atoi32_partial_with_options,
);
from_lexical!(
    type => i64,
    options => ParseIntegerOptions,
    parse => lexical_atoi64,
    partial_parse => lexical_atoi64_partial,
    parse_with_options => lexical_atoi64_with_options,
    partial_parse_with_options => lexical_atoi64_partial_with_options,
);
from_lexical!(
    type => isize,
    options => ParseIntegerOptions,
    parse => lexical_atoisize,
    partial_parse => lexical_atoisize_partial,
    parse_with_options => lexical_atoisize_with_options,
    partial_parse_with_options => lexical_atoisize_partial_with_options,
);
#[cfg(feature = "i128")]
from_lexical!(
    type => i128,
    options => ParseIntegerOptions,
    parse => lexical_atoi128,
    partial_parse => lexical_atoi128_partial,
    parse_with_options => lexical_atoi128_with_options,
    partial_parse_with_options => lexical_atoi128_partial_with_options,
    meta => #[allow(improper_ctypes_definitions)]
);

// FTOA
to_lexical!(
    type => f32,
    options => WriteFloatOptions,
    write => lexical_f32toa,
    write_with_options => lexical_f32toa_with_options,
);
to_lexical!(
    type => f64,
    options => WriteFloatOptions,
    write => lexical_f64toa,
    write_with_options => lexical_f64toa_with_options,
);

// ITOA
to_lexical!(
    type => u8,
    options => WriteIntegerOptions,
    write => lexical_u8toa,
    write_with_options => lexical_u8toa_with_options,
);
to_lexical!(
    type => u16,
    options => WriteIntegerOptions,
    write => lexical_u16toa,
    write_with_options => lexical_u16toa_with_options,
);
to_lexical!(
    type => u32,
    options => WriteIntegerOptions,
    write => lexical_u32toa,
    write_with_options => lexical_u32toa_with_options,
);
to_lexical!(
    type => u64,
    options => WriteIntegerOptions,
    write => lexical_u64toa,
    write_with_options => lexical_u64toa_with_options,
);
to_lexical!(
    type => usize,
    options => WriteIntegerOptions,
    write => lexical_usizetoa,
    write_with_options => lexical_usizetoa_with_options,
);
#[cfg(feature = "i128")]
to_lexical!(
    type => u128,
    options => WriteIntegerOptions,
    write => lexical_u128toa,
    write_with_options => lexical_u128toa_with_options,
    meta => #[allow(improper_ctypes_definitions)]
);

to_lexical!(
    type => i8,
    options => WriteIntegerOptions,
    write => lexical_i8toa,
    write_with_options => lexical_i8toa_with_options,
);
to_lexical!(
    type => i16,
    options => WriteIntegerOptions,
    write => lexical_i16toa,
    write_with_options => lexical_i16toa_with_options,
);
to_lexical!(
    type => i32,
    options => WriteIntegerOptions,
    write => lexical_i32toa,
    write_with_options => lexical_i32toa_with_options,
);
to_lexical!(
    type => i64,
    options => WriteIntegerOptions,
    write => lexical_i64toa,
    write_with_options => lexical_i64toa_with_options,
);
to_lexical!(
    type => isize,
    options => WriteIntegerOptions,
    write => lexical_isizetoa,
    write_with_options => lexical_isizetoa_with_options,
);
#[cfg(feature = "i128")]
to_lexical!(
    type => i128,
    options => WriteIntegerOptions,
    write => lexical_i128toa,
    write_with_options => lexical_i128toa_with_options,
    meta => #[allow(improper_ctypes_definitions)]
);
