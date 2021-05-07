//! Traits to extract high bits from arbitrary-precision types.

use crate::util::traits::*;

// HI BITS
// -------

// NONZERO

/// Check if any of the remaining bits are non-zero.
#[inline]
fn nonzero<T: Integer>(x: &[T], rindex: usize) -> bool {
    let len = x.len();
    let slc = &x[..len - rindex];
    slc.iter().rev().any(|&x| x != T::ZERO)
}

// HI16

/// Shift 16-bit integer to high 16-bits.
#[inline]
fn u16_to_hi16_1(r0: u16) -> (u16, bool) {
    debug_assert!(r0 != 0);
    let ls = r0.leading_zeros();
    (r0 << ls, false)
}

/// Shift 2 16-bit integers to high 16-bits.
#[inline]
fn u16_to_hi16_2(r0: u16, r1: u16) -> (u16, bool) {
    debug_assert!(r0 != 0);
    let ls = r0.leading_zeros();
    let rs = 16 - ls;
    let v = match ls {
        0 => r0,
        _ => (r0 << ls) | (r1 >> rs),
    };
    let n = r1 << ls != 0;
    (v, n)
}

/// Shift 32-bit integer to high 16-bits.
#[inline]
fn u32_to_hi16_1(r0: u32) -> (u16, bool) {
    let r0 = u32_to_hi32_1(r0).0;
    ((r0 >> 16).as_u16(), r0.as_u16() != 0)
}

/// Shift 2 32-bit integers to high 16-bits.
#[inline]
fn u32_to_hi16_2(r0: u32, r1: u32) -> (u16, bool) {
    let (r0, n) = u32_to_hi32_2(r0, r1);
    ((r0 >> 16).as_u16(), n || r0.as_u16() != 0)
}

/// Shift 64-bit integer to high 16-bits.
#[inline]
fn u64_to_hi16_1(r0: u64) -> (u16, bool) {
    let r0 = u64_to_hi64_1(r0).0;
    ((r0 >> 48).as_u16(), r0.as_u16() != 0)
}

/// Shift 2 64-bit integers to high 16-bits.
#[inline]
fn u64_to_hi16_2(r0: u64, r1: u64) -> (u16, bool) {
    let (r0, n) = u64_to_hi64_2(r0, r1);
    ((r0 >> 48).as_u16(), n || r0.as_u16() != 0)
}

/// Trait to export the high 16-bits from a little-endian slice.
pub(super) trait Hi16<T>: SliceLike<T> {
    /// Get the hi16 bits from a 1-limb slice.
    fn hi16_1(&self) -> (u16, bool);

    /// Get the hi16 bits from a 2-limb slice.
    fn hi16_2(&self) -> (u16, bool);

    /// High-level exporter to extract the high 16 bits from a little-endian slice.
    #[inline]
    fn hi16(&self) -> (u16, bool) {
        match self.len() {
            0 => (0, false),
            1 => self.hi16_1(),
            _ => self.hi16_2(),
        }
    }
}

impl Hi16<u16> for [u16] {
    #[inline]
    fn hi16_1(&self) -> (u16, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0];
        u16_to_hi16_1(r0)
    }

    #[inline]
    fn hi16_2(&self) -> (u16, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0];
        let r1 = rview[1];
        let (v, n) = u16_to_hi16_2(r0, r1);
        (v, n || nonzero(self, 2))
    }
}

impl Hi16<u32> for [u32] {
    #[inline]
    fn hi16_1(&self) -> (u16, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0];
        u32_to_hi16_1(r0)
    }

    #[inline]
    fn hi16_2(&self) -> (u16, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0];
        let r1 = rview[1];
        let (v, n) = u32_to_hi16_2(r0, r1);
        (v, n || nonzero(self, 2))
    }
}

impl Hi16<u64> for [u64] {
    #[inline]
    fn hi16_1(&self) -> (u16, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0];
        u64_to_hi16_1(r0)
    }

    #[inline]
    fn hi16_2(&self) -> (u16, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0];
        let r1 = rview[1];
        let (v, n) = u64_to_hi16_2(r0, r1);
        (v, n || nonzero(self, 2))
    }
}

// HI32

/// Shift 32-bit integer to high 32-bits.
#[inline]
fn u32_to_hi32_1(r0: u32) -> (u32, bool) {
    debug_assert!(r0 != 0);
    let ls = r0.leading_zeros();
    (r0 << ls, false)
}

