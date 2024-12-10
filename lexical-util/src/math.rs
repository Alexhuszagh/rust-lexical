//! Arithmetic utilities.
//!
//! This is used for logic to create larger type sizes, allowing
//! multiplication and more from smaller components, while also
//! making testing easier (so the data can be tested from smaller
//! components to known reference values).

#![doc(hidden)]
#![allow(unused_variables)] // TODO: Remove

macro_rules! unsigned_impl {
    (
        // The unsigned type for the low and high bits.
        $u:ty,
        // The signed type for specific conversions.
        $s:ty,add =>
        $add:ident,sub =>
        $sub:ident,mul =>
        $mul:ident,div =>
        $div:ident,rem =>
        $rem:ident,shl =>
        $shl:ident,shr =>
        $shr:ident,rotate_left =>
        $rotate_left:ident,rotate_right =>
        $rotate_right:ident
    ) => {
        /// Const implementation of `Add` for internal algorithm use.
        ///
        /// Returns the value and if the add overflowed.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $add(x0: $u, x1: $u, y0: $u, y1: $u) -> ($u, $u, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            let (v0, c0) = x0.overflowing_add(y0);
            let (v1, c1) = x1.overflowing_add(y1);
            let (v1, c2) = v1.overflowing_add(c0 as $u);
            (v0, v1, c1 || c2)
        }

        /// Const implementation of `Sub` for internal algorithm use.
        ///
        /// Returns the value and if the sub underflowed.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $sub(x0: $u, x1: $u, y0: $u, y1: $u) -> ($u, $u, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            let (v0, c0) = x0.overflowing_sub(y0);
            let (v1, c1) = x1.overflowing_sub(y1);
            let (v1, c2) = v1.overflowing_sub(c0 as $u);
            (v0, v1, c1 || c2)
        }

        /// Const implementation of `Sub` for internal algorithm use.
        ///
        /// Returns the value and the carry.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $mul(x0: $u, x1: $u, y0: $u, y1: $u) -> ($u, $u, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Div` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $div(x0: $u, x1: $u, y0: $u, y1: $u) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Rem` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $rem(x0: $u, x1: $u, y0: $u, y1: $u) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Shl` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `shift` - The number of bits to shift.
        #[inline(always)]
        pub const fn $shl(x0: $u, x1: $u, shift: u32) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            const BITS: u32 = <$u>::BITS as u32;
            debug_assert!(shift < 2 * BITS, "attempt to shift left with overflow");
            let shift = shift % (2 * BITS);
            if shift >= BITS {
                (0, x0.wrapping_shl(shift - BITS))
            } else if shift == 0 {
                (x0, x1)
            } else {
                // NOTE: We have `0xABCD_EFGH`, and we want to shift by 1,
                // so to `0xBCDE_FGH0`, or we need to carry the `D`. So,
                // our mask needs to be `0x000X`, or `0xXXXX >> (4 - 1)`,
                // and then the value needs to be shifted left `<< (4 - 1)`.
                let hi = x1.wrapping_shl(shift);
                let lo = x0.wrapping_shl(shift);
                let carry = x0.wrapping_shr(BITS - shift);
                (lo, hi + carry)
            }
        }

        /// Const implementation of `Shr` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `shift` - The number of bits to shift.
        #[inline(always)]
        pub const fn $shr(x0: $u, x1: $u, shift: u32) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            const BITS: u32 = <$u>::BITS as u32;
            debug_assert!(shift < 2 * BITS, "attempt to shift right with overflow");
            let shift = shift % (2 * BITS);
            if shift >= BITS {
                (x1.wrapping_shr(shift - BITS), 0)
            } else if shift == 0 {
                (x0, x1)
            } else {
                // NOTE: We have `0xABCD_EFGH`, and we want to shift by 1,
                // so to `0x0ABC_DEFG`, or we need to carry the `D`. So,
                // our mask needs to be `0x000X`, or `0xXXXX >> (4 - 1)`,
                // and then the value needs to be shifted left `<< (4 - 1)`.
                let hi = x1.wrapping_shr(shift);
                let lo = x0.wrapping_shr(shift);
                let carry = x1.wrapping_shl(BITS - shift);
                (lo + carry, hi)
            }
        }

        /// Shifts the bits to the left by a specified amount, `n`,
        /// wrapping the truncated bits to the end of the resulting integer.
        ///
        /// Please note this isn't the same operation as the `<<` shifting operator!
        #[inline(always)]
        pub const fn $rotate_left(x0:$u, x1: $u, n: u32) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            // 0bXYFFFF -> 0bFFFFXY
            const BITS: u32 = <$u>::BITS * 2;
            let n = n % BITS;
            // TODO: should just be able to rotate the bits and overflow...
            todo!();
        }

        /// Shifts the bits to the right by a specified amount, `n`,
        /// wrapping the truncated bits to the beginning of the resulting
        /// integer.
        ///
        /// Please note this isn't the same operation as the `>>` shifting operator!
        #[inline(always)]
        pub const fn $rotate_right(x0:$u, x1: $u, n: u32) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            // 0bFFFFXY -> 0bXYFFFF
            const BITS: u32 = <$u>::BITS * 2;
            let n = n % BITS;
            // TODO(is this right?)
            //let mask = Self::MAX.shr_u32(n);
            todo!();
        }
    };
}

