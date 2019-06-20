// Bound support for older Rust versions.

// Glob imports in use groups are experimental before Rustc 1.25.
cfg_if! {
if #[cfg(has_ops_bound)] {
    pub use lib::ops::Bound;
    pub use lib::ops::Bound::*;
} else {
    pub use lib::collections::Bound;
    pub use lib::collections::Bound::*;
}}  // cfg_if
