//! Pre-generated powers for the Eisel-Lemire and Bellerophon algorithms.
//!
//! These values were automatically generated: do not modify them.
//!
//! ```python
//! import math
//! import fractions
//! from collections import deque
//!
//! POWERS_STR = "pub const BASE{0}_POWERS: [(u64, u64); {1}] = ["
//! INT_POW = "pub const BASE{0}_INT_POWERS: [u64; {1}] = ["
//! F32_POW = "pub const BASE{0}_F32_POWERS: [f32; {1}] = ["
//! F64_POW = "pub const BASE{0}_F64_POWERS: [f64; {1}] = ["
//! BIAS_STR = "pub const BASE{0}_BIAS: i32 = {1};"
//! MIN_EXP_STR = "pub const BASE{0}_MIN_EXP: i32 = {1};"
//! MAX_EXP_STR = "pub const BASE{0}_MAX_EXP: i32 = {1};"
//! EXP_STR = "// {}^{}"
//!
//!
//! def normalize(mantissa):
//!     '''Normalize a extended-float so the MSB is the 128th bit'''
//!
//!     # 128 + 2
//!     shift = len(bin(mantissa)) - 130
//!     return mantissa >> shift
//!
//!
//! def remove_pow2(base):
//!     '''Remove a power of 2 from the base'''
//!
//!     while base % 2 == 0:
//!         base = base // 2
//!     return base
//!
//!
//! def generate_range(base):
//!     '''Calculate the min, max, and bias for the exponents.'''
//!
//!     min_exp = math.ceil(math.log(5e-324, base) - math.log(0xFFFFFFFFFFFFFFFF, base))
//!     max_exp = math.floor(math.log(1.7976931348623157e+308, base))
//!     new_base = remove_pow2(base)
//!     if new_base != base:
//!         bias = generate_range(new_base)[2]
//!     else:
//!         bias = -min_exp
//!     return (min_exp, max_exp, bias)
//!
//!
//! def generate_powers(base):
//!     '''Generate the small powers for a given base'''
//!
//!     # Need round-to-0 of both.
//!     min_exp, max_exp, bias = generate_range(base)
//!     fps = deque()
//!     flt0 = 1 << 127
//!
//!     # Need to remove a power-of-two prior to calculating our digits.
//!     base = remove_pow2(base)
//!
//!     # Add negative exponents.
//!     # 2^(2b)/(5^−q) with b=64 + int(math.ceil(log2(5^−q)))
//!     for exp in range(-1, min_exp-1, -1):
//! TODO(ahuszagh) b isn't accurate here.
//!         b = 64 + int(math.ceil(math.log2(base**(-exp))))
//!         num = 2**(2 * b)
//!         den = base**(-exp)
//!         flt = fractions.Fraction(num, den)
//!         fps.appendleft((normalize(math.ceil(flt)), exp))
//!
//!     # Add positive exponents
//!     fps.append((flt0, 0))
//!     for exp in range(1, max_exp+1):
//!         flt = flt0 * base**exp
//!         fps.append((normalize(flt), exp))
//!
//!     return fps
//!
//!
//! def print_powers_array(base, string, fps):
//!     '''Print the entire powers array'''
//!
//!     lo_mask = (1 << 64) - 1
//!     print(string.format(base, len(fps)))
//!     for fp, exp in fps:
//!         hi = hex(fp >> 64)
//!         lo = hex(fp & lo_mask)
//!         value = "    ({}, {}),".format(hi, lo)
//!         exp = EXP_STR.format(base, exp)
//!         print(value.ljust(56, " ") + exp)
//!     print("];")
//!
//!
//! def print_small_array(base, string, values):
//!     '''Print an entire small array'''
//!
//!     print(string.format(base, len(values)))
//!     for exp, value in enumerate(values):
//!         value = "    {},".format(value)
//!         exp = EXP_STR.format(base, exp)
//!         print(value.ljust(30, " ") + exp)
//!     print("];")
//!
//!
//! def generate():
//!     '''Generate all bases.'''
//!
//!     # Include 10 cause we do decimal without radixes
//!     odd_bases = [3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35]
//!     for base in odd_bases + [10]:
//!         print("// BASE{}\n".format(base))
//!         powers = generate_powers(base)
//!         print_powers_array(base, POWERS_STR, powers)
//!
//!         min_exp, max_exp, bias = generate_range(base)
//!         # Handle decimal specially.
//!         if base == 10:
//!             bias = -min_exp
//!         print(BIAS_STR.format(base, bias))
//!         print(MIN_EXP_STR.format(base, min_exp))
//!         print(MAX_EXP_STR.format(base, max_exp))
//!
//!     even_bases = [6, 10, 12, 14, 18, 20, 22, 24, 26, 28, 30, 34, 36]
//!     for base in even_bases:
//!         print("// BASE{}\n".format(base))
//!         min_exp, max_exp, bias = generate_range(base)
//!         print(BIAS_STR.format(base, bias))
//!         print(MIN_EXP_STR.format(base, min_exp))
//!         print(MAX_EXP_STR.format(base, max_exp))
//!
//!     for base in odd_bases + even_bases:
//!         print("// SMALL POWERS - BASE{}\n".format(base))
//!         # Int powers
//!         int_count = int(math.ceil(53 / math.log2(base)))
//!         int_powers = [base**i for i in range(int_count)]
//!         print_small_array(base, INT_POW, int_powers)
//!         # F32 powers
//!         pown_base = remove_pow2(base)
//!         f32_count = int(math.ceil(24 / math.log2(pown_base)))
//!         f32_powers = [float(base**i) for i in range(f32_count)]
//!         print_small_array(base, F32_POW, f32_powers)
//!         # F64 powers
//!         f64_count = int(math.ceil(53 / math.log2(pown_base)))
//!         f64_powers = [float(base**i) for i in range(f64_count)]
//!         print_small_array(base, F64_POW, f64_powers)
//!
//! if __name__ == '__main__':
//!     generate()
//! ```

pub use super::table_decimal::*;
#[cfg(feature = "radix")]
pub use super::table_radix::*;