// Widening and narrowing conversions for primitive types.
macro_rules! unsigned_primitive_cast {
    (
        // The unsigned type for the low bits.
        $u:ty,
        // The signed type for the high bits.
        $s:ty,as_uwide =>
        $as_uwide:ident,as_unarrow =>
        $as_unarrow:ident,as_iwide =>
        $as_iwide:ident,as_inarrow =>
        $as_inarrow:ident,wide_cast =>
        $wide_cast:ident
    ) => {
        /// Convert an unsigned, narrow type to the wide type.
        #[inline(always)]
        pub const fn $as_uwide(x:$u) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            (x, 0)
        }

        /// Convert a signed, narrow type to the wide type.
        ///
        /// This is the same logic, and codegen as `is_wide`
        /// for signed types, just we keep it as an unsigned type
        /// for `hi`.
        #[inline(always)]
        pub const fn $as_iwide(x:$s) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            let hi = <$u>::MIN.wrapping_sub(x.is_negative() as $u);
            (x as $u, hi)
        }

        /// Convert the wide value to a narrow, unsigned type.
        #[inline(always)]
        pub const fn $as_unarrow(x0:$u, x1: $u) -> $u {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            x0
        }

        /// Convert the wide value to a narrow, signed type.
        #[inline(always)]
        pub const fn $as_inarrow(x0:$u, x1: $u) -> $s {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            x0 as $s
        }

        /// Do a wide cast from unsigned to signed.
        #[inline(always)]
        pub const fn $wide_cast(x0:$u, x1: $u) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            (x0, x1 as $s)
        }
    };
}