/// Shift 2 32-bit integers to high 32-bits.
#[inline]
fn u32_to_hi32_2(r0: u32, r1: u32) -> (u32, bool) {
    debug_assert!(r0 != 0);
    let ls = r0.leading_zeros();
    let rs = 32 - ls;
    let v = match ls {
        0 => r0,
        _ => (r0 << ls) | (r1 >> rs),
    };
    let n = r1 << ls != 0;
    (v, n)
}

/// Shift 64-bit integer to high 32-bits.
#[inline]
fn u64_to_hi32_1(r0: u64) -> (u32, bool) {
    let r0 = u64_to_hi64_1(r0).0;
    ((r0 >> 32).as_u32(), r0.as_u32() != 0)
}

/// Shift 2 64-bit integers to high 32-bits.
#[inline]
fn u64_to_hi32_2(r0: u64, r1: u64) -> (u32, bool) {
    let (r0, n) = u64_to_hi64_2(r0, r1);
    ((r0 >> 32).as_u32(), n || r0.as_u32() != 0)
}

/// Trait to export the high 32-bits from a little-endian slice.
pub(super) trait Hi32<T>: SliceLike<T> {
    /// Get the hi32 bits from a 1-limb slice.
    fn hi32_1(&self) -> (u32, bool);

    /// Get the hi32 bits from a 2-limb slice.
    fn hi32_2(&self) -> (u32, bool);

    /// Get the hi32 bits from a 3-limb slice.
    fn hi32_3(&self) -> (u32, bool);

    /// High-level exporter to extract the high 32 bits from a little-endian slice.
    #[inline]
    fn hi32(&self) -> (u32, bool) {
        match self.len() {
            0 => (0, false),
            1 => self.hi32_1(),
            2 => self.hi32_2(),
            _ => self.hi32_3(),
        }
    }
}

impl Hi32<u16> for [u16] {
    #[inline]
    fn hi32_1(&self) -> (u32, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        u32_to_hi32_1(rview[0].as_u32())
    }

    #[inline]
    fn hi32_2(&self) -> (u32, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0].as_u32() << 16;
        let r1 = rview[1].as_u32();
        u32_to_hi32_1(r0 | r1)
    }

    #[inline]
    fn hi32_3(&self) -> (u32, bool) {
        debug_assert!(self.len() >= 3);
        let rview = self.rview();
        let r0 = rview[0].as_u32();
        let r1 = rview[1].as_u32() << 16;
        let r2 = rview[2].as_u32();
        let (v, n) = u32_to_hi32_2(r0, r1 | r2);
        (v, n || nonzero(self, 3))
    }
}

impl Hi32<u32> for [u32] {
    #[inline]
    fn hi32_1(&self) -> (u32, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0];
        u32_to_hi32_1(r0)
    }

    #[inline]
    fn hi32_2(&self) -> (u32, bool) {
        debug_assert!(self.len() >= 2);
        let rview = self.rview();
        let r0 = rview[0];
        let r1 = rview[1];
        let (v, n) = u32_to_hi32_2(r0, r1);
        (v, n || nonzero(self, 2))
    }

    #[inline]
    fn hi32_3(&self) -> (u32, bool) {
        self.hi32_2()
    }
}

impl Hi32<u64> for [u64] {
    #[inline]
    fn hi32_1(&self) -> (u32, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0];
        u64_to_hi32_1(r0)
    }

    #[inline]
    fn hi32_2(&self) -> (u32, bool) {
        debug_assert!(self.len() >= 2);
        let rview = self.rview();
        let r0 = rview[0];
        let r1 = rview[1];
        let (v, n) = u64_to_hi32_2(r0, r1);
        (v, n || nonzero(self, 2))
    }

    #[inline]
    fn hi32_3(&self) -> (u32, bool) {
        self.hi32_2()
    }
}

// HI64

/// Shift 64-bit integer to high 64-bits.
#[inline]
fn u64_to_hi64_1(r0: u64) -> (u64, bool) {
    debug_assert!(r0 != 0);
    let ls = r0.leading_zeros();
    (r0 << ls, false)
}

/// Shift 2 64-bit integers to high 64-bits.
#[inline]
fn u64_to_hi64_2(r0: u64, r1: u64) -> (u64, bool) {
    debug_assert!(r0 != 0);
    let ls = r0.leading_zeros();
    let rs = 64 - ls;
    let v = match ls {
        0 => r0,
        _ => (r0 << ls) | (r1 >> rs),
    };
    let n = r1 << ls != 0;
    (v, n)
}

/// Trait to export the high 64-bits from a little-endian slice.
pub(crate) trait Hi64<T>: SliceLike<T> {
    /// Get the hi64 bits from a 1-limb slice.
    fn hi64_1(&self) -> (u64, bool);

    /// Get the hi64 bits from a 2-limb slice.
    fn hi64_2(&self) -> (u64, bool);

