//! Arithmetic utilities.
//!
//! This is used for logic to create larger type sizes, allowing
//! multiplication and more from smaller components, while also
//! making testing easier (so the data can be tested from smaller
//! components to known reference values).

// TODO: Feature-gate this

mod div;
mod native;
pub use self::native::*;
pub use self::div::{div_rem_big, div_rem_small};
