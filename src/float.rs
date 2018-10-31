//! Extended precision floating-point types.
//!
//! Also contains helpers to convert to and from native rust floats.

/// Extended precision floating-point type.
///
/// Contains conversions to and from f64.
#[repr(C)]
#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct FloatType {
    /// Has ~80 bits of precision (~16 for exponent).
    pub frac: u64,
    pub exp: i32,
}

impl FloatType {
    // MASKS
    // Bit-mask for the sign.
    pub const SIGN_MASK: u64        = 0x8000000000000000;
    // Bit-mask for the exponent, including the hidden bit.
    pub const EXPONENT_MASK: u64    = 0x7FF0000000000000;
    // Bit-mask for the hidden bit in exponent, which is use for the fraction.
    pub const HIDDEN_BIT_MASK: u64  = 0x0010000000000000;
    // Bit-mask for the mantissa (fraction), excluding the hidden bit.
    pub const FRACTION_MASK: u64    = 0x000FFFFFFFFFFFFF;

    // PROPERTIES
    // Positive infinity as bits.
    pub const U64_INFINITY: u64     = 0x7FF0000000000000;
    // Size of the significand (mantissa) without the hidden bit.
    pub const PHYSICAL_SIGNIFICAND_SIZE: i32 = 52;
    /// Bias of the exponent.
    pub const EXPONENT_BIAS: i32 = 1023 + Self::PHYSICAL_SIGNIFICAND_SIZE;
    /// Exponent portion of a denormal float.
    pub const DENORMAL_EXPONENT: i32 = -Self::EXPONENT_BIAS + 1;

    /// Multiply two extended-precision floating point numbers.
    pub fn multiply(&self, b: &FloatType) -> FloatType
    {
        const LOMASK: u64 = 0x00000000FFFFFFFF;

        let ah_bl = (self.frac >> 32)    * (b.frac & LOMASK);
        let al_bh = (self.frac & LOMASK) * (b.frac >> 32);
        let al_bl = (self.frac & LOMASK) * (b.frac & LOMASK);
        let ah_bh = (self.frac >> 32)    * (b.frac >> 32);

        let mut tmp = (ah_bl & LOMASK) + (al_bh & LOMASK) + (al_bl >> 32);
        // round up
        tmp += 1 << 31;

        FloatType {
            frac: ah_bh + (ah_bl >> 32) + (al_bh >> 32) + (tmp >> 32),
            exp: self.exp + b.exp + 64
        }
    }

    /// Normalize float-point number.
    ///
    /// Let the integer component of the mantissa (significand) to be exactly
    /// 1 and the decimal fraction and exponent to be whatever.
    ///
    /// Adapted from Rust's `diy_float` class.
    ///     https://github.com/rust-lang/rust/blob/b7c6e8f1805cd8a4b0a1c1f22f17a89e9e2cea23/src/libcore/num/diy_float.rs#L49
    #[inline]
    pub fn normalize(&mut self)
    {
        if self.frac >> (64 - 32) == 0 {
            self.frac <<= 32;
            self.exp -= 32;
        }
        if self.frac >> (64 - 16) == 0 {
            self.frac <<= 16;
            self.exp -= 16;
        }
        if self.frac >> (64 - 8) == 0 {
            self.frac <<= 8;
            self.exp -= 8;
        }
        if self.frac >> (64 - 4) == 0 {
            self.frac <<= 4;
            self.exp -= 4;
        }
        if self.frac >> (64 - 2) == 0 {
            self.frac <<= 2;
            self.exp -= 2;
        }
        if self.frac >> (64 - 1) == 0 {
            self.frac <<= 1;
            self.exp -= 1;
        }
    }

    /// Get normalized boundaries for float.
    #[inline]
    pub fn normalized_boundaries(&self) -> (FloatType, FloatType) {
        let mut upper = FloatType {
            frac: (self.frac << 1) + 1,
            exp: self.exp - 1,
        };
        upper.normalize();

        let l_shift: i32 = if self.frac == Self::HIDDEN_BIT_MASK { 2 } else { 1 };

        let mut lower = FloatType {
            frac: (self.frac << l_shift) - 1,
            exp: self.exp - l_shift,
        };
        lower.frac <<= lower.exp - upper.exp;
        lower.exp = upper.exp;

        (lower, upper)
    }

