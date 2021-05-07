//! Fast estimation of the accurate representation of a float.
//!
//! Based off the Golang implementation of the Eisel-Lemire algorithm,
//! found here:
//!     https://github.com/golang/go/blob/2ebe77a2fda1ee9ff6fd9a3e08933ad1ebaea039/src/strconv/eisel_lemire.go
//!
//! Which, itself was based off of the Wuff's implementation:
//!     https://github.com/google/wuffs/blob/ba3818cb6b473a2ed0b38ecfc07dbbd3a97e8ae7/internal/cgen/base/floatconv-submodule-code.c
//!
//! The original algorithm may be found here:
//!     https://github.com/lemire/fast_double_parser
//!
//! And an in-depth blogpost describing the algorithms may be found here:
//!     https://nigeltao.github.io/blog/2020/eisel-lemire.html
//!
//! # Magic Number Generation
//!
//! ```python
//! import math
//!
//! def get_range(max_exp, bitshift):
//!     den = 1 << bitshift
//!     num = int(math.ceil(math.log2(10) * den))
//!     for exp10 in range(0, max_exp):
//!         exp2_exact = int(math.log2(10**exp10))
//!         exp2_guess = num * exp10 // den
//!         if exp2_exact != exp2_guess:
//!             raise ValueError(f'{exp10}')
//!     return num, den
//! ```
//!
//! For 64-bit and smaller floats, we therefore need a bitshift of 16,
//! so our magic number is `217706`. For 128-bit floats, we need a bitshift
//! of >= 25, so we round up to 32, and therefore need a magic number
//! of `14267572528`. Note that due to the storage requirements,
//! 128-bit floats do not currently use this algorithm.

#![cfg(feature = "lemire")]

use crate::util::*;

use super::alias::*;
use super::extended_float;

// MUL
// ---

/// Multiply two unsigned, integral values, and return the hi and lo product.
#[inline(always)]
pub(crate) fn mul<M: Mantissa>(x: M, y: M) -> (M, M) {
    // Extract high-and-low masks.
    let x1 = x >> M::HALF;
    let x0 = x & M::LOMASK;
    let y1 = y >> M::HALF;
    let y0 = y & M::LOMASK;

    // Get our products
    let w0 = x0 * y0;
    let tmp = (x1 * y0) + (w0 >> M::HALF);
    let w1 = tmp & M::LOMASK;
    let w2 = tmp >> M::HALF;
    let w1 = w1 + x0 * y1;
    let hi = (x1 * y1) + w2 + (w1 >> M::HALF);
    let lo = x.wrapping_mul(y);

    (hi, lo)
}

// ROUNDING
// --------

/// Round-nearest. Handles both round-nearest tie-even and tie-away-zero.
#[inline(always)]
fn round_nearest<M>(mantissa: M) -> M
where
    M: UnsignedInteger,
{
    mantissa + (mantissa & M::ONE)
}

/// Round-downward. Always a no-op.
#[inline(always)]
fn round_downward<M>(mantissa: M) -> M
where
    M: UnsignedInteger,
{
    mantissa
}

/// Round upward if the value is above the current representation.
#[inline(always)]
fn round_upward<M>(mut mantissa: M, is_truncated: bool) -> M
where
    M: UnsignedInteger,
{
    if is_truncated || mantissa & M::ONE != M::ZERO {
        mantissa += M::ONE;
    }
    mantissa
}

// SHIFT
// -----

/// Shift significant digits to at most the carry bit
/// The carry bit is 1 above the hidden bit, in the exponent,
/// or mantissa size + 2.
#[inline(always)]
fn shift_to_carry<M: Mantissa>(x_hi: M, exp2: i32, carry_shift: i32) -> (M, i32, bool) {
    // Carry out the shift
    let msb_shift = M::FULL - 1;
    let msb = x_hi >> msb_shift;
    let shift = msb.as_i32() + carry_shift;
    let mantissa = x_hi >> shift;
    let exp2 = exp2 - (1i32 ^ msb.as_i32());

    // See if we truncated any digits, for round-upwards.
    let is_truncated: bool;
    if cfg!(feature = "rounding") {
        let truncated_mask = (M::ONE << shift) - M::ONE;
        is_truncated = x_hi & truncated_mask != M::ZERO;
    } else {
        is_truncated = false;
    }

    (mantissa, exp2, is_truncated)
}

// TO FLOAT
// --------

/// Convert mantissa and binary exponent to floating-point representation.
///
/// This function expects the following things:
///     1). The highest mantissa bit set is 1 above the carry bit.
///     2). The lowest mantissa bit set is the carry bit.
///         That is, 2 above the hidden bit, or 1 above the hidden bit.
///     3). The binary exponent is adjusted for the exponent bias.
#[inline(always)]
fn to_float<F, M>(mantissa: M, exp: i32, is_truncated: bool, kind: RoundingKind) -> (F, bool)
where
    M: MantissaType,
    F: FloatType,
{
    // Check denormal values for underflow.
    if exp <= -(F::MANTISSA_SIZE + 2) {
        // Have a literal zero. If we shift the bits, we'll get 0.
        return (F::ZERO, true);
    } else if exp <= 0 {
        // We don't actually care about the accuracy here,
        // since we're going straight to the extended-float algorithm.
        return (F::ZERO, false);
        //// Have a denormal float, try to get `b` from the value. Need it
        //// to be rounded-down. We've shifted to 1 above the carry bit, IE,
        //// the 2 above the normal point point we'd be. Our shift therefore
        //// needs to be exp + 2.
        //let shift = exp.wrapping_neg() + 2;
        //let mut mantissa: F::Unsigned = as_cast(mantissa);
        //mantissa >>= shift;
        //// No exponent bits.
        //return (F::from_bits(mantissa), false);
    }

    // Get our raw bits.
    let mut exp: F::Unsigned = as_cast(exp);
    let mut mantissa: F::Unsigned = as_cast(mantissa);

    // Handle rounding.
    if kind == RoundingKind::NearestTieEven {
        mantissa = round_nearest(mantissa);
    } else if cfg!(feature = "rounding") && kind == RoundingKind::NearestTieAwayZero {
        mantissa = round_nearest(mantissa);
    } else if cfg!(feature = "rounding") && kind == RoundingKind::Upward {
        // This uses the check if any digits were truncated from x_hi
        // when shifting to the mantissa. Since we check with `mantissa+1`,
        // we don't care about any digits truncated from the original
        // mantissa during parsing.
        mantissa = round_upward(mantissa, is_truncated);
    } else {
        mantissa = round_downward(mantissa);
    }

    // Shift them into position.
    mantissa >>= 1;
    let zero = F::Unsigned::ZERO;
    let one = F::Unsigned::ONE;
    let precision = F::MANTISSA_SIZE + 1;
    if mantissa >> precision > zero {
        mantissa >>= 1;
        exp += one;
    }

    // Check our mantissa representation is valid, that is,
    // we didn't have a bit mantissa or hidden bit set.
    let mask = F::MANTISSA_MASK | F::HIDDEN_BIT_MASK;
    debug_assert!(mantissa & mask == mantissa);

    // Check for overflow, if so, return a literal infinity.
    let max_exp = F::MAX_EXPONENT + F::EXPONENT_BIAS;
    if exp >= as_cast(max_exp) {
        return (F::INFINITY, true);
    }

    // Should fail, we shouldn't have any exponent bits set.
    mantissa &= F::MANTISSA_MASK;
    exp <<= F::MANTISSA_SIZE;
    let bits = exp | mantissa;

    (F::from_bits(bits), true)
}

// EISEL-LEMIRE
// ------------

/// Create a precise native float using the Eisel-Lemire algorithm.
///
/// NOTE: If the Eisel-Lamire algorithm cannot differentiate a halfway
/// representation, it cannot determine whether to round up or down
/// to determine the correct `b` value for big-float determination.
///
/// In that case, we fall back to extended-float to determine that
/// representation.
#[inline]
pub(super) fn eisel_lemire<F, M>(
    mantissa: M,
    radix: u32,
    exponent: i32,
    kind: RoundingKind,
) -> (F, bool)
where
    M: MantissaType,
    F: FloatType,
{
    debug_assert!(radix == 10, "Radix must be 10,");
    debug_assert!(M::BITS <= 64, "Cannot support 128-bit floats due to cached storage.");

    // Check if the value is outside of our max range:
    //  If the value is above our max range, we have to have an infinity,
    //  and we have an exact representation (1e348) is infinity, which
    //  is the minimum possible value above this range.
    //
    // For negative values, we're guaranteed to have 0 as well:
    //  with 2470328229206232720e-342, we round to 0, while with
    //  2470328229206232721e-342, we round up to 5e-324. Both of these
    //  contain the maximum number of mantissa digits (19), so our
    //  base-10 exponent cannot get any lower.
    //
    // Note that this only checks beyond the limits of f64, we do
    // checks for narrower types further in.
    if exponent < MIN_DENORMAL_EXP10 {
        return (F::ZERO, true);
    } else if exponent > MAX_NORMAL_EXP10 {
        return (F::INFINITY, true);
    }

    // Normalize the mantissa, and calculate the bias.
    let ctlz = mantissa.leading_zeros() as i32;
    let mantissa = mantissa << ctlz;
    let bias = F::EXPONENT_BIAS - F::MANTISSA_SIZE;

    // Need to convert our base 10 exponent to base 2, as an estimate.
    let unbiased_exp2 = (M::LOG2_10 * exponent as i64) >> M::LOG2_10_SHIFT;
    let exp2 = unbiased_exp2 as i32 + (M::FULL + bias) - ctlz;

    // Now need to get our extended, power of 10:
    let (exp10_hi, exp10_lo) = POWERS_OF_10[(exponent - MIN_DENORMAL_EXP10) as usize];
    let exp10_hi: M = as_cast(exp10_hi);
    let exp10_lo: M = as_cast(exp10_lo);
    let (mut x_hi, mut x_lo) = mul(mantissa, exp10_hi);

    // NOTE:
    //  From here we make a few differences from the Lemire implementation,
    //  to streamline integration with the slow path algorithm.
    //
    //  192-BIT
    //  -------
    //
    //  When we check for halfway representations, for the extended
    //  192-bit representation, we assume the following logic:
    //  - If we have `x_hi & mask == mask` and wrapping behavior,
    //      then we are close to a halfway representation, but 1-bit below.
    //  - If `merged_hi & mask == mask` and `merged_lo + 1 == 0`, then
    //      we are within 1-bit of the halfway representation.
    //  In this case, we should add 1-bit, to get to the halfway
    //  representation, and round-down, so we can get our `b` representation
    //  to differentiate `b` from `b+u` near to `b+h`.
    //
    //  AFTER-SHIFTING
    //  --------------
    //
    //  After shifting and checking for truncated bits, we have shifted
    //  to the carry bit + 1. This means we are 2 bits above the hidden
    //  bit, so we have a halfway representation if `mantissa & 3 == 1`,
    //  and the truncated bits were 0 (`x_lo == 0` and `x_hi & mask == 0`).
    //  Here, since we're at least at a halfway representation, round-down
    //  so we get `b`. We're already **at least** at a halfway representation,
    //  so we should not add any bits to the shifted mantissa.

    // Now need to check for a wider approximation.
    let carry_size = F::MANTISSA_SIZE + 2;
    let carry_shift = M::FULL - carry_size - 1;
    let mask = (M::ONE << carry_shift) - M::ONE;
    if x_hi & mask == mask && x_lo.wrapping_add(mantissa) < mantissa {
        let (y_hi, y_lo) = mul(mantissa, exp10_lo);
        let mut merged_hi = x_hi;
        let merged_lo = x_lo.wrapping_add(y_hi);
        if merged_lo < x_lo {
            merged_hi += M::ONE;
        }

        // Check for a halfway representation.
        if merged_hi & mask == mask
            && merged_lo.wrapping_add(M::ONE) == M::ZERO
            && y_lo.wrapping_add(mantissa) < mantissa
        {
            // We don't actually care about the accuracy here,
            // since we're going straight to the extended-float algorithm.
            return (F::ZERO, false);
            //// Halfway. Use x_hi since it's guaranteed to be within
            //// 1 bit of our desired representation, set below.
            //// Since we're rounding down, to get the base representation,
            //// we might be 1 bit too low. This is because if we have
            //// multiplication issues, then we're 1 bit too low (which
            //// is why we're at the wider approximation: checking for overflow).
            //let (mantissa, exp2, is_truncated) = shift_to_carry(x_hi + M::ONE, exp2, carry_shift);
            //let float = to_float(mantissa, exp2, is_truncated, RoundingKind::Downward).0;
            //return (float, false);
        } else {
            x_hi = merged_hi;
            x_lo = merged_lo;
        }
    }

    // Shift to the carry bit (IE, mantissa size + 2).
    let (mantissa, exp2, is_truncated) = shift_to_carry(x_hi, exp2, carry_shift);

    // Check for a halfway representation.
    let three: M = as_cast(3);
    if x_lo == M::ZERO && x_hi & mask == M::ZERO && mantissa & three == M::ONE {
        // We don't actually care about the accuracy here,
        // since we're going straight to the extended-float algorithm.
        return (F::ZERO, false);
        //// Halfway, invalid representation. Return the representation rounded-down.
        //// We can round down safely here, since we're exactly at a halfway representation.
        //let float = to_float(mantissa, exp2, false, RoundingKind::Downward).0;
        //return (float, false);
    }

    to_float(mantissa, exp2, is_truncated, kind)
}

