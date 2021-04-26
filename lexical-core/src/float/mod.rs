//! Extended-precision floating-point type.

// Hide implementation details.
pub(crate) mod convert;
pub(crate) mod float;
pub(crate) mod mantissa;
pub(crate) mod rounding;
pub(crate) mod shift;

// Re-export rounding types.
pub(crate) use self::rounding::internal_rounding;

// Re-export the extended-precision floating-point type.
pub use self::float::{ExtendedFloat, ExtendedFloat80, ExtendedFloat160};
pub use self::mantissa::Mantissa;
pub use self::rounding::{FloatRounding};
