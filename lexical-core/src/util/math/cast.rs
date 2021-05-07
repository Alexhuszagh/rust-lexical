//! Casts between arbitrary-precision arithmetic scalar types.

use crate::util::config::*;
use crate::util::traits::*;

// CASTS
// -----

/// Cast to limb type.
#[inline]
pub(crate) fn as_limb<T: Integer>(t: T) -> Limb {
    as_cast(t)
}

/// Cast to wide type.
#[inline]
pub(super) fn as_wide<T: Integer>(t: T) -> Wide {
    as_cast(t)
}

/// Cast tosigned wide type.
#[inline]
pub(super) fn as_signed_wide<T: Integer>(t: T) -> SignedWide {
    as_cast(t)
}