unsigned_impl!(
    u8,
    i8,
    add => add_u8,
    sub => sub_u8,
    mul => mul_u8,
    div => div_u8,
    rem => rem_u8,
    shl => shl_u8,
    shr => shr_u8,
    rotate_left => rotate_left_u8,
    rotate_right => rotate_right_u8
);
unsigned_impl!(
    u16,
    i16,
    add => add_u16,
    sub => sub_u16,
    mul => mul_u16,
    div => div_u16,
    rem => rem_u16,
    shl => shl_u16,
    shr => shr_u16,
    rotate_left => rotate_left_u16,
    rotate_right => rotate_right_u16
);
unsigned_impl!(
    u32,
    i32,
    add => add_u32,
    sub => sub_u32,
    mul => mul_u32,
    div => div_u32,
    rem => rem_u32,
    shl => shl_u32,
    shr => shr_u32,
    rotate_left => rotate_left_u32,
    rotate_right => rotate_right_u32
);
unsigned_impl!(
    u64,
    i64,
    add => add_u64,
    sub => sub_u64,
    mul => mul_u64,
    div => div_u64,
    rem => rem_u64,
    shl => shl_u64,
    shr => shr_u64,
    rotate_left => rotate_left_u64,
    rotate_right => rotate_right_u64
);
unsigned_impl!(
    u128,
    i128,
    add => add_u128,
    sub => sub_u128,
    mul => mul_u128,
    div => div_u128,
    rem => rem_u128,
    shl => shl_u128,
    shr => shr_u128,
    rotate_left => rotate_left_u128,
    rotate_right => rotate_right_u128
);
unsigned_impl!(
    usize,
    isize,
    add => add_usize,
    sub => sub_usize,
    mul => mul_usize,
    div => div_usize,
    rem => rem_usize,
    shl => shl_usize,
    shr => shr_usize,
    rotate_left => rotate_left_usize,
    rotate_right => rotate_right_usize
);
unsigned_primitive_cast!(
    u8,
    i8,
    as_uwide => as_uwide_u8,
    as_unarrow => as_unarrow_u8,
    as_iwide => as_iwide_u8,
    as_inarrow => as_inarrow_u8,
    wide_cast => wide_cast_u8
);
unsigned_primitive_cast!(
    u16,
    i16,
    as_uwide => as_uwide_u16,
    as_unarrow => as_unarrow_u16,
    as_iwide => as_iwide_u16,
    as_inarrow => as_inarrow_u16,
    wide_cast => wide_cast_u16
);
unsigned_primitive_cast!(
    u32,
    i32,
    as_uwide => as_uwide_u32,
    as_unarrow => as_unarrow_u32,
    as_iwide => as_iwide_u32,
    as_inarrow => as_inarrow_u32,
    wide_cast => wide_cast_u32
);
unsigned_primitive_cast!(
    u64,
    i64,
    as_uwide => as_uwide_u64,
    as_unarrow => as_unarrow_u64,
    as_iwide => as_iwide_u64,
    as_inarrow => as_inarrow_u64,
    wide_cast => wide_cast_u64
);
unsigned_primitive_cast!(
    u128,
    i128,
    as_uwide => as_uwide_u128,
    as_unarrow => as_unarrow_u128,
    as_iwide => as_iwide_u128,
    as_inarrow => as_inarrow_u128,
    wide_cast => wide_cast_u128
);
unsigned_primitive_cast!(
    usize,
    isize,
    as_uwide => as_uwide_usize,
    as_unarrow => as_unarrow_usize,
    as_iwide => as_iwide_usize,
    as_inarrow => as_inarrow_usize,
    wide_cast => wide_cast_usize
);

