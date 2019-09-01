//! C-compatible result type.

use lib::result::Result as StdResult;
use lexical_core::Error;

/// C-compatible tuple for the partial parsers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tuple<T: Copy, U: Copy> {
    pub x: T,
    pub y: U,
}

// Simplify conversion to and from std's Tuple.
impl<T: Copy, U: Copy> From<(T, U)> for Tuple<T, U> {
    fn from(tup: (T, U)) -> Tuple<T, U> {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl<T: Copy, U: Copy> Into<(T, U)> for Tuple<T, U> {
    fn into(self) -> (T, U) {
        (self.x, self.y)
    }
}

/// Tag for the FFI-compatible result.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum ResultTag {
    Ok = 0,
    Err = 1
}

/// Union for the FFI-compatible result.
#[repr(C)]
#[derive(Copy, Clone)]
union ResultUnion<T: Copy> {
    value: T,
    error: Error,
}

/// C-compatible result type from parsing strings-to-numbers for FFI.
///
/// This is an FFI-safe result type that is returned by the from range
/// APIs, for example, `atou8_range`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Result<T: Copy> {
    tag: ResultTag,
    data: ResultUnion<T>,
}

// Simplify conversion to and from std's Result.
impl<T: Copy> From<StdResult<T, Error>> for Result<T> {
    fn from(res: StdResult<T, Error>) -> Result<T> {
        match res {
            Ok(v)  => {
                let data = ResultUnion { value: v };
                Result { tag: ResultTag::Ok, data }
            },
            Err(e) => {
                let data = ResultUnion { error: e };
                Result { tag: ResultTag::Err, data }
            },
        }
    }
}

impl<T: Copy> Into<StdResult<T, Error>> for Result<T> {
    fn into(self) -> StdResult<T, Error> {
        unsafe {
            match self.tag {
                ResultTag::Ok  => Ok(self.data.value),
                ResultTag::Err => Err(self.data.error),
            }
        }
    }
}