    /// Get the hi64 bits from a 3-limb slice.
    fn hi64_3(&self) -> (u64, bool);

    /// Get the hi64 bits from a 4-limb slice.
    fn hi64_4(&self) -> (u64, bool);

    /// Get the hi64 bits from a 5-limb slice.
    fn hi64_5(&self) -> (u64, bool);

    /// High-level exporter to extract the high 64 bits from a little-endian slice.
    #[inline]
    fn hi64(&self) -> (u64, bool) {
        match self.len() {
            0 => (0, false),
            1 => self.hi64_1(),
            2 => self.hi64_2(),
            3 => self.hi64_3(),
            4 => self.hi64_4(),
            _ => self.hi64_5(),
        }
    }
}

impl Hi64<u16> for [u16] {
    #[inline]
    fn hi64_1(&self) -> (u64, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0].as_u64();
        u64_to_hi64_1(r0)
    }

    #[inline]
    fn hi64_2(&self) -> (u64, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0].as_u64() << 16;
        let r1 = rview[1].as_u64();
        u64_to_hi64_1(r0 | r1)
    }

    #[inline]
    fn hi64_3(&self) -> (u64, bool) {
        debug_assert!(self.len() == 3);
        let rview = self.rview();
        let r0 = rview[0].as_u64() << 32;
        let r1 = rview[1].as_u64() << 16;
        let r2 = rview[2].as_u64();
        u64_to_hi64_1(r0 | r1 | r2)
    }

    #[inline]
    fn hi64_4(&self) -> (u64, bool) {
        debug_assert!(self.len() == 4);
        let rview = self.rview();
        let r0 = rview[0].as_u64() << 48;
        let r1 = rview[1].as_u64() << 32;
        let r2 = rview[2].as_u64() << 16;
        let r3 = rview[3].as_u64();
        u64_to_hi64_1(r0 | r1 | r2 | r3)
    }

    #[inline]
    fn hi64_5(&self) -> (u64, bool) {
        debug_assert!(self.len() >= 5);
        let rview = self.rview();
        let r0 = rview[0].as_u64();
        let r1 = rview[1].as_u64() << 48;
        let r2 = rview[2].as_u64() << 32;
        let r3 = rview[3].as_u64() << 16;
        let r4 = rview[4].as_u64();
        let (v, n) = u64_to_hi64_2(r0, r1 | r2 | r3 | r4);
        (v, n || nonzero(self, 5))
    }
}

impl Hi64<u32> for [u32] {
    #[inline]
    fn hi64_1(&self) -> (u64, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0].as_u64();
        u64_to_hi64_1(r0)
    }

    #[inline]
    fn hi64_2(&self) -> (u64, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0].as_u64() << 32;
        let r1 = rview[1].as_u64();
        u64_to_hi64_1(r0 | r1)
    }

    #[inline]
    fn hi64_3(&self) -> (u64, bool) {
        debug_assert!(self.len() >= 3);
        let rview = self.rview();
        let r0 = rview[0].as_u64();
        let r1 = rview[1].as_u64() << 32;
        let r2 = rview[2].as_u64();
        let (v, n) = u64_to_hi64_2(r0, r1 | r2);
        (v, n || nonzero(self, 3))
    }

    #[inline]
    fn hi64_4(&self) -> (u64, bool) {
        self.hi64_3()
    }

    #[inline]
    fn hi64_5(&self) -> (u64, bool) {
        self.hi64_3()
    }
}

impl Hi64<u64> for [u64] {
    #[inline]
    fn hi64_1(&self) -> (u64, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0];
        u64_to_hi64_1(r0)
    }

    #[inline]
    fn hi64_2(&self) -> (u64, bool) {
        debug_assert!(self.len() >= 2);
        let rview = self.rview();
        let r0 = rview[0];
        let r1 = rview[1];
        let (v, n) = u64_to_hi64_2(r0, r1);
        (v, n || nonzero(self, 2))
    }

    #[inline]
    fn hi64_3(&self) -> (u64, bool) {
        self.hi64_2()
    }

    #[inline]
    fn hi64_4(&self) -> (u64, bool) {
        self.hi64_2()
    }

    #[inline]
    fn hi64_5(&self) -> (u64, bool) {
        self.hi64_2()
    }
}

// HI128

/// Shift 128-bit integer to high 128-bits.
#[inline]
fn u128_to_hi128_1(r0: u128) -> (u128, bool) {
    let ls = r0.leading_zeros();
    (r0 << ls, false)
}

