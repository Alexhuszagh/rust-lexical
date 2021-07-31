//! Fast lexical string-to-integer conversion routines.
//! TODO(ahuszagh) Add more documentation here...

#[macro_use]
mod shared;

pub mod algorithmv2; // TODO(ahuszagh) Rename to algorithm
pub mod compactv2; // TODO(ahuszagh) Rename to compact
pub mod options;
pub mod parse;

mod api;

// Re-exports
pub use self::api::{FromLexical, FromLexicalWithOptions};
pub use self::options::Options;
