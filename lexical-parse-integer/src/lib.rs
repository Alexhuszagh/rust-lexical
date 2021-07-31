//! Fast lexical string-to-integer conversion routines.
//! TODO(ahuszagh) Add more documentation here...

#[macro_use]
mod shared;

//pub mod algorithm;
//pub mod compact;
pub mod options;
pub mod parse;
pub mod bare;   // TODO(ahuszagh) Remove

mod api;

// Re-exports
pub use self::api::{FromLexical, FromLexicalWithOptions};
pub use self::options::Options;
