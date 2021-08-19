#!/usr/bin/env python3

"""
Generate the numeric limits for a given radix.

This is used for the fast-path algorithms, to calculate the
maximum number of digits or exponent bits that can be exactly
represented as a native value.
"""

import math

def is_pow2(value):
    '''Calculate if a value is a power of 2.'''

    floor = int(math.log2(value))
    return value == 2**floor

def remove_pow2(value):
    '''Remove a power of 2 from the value.'''

    while math.floor(value / 2) == value / 2:
        value //= 2
    return value

def feature(radix):
    '''Get the feature gate from the value'''

    if radix == 10:
        return ''
    elif is_pow2(radix):
        return 'if cfg!(feature = "power-of-two") '
    return 'if cfg!(feature = "radix") '


def exponent_limit(radix, mantissa_size, max_exp):
    '''
    Calculate the exponent limit for a float, for a given
    float type, where `radix` is the numerical base
    for the float type, and mantissa size is the length
    of the mantissa in bits. max_exp is the maximum
    binary exponent, where all exponent bits except the lowest
    are set (or, `2**(exponent_size - 1) - 1`).
    '''

    if is_pow2(radix):
        # Can always be exactly represented. We can't handle
        # denormal floats, however.
        scaled = int(max_exp / math.log2(radix))
        return (-scaled, scaled)
    else:
        # Positive and negative should be the same,
        # since we need to find the maximum digit
        # representable with mantissa digits.
        # We first need to remove the highest power-of-
        # two from the radix, since these will be represented
        # with exponent digits.
        base = remove_pow2(radix)
        precision = mantissa_size + 1
        exp_limit = int(precision / math.log2(base))
        return (-exp_limit, exp_limit)

def mantissa_limit(radix, mantissa_size):
    '''
    Calculate mantissa limit for a float type, given
    the radix and the length of the mantissa in bits.
    '''

    precision = mantissa_size + 1
    return int(precision / math.log2(radix))

def all_limits(mantissa_size, exponent_size, type_name):
    '''Print limits for all radixes.'''

    max_exp = 2**(exponent_size - 1) - 1

    print('/// Get the exponent limit as a const fn.')
    print('#[inline(always)]')
    print(f'pub const fn {type_name}_exponent_limit(radix: u32) -> (i64, i64) {{')
    print('    match radix {')
    for radix in range(2, 37):
        exp_limit = exponent_limit(radix, mantissa_size, max_exp)
        print(f'        {radix} {feature(radix)}=> {exp_limit},')
    print('        _ => (0, 0),')
    print('    }')
    print('}')
    print('')

    print('/// Get the mantissa limit as a const fn.')
    print('#[inline(always)]')
    print(f'pub const fn {type_name}_mantissa_limit(radix: u32) -> i64 {{')
    print('    match radix {')
    for radix in range(2, 37):
        mant_limit = mantissa_limit(radix, mantissa_size)
        print(f'        {radix} {feature(radix)}=> {mant_limit},')
    print('        _ => 0,')
    print('    }')
    print('}')
    print('')

all_limits(23, 8, 'f32')
all_limits(52, 11, 'f64')
