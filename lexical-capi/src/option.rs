//! C-compatible Option type.

use crate::lib::option::Option as StdOption;

/// Tag for the FFI-compatible result.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum OptionTag {
    Some = 0,
    None = 1,
}

/// C-compatible Option type for the builder API.
///
/// This is an FFI-safe option type that may be returned when failing
/// to compile a `NumberFormat` or parse or write options.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Option<T: Copy + Default> {
    tag: OptionTag,
    value: T,
}

// Simplify conversion to and from std's Option.
impl<T: Copy + Default> From<StdOption<T>> for Option<T> {
    #[inline(always)]
    fn from(opt: StdOption<T>) -> Option<T> {
        match opt {
            Some(value) => Option {
                tag: OptionTag::Some,
                value,
            },
            None => {
                let value = T::default();
                Option {
                    tag: OptionTag::None,
                    value,
                }
            },
        }
    }
}

impl<T: Copy + Default> Into<StdOption<T>> for Option<T> {
    #[inline(always)]
    fn into(self) -> StdOption<T> {
        match self.tag {
            OptionTag::Some => Some(self.value),
            OptionTag::None => None,
        }
    }
}

impl<T: Copy + Default> Default for Option<T> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            tag: OptionTag::None,
            value: T::default(),
        }
    }
}
