// RangeBounds support for older Rust versions.

use lib::ops::{Range, RangeFrom, RangeTo, RangeFull};
use super::bound::*;

#[cfg(has_full_range_inclusive)]
use lib::ops::{RangeInclusive, RangeToInclusive};

//  The compiler does not have RangeBounds defined, this is code
//  adopted from Rust:
//      https://doc.rust-lang.org/src/core/ops/range.rs.html
//
//  All copyright for the remainder of this file remains with the
//  Rust contributors:
//      https://github.com/rust-lang/rust/blob/master/COPYRIGHT

#[cfg(has_range_bounds)]
pub use lib::ops::RangeBounds;

#[cfg(not(has_range_bounds))]
pub trait RangeBounds<T: ?Sized> {
    /// Start index bound.
    fn start_bound(&self) -> Bound<&T>;

    /// End index bound.
    fn end_bound(&self) -> Bound<&T>;

    /// Detect if item is in range.
    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        (match self.start_bound() {
            Included(ref start) => *start <= item,
            Excluded(ref start) => *start < item,
            Unbounded => true,
        }) && (match self.end_bound() {
            Included(ref end) => item <= *end,
            Excluded(ref end) => item < *end,
            Unbounded => true,
        })
    }
}

#[cfg(not(has_range_bounds))]
impl<T: ?Sized> RangeBounds<T> for RangeFull {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

#[cfg(not(has_range_bounds))]
impl<T> RangeBounds<T> for RangeFrom<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Unbounded
    }
}

#[cfg(not(has_range_bounds))]
impl<T> RangeBounds<T> for RangeTo<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

#[cfg(not(has_range_bounds))]
impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Excluded(&self.end)
    }
}

#[cfg(all(not(has_range_bounds), has_full_range_inclusive))]
impl<T> RangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(self.start())
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(self.end())
    }
}

#[cfg(all(not(has_range_bounds), has_full_range_inclusive))]
impl<T> RangeBounds<T> for RangeToInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Unbounded
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}
