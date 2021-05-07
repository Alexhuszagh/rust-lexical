//! Extended-precision floating-point types and wrappers.

// Hide implementation details.
mod convert;
mod extended_float;
mod rounding;
mod shift;
mod wrapped;

// Re-export the extended-precision floating-point type.
pub use self::extended_float::*;

// Re-export internal tools.
pub(crate) use self::convert::*;
pub(crate) use self::rounding::*;
pub(crate) use self::wrapped::*;    // TODO(Ahuszagh) Rename?