/// Shift 2 128-bit integers to high 128-bits.
#[inline]
fn u128_to_hi128_2(r0: u128, r1: u128) -> (u128, bool) {
    let ls = r0.leading_zeros();
    let rs = 128 - ls;
    let v = (r0 << ls) | (r1 >> rs);
    let n = r1 << ls != 0;
    (v, n)
}

/// Trait to export the high 128-bits from a little-endian slice.
pub(crate) trait Hi128<T>: SliceLike<T> {
    /// Get the hi128 bits from a 1-limb slice.
    fn hi128_1(&self) -> (u128, bool);

    /// Get the hi128 bits from a 2-limb slice.
    fn hi128_2(&self) -> (u128, bool);

    /// Get the hi128 bits from a 3-limb slice.
    fn hi128_3(&self) -> (u128, bool);

    /// Get the hi128 bits from a 4-limb slice.
    fn hi128_4(&self) -> (u128, bool);

    /// Get the hi128 bits from a 5-limb slice.
    fn hi128_5(&self) -> (u128, bool);

    /// Get the hi128 bits from a 5-limb slice.
    fn hi128_6(&self) -> (u128, bool);

    /// Get the hi128 bits from a 5-limb slice.
    fn hi128_7(&self) -> (u128, bool);

    /// Get the hi128 bits from a 5-limb slice.
    fn hi128_8(&self) -> (u128, bool);

    /// Get the hi128 bits from a 5-limb slice.
    fn hi128_9(&self) -> (u128, bool);

    /// High-level exporter to extract the high 128 bits from a little-endian slice.
    #[inline]
    fn hi128(&self) -> (u128, bool) {
        match self.len() {
            0 => (0, false),
            1 => self.hi128_1(),
            2 => self.hi128_2(),
            3 => self.hi128_3(),
            4 => self.hi128_4(),
            6 => self.hi128_6(),
            7 => self.hi128_7(),
            8 => self.hi128_8(),
            _ => self.hi128_9(),
        }
    }
}

impl Hi128<u16> for [u16] {
    #[inline]
    fn hi128_1(&self) -> (u128, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0].as_u128();
        u128_to_hi128_1(r0)
    }

    #[inline]
    fn hi128_2(&self) -> (u128, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 16;
        let r1 = rview[1].as_u128();
        u128_to_hi128_1(r0 | r1)
    }

    #[inline]
    fn hi128_3(&self) -> (u128, bool) {
        debug_assert!(self.len() == 3);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 32;
        let r1 = rview[1].as_u128() << 16;
        let r2 = rview[2].as_u128();
        u128_to_hi128_1(r0 | r1 | r2)
    }

    #[inline]
    fn hi128_4(&self) -> (u128, bool) {
        debug_assert!(self.len() == 4);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 48;
        let r1 = rview[1].as_u128() << 32;
        let r2 = rview[2].as_u128() << 16;
        let r3 = rview[3].as_u128();
        u128_to_hi128_1(r0 | r1 | r2 | r3)
    }

    #[inline]
    fn hi128_5(&self) -> (u128, bool) {
        debug_assert!(self.len() == 5);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 64;
        let r1 = rview[1].as_u128() << 48;
        let r2 = rview[2].as_u128() << 32;
        let r3 = rview[3].as_u128() << 16;
        let r4 = rview[4].as_u128();
        u128_to_hi128_1(r0 | r1 | r2 | r3 | r4)
    }

    #[inline]
    fn hi128_6(&self) -> (u128, bool) {
        debug_assert!(self.len() == 6);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 80;
        let r1 = rview[1].as_u128() << 64;
        let r2 = rview[2].as_u128() << 48;
        let r3 = rview[3].as_u128() << 32;
        let r4 = rview[4].as_u128() << 16;
        let r5 = rview[5].as_u128();
        u128_to_hi128_1(r0 | r1 | r2 | r3 | r4 | r5)
    }

    #[inline]
    fn hi128_7(&self) -> (u128, bool) {
        debug_assert!(self.len() == 7);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 96;
        let r1 = rview[1].as_u128() << 80;
        let r2 = rview[2].as_u128() << 64;
        let r3 = rview[3].as_u128() << 48;
        let r4 = rview[4].as_u128() << 32;
        let r5 = rview[5].as_u128() << 16;
        let r6 = rview[6].as_u128();
        u128_to_hi128_1(r0 | r1 | r2 | r3 | r4 | r5 | r6)
    }

    #[inline]
    fn hi128_8(&self) -> (u128, bool) {
        debug_assert!(self.len() == 8);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 112;
        let r1 = rview[1].as_u128() << 96;
        let r2 = rview[2].as_u128() << 80;
        let r3 = rview[3].as_u128() << 64;
        let r4 = rview[4].as_u128() << 48;
        let r5 = rview[5].as_u128() << 32;
        let r6 = rview[6].as_u128() << 16;
        let r7 = rview[7].as_u128();
        u128_to_hi128_1(r0 | r1 | r2 | r3 | r4 | r5 | r6 | r7)
    }

    #[inline]
    fn hi128_9(&self) -> (u128, bool) {
        debug_assert!(self.len() >= 9);
        let rview = self.rview();
        let r0 = rview[0].as_u128();
        let r1 = rview[1].as_u128() << 112;
        let r2 = rview[2].as_u128() << 96;
        let r3 = rview[3].as_u128() << 80;
        let r4 = rview[4].as_u128() << 64;
        let r5 = rview[5].as_u128() << 48;
        let r6 = rview[6].as_u128() << 32;
        let r7 = rview[7].as_u128() << 16;
        let r8 = rview[8].as_u128();
        let (v, n) = u128_to_hi128_2(r0, r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8);
        (v, n || nonzero(self, 9))
    }
}

