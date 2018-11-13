//! Extended-precision float-type.

// Hide implementation details.
mod convert;
mod float_type;
mod rounding;
mod shift;

// Re-export the extended-precision floating-point type.
pub use self::float_type::FloatType;
