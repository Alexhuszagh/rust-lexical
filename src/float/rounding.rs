//! Defines rounding schemes for floating-point numbers.

// GENERIC
// -------

// ROUND NEAREST TIE EVEN

/// Shift right N-bytes and round to the nearest.
///
/// Return whether we are above or are halfway.
///
/// * `self`        - Floating point to shift bits in.
/// * `mask`        - Mask to extract all bits beyond `64 - mantissa size - 1` bits.
/// * `mid`         - Midway point for Mask mask, `mask/2 + 1`.
/// * `shift`       - Number of bits to shift.
#[allow(unused_macros)]
macro_rules! round_nearest {
    ($self:ident, $mask:ident, $mid:ident, $shift:ident) => ({
        // Extract the truncated bits using $mask.
        // Calculate if the value of the truncated bits are either above
        // the mid-way point, or equal to it.
        //
        // For example, for 4 truncated bytes, the mask would be b1111
        // and the midway point would be b1000.
        let truncated_bits = $self.frac & $mask;
        let is_above = truncated_bits > $mid;
        let is_halfway = truncated_bits == $mid;

        // Bit shift so the leading bit is in the hidden bit.
        shr!($self, $shift);

        (is_above, is_halfway)
    });
}

/// Shift right N-bytes and round nearest, tie-to-even.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
///
/// * `self`        - Floating point to shift bits in.
/// * `mask`        - Mask to extract all bits beyond `64 - mantissa size - 1` bits.
/// * `mid`         - Midway point for Mask mask, `mask/2 + 1`.
/// * `shift`       - Number of bits to shift.
#[allow(unused_macros)]
macro_rules! round_nearest_tie_even {
    ($self:ident, $mask:ident, $mid:ident, $shift:ident) => ({
        let (is_above, is_halfway) = round_nearest!($self, $mask, $mid, $shift);

        // Extract the last bit after shifting (and determine if it is odd).
        let is_odd = $self.frac & 0x1 == 0x1;

        // Calculate if we need to roundup.
        // We need to roundup if we are above halfway, or if we are odd
        // and at half-way (need to tie-to-even).
        let is_roundup = is_above || (is_odd && is_halfway);

        // Roundup as needed.
        $self.frac += is_roundup as u64;
    });
}

/// Shift right N-bytes and round nearest, tie-to-even.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
///
/// * `self`        - Floating point to shift bits in.
/// * `mask`        - Mask to extract all bits beyond `64 - mantissa size - 1` bits.
/// * `mid`         - Midway point for Mask mask, `mask/2 + 1`.
/// * `shift`       - Number of bits to shift.
#[allow(unused_macros)]
macro_rules! round_nearest_tie_away_zero {
    ($self:ident, $mask:ident, $mid:ident, $shift:ident) => ({
        let (is_above, is_halfway) = round_nearest!($self, $mask, $mid, $shift);

        // Calculate if we need to roundup.
        // We need to roundup if we are halfway or above halfway,
        // since the value is always positive and we need to round away
        // from zero.
        let is_roundup = is_above || is_halfway;

        // Roundup as needed.
        $self.frac += is_roundup as u64;
    });
}

// NATIVE FLOAT
// ------------

// ROUND TO FLOAT

/// Shift the FloatType fraction to the fraction bits in a native float.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
///
/// * `self`        - Floating point to shift bits in.
/// * `mask`        - Mask to extract all bits beyond `64 - mantissa size - 1` bits.
/// * `mid`         - Midway point for mask, `mask/2 + 1`.
/// * `shift`       - Number of bits to shift, or `64 - mantissa size - 1`.
/// * `carry_mask`  - Mask to extract the bit after the hidden bit, or `HIDDEN_BIT_MASK * 2`.
macro_rules! round_to_float {
    ($self:ident, $mask:ident, $mid:ident, $shift:ident, $carry_mask:ident) => ({
        // Roundup and then shift if a full carry occurs.
        round_nearest_tie_even!($self, $mask, $mid, $shift);
        if $self.frac & $carry_mask == $carry_mask {
            // Roundup carried over to 1 past the hidden bit.
            shr!($self, 1);
        }
    })
}

// AVOID OVERFLOW/UNDERFLOW

/// Avoid underflow for denormalized values.
///
/// Shift if the shift results in a non-zero mantissa and an exponent
/// >= denormal exponent.
macro_rules! avoid_underflow {
    ($self:ident, $denormal:ident) => ({
        // Calculate the difference to allow a single calculation
        // rather than a loop, to minimize the number of ops required.
        if $self.exp < $denormal {
            let diff = $denormal - $self.exp;
            if $self.frac >> diff != 0 {
                $self.frac >>= diff;
                $self.exp += diff;
            }
        }
    })
}

/// Avoid overflow for large values, shift left as needed.
///
/// Shift until a 1-bit is in the hidden bit, if the mantissa is not 0.
macro_rules! avoid_overflow {
    ($self:ident, $max:ident, $masks:ident) => ({
        // Calculate the difference to allow a single calculation
        // rather than a loop, using a precalculated bitmask table,
        // minimizing the number of ops required.
        if $self.exp >= $max {
            let diff = $self.exp - $max;
            let idx = diff as usize;
            if idx < $masks.len() {
                let mask = unsafe { *$masks.get_unchecked(idx) };
                if $self.frac & mask == 0 {
                    // If we have no 1-bit in the hidden-bit position,
                    // which is index 0, we need to shift 1.
                    let shift = diff + 1;
                    shl!($self, shift);
                }
            }
        }
    })
}

// ROUND TO NATIVE

/// Round a FloatType to an f32 representation.
macro_rules! round_to_f32 {
    ($self:ident, $denormal:ident, $max:ident, $masks:ident) => ({
        // Shift all the way left, to ensure a consistent representation.
        // The following right-shifts do not work for a non-normalized number.
        $self.normalize();

        // Round so the fraction is in a native mantissa representation.
        const TRUNC_MASK: u64 = 0xFFFFFFFFFF;
        const TRUNC_MID: u64 = 0x8000000000;
        const SHIFT: i32 = 40;
        const CARRY_MASK: u64 = 0x1000000;
        round_to_float!($self, TRUNC_MASK, TRUNC_MID, SHIFT, CARRY_MASK);

        // Avoid overflow/underflow
        avoid_underflow!($self, $denormal);
        avoid_overflow!($self, $max, $masks)
    })
}

/// Round a FloatType to an f64 representation.
macro_rules! round_to_f64 {
    ($self:ident, $denormal:ident, $max:ident, $masks:ident) => ({
        // Shift all the way left, to ensure we have a valid start point.
        // The following right-shifts do not work for a non-normalized number.
        $self.normalize();

        // Round so the fraction is in a native mantissa representation.
        const TRUNC_MASK: u64 = 0x7FF;
        const TRUNC_MID: u64 = 0x400;
        const SHIFT: i32 = 11;
        const CARRY_MASK: u64 = 0x20000000000000;
        round_to_float!($self, TRUNC_MASK, TRUNC_MID, SHIFT, CARRY_MASK);

        // Avoid overflow/underflow
        avoid_underflow!($self, $denormal);
        avoid_overflow!($self, $max, $masks)
    })
}
