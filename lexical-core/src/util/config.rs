//! Config settings for lexical-core.

// CONSTANTS

// The f64 buffer is actually a size of 60, but use 64 since it's a
// power of 2.
pub(crate) const I8_FORMATTED_SIZE_DECIMAL: usize = 4;
pub(crate) const I16_FORMATTED_SIZE_DECIMAL: usize = 6;
pub(crate) const I32_FORMATTED_SIZE_DECIMAL: usize = 11;
pub(crate) const I64_FORMATTED_SIZE_DECIMAL: usize = 20;
pub(crate) const U8_FORMATTED_SIZE_DECIMAL: usize = 3;
pub(crate) const U16_FORMATTED_SIZE_DECIMAL: usize = 5;
pub(crate) const U32_FORMATTED_SIZE_DECIMAL: usize = 10;
pub(crate) const U64_FORMATTED_SIZE_DECIMAL: usize = 20;
pub(crate) const F32_FORMATTED_SIZE_DECIMAL: usize = 64;
pub(crate) const F64_FORMATTED_SIZE_DECIMAL: usize = 64;
pub(crate) const I128_FORMATTED_SIZE_DECIMAL: usize = 40;
pub(crate) const U128_FORMATTED_SIZE_DECIMAL: usize = 39;

// Simple, fast optimization.
// Since we're declaring a variable on the stack, and our power-of-two
// alignment dramatically improved atoi performance, do it.
cfg_if! {
if #[cfg(feature = "radix")] {
    // Use 256, actually, since we seem to have memory issues with f64.
    // Clearly not sufficient memory allocated for non-decimal values.
    pub(crate) const I8_FORMATTED_SIZE: usize = 16;
    pub(crate) const I16_FORMATTED_SIZE: usize = 32;
    pub(crate) const I32_FORMATTED_SIZE: usize = 64;
    pub(crate) const I64_FORMATTED_SIZE: usize = 128;
    pub(crate) const U8_FORMATTED_SIZE: usize = 16;
    pub(crate) const U16_FORMATTED_SIZE: usize = 32;
    pub(crate) const U32_FORMATTED_SIZE: usize = 64;
    pub(crate) const U64_FORMATTED_SIZE: usize = 128;
    pub(crate) const F32_FORMATTED_SIZE: usize = 256;
    pub(crate) const F64_FORMATTED_SIZE: usize = 256;
    pub(crate) const I128_FORMATTED_SIZE: usize = 256;
    pub(crate) const U128_FORMATTED_SIZE: usize = 256;
} else {
    // The f64 buffer is actually a size of 60, but use 64 since it's a
    // power of 2.
    pub(crate) const I8_FORMATTED_SIZE: usize = I8_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I16_FORMATTED_SIZE: usize = I16_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I32_FORMATTED_SIZE: usize = I32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I64_FORMATTED_SIZE: usize = I64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U8_FORMATTED_SIZE: usize = U8_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U16_FORMATTED_SIZE: usize = U16_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U32_FORMATTED_SIZE: usize = U32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U64_FORMATTED_SIZE: usize = U64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const F32_FORMATTED_SIZE: usize = F32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const F64_FORMATTED_SIZE: usize = F64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I128_FORMATTED_SIZE: usize = I128_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U128_FORMATTED_SIZE: usize = U128_FORMATTED_SIZE_DECIMAL;
}} // cfg_if

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    pub(crate) const ISIZE_FORMATTED_SIZE: usize = I16_FORMATTED_SIZE;
    pub(crate) const ISIZE_FORMATTED_SIZE_DECIMAL: usize = I16_FORMATTED_SIZE_DECIMAL;
    pub(crate) const USIZE_FORMATTED_SIZE: usize = U16_FORMATTED_SIZE;
    pub(crate) const USIZE_FORMATTED_SIZE_DECIMAL: usize = U16_FORMATTED_SIZE_DECIMAL;
} else if #[cfg(target_pointer_width = "32")] {
    pub(crate) const ISIZE_FORMATTED_SIZE: usize = I32_FORMATTED_SIZE;
    pub(crate) const ISIZE_FORMATTED_SIZE_DECIMAL: usize = I32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const USIZE_FORMATTED_SIZE: usize = U32_FORMATTED_SIZE;
    pub(crate) const USIZE_FORMATTED_SIZE_DECIMAL: usize = U32_FORMATTED_SIZE_DECIMAL;
} else if #[cfg(target_pointer_width = "64")] {
    pub(crate) const ISIZE_FORMATTED_SIZE: usize = I64_FORMATTED_SIZE;
    pub(crate) const ISIZE_FORMATTED_SIZE_DECIMAL: usize = I64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const USIZE_FORMATTED_SIZE: usize = U64_FORMATTED_SIZE;
    pub(crate) const USIZE_FORMATTED_SIZE_DECIMAL: usize = U64_FORMATTED_SIZE_DECIMAL;
}}  // cfg_if

/// Maximum number of bytes required to serialize any number to string.
pub const BUFFER_SIZE: usize = F64_FORMATTED_SIZE;
