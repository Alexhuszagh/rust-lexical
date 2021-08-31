'''
    log
    ===

    Generate pre-computed constants for fast integral logs,
    and ensure they work exactly like the dragonbox and exact
    implementations, to avoid calculation errors.
'''

import math
import numpy as np
import sys

# GENERATORS
# ----------

def floor(x):
    # Valid even when x is negative
    return int(math.floor(x))

# Does a quick generation of `x * log(a, b)` for the floor,
# and validates it over the entire range of values.
def calc_fast_log(max_exp, bitshift, log_base, radix, cb):
    den = 1 << bitshift
    num = int(math.ceil(math.log(radix, log_base) * den))
    for exp in range(-max_exp, max_exp + 1):
        exact = cb(exp)
        guess = num * exp // den
        if exact != guess:
            raise ValueError(f'exp={exp}, exact={exact}, guess={guess}')
    return num, bitshift

# Does a quick generation of `x * log(r1, b1) - log(r2, b2)` for the
# floor, and validates it over the entire range of values.
def calc_fast_log_sub(max_exp, bitshift, b1, r1, b2, r2, cb):
    den = 1 << bitshift
    num = int(math.ceil(math.log(r1, b1) * den))
    sub = int(math.ceil(math.log(r2, b2) * den))
    for exp in range(-max_exp, max_exp + 1):
        exact = cb(exp)
        guess = (num * exp - sub) // den
        if exact != guess:
            raise ValueError(f'exp={exp}, exact={exact}, guess={guess}')
    return num, sub, bitshift

# Does a quick generation of `x * log(r1, b1) - log(r2, b2) / div` for the
# floor, and validates it over the entire range of values.
def calc_fast_log_sub_div(max_exp, bitshift, b1, r1, b2, r2, div, cb):
    den = 1 << bitshift
    num = int(math.ceil(math.log(r1, b1) * den))
    sub = int(math.floor(math.log(r2, b2) * den) / div)
    for exp in range(-max_exp, max_exp + 1):
        exact = cb(exp)
        guess = (num * exp - sub) // den
        if exact != guess:
            raise ValueError(f'exp={exp}, exact={exact}, guess={guess}')
    return num, sub, bitshift

# Iterates over the range of valid bitshifts to try to calculate
# the log constants.
def gen_fast_log(max_exp, log_base, radix, cb):
    bitshift = 1
    while bitshift <= 25:
        try:
            return calc_fast_log(max_exp, bitshift, log_base, radix, cb)
        except ValueError:
            bitshift += 1

    raise ValueError('Calculating constants for log failed.')

def gen_fast_log_sub(max_exp, b1, r1, b2, r2, cb):
    bitshift = 1
    while bitshift <= 25:
        try:
            return calc_fast_log_sub(max_exp, bitshift, b1, r1, b2, r2, cb)
        except ValueError:
            bitshift += 1

    raise ValueError('Calculating constants for log failed.')

def gen_fast_log_sub_div(max_exp, b1, r1, b2, r2, div, cb):
    bitshift = 1
    while bitshift <= 25:
        try:
            return calc_fast_log_sub_div(max_exp, bitshift, b1, r1, b2, r2, div, cb)
        except ValueError:
            bitshift += 1

    raise ValueError('Calculating constants for log failed.')

# This is for generating x * log2(10)
def gen_log2_10(max_exp=1233):
    return gen_fast_log(max_exp, 2, 10, dragonbox_log2_10)

# This is for generating x * log10(2)
def gen_log10_2(max_exp=1700):
    return gen_fast_log(max_exp, 10, 2, dragonbox_log10_2)

# This is for generating x * log5(2)
def gen_log5_2(max_exp=1492):
    return gen_fast_log(max_exp, 5, 2, dragonbox_log5_2)

# This is for generating x * log5(2) - log5(3)
def gen_log5_2_sub_log5_3(max_exp=2427):
    return gen_fast_log_sub(max_exp, 5, 2, 5, 3, dragonbox_log5_2_sub_log5_3)

# This is for generating x * log10(2) - log10(4) / 3
def gen_log10_2_sub_log10_4_div3(max_exp=1700):
    return gen_fast_log_sub_div(max_exp, 10, 2, 10, 4, 3, dragonbox_log10_2_sub_log10_4_div3)

# GENERIC
# -------

# Generic, for other radixes.
def exact_log(exponent, log_base, radix):
    return floor(exponent * math.log(radix, log_base))

def exact_sub_log(exponent, b1, r1, b2, r2):
    v1 = exponent * math.log(r1, b1)
    v2 = math.log(r2, b2)
    return floor(v1 - v2)

def exact_sub_log_div(exponent, b1, r1, b2, r2, den):
    v1 = exponent * math.log(r1, b1)
    v2 = math.log(r2, b2) / den
    return floor(v1 - v2)

def lemire_log(exponent, multiplier, bitshift):
    return int((exponent * multiplier) >> bitshift)

def lemire_sub_log(exponent, multiplier, sub, bitshift):
    return int((exponent * multiplier - sub) >> bitshift)

