'''
    to_toml
    -------

    Convert float conformance test cases to TOML.
'''

import argparse
import json
import sys
import unittest

import numpy as np
import tomlkit

# ARGPARSE
# --------

parser = argparse.ArgumentParser(
    description='Convert floats to TOML.',
    allow_abbrev=False
)
parser.add_argument(
    '--test',
    action='store_true',
    help='Run unittests.'
)
parser.add_argument(
    '--source',
    help='JSON source file of test conditions to export to TOML.'
)
parser.add_argument(
    '--destination',
    help='TOML export file (default is stdout).'
)

# FLOAT HELPERS
# -------------

class FloatMixin:
    '''Mixing for floating-point methods.'''

    def to_bits(self):
        '''Extract bitwise representation of float.'''
        return np.frombuffer(self.value.tobytes(), dtype=self.dtype)[0]

    def to_hex(self):
        '''Convert double to hex.'''
        return '{0:0{1}x}'.format(self.to_bits(), self.value.itemsize * 2)

    def is_denormal(self):
        '''Returns true if the float is a denormal.'''
        return self.to_bits() & self.EXPONENT_MASK == 0

    def is_special(self):
        '''Returns true if the float is NaN or Infinite.'''
        return self.to_bits() & self.EXPONENT_MASK == self.EXPONENT_MASK

    def is_nan(self):
        '''Returns true if the float is NaN.'''
        return self.is_special() and self.to_bits() & self.MANTISSA_MASK != 0

    def is_inf(self):
        '''Returns true if the float is Infinite.'''
        return self.is_special() and self.to_bits() & self.MANTISSA_MASK == 0

    def exponent(self):
        '''Get exponent component from the float.'''

        if self.is_denormal():
            return self.DENORMAL_EXPONENT
        bits = self.to_bits()
        exp_bits = bits & self.EXPONENT_MASK
        biased_e = np.int32(exp_bits >> self.dtype(self.MANTISSA_SIZE))
        return biased_e - self.EXPONENT_BIAS

    def mantissa(self):
        '''Get mantissa component from the float.'''

        bits = self.to_bits()
        s = bits & self.MANTISSA_MASK
        if not self.is_denormal():
            return s + self.HIDDEN_BIT_MASK
        return s


class Float32(FloatMixin):
    '''Wrapper around a 32-bit floating point value.'''

    SIGN_MASK           = np.uint32(0x80000000)
    EXPONENT_MASK       = np.uint32(0x7F800000)
    HIDDEN_BIT_MASK     = np.uint32(0x00800000)
    MANTISSA_MASK       = np.uint32(0x007FFFFF)
    MANTISSA_SIZE       = np.int32(23)
    EXPONENT_BIAS       = np.int32(127 + MANTISSA_SIZE)
    DENORMAL_EXPONENT   = np.int32(1 - EXPONENT_BIAS)

    def __init__(self, value):
        self.value = np.float32(value)
        self.dtype = np.uint32


class Float64(FloatMixin):
    '''Wrapper around a 64-bit floating point value.'''

    SIGN_MASK           = np.uint64(0x8000000000000000)
    EXPONENT_MASK       = np.uint64(0x7FF0000000000000)
    HIDDEN_BIT_MASK     = np.uint64(0x0010000000000000)
    MANTISSA_MASK       = np.uint64(0x000FFFFFFFFFFFFF)
    MANTISSA_SIZE       = np.int32(52)
    EXPONENT_BIAS       = np.int32(1023 + MANTISSA_SIZE)
    DENORMAL_EXPONENT   = np.int32(1 - EXPONENT_BIAS)

    def __init__(self, value):
        self.value = np.float64(value)
        self.dtype = np.uint64


# TESTS
# -----

