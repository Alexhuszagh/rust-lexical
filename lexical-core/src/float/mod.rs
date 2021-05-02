//! Extended-precision floating-point type.

// Hide implementation details.
mod convert;
mod float;
mod mantissa;
mod mask;
mod rounding;
mod shift;

// Re-export the extended-precision floating-point type.
pub use self::float::*;
pub use self::mantissa::*;
pub use self::rounding::*;

// Re-export internal tools.
pub(crate) use self::convert::*;
pub(crate) use self::mask::*;

cfg_if! {
if #[cfg(feature = "atof")] {
    mod wrapped;
    pub(crate) use self::wrapped::*;
}}  // cfg_if
