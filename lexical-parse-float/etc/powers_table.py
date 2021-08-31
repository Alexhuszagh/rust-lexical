#!/usr/bin/env python3

"""
Generate powers of a given radix.

This is used for both the fast-path algorithms, for native
multiplication, and as pre-computed powers for exponentiation
for the slow path algorithms.
"""

def print_int(radix, max_exp):
    '''Print pre-computed small powers as u64.'''

    print(f'/// Pre-computed, small powers-of-{radix}.')
    print(f'pub const SMALL_INT_POW{radix}: [u64; {max_exp + 1}] = [')
    for exponent in range(0, max_exp + 1):
        print(f'    {radix**exponent},')
    print('];')
    print(f'const_assert!(SMALL_INT_POW{radix}.len() > f64_mantissa_limit({radix}) as usize);')
    print(f'const_assert!(SMALL_INT_POW{radix}.len() == u64_power_limit({radix}) as usize + 1);')
    print('')

def print_f32(radix, max_exp):
    '''Print pre-computed small powers as f32.'''

    print(f'/// Pre-computed, small powers-of-{radix}.')
    print(f'pub const SMALL_F32_POW{radix}: [f32; {max_exp + 1}] = [')
    for exponent in range(0, max_exp + 1):
        print(f'    {float(radix)**exponent},')
    print('];')
    print(f'const_assert!(SMALL_F32_POW{radix}.len() > f32_exponent_limit({radix}).1 as usize);')
    print('')

def print_f64(radix, max_exp):
    '''Print pre-computed small powers as f64.'''

    print(f'/// Pre-computed, small powers-of-{radix}.')
    print(f'pub const SMALL_F64_POW{radix}: [f64; {max_exp + 1}] = [')
    for exponent in range(0, max_exp + 1):
        print(f'    {float(radix)**exponent},')
    print('];')
    print(f'const_assert!(SMALL_F64_POW{radix}.len() > f64_exponent_limit({radix}).1 as usize);')
    print('')

def as_u32(value):
    '''Convert a big integer to an array of 32-bit values.'''

    result = []
    max_u32 = 2**32 - 1
    while value:
        result.append(value & max_u32)
        value >>= 32
    return result

def as_u64(value):
    '''Convert a big integer to an array of 64-bit values.'''

    result = []
    max_u64 = 2**64 - 1
    while value:
        result.append(value & max_u64)
        value >>= 64
    return result

def print_large(radix, max_exp):
    '''Print a pre-computed large power as a native limb.'''

    power = radix**(5 * max_exp)
    limb32 = as_u32(power)
    limb64 = as_u64(power)
    print(f'/// Pre-computed large power-of-{radix} for 32-bit limbs.')
    print('#[cfg(not(all(target_pointer_width = "64", not(target_arch = "sparc"))))]')
    print(f'pub const LARGE_POW{radix}: [u32; {len(limb32)}] = [')
    for value in limb32:
        print(f'    {value},')
    print(f'];')
    print(f'')

    print(f'/// Pre-computed large power-of-{radix} for 64-bit limbs.')
    print('#[cfg(all(target_pointer_width = "64", not(target_arch = "sparc")))]')
    print(f'pub const LARGE_POW{radix}: [u64; {len(limb64)}] = [')
    for value in limb64:
        print(f'    {value},')
    print(f'];')
    print(f'')
    print(f'/// Step for large power-of-{radix} for 32-bit limbs.')
    print(f'pub const LARGE_POW{radix}_STEP: u32 = {5 * max_exp};')
    print(f'')

def print_tables(radix, f64_pow_limit, f32_exp_limit, f64_exp_limit):
    print_int(radix, f64_pow_limit)
    print_f32(radix, f32_exp_limit)
    print_f64(radix, f64_exp_limit)
    if radix % 2 != 0:
        print_large(radix, f64_pow_limit)

def f32_exponent_limit(radix):
    return {
        3 : (-15, 15),
        5 : (-10, 10),
        6 : (-15, 15),
        7 : (-8, 8),
        9 : (-7, 7),
        11: (-6, 6),
        12: (-15, 15),
        13: (-6, 6),
        14: (-8, 8),
        15: (-6, 6),
        17: (-5, 5),
        18: (-7, 7),
        19: (-5, 5),
        20: (-10, 10),
        21: (-5, 5),
        22: (-6, 6),
        23: (-5, 5),
        24: (-15, 15),
        25: (-5, 5),
        26: (-6, 6),
        27: (-5, 5),
        28: (-8, 8),
        29: (-4, 4),
        30: (-6, 6),
        31: (-4, 4),
        33: (-4, 4),
        34: (-5, 5),
        35: (-4, 4),
        36: (-7, 7),
    }[radix]

def f64_exponent_limit(radix):
    return {
        3: (-33, 33),
        5: (-22, 22),
        6: (-33, 33),
        7: (-18, 18),
        9: (-16, 16),
        11: (-15, 15),
        12: (-33, 33),
        13: (-14, 14),
        14: (-18, 18),
        15: (-13, 13),
        17: (-12, 12),
        18: (-16, 16),
        19: (-12, 12),
        20: (-22, 22),
        21: (-12, 12),
        22: (-15, 15),
        23: (-11, 11),
        24: (-33, 33),
        25: (-11, 11),
        26: (-14, 14),
        27: (-11, 11),
        28: (-18, 18),
        29: (-10, 10),
        30: (-13, 13),
        31: (-10, 10),
        33: (-10, 10),
        34: (-12, 12),
        35: (-10, 10),
        36: (-16, 16),
    }[radix]

def f64_power_limit(radix):
    return {
        3: 40,
        5: 27,
        6: 24,
        7: 22,
        9: 20,
        11: 18,
        12: 17,
        13: 17,
        14: 16,
        15: 16,
        17: 15,
        18: 15,
        19: 15,
        20: 14,
        21: 14,
        22: 14,
        23: 14,
        24: 13,
        25: 13,
        26: 13,
        27: 13,
        28: 13,
        29: 13,
        30: 13,
        31: 12,
        33: 12,
        34: 12,
        35: 12,
        36: 12
    }[radix]

radixes = [3, 5, 6, 7, 9, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36]
for radix in radixes:
    f64_pow_limit = f64_power_limit(radix)
    f32_exp_limit = f32_exponent_limit(radix)[1]
    f64_exp_limit = f64_exponent_limit(radix)[1]
    print_tables(radix, f64_pow_limit, f32_exp_limit, f64_exp_limit)
