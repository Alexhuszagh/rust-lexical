//! Fast multiplication algorithms emulating higher-precision types.
//!
//! This returns the results as hi and lo values of each type.

use super::mantissa::*;

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

/// Multiply two unsigned ints normalized so the highest bit is set.
#[inline(always)]
pub fn mul_normalized<M: Mantissa>(x: M, y: M) -> (M, M) {
    let (hi, lo) = mul(x, y);

    // Now, we've multiplied the two, but we need to normalize.
    // Normalize the bits here.
    let ctlz = hi.leading_zeros();
    return (hi << ctlz, lo << ctlz)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    // TODO(ahuszagh) Here...

//    #[test]
//    fn test_mul() {
//        // TODO(ahuszagh) Going to need to shift by the exp.
//        // Cause ofc.
//        let e0 = 9223372036854775808u64;    // 1e0
//        let e1 = 11529215046068469760u64;   // 1e1
//        let e10 = 10737418240000000000u64;  // 1e10
//        let (hi, lo) = mul_normalized(e1, e10);
//        println!("e11=0x{:016X}_{:016X}", hi, lo);
//
//        let e9 = 17179869184000000000u64;   // 1e9
//        let e70 = 13363823550460978230u64;  // 1e70
//        let (hi, lo) = mul_normalized(e9, e70);
//        println!("e79=0x{:016X}_{:016X}", hi, lo);
//        //
//
//        // e289
//        // 0xC831FD53C5FF7EAB, 0x83585D8FD9C25DB7
//        let e280 = 10162340898095201970u64;
//        let (hi, lo) = mul_normalized(e9, e280);
//        println!("289=0x{:016X}_{:016X}", hi, lo);
//
//        // e290
//        // 0xBA3E7CA8B77F5E55, 0xA42E74F3D032F
//        let e290 = 11830521861667747109u64;
//        let (hi, lo) = mul_normalized(e0, e290);
//        println!("e290=0x{:016X}_{:016X}", hi, lo);
//
//        // TODO(ahuszagh) How do we adjust the exp then?
//        //      Exp shift is: self.exp + b.exp + M::FULL
//        // pub fn get_extended_float(&self, index: usize) -> ExtendedFloat<M> {
//        //     let mant = self.mant[index];
//        //     let exp = self.exp[index];
//        //     ExtendedFloat {
//        //         mant,
//        //         exp,
//        //     }
//        // }
//
//        // exp is 900? What? Ah yeah... Hmmm
//
//
//// hi.leading_zeros()=1
//// lo.leading_zeros()=64
//// e11 mul=(6710886400000000000, 0)
//// e11 mul2=6710886400000000000
//// hi.leading_zeros()=0
//// lo.leading_zeros()=0
//// e79 mul=(12446030555722283413, 11689778928095854592)
//// e79 mul2=12446030555722283414
//// hi.leading_zeros()=0
//// lo.leading_zeros()=0
//// e289 mul=(9464417489334197686, 18172534669835239424)
//// e289 mul2=9464417489334197687
//// hi.leading_zeros()=1
//// lo.leading_zeros()=0
//// e290 mul=(5915260930833873554, 9223372036854775808)
//// e290 mul2=5915260930833873555
//
//        panic!("...");
//    }
}
