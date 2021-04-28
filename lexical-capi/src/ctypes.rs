//! Type aliases for C types.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

// We want these to be similar to the C type names, so use C
// type name conventions.

pub(crate) type size_t = usize;
pub(crate) type uint8_t = u8;
pub(crate) type uint16_t = u16;
pub(crate) type uint32_t = u32;
pub(crate) type uint64_t = u64;
pub(crate) type int8_t = i8;
pub(crate) type int16_t = i16;
pub(crate) type int32_t = i32;
pub(crate) type int64_t = i64;