    /// Create extended float from 64-bit float.
    #[inline]
    pub fn from_f64(f: f64) -> FloatType {
        let bits = f.to_bits();
        let mut fp = FloatType {
            frac: bits & Self::FRACTION_MASK,
            exp: ((bits & Self::EXPONENT_MASK) >> 52) as i32,
        };

        if fp.exp != 0 {
            fp.frac += Self::HIDDEN_BIT_MASK;
            fp.exp -= Self::EXPONENT_BIAS;
        } else {
            fp.exp = -Self::EXPONENT_BIAS + 1;
        }

        fp
    }

    /// Convert to lower-precision 64-bit float.
    #[inline]
    #[allow(dead_code)]
    pub fn as_f64(&self) -> f64 {
        const MAX_EXPONENT: i32 = 0x7FF - FloatType::EXPONENT_BIAS;

        if self.exp < Self::DENORMAL_EXPONENT {
            // sub-denormal, underflow
            0.0
        } else if self.exp >= MAX_EXPONENT {
            // overflow
            f64::from_bits(Self::U64_INFINITY)
        } else {
            // calculate the exp and fraction bits, and return a float from bits.
            let exp: u64;
            if (self.exp == Self::DENORMAL_EXPONENT) && (self.frac & Self::HIDDEN_BIT_MASK) == 0 {
                exp = 0;
            } else {
                exp = (self.exp + Self::EXPONENT_BIAS) as u64;
            }
            let exp = exp << Self::PHYSICAL_SIGNIFICAND_SIZE;
            let frac = self.frac & Self::FRACTION_MASK;
            f64::from_bits(frac | exp)
        }
    }
}


// CACHED POWERS

// FLOATING POINT CONSTANTS
const ONE_LOG_TEN: f64 = 0.30102999566398114;
const NPOWERS: i32 = 87;
const FIRSTPOWER: i32 = -348;       // 10 ^ -348
const STEPPOWERS: i32 = 8;
const EXPMAX: i32 = -32;
const EXPMIN: i32 = -60;

/// Find cached power of 10 from the exponent.
#[inline]
pub(crate) unsafe extern "C" fn cached_power(exp: i32, k: *mut i32)
    -> &'static FloatType
{
    let approx = -((exp + NPOWERS) as f64) * ONE_LOG_TEN;
    let approx = approx as i32;
    let mut idx = ((approx - FIRSTPOWER) / STEPPOWERS) as usize;

    loop {
        let power = POWERS_OF_TEN.get_unchecked(idx);
        let current = exp + power.exp + 64;
        if current < EXPMIN {
            idx += 1;
            continue;
        }

        if current > EXPMAX {
            idx -= 1;
            continue;
        }

        *k = FIRSTPOWER + (idx as i32) * STEPPOWERS;
        return power;
    }
}

