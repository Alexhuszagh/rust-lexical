#!/usr/bin/env python3

"""
Generate powers of a given radix for the Bellerophon algorithm.

Specifically, computes and outputs (as Rust code) a table of 10^e for some
range of exponents e. The output is one array of 128 bit significands.
The base two exponents can be inferred using a logarithmic slope
of the decimal exponent. The approximations are normalized and rounded perfectly,
i.e., within 0.5 ULP of the true value.

Ported from Rust's core library implementation, which itself is
adapted from Daniel Lemire's fast_float ``table_generation.py``,
available here: <https://github.com/fastfloat/fast_float/blob/main/script/table_generation.py>.
"""

import math
from collections import deque

STEP_STR = "static BASE{0}_STEP: i32 = {1};"
SMALL_MANTISSA_STR = "static BASE{0}_SMALL_MANTISSA: [u64; {1}] = ["
SMALL_EXPONENT_STR = "static BASE{0}_SMALL_EXPONENT: [i32; {1}] = ["
LARGE_MANTISSA_STR = "static BASE{0}_LARGE_MANTISSA: [u64; {1}] = ["
LARGE_EXPONENT_STR = "static BASE{0}_LARGE_EXPONENT: [i32; {1}] = ["
SMALL_INT_STR = "static BASE{0}_SMALL_INT_POWERS: [u64; {1}] = {2};"
BIAS_STR = "static BASE{0}_BIAS: i32 = {1};"
EXP_STR = "// {}^{}"
POWER_STR = """pub static BASE{0}_POWERS: BellerophonPowers = BellerophonPowers {{
    small: ExtendedFloatArray {{ mant: &BASE{0}_SMALL_MANTISSA, exp: &BASE{0}_SMALL_EXPONENT }},
    large: ExtendedFloatArray {{ mant: &BASE{0}_LARGE_MANTISSA, exp: &BASE{0}_LARGE_EXPONENT }},
    small_int: &BASE{0}_SMALL_INT_POWERS,
    step: BASE{0}_STEP,
    bias: BASE{0}_BIAS,
}};\n"""

def calculate_bitshift(base, exponent):
    '''
    Calculate the bitshift required for a given base. The exponent
    is the absolute value of the max exponent (log distance from 1.)
    '''

    return 63 + math.ceil(math.log2(base**exponent))


def next_fp(fp, base, step = 1):
    '''Generate the next extended-floating point value.'''

    return (fp[0] * (base**step), fp[1])


def prev_fp(fp, base, step = 1):
    '''Generate the previous extended-floating point value.'''

    return (fp[0] // (base**step), fp[1])


def normalize_fp(fp):
    '''Normalize a extended-float so the MSB is the 64th bit'''

    while fp[0] >> 64 != 0:
        fp = (fp[0] >> 1, fp[1] + 1)
    return fp


def generate_small(base, count):
    '''Generate the small powers for a given base'''

    bitshift = calculate_bitshift(base, count)
    fps = []
    fp = (1 << bitshift, -bitshift)
    for exp in range(count):
        fps.append((normalize_fp(fp), exp))
        fp = next_fp(fp, base)

    # Print the small powers as integers.
    ints = [base**i for _, i in fps]

    return fps, ints


def generate_large(base, step):
    '''Generate the large powers for a given base.'''

    # Get our starting parameters
    min_exp = math.floor(math.log(5e-324, base) - math.log(0xFFFFFFFFFFFFFFFF, base))
    max_exp = math.ceil(math.log(1.7976931348623157e+308, base))
    bitshift = calculate_bitshift(base, abs(min_exp - step))
    fps = deque()

    # Add negative exponents
    # We need to go below the minimum exponent, since we need
    # all resulting exponents to be positive.
    fp = (1 << bitshift, -bitshift)
    for exp in range(-step, min_exp-step, -step):
        fp = prev_fp(fp, base, step)
        fps.appendleft((normalize_fp(fp), exp))

    # Add positive exponents
    fp = (1 << bitshift, -bitshift)
    fps.append((normalize_fp(fp), 0))
    for exp in range(step, max_exp, step):
        fp = next_fp(fp, base, step)
        fps.append((normalize_fp(fp), exp))

    # Return the smallest exp, AKA, the bias
    return fps, -fps[0][1]


def print_array(base, string, fps, index):
    '''Print an entire array'''

    print(string.format(base, len(fps)))
    for fp, exp in fps:
        value = "    {},".format(fp[index])
        exp = EXP_STR.format(base, exp)
        print(value.ljust(30, " ") + exp)
    print("];")


def generate_base(base):
    '''Generate all powers and variables.'''

    step = math.floor(math.log(1e10, base))
    small, ints = generate_small(base, step)
    large, bias = generate_large(base, step)

    print_array(base, SMALL_MANTISSA_STR, small, 0)
    print_array(base, SMALL_EXPONENT_STR, small, 1)
    print_array(base, LARGE_MANTISSA_STR, large, 0)
    print_array(base, LARGE_EXPONENT_STR, large, 1)
    print(SMALL_INT_STR.format(base, len(ints), ints))
    print(STEP_STR.format(base, step))
    print(BIAS_STR.format(base, bias))


def generate():
    '''Generate all bases.'''

    bases = [
        3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
        22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
    ]

    for base in bases:
        print("// BASE{}\n".format(base))
        generate_base(base)
        print("")

    print("// HIGH LEVEL\n// ----------\n")

    for base in bases:
        print(POWER_STR.format(base))


if __name__ == '__main__':
    generate()