class TestFloat32(unittest.TestCase):

    def test_to_bits(self):
        float32 = Float32("1.0")
        self.assertEqual(float32.to_bits(), np.uint32(1065353216))

    def test_to_hex(self):
        float32 = Float32("1.0")
        self.assertEqual(float32.to_hex(), '3f800000')

    def test_is_denormal(self):
        float32 = Float32("1.0")
        self.assertFalse(float32.is_denormal())

        float32 = Float32("1e-45")
        self.assertTrue(float32.is_denormal())

    def test_is_special(self):
        float32 = Float32("1e-45")
        self.assertFalse(float32.is_special())

        float32 = Float32("nan")
        self.assertTrue(float32.is_special())

        float32 = Float32("inf")
        self.assertTrue(float32.is_special())

    def test_is_nan(self):
        float32 = Float32("1e-45")
        self.assertFalse(float32.is_nan())

        float32 = Float32("nan")
        self.assertTrue(float32.is_nan())

        float32 = Float32("inf")
        self.assertFalse(float32.is_nan())

    def test_is_inf(self):
        float32 = Float32("1e-45")
        self.assertFalse(float32.is_inf())

        float32 = Float32("nan")
        self.assertFalse(float32.is_inf())

        float32 = Float32("inf")
        self.assertTrue(float32.is_inf())

    def test_exponent(self):
        float32 = Float32("1.0")
        self.assertEqual(float32.exponent(), np.int32(-23))

        float32 = Float32("1e-45")
        self.assertEqual(float32.exponent(), np.int32(-149))

    def test_mantissa(self):
        float32 = Float32("1.0")
        self.assertEqual(float32.mantissa(), np.uint32(8388608))

        float32 = Float32("1e-45")
        self.assertEqual(float32.mantissa(), np.uint32(1))

class TestFloat64(unittest.TestCase):

    def test_to_bits(self):
        float64 = Float64("1.0")
        self.assertEqual(float64.to_bits(), np.uint64(4607182418800017408))

    def test_to_hex(self):
        float64 = Float64("1.0")
        self.assertEqual(float64.to_hex(), '3ff0000000000000')

    def test_is_denormal(self):
        float64 = Float64("1.0")
        self.assertFalse(float64.is_denormal())

        float64 = Float64("3e-324")
        self.assertTrue(float64.is_denormal())

    def test_is_special(self):
        float64 = Float64("5e-324")
        self.assertFalse(float64.is_special())

        float64 = Float64("nan")
        self.assertTrue(float64.is_special())

        float64 = Float64("inf")
        self.assertTrue(float64.is_special())

    def test_is_nan(self):
        float64 = Float64("5e-324")
        self.assertFalse(float64.is_nan())

        float64 = Float64("nan")
        self.assertTrue(float64.is_nan())

        float64 = Float64("inf")
        self.assertFalse(float64.is_nan())

    def test_is_inf(self):
        float64 = Float64("5e-324")
        self.assertFalse(float64.is_inf())

        float64 = Float64("nan")
        self.assertFalse(float64.is_inf())

        float64 = Float64("inf")
        self.assertTrue(float64.is_inf())

    def test_exponent(self):
        float64 = Float64("1.0")
        self.assertEqual(float64.exponent(), np.int32(-52))

        float64 = Float64("3e-324")
        self.assertEqual(float64.exponent(), np.int32(-1074))

    def test_mantissa(self):
        float64 = Float64("1.0")
        self.assertEqual(float64.mantissa(), np.uint64(4503599627370496))

        float64 = Float64("3e-324")
        self.assertEqual(float64.mantissa(), np.uint64(1))


# MAIN
# ----

def run_tests():
    '''Run unittest suite.'''
    unittest.main(argv=sys.argv[:1])

def create_test(test):
    '''Create conversion test table.'''

    conversion_test = tomlkit.table()
    float64 = Float64(test)
    mantissa = float64.mantissa()
    exponent = float64.exponent()
    conversion_test.add('UID', '')
    conversion_test.add('str', test)
    conversion_test.add('hex', float64.to_hex())
    conversion_test.add('int', '{}*2^{}'.format(mantissa, exponent))

    return conversion_test

def main(source, destination):
    '''Run main script.'''

    with open(source, 'rb') as fin:
        data = json.load(fin)

    # Process the tests.
    document = tomlkit.document()
    for key, tests in data.items():
        aot = tomlkit.aot()
        for test in tests:
            aot.append(create_test(test))
        document.add(key, aot)

    # Write to file.
    if destination is None:
        print(tomlkit.dumps(document), file=sys.stdout)
    else:
        with open(destination, 'w') as fout:
            print(tomlkit.dumps(document), file=fout)

if __name__ == '__main__':
    args = parser.parse_args()
    if args.test:
        run_tests()
    else:
        main(args.source, args.destination)