macro_rules! signed_impl {
    (
        // The unsigned type for the low bits.
        $u:ty,
        // The signed type for the high bits.
        $s:ty,add =>
        $add:ident,sub =>
        $sub:ident,mul =>
        $mul:ident,div =>
        $div:ident,rem =>
        $rem:ident,shl =>
        $shl:ident,shr =>
        $shr:ident,swap_bytes =>
        $swap_bytes:ident,reverse_bits =>
        $reverse_bits:ident,rotate_left =>
        $rotate_left:ident,rotate_right =>
        $rotate_right:ident
    ) => {
        /// Const implementation of `Add` for internal algorithm use.
        ///
        /// Returns the value and if the add overflowed.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $add(x0: $u, x1: $s, y0: $u, y1: $s) -> ($u, $s, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Sub` for internal algorithm use.
        ///
        /// Returns the value and if the sub underflowed.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $sub(x0: $u, x1: $s, y0: $u, y1: $s) -> ($u, $s, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Sub` for internal algorithm use.
        ///
        /// Returns the value and the carry.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $mul(x0: $u, x1: $s, y0: $u, y1: $s) -> ($u, $s, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Div` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $div(x0: $u, x1: $s, y0: $u, y1: $s) -> ($u, $s, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Rem` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `y0` - The lower half of y.
        /// * `y1` - The upper half of y.
        #[inline(always)]
        pub const fn $rem(x0: $u, x1: $s, y0: $u, y1: $s) -> ($u, $s, bool) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Shl` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `shift` - The number of bits to shift.
        #[inline(always)]
        pub const fn $shl(x0: $u, x1: $s, shift: u32) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Const implementation of `Shr` for internal algorithm use.
        ///
        /// * `x0` - The lower half of x.
        /// * `x1` - The upper half of x.
        /// * `shift` - The number of bits to shift.
        #[inline(always)]
        pub const fn $shr(x0: $u, x1: $s, shift: u32) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Reverses the byte order of the integer.
        #[inline(always)]
        pub const fn $swap_bytes(x0: $u, x1: $s) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Reverses the order of bits in the integer. The least significant
        /// bit becomes the most significant bit, second least-significant bit
        /// becomes second most-significant bit, etc.
        #[inline(always)]
        pub const fn $reverse_bits(x0: $u, x1: $s) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            todo!();
        }

        /// Shifts the bits to the left by a specified amount, `n`,
        /// wrapping the truncated bits to the end of the resulting integer.
        ///
        /// Please note this isn't the same operation as the `<<` shifting operator!
        #[inline(always)]
        pub const fn $rotate_left(x0:$u, x1: $s, n: u32) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            // 0bXYFFFF -> 0bFFFFXY
            const BITS: u32 = <$u>::BITS * 2;
            let n = n % BITS;
            // TODO: should just be able to rotate the bits and overflow...
            todo!();
        }

        /// Shifts the bits to the right by a specified amount, `n`,
        /// wrapping the truncated bits to the beginning of the resulting
        /// integer.
        ///
        /// Please note this isn't the same operation as the `>>` shifting operator!
        #[inline(always)]
        pub const fn $rotate_right(x0:$u, x1: $s, n: u32) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            // 0bFFFFXY -> 0bXYFFFF
            const BITS: u32 = <$u>::BITS * 2;
            let n = n % BITS;
            todo!();
        }
    };
}

// Widening and narrowing conversions for primitive types.
macro_rules! signed_primitive_cast {
    (
        // The unsigned type for the low bits.
        $u:ty,
        // The signed type for the high bits.
        $s:ty,as_uwide =>
        $as_uwide:ident,as_unarrow =>
        $as_unarrow:ident,as_iwide =>
        $as_iwide:ident,as_inarrow =>
        $as_inarrow:ident,wide_cast =>
        $wide_cast:ident
    ) => {
        // NOTE: This code was all test with the same algorithms in C++,
        // compiling for both little and big endian to ensure the logic
        // is the same, just as a precaution. For example:
        //
        // ```cpp
        // #include <cstdint>
        // #include <limits>
        //
        // int32_t as_inarrow_hard(int64_t v) {
        //     return (int32_t)v;
        // }
        //
        // int32_t as_inarrow_soft(int64_t v) {
        //     uint64_t mask = (uint64_t)std::numeric_limits<uint32_t>::max();
        //     uint64_t lo = ((uint64_t)v) & mask;
        //     return (int32_t)lo;
        // }
        // ```

        /// Convert an unsigned, narrow type to the wide type.
        ///
        /// This is the same as:
        ///
        /// ```rust
        /// #[inline(never)]
        /// pub const fn as_uwide(v: u32) -> u64 {
        ///     // hi bits will always be 0
        ///     v as u64
        /// }
        /// ```
        #[inline(always)]
        pub const fn $as_uwide(x:$u) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            (x, 0)
        }

        /// Convert a signed, narrow type to the wide type.
        ///
        /// This is the same as:
        ///
        /// ```rust
        /// #[inline(never)]
        /// pub const fn as_iwide_hard(v: i32) -> i64 {
        ///     v as i64
        /// }
        ///
        /// #[inline(never)]
        /// pub const fn as_iwide_soft(x: i32) -> i64 {
        ///     let hi = u32::MIN.wrapping_sub(x.is_negative() as u32) as u64;
        ///     let hi = hi << 32;
        ///     let lo = (x as u32) as u64;
        ///     let x = lo | hi;
        ///     return x as i64;
        /// }
        /// ```
        ///
        /// This is analogous to the following C++ code:
        ///
        /// ```cpp
        /// int64_t as_iwide_hard(int32_t v) {
        ///     return v;
        /// }
        ///
        /// int64_t as_iwide_soft(int32_t v) {
        ///     bool is_negative = v < 0;
        ///     uint64_t hi = uint64_t(0) - uint64_t(is_negative);
        ///     hi <<= 32;
        ///     uint64_t lo = (uint64_t)((uint32_t)v);
        ///     uint64_t x = lo | hi;
        ///     return (int64_t)x;
        /// }
        /// ```
        ///
        /// This is way more efficient than using naive approaches, like checking `< 0` which brings
        /// in a `test` instruction.
        #[inline(always)]
        pub const fn $as_iwide(x:$s) -> ($u, $s) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            // NOTE: This optimizes somewhat poorly for primitive types but it's not **TOO BAD**.
            // On x86_64, the output is as follows:
            // as_iwide_hard
            //     movsxd  rax, edi
            //     ret
            //
            // as_iwide_soft
            //     mov     eax, edi
            //     sar     edi, 31
            //     shl     rdi, 32
            //     or      rax, rdi
            //     ret
            let hi = <$u>::MIN.wrapping_sub(x.is_negative() as $u);
            (x as $u, hi as $s)
        }

        /// Convert the wide value to a narrow, unsigned type.
        ///
        /// This is the same as:
        ///
        /// ```rust
        /// #[inline(never)]
        /// pub const fn as_unarrow_hard(v: i64) -> u32 {
        ///     v as u32
        /// }
        ///
        /// #[inline(never)]
        /// pub const fn as_unarrow_soft(v: i64) -> u32 {
        ///     const MASK: u64 = u32::MAX as u64;
        ///     let lo = (v as u64) & MASK;
        ///     lo as u32
        /// }
        /// ```
        #[inline(always)]
        pub const fn $as_unarrow(x0:$u, x1: $s) -> $u {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            x0 as $u
        }

        /// Convert the wide value to a narrow, signed type.
        ///
        /// This is the same as:
        ///
        /// ```rust
        /// #[inline(never)]
        /// pub const fn as_inarrow_hard(v: i64) -> i32 {
        ///     v as i32
        /// }
        ///
        /// #[inline(never)]
        /// pub const fn as_inarrow_soft(v: i64) -> i32 {
        ///     const MASK: u64 = u32::MAX as u64;
        ///     let lo = (v as u64) & MASK;
        ///     (lo as u32) as i32
        /// }
        /// ```
        #[inline(always)]
        pub const fn $as_inarrow(x0:$u, x1: $s) -> $s {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            x0 as $s
        }

        /// Do a wide cast from signed to unsigned.
        #[inline(always)]
        pub const fn $wide_cast(x0:$u, x1: $s) -> ($u, $u) {
            debug_assert!(<$u>::BITS == <$s>::BITS);
            (x0, x1 as $u)
        }
    };
}

