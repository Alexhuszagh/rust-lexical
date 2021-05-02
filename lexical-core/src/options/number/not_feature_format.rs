//! Configuration for the syntax and valid characters of a number.

use super::super::lexer::*;
use super::super::syntax::*;

// TODO(ahuszagh) Need to have the builder here.
// The builders above shouldn't be used, for obvious reasons.

// NUMBER FORMAT

// TODO(ahuszagh) Rename to NumberFormat
#[repr(C)]
#[repr(align(16))]
pub struct NumberFormatV2 {
    syntax: SyntaxFormat,
    lexer: LexerFormat,
}

// NUMBER FORMAT BUILDER

// TODO(ahuszagh) Rename to NumberFormatBuilder
pub struct NumberFormatBuilderV2 {

}