/// Create a precise native float using the Eisel-Lemire algorithm.
///
/// Note that the Eisel-Lemire algorithm may not be accurate if
/// truncated digits occur, so we do a second pass with the
/// mantissa + 1 (to solve any halfway issues with truncated
/// digits), and if the two values are the same, return true.
/// This avoids any costly error estimation, since if `mantissa`
/// `mantissa+1` are the same, we cannot have had a halfway case.
///
/// Note that if we cannot determine a valid representation,
/// we fall back to the extended-float moderate path, so we can
/// get an accurate, base representation for big-integer
/// algorithms.
#[inline]
pub(super) fn moderate_path<F>(
    mantissa: F::MantissaType,
    radix: u32,
    exponent: i32,
    is_truncated: bool,
    is_lossy: bool,
    kind: RoundingKind,
) -> (F, bool)
where
    F: FloatType,
{
    let (float, valid) = eisel_lemire::<F, _>(mantissa, radix, exponent, kind);
    if valid {
        if !is_truncated || is_lossy {
            (float, true)
        } else {
            let mantissa_up = mantissa + F::MantissaType::ONE;
            let (float_up, valid) = eisel_lemire::<F, _>(mantissa_up, radix, exponent, kind);
            if valid && float == float_up {
                (float, true)
            } else {
                (float, false)
            }
        }
    } else {
        // If the first representation failed, try the extended-float
        // algorithm, since it's a lot faster for small, denormal floats.
        extended_float::moderate_path(mantissa, radix, exponent, is_truncated, is_lossy, kind)
    }
}

// Power of 10 lookup table generate via this Go program:
//      https://github.com/google/wuffs/blob/ba3818cb6b473a2ed0b38ecfc07dbbd3a97e8ae7/script/print-mpb-powers-of-10.go
//
// The values are store in (hi, lo) order, and while the original
// implementation stores them in (lo, hi) order. We can infer the
// exponent since the values are printed relative to the bias.
// We do not store values below -327 or above 308, so we can
// encompass the entire range of normal values.
//  `u64::MAX * 1e-343` is 0.0.
//  `u64::MAX * 1e-327` is less than the smallest normal, positive value.
//  `1e309` is larger than the maximum, positive, non-special value.
//
// This algorithm does not handle denormal (subnormal) or special values,
// so we short-circuit if we have a guaranteed, 0 value (if `exp10 < -342`),
// or if we have a guaranteed infinite value (`exp10 > 308`), otherwise,
// we assume we have normal data and return that we do not have a valid
// representation otherwise.
//
// Using a more narrow range means a smaller lookup table, and saves us
// ~1KB in our binary.

