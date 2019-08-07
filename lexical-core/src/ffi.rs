//! Foreign-function interface declarations.
//!
//! Re-imports FFI declarations nested in other modules for documentation
//! purposes.

// Re-export the config constants, mutable globals, and functions.
pub use config_ffi::*;

// Re-export the result and error declarations.
pub use error_ffi::*;
pub use result_ffi::*;

// Re-export the low-level parsers and formatters.
pub use atof_ffi::*;
pub use atoi_ffi::*;
pub use ftoa_ffi::*;
pub use itoa_ffi::*;