signed_impl!(
    u8,
    i8,
    add => add_i8,
    sub => sub_i8,
    mul => mul_i8,
    div =>div_i8,
    rem => rem_i8,
    shl => shl_i8,
    shr => shr_i8,
    swap_bytes => swap_bytes_i8,
    reverse_bits => reverse_bits_i8,
    rotate_left => rotate_left_i8,
    rotate_right => rotate_right_i8
);
signed_impl!(
    u16,
    i16,
    add => add_i16,
    sub => sub_i16,
    mul => mul_i16,
    div =>div_i16,
    rem => rem_i16,
    shl => shl_i16,
    shr => shr_i16,
    swap_bytes => swap_bytes_i16,
    reverse_bits => reverse_bits_i16,
    rotate_left => rotate_left_i16,
    rotate_right => rotate_right_i16
);
signed_impl!(
    u32,
    i32,
    add => add_i32,
    sub => sub_i32,
    mul => mul_i32,
    div =>div_i32,
    rem => rem_i32,
    shl => shl_i32,
    shr => shr_i32,
    swap_bytes => swap_bytes_i32,
    reverse_bits => reverse_bits_i32,
    rotate_left => rotate_left_i32,
    rotate_right => rotate_right_i32
);
signed_impl!(
    u64,
    i64,
    add => add_i64,
    sub => sub_i64,
    mul => mul_i64,
    div =>div_i64,
    rem => rem_i64,
    shl => shl_i64,
    shr => shr_i64,
    swap_bytes => swap_bytes_i64,
    reverse_bits => reverse_bits_i64,
    rotate_left => rotate_left_i64,
    rotate_right => rotate_right_i64
);
signed_impl!(
    u128,
    i128,
    add => add_i128,
    sub => sub_i128,
    mul => mul_i128,
    div =>div_i128,
    rem => rem_i128,
    shl => shl_i128,
    shr => shr_i128,
    swap_bytes => swap_bytes_i128,
    reverse_bits => reverse_bits_i128,
    rotate_left => rotate_left_i128,
    rotate_right => rotate_right_i128
);
signed_impl!(
    usize,
    isize,
    add => add_isize,
    sub => sub_isize,
    mul => mul_isize,
    div =>div_isize,
    rem => rem_isize,
    shl => shl_isize,
    shr => shr_isize,
    swap_bytes => swap_bytes_isize,
    reverse_bits => reverse_bits_isize,
    rotate_left => rotate_left_isize,
    rotate_right => rotate_right_isize
);
signed_primitive_cast!(
    u8,
    i8,
    as_uwide => as_uwide_i8,
    as_unarrow => as_unarrow_i8,
    as_iwide => as_iwide_i8,
    as_inarrow => as_inarrow_i8,
    wide_cast => wide_cast_i8
);
signed_primitive_cast!(
    u16,
    i16,
    as_uwide => as_uwide_i16,
    as_unarrow => as_unarrow_i16,
    as_iwide => as_iwide_i16,
    as_inarrow => as_inarrow_i16,
    wide_cast => wide_cast_i16
);
signed_primitive_cast!(
    u32,
    i32,
    as_uwide => as_uwide_i32,
    as_unarrow => as_unarrow_i32,
    as_iwide => as_iwide_i32,
    as_inarrow => as_inarrow_i32,
    wide_cast => wide_cast_i32
);
signed_primitive_cast!(
    u64,
    i64,
    as_uwide => as_uwide_i64,
    as_unarrow => as_unarrow_i64,
    as_iwide => as_iwide_i64,
    as_inarrow => as_inarrow_i64,
    wide_cast => wide_cast_i64
);
signed_primitive_cast!(
    u128,
    i128,
    as_uwide => as_uwide_i128,
    as_unarrow => as_unarrow_i128,
    as_iwide => as_iwide_i128,
    as_inarrow => as_inarrow_i128,
    wide_cast => wide_cast_i128
);
signed_primitive_cast!(
    usize,
    isize,
    as_uwide => as_uwide_isize,
    as_unarrow => as_unarrow_isize,
    as_iwide => as_iwide_isize,
    as_inarrow => as_inarrow_isize,
    wide_cast => wide_cast_isize
);

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;
    use super::*;

    const LO32: u64 = u32::MAX as u64;

    #[test]
    fn add_u32_test() {
        assert_eq!(add_u32(1, 0, 1, 0), (2, 0, false));
        assert_eq!(add_u32(u32::MAX, 0, u32::MAX, 0), (u32::MAX - 1, 1, false));
        assert_eq!(add_u32(u32::MAX, 1, u32::MAX, 0), (u32::MAX - 1, 2, false));
        assert_eq!(add_u32(u32::MAX, u32::MAX, 1, 0), (0, 0, true));
        assert_eq!(add_u32(u32::MAX, u32::MAX, 2, 0), (1, 0, true));
        assert_eq!(add_u32(u32::MAX, u32::MAX, u32::MAX, u32::MAX), (u32::MAX - 1, u32::MAX, true));
    }

    #[test]
    fn sub_u32_test() {
        assert_eq!(sub_u32(0, 0, 1, 0), (u32::MAX, u32::MAX, true));
        assert_eq!(sub_u32(1, 0, 1, 0), (0, 0, false));
        assert_eq!(sub_u32(1, 0, 0, 0), (1, 0, false));
        assert_eq!(sub_u32(u32::MAX, 1, 0, 2), (u32::MAX, u32::MAX, true));
        assert_eq!(sub_u32(0, 1, 0, 2), (0, 4294967295, true));
    }

    #[test]
    fn mul_u32_test() {
        //assert_eq!(mul_u32(u32::MAX, u32::MAX, u32::MAX, u32::MAX), (1, 0, true));
        assert_eq!(mul_u32(1, 0, u32::MAX, 1), (u32::MAX, 1, false));
        assert_eq!(mul_u32(2, 0, 2147483648, 0), (u32::MAX, 1, false));
        //assert_eq!(mul_u32(1, 0, 1, 0), (0, 0, false));
        //assert_eq!(mul_u32(1, 0, 0, 0), (1, 0, false));
        //assert_eq!(mul_u32(u32::MAX, 1, 0, 2), (u32::MAX, u32::MAX, true));
        //assert_eq!(mul_u32(0, 1, 0, 2), (0, 4294967295, true));
    }

    // TODO: Div, rem

    #[test]
    fn shl_u32_test() {
        assert_eq!(shl_u32(1, 0, 1), (2, 0));
        assert_eq!(shl_u32(0, 1, 0), (0, 1));
        assert_eq!(shl_u32(0, 1, 1), (0, 2));
        assert_eq!(shl_u32(1, 0, 32), (0, 1));
        assert_eq!(shl_u32(0, 1, 32), (0, 0));
        assert_eq!(shl_u32(2, 0, 31), (0, 1));
        assert_eq!(shl_u32(0, 2, 31), (0, 0));
        assert_eq!(shl_u32(1, 2, 31), (2147483648, 0));
    }

    #[test]
    fn shr_u32_test() {
        assert_eq!(shr_u32(1, 0, 1), (0, 0));
        assert_eq!(shr_u32(0, 1, 0), (0, 1));
        assert_eq!(shr_u32(0, 1, 1), (2147483648, 0));
        assert_eq!(shr_u32(1, 0, 32), (0, 0));
        assert_eq!(shr_u32(0, 1, 32), (1, 0));
        assert_eq!(shr_u32(2, 0, 31), (0, 0));
        assert_eq!(shr_u32(0, 2, 31), (4, 0));
        assert_eq!(shr_u32(1, 2, 31), (4, 0));
    }

    quickcheck! {
        fn add_u32_quickcheck(x: u64, y: u64) -> bool {
            let x0 = (x & LO32) as u32;
            let x1 = (x >> 32) as u32;
            let y0 = (y & LO32) as u32;
            let y1 = (y >> 32) as u32;
            let (lo, hi, overflowed) = add_u32(x0, x1, y0, y1);
            let expected = x.overflowing_add(y);
            let actual = lo as u64 + ((hi as u64) << 32);
            expected == (actual, overflowed)
        }

        fn sub_u32_quickcheck(x: u64, y: u64) -> bool {
            let x0 = (x & LO32) as u32;
            let x1 = (x >> 32) as u32;
            let y0 = (y & LO32) as u32;
            let y1 = (y >> 32) as u32;
            let (lo, hi, overflowed) = sub_u32(x0, x1, y0, y1);
            let expected = x.overflowing_sub(y);
            let actual = lo as u64 + ((hi as u64) << 32);
            expected == (actual, overflowed)
        }

        fn mul_u32_quickcheck(x: u64, y: u64) -> bool {
            let x0 = (x & LO32) as u32;
            let x1 = (x >> 32) as u32;
            let y0 = (y & LO32) as u32;
            let y1 = (y >> 32) as u32;
            let (lo, hi, overflowed) = mul_u32(x0, x1, y0, y1);
            println!("lo {lo} hi {hi} overflowed {overflowed}");
            let expected = x.overflowing_mul(y);
            let actual = lo as u64 + ((hi as u64) << 32);
            expected == (actual, overflowed)
        }

        // TODO: Div, rem

        fn shl_u32_quickcheck(x: u64, n: u32) -> bool {
            let x0 = (x & LO32) as u32;
            let x1 = (x >> 32) as u32;
            let n = (n % 64) as u32;
            let expected = x << n;
            let (lo, hi) = shl_u32(x0, x1, n);
            let actual = lo as u64 + ((hi as u64) << 32);
            expected == actual
        }

        fn shr_u32_quickcheck(x: u64, n: u32) -> bool {
            let x0 = (x & LO32) as u32;
            let x1 = (x >> 32) as u32;
            let n = (n % 64) as u32;
            let expected = x >> n;
            let (lo, hi) = shr_u32(x0, x1, n);
            let actual = lo as u64 + ((hi as u64) << 32);
            expected == actual
        }

        fn as_uwide_u32_quickcheck(x: u32) -> bool {
            let expected = x as u64;
            let (lo, hi) = as_uwide_u32(x);
            let hi = hi as u64;
            let actual = (hi << 32) + lo as u64;
            expected == actual
        }

        fn as_iwide_u32_quickcheck(x: i32) -> bool {
            let expected = x as u64;
            let (lo, hi) = as_iwide_u32(x);
            let hi = hi as u64;
            let actual = (hi << 32) + lo as u64;
            expected == actual
        }

        fn as_unarrow_u32_quickcheck(x: u64) -> bool {
            let lo = x as u32;
            let hi = (x >> 32) as u32;
            let expected = x as u32;
            let actual = as_unarrow_u32(lo, hi);
            expected == actual && x as u16 == actual as u16
        }

        fn as_inarrow_u32_quickcheck(x: u64) -> bool {
            let lo = x as u32;
            let hi = (x >> 32) as u32;
            let expected = x as i32;
            let actual = as_inarrow_u32(lo, hi);
            expected == actual && x as i16 == actual as i16
        }

        fn wide_cast_u32_quickcheck(x: u64) -> bool {
            let lo = x as u32;
            let hi = (x >> 32) as u32;
            let expected = (x as u32, hi as i32);
            let actual = wide_cast_u32(lo, hi);
            expected == actual
        }

        fn as_uwide_i32_quickcheck(x: u32) -> bool {
            let expected = x as i64;
            let (lo, hi) = as_uwide_i32(x);
            let hi = (hi as u32) as u64;
            let unsigned = (hi << 32) + lo as u64;
            let actual = unsigned as i64;
            expected == actual
        }

        fn as_iwide_i32_quickcheck(x: i32) -> bool {
            let expected = x as i64;
            let (lo, hi) = as_iwide_i32(x);
            let hi = (hi as u32) as u64;
            let unsigned = (hi << 32) + lo as u64;
            let actual = unsigned as i64;
            expected == actual
        }

        fn as_unarrow_i32_quickcheck(x: u64) -> bool {
            let lo = x as u32;
            let hi = (x >> 32) as i32;
            let expected = x as u32;
            let actual = as_unarrow_i32(lo, hi);
            expected == actual && x as u16 == actual as u16
        }

        fn as_inarrow_i32_quickcheck(x: i64) -> bool {
            let lo = x as u32;
            let hi = (x >> 32) as i32;
            let expected = x as i32;
            let actual = as_inarrow_i32(lo, hi);
            expected == actual && x as i16 == actual as i16
        }

        fn wide_cast_i32_quickcheck(x: i64) -> bool {
            let lo = x as u32;
            let hi = (x >> 32) as i32;
            let expected = (x as u32, hi as u32);
            let actual = wide_cast_i32(lo, hi);
            expected == actual
        }
    }
}
