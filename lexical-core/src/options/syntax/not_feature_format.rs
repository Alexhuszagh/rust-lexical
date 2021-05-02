//! Implementation of `SyntaxFormat` with the `format` feature disabled.

#![cfg(not(feature = "format"))]

use bitflags::bitflags;

use super::flags;

// SYNTAX FORMAT
// TODO(ahuszagh) IMplement...
#[repr(C)]
#[repr(align(8))]
#[derive(Default)]
pub struct SyntaxFormat {
    __value: u64
}