/// Cached powers of ten as specified by the Grisu algorithm.
///
/// Cached powers of 10^k, calculated as if by:
/// `ceil((alpha-e+63) * ONE_LOG_TEN);`
pub(crate) const POWERS_OF_TEN: [FloatType; 87] = [
    FloatType { frac: 18054884314459144840, exp: -1220 },
    FloatType { frac: 13451937075301367670, exp: -1193 },
    FloatType { frac: 10022474136428063862, exp: -1166 },
    FloatType { frac: 14934650266808366570, exp: -1140 },
    FloatType { frac: 11127181549972568877, exp: -1113 },
    FloatType { frac: 16580792590934885855, exp: -1087 },
    FloatType { frac: 12353653155963782858, exp: -1060 },
    FloatType { frac: 18408377700990114895, exp: -1034 },
    FloatType { frac: 13715310171984221708, exp: -1007 },
    FloatType { frac: 10218702384817765436, exp: -980 },
    FloatType { frac: 15227053142812498563, exp: -954 },
    FloatType { frac: 11345038669416679861, exp: -927 },
    FloatType { frac: 16905424996341287883, exp: -901 },
    FloatType { frac: 12595523146049147757, exp: -874 },
    FloatType { frac: 9384396036005875287, exp: -847 },
    FloatType { frac: 13983839803942852151, exp: -821 },
    FloatType { frac: 10418772551374772303, exp: -794 },
    FloatType { frac: 15525180923007089351, exp: -768 },
    FloatType { frac: 11567161174868858868, exp: -741 },
    FloatType { frac: 17236413322193710309, exp: -715 },
    FloatType { frac: 12842128665889583758, exp: -688 },
    FloatType { frac: 9568131466127621947, exp: -661 },
    FloatType { frac: 14257626930069360058, exp: -635 },
    FloatType { frac: 10622759856335341974, exp: -608 },
    FloatType { frac: 15829145694278690180, exp: -582 },
    FloatType { frac: 11793632577567316726, exp: -555 },
    FloatType { frac: 17573882009934360870, exp: -529 },
    FloatType { frac: 13093562431584567480, exp: -502 },
    FloatType { frac: 9755464219737475723, exp: -475 },
    FloatType { frac: 14536774485912137811, exp: -449 },
    FloatType { frac: 10830740992659433045, exp: -422 },
    FloatType { frac: 16139061738043178685, exp: -396 },
    FloatType { frac: 12024538023802026127, exp: -369 },
    FloatType { frac: 17917957937422433684, exp: -343 },
    FloatType { frac: 13349918974505688015, exp: -316 },
    FloatType { frac: 9946464728195732843, exp: -289 },
    FloatType { frac: 14821387422376473014, exp: -263 },
    FloatType { frac: 11042794154864902060, exp: -236 },
    FloatType { frac: 16455045573212060422, exp: -210 },
    FloatType { frac: 12259964326927110867, exp: -183 },
    FloatType { frac: 18268770466636286478, exp: -157 },
    FloatType { frac: 13611294676837538539, exp: -130 },
    FloatType { frac: 10141204801825835212, exp: -103 },
    FloatType { frac: 15111572745182864684, exp: -77 },
    FloatType { frac: 11258999068426240000, exp: -50 },
    FloatType { frac: 16777216000000000000, exp: -24 },
    FloatType { frac: 12500000000000000000, exp:  3 },
    FloatType { frac: 9313225746154785156, exp:  30 },
    FloatType { frac: 13877787807814456755, exp: 56 },
    FloatType { frac: 10339757656912845936, exp: 83 },
    FloatType { frac: 15407439555097886824, exp: 109 },
    FloatType { frac: 11479437019748901445, exp: 136 },
    FloatType { frac: 17105694144590052135, exp: 162 },
    FloatType { frac: 12744735289059618216, exp: 189 },
    FloatType { frac: 9495567745759798747, exp: 216 },
    FloatType { frac: 14149498560666738074, exp: 242 },
    FloatType { frac: 10542197943230523224, exp: 269 },
    FloatType { frac: 15709099088952724970, exp: 295 },
    FloatType { frac: 11704190886730495818, exp: 322 },
    FloatType { frac: 17440603504673385349, exp: 348 },
    FloatType { frac: 12994262207056124023, exp: 375 },
    FloatType { frac: 9681479787123295682, exp: 402 },
    FloatType { frac: 14426529090290212157, exp: 428 },
    FloatType { frac: 10748601772107342003, exp: 455 },
    FloatType { frac: 16016664761464807395, exp: 481 },
    FloatType { frac: 11933345169920330789, exp: 508 },
    FloatType { frac: 17782069995880619868, exp: 534 },
    FloatType { frac: 13248674568444952270, exp: 561 },
    FloatType { frac: 9871031767461413346, exp: 588 },
    FloatType { frac: 14708983551653345445, exp: 614 },
    FloatType { frac: 10959046745042015199, exp: 641 },
    FloatType { frac: 16330252207878254650, exp: 667 },
    FloatType { frac: 12166986024289022870, exp: 694 },
    FloatType { frac: 18130221999122236476, exp: 720 },
    FloatType { frac: 13508068024458167312, exp: 747 },
    FloatType { frac: 10064294952495520794, exp: 774 },
    FloatType { frac: 14996968138956309548, exp: 800 },
    FloatType { frac: 11173611982879273257, exp: 827 },
    FloatType { frac: 16649979327439178909, exp: 853 },
    FloatType { frac: 12405201291620119593, exp: 880 },
    FloatType { frac: 9242595204427927429, exp: 907 },
    FloatType { frac: 13772540099066387757, exp: 933 },
    FloatType { frac: 10261342003245940623, exp: 960 },
    FloatType { frac: 15290591125556738113, exp: 986 },
    FloatType { frac: 11392378155556871081, exp: 1013 },
    FloatType { frac: 16975966327722178521, exp: 1039 },
    FloatType { frac: 12648080533535911531, exp: 1066 }
];
