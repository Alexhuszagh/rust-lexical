//! Configuration for the numerical syntax.

#[macro_use]
mod flags;

cfg_if! {
if #[cfg(feature = "format")] {
    mod feature_format;
    pub use self::feature_format::*;
} else {
    mod not_feature_format;
    pub use self::not_feature_format::*;
}} // cfg_if