def floor_shift(integer, fraction, shift):
    integer = np.uint32(integer)
    fraction = np.uint64(fraction)
    fraction_shr = fraction >> np.uint64(64 - shift)
    return int((integer << np.uint32(shift)) | np.uint32(fraction_shr))

# COMPUTE
# -------

# These all do `x * log5(2)`
def exact_log5_2(exponent):
    return exact_log(exponent, 5, 2)

def lemire_log5_2(exponent):
    return lemire_log(exponent, 225799, 19)

def dragonbox_log5_2(exponent):
    q = np.int32(exponent)
    c = floor_shift(0, 0x6e40d1a4143dcb94, 20)
    s = floor_shift(0, 0, 20)
    return (q * c - s) >> 20

# These all do `x * log10(2)`
def exact_log10_2(exponent):
    return exact_log(exponent, 10, 2)

def lemire_log10_2(exponent):
    return lemire_log(exponent, 315653, 20)

def dragonbox_log10_2(exponent):
    q = np.int32(exponent)
    c = floor_shift(0, 0x4d104d427de7fbcc, 22)
    s = floor_shift(0, 0, 22)
    return (q * c - s) >> 22

# These all do `x * log2(10)`
def exact_log2_10(exponent):
    return exact_log(exponent, 2, 10)

def lemire_log2_10(exponent):
    return lemire_log(exponent, 1741647, 19)

def dragonbox_log2_10(exponent):
    q = np.int32(exponent)
    c = floor_shift(3, 0x5269e12f346e2bf9, 19)
    s = floor_shift(0, 0, 19)
    return (q * c - s) >> 19

# These all do `x * log5(2) - log5(3)`
def exact_log5_2_sub_log5_3(exponent):
    return exact_sub_log(exponent, 5, 2, 5, 3)

def lemire_log5_2_sub_log5_3(exponent):
    return lemire_sub_log(exponent, 451597, 715764, 20)

def dragonbox_log5_2_sub_log5_3(exponent):
    q = np.int32(exponent)
    c = floor_shift(0, 0x6e40d1a4143dcb94, 20)
    s = floor_shift(0, 0xaebf47915d443b24, 20)
    return (q * c - s) >> 20

# These all do `x * log10(2) - log10(4) / 3`
def exact_log10_2_sub_log10_4_div3(exponent):
    return exact_sub_log_div(exponent, 10, 2, 10, 4, 3)

def lemire_log10_2_sub_log10_4_div3(exponent):
    return lemire_sub_log(exponent, 1262611, 524031, 22)

def dragonbox_log10_2_sub_log10_4_div3(exponent):
    # This value **isn't** actually exact, so it can't work
    # with any automated generator.
    q = np.int32(exponent)
    c = floor_shift(0, 0x4d104d427de7fbcc, 22)
    s = floor_shift(0, 0x1ffbfc2bbc780375, 22)
    return (q * c - s) >> 22

# VALIDATE
# --------

def check_ratio(
    exact_cb,
    lemire_cb,
    dragonbox_cb,
    max_exp=1492,
    skip_exact=False
):
    for exponent in range(-max_exp, max_exp + 1):
        exact = exact_cb(exponent)
        lemire = lemire_cb(exponent)
        dbox = dragonbox_cb(exponent)
        if lemire != dbox:
            print(f'exponent={exponent}, lemire={lemire}, dbox={dbox}', file=sys.stderr)
            sys.exit(1)
        if not skip_exact and exact != dbox:
            print(f'exponent={exponent}, exact={exact}, dbox={dbox}', file=sys.stderr)
            sys.exit(1)


def main():
    print(f'gen_log5_2={gen_log5_2()}')
    check_ratio(
        exact_log5_2,
        lemire_log5_2,
        dragonbox_log5_2,
        max_exp=1492,
    )

    print(f'gen_log10_2={gen_log10_2()}')
    check_ratio(
        exact_log10_2,
        lemire_log10_2,
        dragonbox_log10_2,
        max_exp=1700,
    )

    print(f'gen_log2_10={gen_log2_10()}')
    check_ratio(
        exact_log2_10,
        lemire_log2_10,
        dragonbox_log2_10,
        max_exp=1233,
    )

    print(f'gen_log5_2_sub_log5_3={gen_log5_2_sub_log5_3()}')
    check_ratio(
        exact_log5_2_sub_log5_3,
        lemire_log5_2_sub_log5_3,
        dragonbox_log5_2_sub_log5_3,
        max_exp=2427,
    )

    #print(f'gen_log10_2_sub_log10_4_div3={gen_log10_2_sub_log10_4_div3()}')
    check_ratio(
        exact_log10_2_sub_log10_4_div3,
        lemire_log10_2_sub_log10_4_div3,
        dragonbox_log10_2_sub_log10_4_div3,
        # Exact actually fails here, since we don't actually care if we round-up.
        skip_exact=True,
        max_exp=1700,
    )

if __name__ == '__main__':
    main()