const MIN_DENORMAL_EXP10: i32 = -342;
const MAX_NORMAL_EXP10: i32 = 308;
const POWERS_OF_10: [(u64, u64); 651] = [
    (0xEEF453D6923BD65A, 0x113FAA2906A13B3F), // 10^-342
    (0x9558B4661B6565F8, 0x4AC7CA59A424C507), // 10^-341
    (0xBAAEE17FA23EBF76, 0x5D79BCF00D2DF649), // 10^-340
    (0xE95A99DF8ACE6F53, 0xF4D82C2C107973DC), // 10^-339
    (0x91D8A02BB6C10594, 0x79071B9B8A4BE869), // 10^-338
    (0xB64EC836A47146F9, 0x9748E2826CDEE284), // 10^-337
    (0xE3E27A444D8D98B7, 0xFD1B1B2308169B25), // 10^-336
    (0x8E6D8C6AB0787F72, 0xFE30F0F5E50E20F7), // 10^-335
    (0xB208EF855C969F4F, 0xBDBD2D335E51A935), // 10^-334
    (0xDE8B2B66B3BC4723, 0xAD2C788035E61382), // 10^-333
    (0x8B16FB203055AC76, 0x4C3BCB5021AFCC31), // 10^-332
    (0xADDCB9E83C6B1793, 0xDF4ABE242A1BBF3D), // 10^-331
    (0xD953E8624B85DD78, 0xD71D6DAD34A2AF0D), // 10^-330
    (0x87D4713D6F33AA6B, 0x8672648C40E5AD68), // 10^-329
    (0xA9C98D8CCB009506, 0x680EFDAF511F18C2), // 10^-328
    (0xD43BF0EFFDC0BA48, 0x0212BD1B2566DEF2), // 10^-327
    (0x84A57695FE98746D, 0x014BB630F7604B57), // 10^-326
    (0xA5CED43B7E3E9188, 0x419EA3BD35385E2D), // 10^-325
    (0xCF42894A5DCE35EA, 0x52064CAC828675B9), // 10^-324
    (0x818995CE7AA0E1B2, 0x7343EFEBD1940993), // 10^-323
    (0xA1EBFB4219491A1F, 0x1014EBE6C5F90BF8), // 10^-322
    (0xCA66FA129F9B60A6, 0xD41A26E077774EF6), // 10^-321
    (0xFD00B897478238D0, 0x8920B098955522B4), // 10^-320
    (0x9E20735E8CB16382, 0x55B46E5F5D5535B0), // 10^-319
    (0xC5A890362FDDBC62, 0xEB2189F734AA831D), // 10^-318
    (0xF712B443BBD52B7B, 0xA5E9EC7501D523E4), // 10^-317
    (0x9A6BB0AA55653B2D, 0x47B233C92125366E), // 10^-316
    (0xC1069CD4EABE89F8, 0x999EC0BB696E840A), // 10^-315
    (0xF148440A256E2C76, 0xC00670EA43CA250D), // 10^-314
    (0x96CD2A865764DBCA, 0x380406926A5E5728), // 10^-313
    (0xBC807527ED3E12BC, 0xC605083704F5ECF2), // 10^-312
    (0xEBA09271E88D976B, 0xF7864A44C633682E), // 10^-311
    (0x93445B8731587EA3, 0x7AB3EE6AFBE0211D), // 10^-310
    (0xB8157268FDAE9E4C, 0x5960EA05BAD82964), // 10^-309
    (0xE61ACF033D1A45DF, 0x6FB92487298E33BD), // 10^-308
    (0x8FD0C16206306BAB, 0xA5D3B6D479F8E056), // 10^-307
    (0xB3C4F1BA87BC8696, 0x8F48A4899877186C), // 10^-306
    (0xE0B62E2929ABA83C, 0x331ACDABFE94DE87), // 10^-305
    (0x8C71DCD9BA0B4925, 0x9FF0C08B7F1D0B14), // 10^-304
    (0xAF8E5410288E1B6F, 0x07ECF0AE5EE44DD9), // 10^-303
    (0xDB71E91432B1A24A, 0xC9E82CD9F69D6150), // 10^-302
    (0x892731AC9FAF056E, 0xBE311C083A225CD2), // 10^-301
    (0xAB70FE17C79AC6CA, 0x6DBD630A48AAF406), // 10^-300
    (0xD64D3D9DB981787D, 0x092CBBCCDAD5B108), // 10^-299
    (0x85F0468293F0EB4E, 0x25BBF56008C58EA5), // 10^-298
    (0xA76C582338ED2621, 0xAF2AF2B80AF6F24E), // 10^-297
    (0xD1476E2C07286FAA, 0x1AF5AF660DB4AEE1), // 10^-296
    (0x82CCA4DB847945CA, 0x50D98D9FC890ED4D), // 10^-295
    (0xA37FCE126597973C, 0xE50FF107BAB528A0), // 10^-294
    (0xCC5FC196FEFD7D0C, 0x1E53ED49A96272C8), // 10^-293
    (0xFF77B1FCBEBCDC4F, 0x25E8E89C13BB0F7A), // 10^-292
    (0x9FAACF3DF73609B1, 0x77B191618C54E9AC), // 10^-291
    (0xC795830D75038C1D, 0xD59DF5B9EF6A2417), // 10^-290
    (0xF97AE3D0D2446F25, 0x4B0573286B44AD1D), // 10^-289
    (0x9BECCE62836AC577, 0x4EE367F9430AEC32), // 10^-288
    (0xC2E801FB244576D5, 0x229C41F793CDA73F), // 10^-287
    (0xF3A20279ED56D48A, 0x6B43527578C1110F), // 10^-286
    (0x9845418C345644D6, 0x830A13896B78AAA9), // 10^-285
    (0xBE5691EF416BD60C, 0x23CC986BC656D553), // 10^-284
    (0xEDEC366B11C6CB8F, 0x2CBFBE86B7EC8AA8), // 10^-283
    (0x94B3A202EB1C3F39, 0x7BF7D71432F3D6A9), // 10^-282
    (0xB9E08A83A5E34F07, 0xDAF5CCD93FB0CC53), // 10^-281
    (0xE858AD248F5C22C9, 0xD1B3400F8F9CFF68), // 10^-280
    (0x91376C36D99995BE, 0x23100809B9C21FA1), // 10^-279
    (0xB58547448FFFFB2D, 0xABD40A0C2832A78A), // 10^-278
    (0xE2E69915B3FFF9F9, 0x16C90C8F323F516C), // 10^-277
    (0x8DD01FAD907FFC3B, 0xAE3DA7D97F6792E3), // 10^-276
    (0xB1442798F49FFB4A, 0x99CD11CFDF41779C), // 10^-275
    (0xDD95317F31C7FA1D, 0x40405643D711D583), // 10^-274
    (0x8A7D3EEF7F1CFC52, 0x482835EA666B2572), // 10^-273
    (0xAD1C8EAB5EE43B66, 0xDA3243650005EECF), // 10^-272
    (0xD863B256369D4A40, 0x90BED43E40076A82), // 10^-271
    (0x873E4F75E2224E68, 0x5A7744A6E804A291), // 10^-270
    (0xA90DE3535AAAE202, 0x711515D0A205CB36), // 10^-269
    (0xD3515C2831559A83, 0x0D5A5B44CA873E03), // 10^-268
    (0x8412D9991ED58091, 0xE858790AFE9486C2), // 10^-267
    (0xA5178FFF668AE0B6, 0x626E974DBE39A872), // 10^-266
    (0xCE5D73FF402D98E3, 0xFB0A3D212DC8128F), // 10^-265
    (0x80FA687F881C7F8E, 0x7CE66634BC9D0B99), // 10^-264
    (0xA139029F6A239F72, 0x1C1FFFC1EBC44E80), // 10^-263
    (0xC987434744AC874E, 0xA327FFB266B56220), // 10^-262
    (0xFBE9141915D7A922, 0x4BF1FF9F0062BAA8), // 10^-261
    (0x9D71AC8FADA6C9B5, 0x6F773FC3603DB4A9), // 10^-260
    (0xC4CE17B399107C22, 0xCB550FB4384D21D3), // 10^-259
    (0xF6019DA07F549B2B, 0x7E2A53A146606A48), // 10^-258
    (0x99C102844F94E0FB, 0x2EDA7444CBFC426D), // 10^-257
    (0xC0314325637A1939, 0xFA911155FEFB5308), // 10^-256
    (0xF03D93EEBC589F88, 0x793555AB7EBA27CA), // 10^-255
    (0x96267C7535B763B5, 0x4BC1558B2F3458DE), // 10^-254
    (0xBBB01B9283253CA2, 0x9EB1AAEDFB016F16), // 10^-253
    (0xEA9C227723EE8BCB, 0x465E15A979C1CADC), // 10^-252
    (0x92A1958A7675175F, 0x0BFACD89EC191EC9), // 10^-251
    (0xB749FAED14125D36, 0xCEF980EC671F667B), // 10^-250
    (0xE51C79A85916F484, 0x82B7E12780E7401A), // 10^-249
    (0x8F31CC0937AE58D2, 0xD1B2ECB8B0908810), // 10^-248
    (0xB2FE3F0B8599EF07, 0x861FA7E6DCB4AA15), // 10^-247
    (0xDFBDCECE67006AC9, 0x67A791E093E1D49A), // 10^-246
    (0x8BD6A141006042BD, 0xE0C8BB2C5C6D24E0), // 10^-245
    (0xAECC49914078536D, 0x58FAE9F773886E18), // 10^-244
    (0xDA7F5BF590966848, 0xAF39A475506A899E), // 10^-243
    (0x888F99797A5E012D, 0x6D8406C952429603), // 10^-242
    (0xAAB37FD7D8F58178, 0xC8E5087BA6D33B83), // 10^-241
    (0xD5605FCDCF32E1D6, 0xFB1E4A9A90880A64), // 10^-240
    (0x855C3BE0A17FCD26, 0x5CF2EEA09A55067F), // 10^-239
    (0xA6B34AD8C9DFC06F, 0xF42FAA48C0EA481E), // 10^-238
    (0xD0601D8EFC57B08B, 0xF13B94DAF124DA26), // 10^-237
    (0x823C12795DB6CE57, 0x76C53D08D6B70858), // 10^-236
    (0xA2CB1717B52481ED, 0x54768C4B0C64CA6E), // 10^-235
    (0xCB7DDCDDA26DA268, 0xA9942F5DCF7DFD09), // 10^-234
    (0xFE5D54150B090B02, 0xD3F93B35435D7C4C), // 10^-233
    (0x9EFA548D26E5A6E1, 0xC47BC5014A1A6DAF), // 10^-232
    (0xC6B8E9B0709F109A, 0x359AB6419CA1091B), // 10^-231
    (0xF867241C8CC6D4C0, 0xC30163D203C94B62), // 10^-230
    (0x9B407691D7FC44F8, 0x79E0DE63425DCF1D), // 10^-229
    (0xC21094364DFB5636, 0x985915FC12F542E4), // 10^-228
    (0xF294B943E17A2BC4, 0x3E6F5B7B17B2939D), // 10^-227
    (0x979CF3CA6CEC5B5A, 0xA705992CEECF9C42), // 10^-226
    (0xBD8430BD08277231, 0x50C6FF782A838353), // 10^-225
    (0xECE53CEC4A314EBD, 0xA4F8BF5635246428), // 10^-224
    (0x940F4613AE5ED136, 0x871B7795E136BE99), // 10^-223
    (0xB913179899F68584, 0x28E2557B59846E3F), // 10^-222
    (0xE757DD7EC07426E5, 0x331AEADA2FE589CF), // 10^-221
    (0x9096EA6F3848984F, 0x3FF0D2C85DEF7621), // 10^-220
    (0xB4BCA50B065ABE63, 0x0FED077A756B53A9), // 10^-219
    (0xE1EBCE4DC7F16DFB, 0xD3E8495912C62894), // 10^-218
    (0x8D3360F09CF6E4BD, 0x64712DD7ABBBD95C), // 10^-217
    (0xB080392CC4349DEC, 0xBD8D794D96AACFB3), // 10^-216
    (0xDCA04777F541C567, 0xECF0D7A0FC5583A0), // 10^-215
    (0x89E42CAAF9491B60, 0xF41686C49DB57244), // 10^-214
    (0xAC5D37D5B79B6239, 0x311C2875C522CED5), // 10^-213
    (0xD77485CB25823AC7, 0x7D633293366B828B), // 10^-212
    (0x86A8D39EF77164BC, 0xAE5DFF9C02033197), // 10^-211
    (0xA8530886B54DBDEB, 0xD9F57F830283FDFC), // 10^-210
    (0xD267CAA862A12D66, 0xD072DF63C324FD7B), // 10^-209
    (0x8380DEA93DA4BC60, 0x4247CB9E59F71E6D), // 10^-208
    (0xA46116538D0DEB78, 0x52D9BE85F074E608), // 10^-207
    (0xCD795BE870516656, 0x67902E276C921F8B), // 10^-206
    (0x806BD9714632DFF6, 0x00BA1CD8A3DB53B6), // 10^-205
    (0xA086CFCD97BF97F3, 0x80E8A40ECCD228A4), // 10^-204
    (0xC8A883C0FDAF7DF0, 0x6122CD128006B2CD), // 10^-203
    (0xFAD2A4B13D1B5D6C, 0x796B805720085F81), // 10^-202
    (0x9CC3A6EEC6311A63, 0xCBE3303674053BB0), // 10^-201
    (0xC3F490AA77BD60FC, 0xBEDBFC4411068A9C), // 10^-200
    (0xF4F1B4D515ACB93B, 0xEE92FB5515482D44), // 10^-199
    (0x991711052D8BF3C5, 0x751BDD152D4D1C4A), // 10^-198
    (0xBF5CD54678EEF0B6, 0xD262D45A78A0635D), // 10^-197
    (0xEF340A98172AACE4, 0x86FB897116C87C34), // 10^-196
    (0x9580869F0E7AAC0E, 0xD45D35E6AE3D4DA0), // 10^-195
    (0xBAE0A846D2195712, 0x8974836059CCA109), // 10^-194
    (0xE998D258869FACD7, 0x2BD1A438703FC94B), // 10^-193
    (0x91FF83775423CC06, 0x7B6306A34627DDCF), // 10^-192
    (0xB67F6455292CBF08, 0x1A3BC84C17B1D542), // 10^-191
    (0xE41F3D6A7377EECA, 0x20CABA5F1D9E4A93), // 10^-190
    (0x8E938662882AF53E, 0x547EB47B7282EE9C), // 10^-189
    (0xB23867FB2A35B28D, 0xE99E619A4F23AA43), // 10^-188
    (0xDEC681F9F4C31F31, 0x6405FA00E2EC94D4), // 10^-187
    (0x8B3C113C38F9F37E, 0xDE83BC408DD3DD04), // 10^-186
    (0xAE0B158B4738705E, 0x9624AB50B148D445), // 10^-185
    (0xD98DDAEE19068C76, 0x3BADD624DD9B0957), // 10^-184
    (0x87F8A8D4CFA417C9, 0xE54CA5D70A80E5D6), // 10^-183
    (0xA9F6D30A038D1DBC, 0x5E9FCF4CCD211F4C), // 10^-182
    (0xD47487CC8470652B, 0x7647C3200069671F), // 10^-181
    (0x84C8D4DFD2C63F3B, 0x29ECD9F40041E073), // 10^-180
    (0xA5FB0A17C777CF09, 0xF468107100525890), // 10^-179
    (0xCF79CC9DB955C2CC, 0x7182148D4066EEB4), // 10^-178
    (0x81AC1FE293D599BF, 0xC6F14CD848405530), // 10^-177
    (0xA21727DB38CB002F, 0xB8ADA00E5A506A7C), // 10^-176
    (0xCA9CF1D206FDC03B, 0xA6D90811F0E4851C), // 10^-175
    (0xFD442E4688BD304A, 0x908F4A166D1DA663), // 10^-174
    (0x9E4A9CEC15763E2E, 0x9A598E4E043287FE), // 10^-173
    (0xC5DD44271AD3CDBA, 0x40EFF1E1853F29FD), // 10^-172
    (0xF7549530E188C128, 0xD12BEE59E68EF47C), // 10^-171
    (0x9A94DD3E8CF578B9, 0x82BB74F8301958CE), // 10^-170
    (0xC13A148E3032D6E7, 0xE36A52363C1FAF01), // 10^-169
    (0xF18899B1BC3F8CA1, 0xDC44E6C3CB279AC1), // 10^-168
    (0x96F5600F15A7B7E5, 0x29AB103A5EF8C0B9), // 10^-167
    (0xBCB2B812DB11A5DE, 0x7415D448F6B6F0E7), // 10^-166
    (0xEBDF661791D60F56, 0x111B495B3464AD21), // 10^-165
    (0x936B9FCEBB25C995, 0xCAB10DD900BEEC34), // 10^-164
    (0xB84687C269EF3BFB, 0x3D5D514F40EEA742), // 10^-163
    (0xE65829B3046B0AFA, 0x0CB4A5A3112A5112), // 10^-162
    (0x8FF71A0FE2C2E6DC, 0x47F0E785EABA72AB), // 10^-161
    (0xB3F4E093DB73A093, 0x59ED216765690F56), // 10^-160
    (0xE0F218B8D25088B8, 0x306869C13EC3532C), // 10^-159
    (0x8C974F7383725573, 0x1E414218C73A13FB), // 10^-158
    (0xAFBD2350644EEACF, 0xE5D1929EF90898FA), // 10^-157
    (0xDBAC6C247D62A583, 0xDF45F746B74ABF39), // 10^-156
    (0x894BC396CE5DA772, 0x6B8BBA8C328EB783), // 10^-155
    (0xAB9EB47C81F5114F, 0x066EA92F3F326564), // 10^-154
    (0xD686619BA27255A2, 0xC80A537B0EFEFEBD), // 10^-153
    (0x8613FD0145877585, 0xBD06742CE95F5F36), // 10^-152
    (0xA798FC4196E952E7, 0x2C48113823B73704), // 10^-151
    (0xD17F3B51FCA3A7A0, 0xF75A15862CA504C5), // 10^-150
    (0x82EF85133DE648C4, 0x9A984D73DBE722FB), // 10^-149
    (0xA3AB66580D5FDAF5, 0xC13E60D0D2E0EBBA), // 10^-148
    (0xCC963FEE10B7D1B3, 0x318DF905079926A8), // 10^-147
    (0xFFBBCFE994E5C61F, 0xFDF17746497F7052), // 10^-146
    (0x9FD561F1FD0F9BD3, 0xFEB6EA8BEDEFA633), // 10^-145
    (0xC7CABA6E7C5382C8, 0xFE64A52EE96B8FC0), // 10^-144
    (0xF9BD690A1B68637B, 0x3DFDCE7AA3C673B0), // 10^-143
    (0x9C1661A651213E2D, 0x06BEA10CA65C084E), // 10^-142
    (0xC31BFA0FE5698DB8, 0x486E494FCFF30A62), // 10^-141
    (0xF3E2F893DEC3F126, 0x5A89DBA3C3EFCCFA), // 10^-140
    (0x986DDB5C6B3A76B7, 0xF89629465A75E01C), // 10^-139
    (0xBE89523386091465, 0xF6BBB397F1135823), // 10^-138
    (0xEE2BA6C0678B597F, 0x746AA07DED582E2C), // 10^-137
    (0x94DB483840B717EF, 0xA8C2A44EB4571CDC), // 10^-136
    (0xBA121A4650E4DDEB, 0x92F34D62616CE413), // 10^-135
    (0xE896A0D7E51E1566, 0x77B020BAF9C81D17), // 10^-134
    (0x915E2486EF32CD60, 0x0ACE1474DC1D122E), // 10^-133
    (0xB5B5ADA8AAFF80B8, 0x0D819992132456BA), // 10^-132
    (0xE3231912D5BF60E6, 0x10E1FFF697ED6C69), // 10^-131
    (0x8DF5EFABC5979C8F, 0xCA8D3FFA1EF463C1), // 10^-130
    (0xB1736B96B6FD83B3, 0xBD308FF8A6B17CB2), // 10^-129
    (0xDDD0467C64BCE4A0, 0xAC7CB3F6D05DDBDE), // 10^-128
    (0x8AA22C0DBEF60EE4, 0x6BCDF07A423AA96B), // 10^-127
    (0xAD4AB7112EB3929D, 0x86C16C98D2C953C6), // 10^-126
    (0xD89D64D57A607744, 0xE871C7BF077BA8B7), // 10^-125
    (0x87625F056C7C4A8B, 0x11471CD764AD4972), // 10^-124
    (0xA93AF6C6C79B5D2D, 0xD598E40D3DD89BCF), // 10^-123
    (0xD389B47879823479, 0x4AFF1D108D4EC2C3), // 10^-122
    (0x843610CB4BF160CB, 0xCEDF722A585139BA), // 10^-121
    (0xA54394FE1EEDB8FE, 0xC2974EB4EE658828), // 10^-120
    (0xCE947A3DA6A9273E, 0x733D226229FEEA32), // 10^-119
    (0x811CCC668829B887, 0x0806357D5A3F525F), // 10^-118
    (0xA163FF802A3426A8, 0xCA07C2DCB0CF26F7), // 10^-117
    (0xC9BCFF6034C13052, 0xFC89B393DD02F0B5), // 10^-116
    (0xFC2C3F3841F17C67, 0xBBAC2078D443ACE2), // 10^-115
    (0x9D9BA7832936EDC0, 0xD54B944B84AA4C0D), // 10^-114
    (0xC5029163F384A931, 0x0A9E795E65D4DF11), // 10^-113
    (0xF64335BCF065D37D, 0x4D4617B5FF4A16D5), // 10^-112
    (0x99EA0196163FA42E, 0x504BCED1BF8E4E45), // 10^-111
    (0xC06481FB9BCF8D39, 0xE45EC2862F71E1D6), // 10^-110
    (0xF07DA27A82C37088, 0x5D767327BB4E5A4C), // 10^-109
    (0x964E858C91BA2655, 0x3A6A07F8D510F86F), // 10^-108
    (0xBBE226EFB628AFEA, 0x890489F70A55368B), // 10^-107
    (0xEADAB0ABA3B2DBE5, 0x2B45AC74CCEA842E), // 10^-106
    (0x92C8AE6B464FC96F, 0x3B0B8BC90012929D), // 10^-105
    (0xB77ADA0617E3BBCB, 0x09CE6EBB40173744), // 10^-104
    (0xE55990879DDCAABD, 0xCC420A6A101D0515), // 10^-103
    (0x8F57FA54C2A9EAB6, 0x9FA946824A12232D), // 10^-102
    (0xB32DF8E9F3546564, 0x47939822DC96ABF9), // 10^-101
    (0xDFF9772470297EBD, 0x59787E2B93BC56F7), // 10^-100
    (0x8BFBEA76C619EF36, 0x57EB4EDB3C55B65A), // 10^-99
    (0xAEFAE51477A06B03, 0xEDE622920B6B23F1), // 10^-98
    (0xDAB99E59958885C4, 0xE95FAB368E45ECED), // 10^-97
    (0x88B402F7FD75539B, 0x11DBCB0218EBB414), // 10^-96
    (0xAAE103B5FCD2A881, 0xD652BDC29F26A119), // 10^-95
    (0xD59944A37C0752A2, 0x4BE76D3346F0495F), // 10^-94
    (0x857FCAE62D8493A5, 0x6F70A4400C562DDB), // 10^-93
    (0xA6DFBD9FB8E5B88E, 0xCB4CCD500F6BB952), // 10^-92
    (0xD097AD07A71F26B2, 0x7E2000A41346A7A7), // 10^-91
    (0x825ECC24C873782F, 0x8ED400668C0C28C8), // 10^-90
    (0xA2F67F2DFA90563B, 0x728900802F0F32FA), // 10^-89
    (0xCBB41EF979346BCA, 0x4F2B40A03AD2FFB9), // 10^-88
    (0xFEA126B7D78186BC, 0xE2F610C84987BFA8), // 10^-87
    (0x9F24B832E6B0F436, 0x0DD9CA7D2DF4D7C9), // 10^-86
    (0xC6EDE63FA05D3143, 0x91503D1C79720DBB), // 10^-85
    (0xF8A95FCF88747D94, 0x75A44C6397CE912A), // 10^-84
    (0x9B69DBE1B548CE7C, 0xC986AFBE3EE11ABA), // 10^-83
    (0xC24452DA229B021B, 0xFBE85BADCE996168), // 10^-82
    (0xF2D56790AB41C2A2, 0xFAE27299423FB9C3), // 10^-81
    (0x97C560BA6B0919A5, 0xDCCD879FC967D41A), // 10^-80
    (0xBDB6B8E905CB600F, 0x5400E987BBC1C920), // 10^-79
    (0xED246723473E3813, 0x290123E9AAB23B68), // 10^-78
    (0x9436C0760C86E30B, 0xF9A0B6720AAF6521), // 10^-77
    (0xB94470938FA89BCE, 0xF808E40E8D5B3E69), // 10^-76
    (0xE7958CB87392C2C2, 0xB60B1D1230B20E04), // 10^-75
    (0x90BD77F3483BB9B9, 0xB1C6F22B5E6F48C2), // 10^-74
    (0xB4ECD5F01A4AA828, 0x1E38AEB6360B1AF3), // 10^-73
    (0xE2280B6C20DD5232, 0x25C6DA63C38DE1B0), // 10^-72
    (0x8D590723948A535F, 0x579C487E5A38AD0E), // 10^-71
    (0xB0AF48EC79ACE837, 0x2D835A9DF0C6D851), // 10^-70
    (0xDCDB1B2798182244, 0xF8E431456CF88E65), // 10^-69
    (0x8A08F0F8BF0F156B, 0x1B8E9ECB641B58FF), // 10^-68
    (0xAC8B2D36EED2DAC5, 0xE272467E3D222F3F), // 10^-67
    (0xD7ADF884AA879177, 0x5B0ED81DCC6ABB0F), // 10^-66
    (0x86CCBB52EA94BAEA, 0x98E947129FC2B4E9), // 10^-65
    (0xA87FEA27A539E9A5, 0x3F2398D747B36224), // 10^-64
    (0xD29FE4B18E88640E, 0x8EEC7F0D19A03AAD), // 10^-63
    (0x83A3EEEEF9153E89, 0x1953CF68300424AC), // 10^-62
    (0xA48CEAAAB75A8E2B, 0x5FA8C3423C052DD7), // 10^-61
    (0xCDB02555653131B6, 0x3792F412CB06794D), // 10^-60
    (0x808E17555F3EBF11, 0xE2BBD88BBEE40BD0), // 10^-59
    (0xA0B19D2AB70E6ED6, 0x5B6ACEAEAE9D0EC4), // 10^-58
    (0xC8DE047564D20A8B, 0xF245825A5A445275), // 10^-57
    (0xFB158592BE068D2E, 0xEED6E2F0F0D56712), // 10^-56
    (0x9CED737BB6C4183D, 0x55464DD69685606B), // 10^-55
    (0xC428D05AA4751E4C, 0xAA97E14C3C26B886), // 10^-54
    (0xF53304714D9265DF, 0xD53DD99F4B3066A8), // 10^-53
    (0x993FE2C6D07B7FAB, 0xE546A8038EFE4029), // 10^-52
    (0xBF8FDB78849A5F96, 0xDE98520472BDD033), // 10^-51
    (0xEF73D256A5C0F77C, 0x963E66858F6D4440), // 10^-50
    (0x95A8637627989AAD, 0xDDE7001379A44AA8), // 10^-49
    (0xBB127C53B17EC159, 0x5560C018580D5D52), // 10^-48
    (0xE9D71B689DDE71AF, 0xAAB8F01E6E10B4A6), // 10^-47
    (0x9226712162AB070D, 0xCAB3961304CA70E8), // 10^-46
    (0xB6B00D69BB55C8D1, 0x3D607B97C5FD0D22), // 10^-45
    (0xE45C10C42A2B3B05, 0x8CB89A7DB77C506A), // 10^-44
    (0x8EB98A7A9A5B04E3, 0x77F3608E92ADB242), // 10^-43
    (0xB267ED1940F1C61C, 0x55F038B237591ED3), // 10^-42
    (0xDF01E85F912E37A3, 0x6B6C46DEC52F6688), // 10^-41
    (0x8B61313BBABCE2C6, 0x2323AC4B3B3DA015), // 10^-40
    (0xAE397D8AA96C1B77, 0xABEC975E0A0D081A), // 10^-39
    (0xD9C7DCED53C72255, 0x96E7BD358C904A21), // 10^-38
    (0x881CEA14545C7575, 0x7E50D64177DA2E54), // 10^-37
    (0xAA242499697392D2, 0xDDE50BD1D5D0B9E9), // 10^-36
    (0xD4AD2DBFC3D07787, 0x955E4EC64B44E864), // 10^-35
    (0x84EC3C97DA624AB4, 0xBD5AF13BEF0B113E), // 10^-34
    (0xA6274BBDD0FADD61, 0xECB1AD8AEACDD58E), // 10^-33
    (0xCFB11EAD453994BA, 0x67DE18EDA5814AF2), // 10^-32
    (0x81CEB32C4B43FCF4, 0x80EACF948770CED7), // 10^-31
    (0xA2425FF75E14FC31, 0xA1258379A94D028D), // 10^-30
    (0xCAD2F7F5359A3B3E, 0x096EE45813A04330), // 10^-29
    (0xFD87B5F28300CA0D, 0x8BCA9D6E188853FC), // 10^-28
    (0x9E74D1B791E07E48, 0x775EA264CF55347D), // 10^-27
    (0xC612062576589DDA, 0x95364AFE032A819D), // 10^-26
    (0xF79687AED3EEC551, 0x3A83DDBD83F52204), // 10^-25
    (0x9ABE14CD44753B52, 0xC4926A9672793542), // 10^-24
    (0xC16D9A0095928A27, 0x75B7053C0F178293), // 10^-23
    (0xF1C90080BAF72CB1, 0x5324C68B12DD6338), // 10^-22
    (0x971DA05074DA7BEE, 0xD3F6FC16EBCA5E03), // 10^-21
    (0xBCE5086492111AEA, 0x88F4BB1CA6BCF584), // 10^-20
    (0xEC1E4A7DB69561A5, 0x2B31E9E3D06C32E5), // 10^-19
    (0x9392EE8E921D5D07, 0x3AFF322E62439FCF), // 10^-18
    (0xB877AA3236A4B449, 0x09BEFEB9FAD487C2), // 10^-17
    (0xE69594BEC44DE15B, 0x4C2EBE687989A9B3), // 10^-16
    (0x901D7CF73AB0ACD9, 0x0F9D37014BF60A10), // 10^-15
    (0xB424DC35095CD80F, 0x538484C19EF38C94), // 10^-14
    (0xE12E13424BB40E13, 0x2865A5F206B06FB9), // 10^-13
    (0x8CBCCC096F5088CB, 0xF93F87B7442E45D3), // 10^-12
    (0xAFEBFF0BCB24AAFE, 0xF78F69A51539D748), // 10^-11
    (0xDBE6FECEBDEDD5BE, 0xB573440E5A884D1B), // 10^-10
    (0x89705F4136B4A597, 0x31680A88F8953030), // 10^-9
    (0xABCC77118461CEFC, 0xFDC20D2B36BA7C3D), // 10^-8
    (0xD6BF94D5E57A42BC, 0x3D32907604691B4C), // 10^-7
    (0x8637BD05AF6C69B5, 0xA63F9A49C2C1B10F), // 10^-6
    (0xA7C5AC471B478423, 0x0FCF80DC33721D53), // 10^-5
    (0xD1B71758E219652B, 0xD3C36113404EA4A8), // 10^-4
    (0x83126E978D4FDF3B, 0x645A1CAC083126E9), // 10^-3
    (0xA3D70A3D70A3D70A, 0x3D70A3D70A3D70A3), // 10^-2
    (0xCCCCCCCCCCCCCCCC, 0xCCCCCCCCCCCCCCCC), // 10^-1
    (0x8000000000000000, 0x0000000000000000), // 10^0
    (0xA000000000000000, 0x0000000000000000), // 10^1
    (0xC800000000000000, 0x0000000000000000), // 10^2
    (0xFA00000000000000, 0x0000000000000000), // 10^3
    (0x9C40000000000000, 0x0000000000000000), // 10^4
    (0xC350000000000000, 0x0000000000000000), // 10^5
    (0xF424000000000000, 0x0000000000000000), // 10^6
    (0x9896800000000000, 0x0000000000000000), // 10^7
    (0xBEBC200000000000, 0x0000000000000000), // 10^8
    (0xEE6B280000000000, 0x0000000000000000), // 10^9
    (0x9502F90000000000, 0x0000000000000000), // 10^10
    (0xBA43B74000000000, 0x0000000000000000), // 10^11
    (0xE8D4A51000000000, 0x0000000000000000), // 10^12
    (0x9184E72A00000000, 0x0000000000000000), // 10^13
    (0xB5E620F480000000, 0x0000000000000000), // 10^14
    (0xE35FA931A0000000, 0x0000000000000000), // 10^15
    (0x8E1BC9BF04000000, 0x0000000000000000), // 10^16
    (0xB1A2BC2EC5000000, 0x0000000000000000), // 10^17
    (0xDE0B6B3A76400000, 0x0000000000000000), // 10^18
    (0x8AC7230489E80000, 0x0000000000000000), // 10^19
    (0xAD78EBC5AC620000, 0x0000000000000000), // 10^20
    (0xD8D726B7177A8000, 0x0000000000000000), // 10^21
    (0x878678326EAC9000, 0x0000000000000000), // 10^22
    (0xA968163F0A57B400, 0x0000000000000000), // 10^23
    (0xD3C21BCECCEDA100, 0x0000000000000000), // 10^24
    (0x84595161401484A0, 0x0000000000000000), // 10^25
    (0xA56FA5B99019A5C8, 0x0000000000000000), // 10^26
    (0xCECB8F27F4200F3A, 0x0000000000000000), // 10^27
    (0x813F3978F8940984, 0x4000000000000000), // 10^28
    (0xA18F07D736B90BE5, 0x5000000000000000), // 10^29
    (0xC9F2C9CD04674EDE, 0xA400000000000000), // 10^30
    (0xFC6F7C4045812296, 0x4D00000000000000), // 10^31
    (0x9DC5ADA82B70B59D, 0xF020000000000000), // 10^32
    (0xC5371912364CE305, 0x6C28000000000000), // 10^33
    (0xF684DF56C3E01BC6, 0xC732000000000000), // 10^34
    (0x9A130B963A6C115C, 0x3C7F400000000000), // 10^35
    (0xC097CE7BC90715B3, 0x4B9F100000000000), // 10^36
    (0xF0BDC21ABB48DB20, 0x1E86D40000000000), // 10^37
    (0x96769950B50D88F4, 0x1314448000000000), // 10^38
    (0xBC143FA4E250EB31, 0x17D955A000000000), // 10^39
    (0xEB194F8E1AE525FD, 0x5DCFAB0800000000), // 10^40
    (0x92EFD1B8D0CF37BE, 0x5AA1CAE500000000), // 10^41
    (0xB7ABC627050305AD, 0xF14A3D9E40000000), // 10^42
    (0xE596B7B0C643C719, 0x6D9CCD05D0000000), // 10^43
    (0x8F7E32CE7BEA5C6F, 0xE4820023A2000000), // 10^44
    (0xB35DBF821AE4F38B, 0xDDA2802C8A800000), // 10^45
    (0xE0352F62A19E306E, 0xD50B2037AD200000), // 10^46
    (0x8C213D9DA502DE45, 0x4526F422CC340000), // 10^47
    (0xAF298D050E4395D6, 0x9670B12B7F410000), // 10^48
    (0xDAF3F04651D47B4C, 0x3C0CDD765F114000), // 10^49
    (0x88D8762BF324CD0F, 0xA5880A69FB6AC800), // 10^50
    (0xAB0E93B6EFEE0053, 0x8EEA0D047A457A00), // 10^51
    (0xD5D238A4ABE98068, 0x72A4904598D6D880), // 10^52
    (0x85A36366EB71F041, 0x47A6DA2B7F864750), // 10^53
    (0xA70C3C40A64E6C51, 0x999090B65F67D924), // 10^54
    (0xD0CF4B50CFE20765, 0xFFF4B4E3F741CF6D), // 10^55
    (0x82818F1281ED449F, 0xBFF8F10E7A8921A4), // 10^56
    (0xA321F2D7226895C7, 0xAFF72D52192B6A0D), // 10^57
    (0xCBEA6F8CEB02BB39, 0x9BF4F8A69F764490), // 10^58
    (0xFEE50B7025C36A08, 0x02F236D04753D5B4), // 10^59
    (0x9F4F2726179A2245, 0x01D762422C946590), // 10^60
    (0xC722F0EF9D80AAD6, 0x424D3AD2B7B97EF5), // 10^61
    (0xF8EBAD2B84E0D58B, 0xD2E0898765A7DEB2), // 10^62
    (0x9B934C3B330C8577, 0x63CC55F49F88EB2F), // 10^63
    (0xC2781F49FFCFA6D5, 0x3CBF6B71C76B25FB), // 10^64
    (0xF316271C7FC3908A, 0x8BEF464E3945EF7A), // 10^65
    (0x97EDD871CFDA3A56, 0x97758BF0E3CBB5AC), // 10^66
    (0xBDE94E8E43D0C8EC, 0x3D52EEED1CBEA317), // 10^67
    (0xED63A231D4C4FB27, 0x4CA7AAA863EE4BDD), // 10^68
    (0x945E455F24FB1CF8, 0x8FE8CAA93E74EF6A), // 10^69
    (0xB975D6B6EE39E436, 0xB3E2FD538E122B44), // 10^70
    (0xE7D34C64A9C85D44, 0x60DBBCA87196B616), // 10^71
    (0x90E40FBEEA1D3A4A, 0xBC8955E946FE31CD), // 10^72
    (0xB51D13AEA4A488DD, 0x6BABAB6398BDBE41), // 10^73
    (0xE264589A4DCDAB14, 0xC696963C7EED2DD1), // 10^74
    (0x8D7EB76070A08AEC, 0xFC1E1DE5CF543CA2), // 10^75
    (0xB0DE65388CC8ADA8, 0x3B25A55F43294BCB), // 10^76
    (0xDD15FE86AFFAD912, 0x49EF0EB713F39EBE), // 10^77
    (0x8A2DBF142DFCC7AB, 0x6E3569326C784337), // 10^78
    (0xACB92ED9397BF996, 0x49C2C37F07965404), // 10^79
    (0xD7E77A8F87DAF7FB, 0xDC33745EC97BE906), // 10^80
    (0x86F0AC99B4E8DAFD, 0x69A028BB3DED71A3), // 10^81
    (0xA8ACD7C0222311BC, 0xC40832EA0D68CE0C), // 10^82
    (0xD2D80DB02AABD62B, 0xF50A3FA490C30190), // 10^83
    (0x83C7088E1AAB65DB, 0x792667C6DA79E0FA), // 10^84
    (0xA4B8CAB1A1563F52, 0x577001B891185938), // 10^85
    (0xCDE6FD5E09ABCF26, 0xED4C0226B55E6F86), // 10^86
    (0x80B05E5AC60B6178, 0x544F8158315B05B4), // 10^87
    (0xA0DC75F1778E39D6, 0x696361AE3DB1C721), // 10^88
    (0xC913936DD571C84C, 0x03BC3A19CD1E38E9), // 10^89
    (0xFB5878494ACE3A5F, 0x04AB48A04065C723), // 10^90
    (0x9D174B2DCEC0E47B, 0x62EB0D64283F9C76), // 10^91
    (0xC45D1DF942711D9A, 0x3BA5D0BD324F8394), // 10^92
    (0xF5746577930D6500, 0xCA8F44EC7EE36479), // 10^93
    (0x9968BF6ABBE85F20, 0x7E998B13CF4E1ECB), // 10^94
    (0xBFC2EF456AE276E8, 0x9E3FEDD8C321A67E), // 10^95
    (0xEFB3AB16C59B14A2, 0xC5CFE94EF3EA101E), // 10^96
    (0x95D04AEE3B80ECE5, 0xBBA1F1D158724A12), // 10^97
    (0xBB445DA9CA61281F, 0x2A8A6E45AE8EDC97), // 10^98
    (0xEA1575143CF97226, 0xF52D09D71A3293BD), // 10^99
    (0x924D692CA61BE758, 0x593C2626705F9C56), // 10^100
    (0xB6E0C377CFA2E12E, 0x6F8B2FB00C77836C), // 10^101
    (0xE498F455C38B997A, 0x0B6DFB9C0F956447), // 10^102
    (0x8EDF98B59A373FEC, 0x4724BD4189BD5EAC), // 10^103
    (0xB2977EE300C50FE7, 0x58EDEC91EC2CB657), // 10^104
    (0xDF3D5E9BC0F653E1, 0x2F2967B66737E3ED), // 10^105
    (0x8B865B215899F46C, 0xBD79E0D20082EE74), // 10^106
    (0xAE67F1E9AEC07187, 0xECD8590680A3AA11), // 10^107
    (0xDA01EE641A708DE9, 0xE80E6F4820CC9495), // 10^108
    (0x884134FE908658B2, 0x3109058D147FDCDD), // 10^109
    (0xAA51823E34A7EEDE, 0xBD4B46F0599FD415), // 10^110
    (0xD4E5E2CDC1D1EA96, 0x6C9E18AC7007C91A), // 10^111
    (0x850FADC09923329E, 0x03E2CF6BC604DDB0), // 10^112
    (0xA6539930BF6BFF45, 0x84DB8346B786151C), // 10^113
    (0xCFE87F7CEF46FF16, 0xE612641865679A63), // 10^114
    (0x81F14FAE158C5F6E, 0x4FCB7E8F3F60C07E), // 10^115
    (0xA26DA3999AEF7749, 0xE3BE5E330F38F09D), // 10^116
    (0xCB090C8001AB551C, 0x5CADF5BFD3072CC5), // 10^117
    (0xFDCB4FA002162A63, 0x73D9732FC7C8F7F6), // 10^118
    (0x9E9F11C4014DDA7E, 0x2867E7FDDCDD9AFA), // 10^119
    (0xC646D63501A1511D, 0xB281E1FD541501B8), // 10^120
    (0xF7D88BC24209A565, 0x1F225A7CA91A4226), // 10^121
    (0x9AE757596946075F, 0x3375788DE9B06958), // 10^122
    (0xC1A12D2FC3978937, 0x0052D6B1641C83AE), // 10^123
    (0xF209787BB47D6B84, 0xC0678C5DBD23A49A), // 10^124
    (0x9745EB4D50CE6332, 0xF840B7BA963646E0), // 10^125
    (0xBD176620A501FBFF, 0xB650E5A93BC3D898), // 10^126
    (0xEC5D3FA8CE427AFF, 0xA3E51F138AB4CEBE), // 10^127
    (0x93BA47C980E98CDF, 0xC66F336C36B10137), // 10^128
    (0xB8A8D9BBE123F017, 0xB80B0047445D4184), // 10^129
    (0xE6D3102AD96CEC1D, 0xA60DC059157491E5), // 10^130
    (0x9043EA1AC7E41392, 0x87C89837AD68DB2F), // 10^131
    (0xB454E4A179DD1877, 0x29BABE4598C311FB), // 10^132
    (0xE16A1DC9D8545E94, 0xF4296DD6FEF3D67A), // 10^133
    (0x8CE2529E2734BB1D, 0x1899E4A65F58660C), // 10^134
    (0xB01AE745B101E9E4, 0x5EC05DCFF72E7F8F), // 10^135
    (0xDC21A1171D42645D, 0x76707543F4FA1F73), // 10^136
    (0x899504AE72497EBA, 0x6A06494A791C53A8), // 10^137
    (0xABFA45DA0EDBDE69, 0x0487DB9D17636892), // 10^138
    (0xD6F8D7509292D603, 0x45A9D2845D3C42B6), // 10^139
    (0x865B86925B9BC5C2, 0x0B8A2392BA45A9B2), // 10^140
    (0xA7F26836F282B732, 0x8E6CAC7768D7141E), // 10^141
    (0xD1EF0244AF2364FF, 0x3207D795430CD926), // 10^142
    (0x8335616AED761F1F, 0x7F44E6BD49E807B8), // 10^143
    (0xA402B9C5A8D3A6E7, 0x5F16206C9C6209A6), // 10^144
    (0xCD036837130890A1, 0x36DBA887C37A8C0F), // 10^145
    (0x802221226BE55A64, 0xC2494954DA2C9789), // 10^146
    (0xA02AA96B06DEB0FD, 0xF2DB9BAA10B7BD6C), // 10^147
    (0xC83553C5C8965D3D, 0x6F92829494E5ACC7), // 10^148
    (0xFA42A8B73ABBF48C, 0xCB772339BA1F17F9), // 10^149
    (0x9C69A97284B578D7, 0xFF2A760414536EFB), // 10^150
    (0xC38413CF25E2D70D, 0xFEF5138519684ABA), // 10^151
    (0xF46518C2EF5B8CD1, 0x7EB258665FC25D69), // 10^152
    (0x98BF2F79D5993802, 0xEF2F773FFBD97A61), // 10^153
    (0xBEEEFB584AFF8603, 0xAAFB550FFACFD8FA), // 10^154
    (0xEEAABA2E5DBF6784, 0x95BA2A53F983CF38), // 10^155
    (0x952AB45CFA97A0B2, 0xDD945A747BF26183), // 10^156
    (0xBA756174393D88DF, 0x94F971119AEEF9E4), // 10^157
    (0xE912B9D1478CEB17, 0x7A37CD5601AAB85D), // 10^158
    (0x91ABB422CCB812EE, 0xAC62E055C10AB33A), // 10^159
    (0xB616A12B7FE617AA, 0x577B986B314D6009), // 10^160
    (0xE39C49765FDF9D94, 0xED5A7E85FDA0B80B), // 10^161
    (0x8E41ADE9FBEBC27D, 0x14588F13BE847307), // 10^162
    (0xB1D219647AE6B31C, 0x596EB2D8AE258FC8), // 10^163
    (0xDE469FBD99A05FE3, 0x6FCA5F8ED9AEF3BB), // 10^164
    (0x8AEC23D680043BEE, 0x25DE7BB9480D5854), // 10^165
    (0xADA72CCC20054AE9, 0xAF561AA79A10AE6A), // 10^166
    (0xD910F7FF28069DA4, 0x1B2BA1518094DA04), // 10^167
    (0x87AA9AFF79042286, 0x90FB44D2F05D0842), // 10^168
    (0xA99541BF57452B28, 0x353A1607AC744A53), // 10^169
    (0xD3FA922F2D1675F2, 0x42889B8997915CE8), // 10^170
    (0x847C9B5D7C2E09B7, 0x69956135FEBADA11), // 10^171
    (0xA59BC234DB398C25, 0x43FAB9837E699095), // 10^172
    (0xCF02B2C21207EF2E, 0x94F967E45E03F4BB), // 10^173
    (0x8161AFB94B44F57D, 0x1D1BE0EEBAC278F5), // 10^174
    (0xA1BA1BA79E1632DC, 0x6462D92A69731732), // 10^175
    (0xCA28A291859BBF93, 0x7D7B8F7503CFDCFE), // 10^176
    (0xFCB2CB35E702AF78, 0x5CDA735244C3D43E), // 10^177
    (0x9DEFBF01B061ADAB, 0x3A0888136AFA64A7), // 10^178
    (0xC56BAEC21C7A1916, 0x088AAA1845B8FDD0), // 10^179
    (0xF6C69A72A3989F5B, 0x8AAD549E57273D45), // 10^180
    (0x9A3C2087A63F6399, 0x36AC54E2F678864B), // 10^181
    (0xC0CB28A98FCF3C7F, 0x84576A1BB416A7DD), // 10^182
    (0xF0FDF2D3F3C30B9F, 0x656D44A2A11C51D5), // 10^183
    (0x969EB7C47859E743, 0x9F644AE5A4B1B325), // 10^184
    (0xBC4665B596706114, 0x873D5D9F0DDE1FEE), // 10^185
    (0xEB57FF22FC0C7959, 0xA90CB506D155A7EA), // 10^186
    (0x9316FF75DD87CBD8, 0x09A7F12442D588F2), // 10^187
    (0xB7DCBF5354E9BECE, 0x0C11ED6D538AEB2F), // 10^188
    (0xE5D3EF282A242E81, 0x8F1668C8A86DA5FA), // 10^189
    (0x8FA475791A569D10, 0xF96E017D694487BC), // 10^190
    (0xB38D92D760EC4455, 0x37C981DCC395A9AC), // 10^191
    (0xE070F78D3927556A, 0x85BBE253F47B1417), // 10^192
    (0x8C469AB843B89562, 0x93956D7478CCEC8E), // 10^193
    (0xAF58416654A6BABB, 0x387AC8D1970027B2), // 10^194
    (0xDB2E51BFE9D0696A, 0x06997B05FCC0319E), // 10^195
    (0x88FCF317F22241E2, 0x441FECE3BDF81F03), // 10^196
    (0xAB3C2FDDEEAAD25A, 0xD527E81CAD7626C3), // 10^197
    (0xD60B3BD56A5586F1, 0x8A71E223D8D3B074), // 10^198
    (0x85C7056562757456, 0xF6872D5667844E49), // 10^199
    (0xA738C6BEBB12D16C, 0xB428F8AC016561DB), // 10^200
    (0xD106F86E69D785C7, 0xE13336D701BEBA52), // 10^201
    (0x82A45B450226B39C, 0xECC0024661173473), // 10^202
    (0xA34D721642B06084, 0x27F002D7F95D0190), // 10^203
    (0xCC20CE9BD35C78A5, 0x31EC038DF7B441F4), // 10^204
    (0xFF290242C83396CE, 0x7E67047175A15271), // 10^205
    (0x9F79A169BD203E41, 0x0F0062C6E984D386), // 10^206
    (0xC75809C42C684DD1, 0x52C07B78A3E60868), // 10^207
    (0xF92E0C3537826145, 0xA7709A56CCDF8A82), // 10^208
    (0x9BBCC7A142B17CCB, 0x88A66076400BB691), // 10^209
    (0xC2ABF989935DDBFE, 0x6ACFF893D00EA435), // 10^210
    (0xF356F7EBF83552FE, 0x0583F6B8C4124D43), // 10^211
    (0x98165AF37B2153DE, 0xC3727A337A8B704A), // 10^212
    (0xBE1BF1B059E9A8D6, 0x744F18C0592E4C5C), // 10^213
    (0xEDA2EE1C7064130C, 0x1162DEF06F79DF73), // 10^214
    (0x9485D4D1C63E8BE7, 0x8ADDCB5645AC2BA8), // 10^215
    (0xB9A74A0637CE2EE1, 0x6D953E2BD7173692), // 10^216
    (0xE8111C87C5C1BA99, 0xC8FA8DB6CCDD0437), // 10^217
    (0x910AB1D4DB9914A0, 0x1D9C9892400A22A2), // 10^218
    (0xB54D5E4A127F59C8, 0x2503BEB6D00CAB4B), // 10^219
    (0xE2A0B5DC971F303A, 0x2E44AE64840FD61D), // 10^220
    (0x8DA471A9DE737E24, 0x5CEAECFED289E5D2), // 10^221
    (0xB10D8E1456105DAD, 0x7425A83E872C5F47), // 10^222
    (0xDD50F1996B947518, 0xD12F124E28F77719), // 10^223
    (0x8A5296FFE33CC92F, 0x82BD6B70D99AAA6F), // 10^224
    (0xACE73CBFDC0BFB7B, 0x636CC64D1001550B), // 10^225
    (0xD8210BEFD30EFA5A, 0x3C47F7E05401AA4E), // 10^226
    (0x8714A775E3E95C78, 0x65ACFAEC34810A71), // 10^227
    (0xA8D9D1535CE3B396, 0x7F1839A741A14D0D), // 10^228
    (0xD31045A8341CA07C, 0x1EDE48111209A050), // 10^229
    (0x83EA2B892091E44D, 0x934AED0AAB460432), // 10^230
    (0xA4E4B66B68B65D60, 0xF81DA84D5617853F), // 10^231
    (0xCE1DE40642E3F4B9, 0x36251260AB9D668E), // 10^232
    (0x80D2AE83E9CE78F3, 0xC1D72B7C6B426019), // 10^233
    (0xA1075A24E4421730, 0xB24CF65B8612F81F), // 10^234
    (0xC94930AE1D529CFC, 0xDEE033F26797B627), // 10^235
    (0xFB9B7CD9A4A7443C, 0x169840EF017DA3B1), // 10^236
    (0x9D412E0806E88AA5, 0x8E1F289560EE864E), // 10^237
    (0xC491798A08A2AD4E, 0xF1A6F2BAB92A27E2), // 10^238
    (0xF5B5D7EC8ACB58A2, 0xAE10AF696774B1DB), // 10^239
    (0x9991A6F3D6BF1765, 0xACCA6DA1E0A8EF29), // 10^240
    (0xBFF610B0CC6EDD3F, 0x17FD090A58D32AF3), // 10^241
    (0xEFF394DCFF8A948E, 0xDDFC4B4CEF07F5B0), // 10^242
    (0x95F83D0A1FB69CD9, 0x4ABDAF101564F98E), // 10^243
    (0xBB764C4CA7A4440F, 0x9D6D1AD41ABE37F1), // 10^244
    (0xEA53DF5FD18D5513, 0x84C86189216DC5ED), // 10^245
    (0x92746B9BE2F8552C, 0x32FD3CF5B4E49BB4), // 10^246
    (0xB7118682DBB66A77, 0x3FBC8C33221DC2A1), // 10^247
    (0xE4D5E82392A40515, 0x0FABAF3FEAA5334A), // 10^248
    (0x8F05B1163BA6832D, 0x29CB4D87F2A7400E), // 10^249
    (0xB2C71D5BCA9023F8, 0x743E20E9EF511012), // 10^250
    (0xDF78E4B2BD342CF6, 0x914DA9246B255416), // 10^251
    (0x8BAB8EEFB6409C1A, 0x1AD089B6C2F7548E), // 10^252
    (0xAE9672ABA3D0C320, 0xA184AC2473B529B1), // 10^253
    (0xDA3C0F568CC4F3E8, 0xC9E5D72D90A2741E), // 10^254
    (0x8865899617FB1871, 0x7E2FA67C7A658892), // 10^255
    (0xAA7EEBFB9DF9DE8D, 0xDDBB901B98FEEAB7), // 10^256
    (0xD51EA6FA85785631, 0x552A74227F3EA565), // 10^257
    (0x8533285C936B35DE, 0xD53A88958F87275F), // 10^258
    (0xA67FF273B8460356, 0x8A892ABAF368F137), // 10^259
    (0xD01FEF10A657842C, 0x2D2B7569B0432D85), // 10^260
    (0x8213F56A67F6B29B, 0x9C3B29620E29FC73), // 10^261
    (0xA298F2C501F45F42, 0x8349F3BA91B47B8F), // 10^262
    (0xCB3F2F7642717713, 0x241C70A936219A73), // 10^263
    (0xFE0EFB53D30DD4D7, 0xED238CD383AA0110), // 10^264
    (0x9EC95D1463E8A506, 0xF4363804324A40AA), // 10^265
    (0xC67BB4597CE2CE48, 0xB143C6053EDCD0D5), // 10^266
    (0xF81AA16FDC1B81DA, 0xDD94B7868E94050A), // 10^267
    (0x9B10A4E5E9913128, 0xCA7CF2B4191C8326), // 10^268
    (0xC1D4CE1F63F57D72, 0xFD1C2F611F63A3F0), // 10^269
    (0xF24A01A73CF2DCCF, 0xBC633B39673C8CEC), // 10^270
    (0x976E41088617CA01, 0xD5BE0503E085D813), // 10^271
    (0xBD49D14AA79DBC82, 0x4B2D8644D8A74E18), // 10^272
    (0xEC9C459D51852BA2, 0xDDF8E7D60ED1219E), // 10^273
    (0x93E1AB8252F33B45, 0xCABB90E5C942B503), // 10^274
    (0xB8DA1662E7B00A17, 0x3D6A751F3B936243), // 10^275
    (0xE7109BFBA19C0C9D, 0x0CC512670A783AD4), // 10^276
    (0x906A617D450187E2, 0x27FB2B80668B24C5), // 10^277
    (0xB484F9DC9641E9DA, 0xB1F9F660802DEDF6), // 10^278
    (0xE1A63853BBD26451, 0x5E7873F8A0396973), // 10^279
    (0x8D07E33455637EB2, 0xDB0B487B6423E1E8), // 10^280
    (0xB049DC016ABC5E5F, 0x91CE1A9A3D2CDA62), // 10^281
    (0xDC5C5301C56B75F7, 0x7641A140CC7810FB), // 10^282
    (0x89B9B3E11B6329BA, 0xA9E904C87FCB0A9D), // 10^283
    (0xAC2820D9623BF429, 0x546345FA9FBDCD44), // 10^284
    (0xD732290FBACAF133, 0xA97C177947AD4095), // 10^285
    (0x867F59A9D4BED6C0, 0x49ED8EABCCCC485D), // 10^286
    (0xA81F301449EE8C70, 0x5C68F256BFFF5A74), // 10^287
    (0xD226FC195C6A2F8C, 0x73832EEC6FFF3111), // 10^288
    (0x83585D8FD9C25DB7, 0xC831FD53C5FF7EAB), // 10^289
    (0xA42E74F3D032F525, 0xBA3E7CA8B77F5E55), // 10^290
    (0xCD3A1230C43FB26F, 0x28CE1BD2E55F35EB), // 10^291
    (0x80444B5E7AA7CF85, 0x7980D163CF5B81B3), // 10^292
    (0xA0555E361951C366, 0xD7E105BCC332621F), // 10^293
    (0xC86AB5C39FA63440, 0x8DD9472BF3FEFAA7), // 10^294
    (0xFA856334878FC150, 0xB14F98F6F0FEB951), // 10^295
    (0x9C935E00D4B9D8D2, 0x6ED1BF9A569F33D3), // 10^296
    (0xC3B8358109E84F07, 0x0A862F80EC4700C8), // 10^297
    (0xF4A642E14C6262C8, 0xCD27BB612758C0FA), // 10^298
    (0x98E7E9CCCFBD7DBD, 0x8038D51CB897789C), // 10^299
    (0xBF21E44003ACDD2C, 0xE0470A63E6BD56C3), // 10^300
    (0xEEEA5D5004981478, 0x1858CCFCE06CAC74), // 10^301
    (0x95527A5202DF0CCB, 0x0F37801E0C43EBC8), // 10^302
    (0xBAA718E68396CFFD, 0xD30560258F54E6BA), // 10^303
    (0xE950DF20247C83FD, 0x47C6B82EF32A2069), // 10^304
    (0x91D28B7416CDD27E, 0x4CDC331D57FA5441), // 10^305
    (0xB6472E511C81471D, 0xE0133FE4ADF8E952), // 10^306
    (0xE3D8F9E563A198E5, 0x58180FDDD97723A6), // 10^307
    (0x8E679C2F5E44FF8F, 0x570F09EAA7EA7648), // 10^308
];

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halfway_round_down() {
        let radix = 10;
        let kind = RoundingKind::NearestTieEven;

        // Check only Eisel-Lemire.
        assert_eq!(
            (9007199254740992.0, true),
            eisel_lemire::<f64, _>(9007199254740992u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740993u64, radix, 0, kind));
        assert_eq!(
            (9007199254740994.0, true),
            eisel_lemire::<f64, _>(9007199254740994u64, radix, 0, kind)
        );
        assert_eq!(
            (9223372036854775808.0, true),
            eisel_lemire::<f64, _>(9223372036854775808u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9223372036854776832u64, radix, 0, kind));
        assert_eq!(
            (9223372036854777856.0, true),
            eisel_lemire::<f64, _>(9223372036854777856u64, radix, 0, kind)
        );

        // We can't get an accurate representation here.
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740992000u64, radix, -3, kind));
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740993000u64, radix, -3, kind));
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740994000u64, radix, -3, kind));

        // Check with the extended-float backup.
        assert_eq!(
            (9007199254740992.0, true),
            moderate_path::<f64>(9007199254740992u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9007199254740992.0, false),
            moderate_path::<f64>(9007199254740993u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9007199254740994.0, true),
            moderate_path::<f64>(9007199254740994u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9223372036854775808.0, true),
            moderate_path::<f64>(9223372036854775808u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9223372036854775808.0, false),
            moderate_path::<f64>(9223372036854776832u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9223372036854777856.0, true),
            moderate_path::<f64>(9223372036854777856u64, radix, 0, false, false, kind)
        );

        // We can't get an accurate from Lemire representation here.
        assert_eq!(
            (9007199254740992.0, true),
            moderate_path::<f64>(9007199254740992000u64, radix, -3, false, false, kind)
        );
        assert_eq!(
            (9007199254740992.0, false),
            moderate_path::<f64>(9007199254740993000u64, radix, -3, false, false, kind)
        );
        assert_eq!(
            (9007199254740994.0, true),
            moderate_path::<f64>(9007199254740994000u64, radix, -3, false, false, kind)
        );
    }

    #[test]
    fn test_halfway_round_up() {
        let radix = 10;
        let kind = RoundingKind::NearestTieEven;

        // Check only Eisel-Lemire.
        assert_eq!(
            (9007199254740994.0, true),
            eisel_lemire::<f64, _>(9007199254740994u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740995u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740996u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481988.0, true),
            eisel_lemire::<f64, _>(18014398509481988u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481990u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481992u64, radix, 0, kind)
        );
        assert_eq!(
            (9223372036854777856.0, true),
            eisel_lemire::<f64, _>(9223372036854777856u64, radix, 0, kind)
        );
        assert_eq!(
            (9223372036854779904.0, true),
            eisel_lemire::<f64, _>(9223372036854778880u64, radix, 0, kind)
        );
        assert_eq!(
            (9223372036854779904.0, true),
            eisel_lemire::<f64, _>(9223372036854779904u64, radix, 0, kind)
        );

        // We can't get an accurate representation here.
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740994000u64, radix, -3, kind));
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740995000u64, radix, -3, kind));
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740996000u64, radix, -3, kind));

        // Check with the extended-float backup.
        assert_eq!(
            (9007199254740994.0, true),
            moderate_path::<f64>(9007199254740994u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            moderate_path::<f64>(9007199254740995u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            moderate_path::<f64>(9007199254740996u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (18014398509481988.0, true),
            moderate_path::<f64>(18014398509481988u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            moderate_path::<f64>(18014398509481990u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            moderate_path::<f64>(18014398509481992u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9223372036854777856.0, true),
            moderate_path::<f64>(9223372036854777856u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9223372036854779904.0, true),
            moderate_path::<f64>(9223372036854778880u64, radix, 0, false, false, kind)
        );
        assert_eq!(
            (9223372036854779904.0, true),
            moderate_path::<f64>(9223372036854779904u64, radix, 0, false, false, kind)
        );

        // We can't get an accurate from Lemire representation here.
        assert_eq!(
            (9007199254740994.0, true),
            moderate_path::<f64>(9007199254740994000u64, radix, -3, false, false, kind)
        );
        assert_eq!(
            (9007199254740994.0, false),
            moderate_path::<f64>(9007199254740995000u64, radix, -3, false, false, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            moderate_path::<f64>(9007199254740996000u64, radix, -3, false, false, kind)
        );
    }

    #[test]
    #[cfg(feature = "rounding")]
    fn test_lemire_rounding() {
        let radix = 10;

        // Nearest, tie-even
        let kind = RoundingKind::NearestTieEven;
        assert_eq!(
            (9007199254740992.0, true),
            eisel_lemire::<f64, _>(9007199254740992u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740993u64, radix, 0, kind));
        assert_eq!(
            (9007199254740994.0, true),
            eisel_lemire::<f64, _>(9007199254740994u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740995u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740996u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481984.0, true),
            eisel_lemire::<f64, _>(18014398509481984u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(18014398509481986u64, radix, 0, kind));
        assert_eq!(
            (18014398509481988.0, true),
            eisel_lemire::<f64, _>(18014398509481988u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481990u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481992u64, radix, 0, kind)
        );

        // Nearest, tie-away zero
        let kind = RoundingKind::NearestTieAwayZero;
        assert_eq!(
            (9007199254740992.0, true),
            eisel_lemire::<f64, _>(9007199254740992u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740993u64, radix, 0, kind));
        assert_eq!(
            (9007199254740994.0, true),
            eisel_lemire::<f64, _>(9007199254740994u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740995u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740996u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481984.0, true),
            eisel_lemire::<f64, _>(18014398509481984u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(18014398509481986u64, radix, 0, kind));
        assert_eq!(
            (18014398509481988.0, true),
            eisel_lemire::<f64, _>(18014398509481988u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481990u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481992u64, radix, 0, kind)
        );

        // Upward
        let kind = RoundingKind::Upward;
        assert_eq!(
            (9007199254740992.0, true),
            eisel_lemire::<f64, _>(9007199254740992u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740993u64, radix, 0, kind));
        assert_eq!(
            (9007199254740994.0, true),
            eisel_lemire::<f64, _>(9007199254740994u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740995u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740996u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481984.0, true),
            eisel_lemire::<f64, _>(18014398509481984u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(18014398509481986u64, radix, 0, kind));
        assert_eq!(
            (18014398509481988.0, true),
            eisel_lemire::<f64, _>(18014398509481988u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481990u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481992u64, radix, 0, kind)
        );

        // Downward
        let kind = RoundingKind::Downward;
        assert_eq!(
            (9007199254740992.0, true),
            eisel_lemire::<f64, _>(9007199254740992u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740993u64, radix, 0, kind));
        assert_eq!(
            (9007199254740994.0, true),
            eisel_lemire::<f64, _>(9007199254740994u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740994.0, true),
            eisel_lemire::<f64, _>(9007199254740995u64, radix, 0, kind)
        );
        assert_eq!(
            (9007199254740996.0, true),
            eisel_lemire::<f64, _>(9007199254740996u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481984.0, true),
            eisel_lemire::<f64, _>(18014398509481984u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(18014398509481986u64, radix, 0, kind));
        assert_eq!(
            (18014398509481988.0, true),
            eisel_lemire::<f64, _>(18014398509481988u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481988.0, true),
            eisel_lemire::<f64, _>(18014398509481990u64, radix, 0, kind)
        );
        assert_eq!(
            (18014398509481992.0, true),
            eisel_lemire::<f64, _>(18014398509481992u64, radix, 0, kind)
        );
        assert_eq!((0.0, false), eisel_lemire::<f64, _>(9007199254740992000u64, radix, -3, kind));
    }

    #[test]
    fn test_mul() {
        let e1 = 11529215046068469760u64; // 1e1
        let e10 = 10737418240000000000u64; // 1e10
        assert_eq!((0x5D21DBA000000000, 0x0000000000000000), mul(e1, e10));

        let e9 = 17179869184000000000u64; // 1e9
        let e70 = 13363823550460978230u64; // 1e70
        assert_eq!((0xACB92ED9397BF995, 0xA23A700000000000), mul(e9, e70));

        // e289
        let e280 = 10162340898095201970u64; // 1e280
        assert_eq!((0x83585D8FD9C25DB6, 0xFC31D00000000000), mul(e9, e280));

        // e290
        let e0 = 9223372036854775808u64; // 1e0
        let e290 = 11830521861667747109u64; // 1e290
        assert_eq!((0x52173A79E8197A92, 0x8000000000000000), mul(e0, e290));
    }
}