impl Hi128<u32> for [u32] {
    #[inline]
    fn hi128_1(&self) -> (u128, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0].as_u128();
        u128_to_hi128_1(r0)
    }

    #[inline]
    fn hi128_2(&self) -> (u128, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 32;
        let r1 = rview[1].as_u128();
        u128_to_hi128_1(r0 | r1)
    }

    #[inline]
    fn hi128_3(&self) -> (u128, bool) {
        debug_assert!(self.len() == 3);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 64;
        let r1 = rview[1].as_u128() << 32;
        let r2 = rview[2].as_u128();
        u128_to_hi128_1(r0 | r1 | r2)
    }

    #[inline]
    fn hi128_4(&self) -> (u128, bool) {
        debug_assert!(self.len() == 4);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 96;
        let r1 = rview[1].as_u128() << 64;
        let r2 = rview[2].as_u128() << 32;
        let r3 = rview[3].as_u128();
        u128_to_hi128_1(r0 | r1 | r2 | r3)
    }

    #[inline]
    fn hi128_5(&self) -> (u128, bool) {
        debug_assert!(self.len() >= 5);
        let rview = self.rview();
        let r0 = rview[0].as_u128();
        let r1 = rview[1].as_u128() << 96;
        let r2 = rview[2].as_u128() << 64;
        let r3 = rview[3].as_u128() << 32;
        let r4 = rview[4].as_u128();
        let (v, n) = u128_to_hi128_2(r0, r1 | r2 | r3 | r4);
        (v, n || nonzero(self, 5))
    }

    #[inline]
    fn hi128_6(&self) -> (u128, bool) {
        self.hi128_5()
    }

    #[inline]
    fn hi128_7(&self) -> (u128, bool) {
        self.hi128_5()
    }

    #[inline]
    fn hi128_8(&self) -> (u128, bool) {
        self.hi128_5()
    }

    #[inline]
    fn hi128_9(&self) -> (u128, bool) {
        self.hi128_5()
    }
}

impl Hi128<u64> for [u64] {
    #[inline]
    fn hi128_1(&self) -> (u128, bool) {
        debug_assert!(self.len() == 1);
        let rview = self.rview();
        let r0 = rview[0].as_u128();
        u128_to_hi128_1(r0)
    }

    #[inline]
    fn hi128_2(&self) -> (u128, bool) {
        debug_assert!(self.len() == 2);
        let rview = self.rview();
        let r0 = rview[0].as_u128() << 64;
        let r1 = rview[1].as_u128();
        u128_to_hi128_1(r0 | r1)
    }

    #[inline]
    fn hi128_3(&self) -> (u128, bool) {
        debug_assert!(self.len() >= 3);
        let rview = self.rview();
        let r0 = rview[0].as_u128();
        let r1 = rview[1].as_u128() << 64;
        let r2 = rview[2].as_u128();
        let (v, n) = u128_to_hi128_2(r0, r1 | r2);
        (v, n || nonzero(self, 3))
    }

    #[inline]
    fn hi128_4(&self) -> (u128, bool) {
        self.hi128_3()
    }

    #[inline]
    fn hi128_5(&self) -> (u128, bool) {
        self.hi128_3()
    }

    #[inline]
    fn hi128_6(&self) -> (u128, bool) {
        self.hi128_3()
    }

    #[inline]
    fn hi128_7(&self) -> (u128, bool) {
        self.hi128_3()
    }

    #[inline]
    fn hi128_8(&self) -> (u128, bool) {
        self.hi128_3()
    }

    #[inline]
    fn hi128_9(&self) -> (u128, bool) {
        self.hi128_3()
    }
}
