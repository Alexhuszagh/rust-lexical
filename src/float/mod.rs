//! Extended-precision float-type.

// Hide implementation details.
#[macro_use]
mod shift;

#[macro_use]
mod rounding;

#[macro_use]
mod convert;

mod float_type;

// Re-export the extended-precision floating-point type.
pub use self::float_type::FloatType;
