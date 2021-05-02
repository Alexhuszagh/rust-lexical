//! Extended-precision floating-point type.

// Hide implementation details.
mod convert;
mod float;
mod mantissa;
mod mask;
mod rounding;
mod shift;
mod wrapped;

// Re-export the extended-precision floating-point type.
pub use self::float::*;
pub use self::mantissa::*;
pub use self::rounding::*;

// Re-export internal tools.
pub(crate) use self::convert::*;
pub(crate) use self::mask::*;
pub(crate) use self::wrapped::*;
