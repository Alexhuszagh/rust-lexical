//! Configuration for the syntax and valid characters of a number.

mod validate;

cfg_if! {
    if #[cfg(feature = "format")] {
        mod feature_format;
        pub use self::feature_format::*;
    } else {
        mod not_feature_format;
        pub use self::not_feature_format::*;
    }
}
