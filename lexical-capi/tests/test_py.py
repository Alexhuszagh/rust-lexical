"""
    Unittests for the Python API to lexical-core.

    License
    -------

    This is free and unencumbered software released into the public domain.

    Anyone is free to copy, modify, publish, use, compile, sell, or
    distribute this software, either in source code form or as a compiled
    binary, for any purpose, commercial or non-commercial, and by any
    means.

    In jurisdictions that recognize copyright laws, the author or authors
    of this software dedicate any and all copyright interest in the
    software to the public domain. We make this dedication for the benefit
    of the public at large and to the detriment of our heirs and
    successors. We intend this dedication to be an overt act of
    relinquishment in perpetuity of all present and future rights to this
    software under copyright law.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
    IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
    OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
    ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE.

    For more information, please refer to <http://unlicense.org/>
"""

import contextlib
import ctypes
import os
import sys
import unittest

# Get path to DLL and Python wrapper.
PROJECT_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
INCLUDE_DIR = os.path.join(PROJECT_DIR, "include")
RELEASE_DIR = os.path.join(PROJECT_DIR, "target", "release")

# Change our working directory to the release directory, and our include
# path to include the Python source file.
sys.path.insert(0, INCLUDE_DIR)
os.chdir(RELEASE_DIR)
import lexical


class ConfigTests(unittest.TestCase):
    '''Test the config functions for string literals.'''

    def test_get_nan_string(self):
        self.assertEqual(lexical.get_nan_string(), 'NaN')

    def test_set_nan_string(self):
        lexical.set_nan_string('NaN')

    def test_get_inf_string(self):
        self.assertEqual(lexical.get_inf_string(), 'inf')

    def test_set_inf_string(self):
        lexical.set_inf_string('inf')

    def test_get_infinity_string(self):
        self.assertEqual(lexical.get_infinity_string(), 'infinity')

    def test_set_infinity_string(self):
        lexical.set_infinity_string('infinity')

    def test_get_exponent_default_char(self):
        self.assertEqual(lexical.get_exponent_default_char(), 'e')

    def test_set_exponent_default_char(self):
        lexical.set_exponent_default_char('e')

    def test_get_exponent_backup_char(self):
        if lexical.HAVE_RADIX:
            self.assertEqual(lexical.get_exponent_backup_char(), '^')

    def test_set_exponent_backup_char(self):
        if lexical.HAVE_RADIX:
            lexical.set_exponent_backup_char('^')

    def test_get_float_rounding(self):
        if lexical.HAVE_ROUNDING:
            self.assertEqual(lexical.get_float_rounding(), lexical.RoundingKind.NearestTieEven)

    def test_set_float_rounding(self):
        if lexical.HAVE_ROUNDING:
            lexical.set_float_rounding(lexical.RoundingKind.NearestTieEven)

    def test_number_format(self):
        if lexical.HAVE_FORMAT:
            format = lexical.NumberFormat.ignore(b'_')
            self.assertEqual(format.digit_separator, b'_')
            self.assertEqual(format.flags, lexical.NumberFormatFlags.DigitSeparatorFlagMask)
            self.assertFalse(format.required_integer_digits)
            self.assertTrue(format.integer_internal_digit_separator)

            format = lexical.NumberFormat.permissive()
            self.assertEqual(format.digit_separator, b'\x00')
            self.assertEqual(format.flags, lexical.NumberFormatFlags.Permissive)

            format = lexical.NumberFormat.standard()
            self.assertEqual(format.digit_separator, b'\x00')
            self.assertEqual(format.flags, lexical.NumberFormatFlags.Standard)

            format = lexical.NumberFormat.compile(
                digit_separator=b'_',
                no_special=True,
                integer_internal_digit_separator=True
            )
            self.assertEqual(format.digit_separator, b'_')
            self.assertTrue(format.no_special)
            self.assertTrue(format.integer_internal_digit_separator)
            self.assertTrue(format.internal_digit_separator)
            self.assertFalse(format.integer_leading_digit_separator)

            format = lexical.NumberFormat.Json
            self.assertEqual(format.digit_separator, b'\x00')
            self.assertTrue(format.required_digits)
            self.assertTrue(format.no_special)


class GlobalTests(unittest.TestCase):
    '''Test the global config variables.'''

    def test_max_size(self):
        self.assertIsInstance(lexical.I8_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I16_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I32_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I64_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.ISIZE_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U8_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U16_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U32_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.U64_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.USIZE_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.F32_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.F64_FORMATTED_SIZE, int)
        self.assertIsInstance(lexical.I8_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.I16_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.I32_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.I64_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.ISIZE_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U8_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U16_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U32_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.U64_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.USIZE_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.F32_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.F64_FORMATTED_SIZE_DECIMAL, int)
        self.assertIsInstance(lexical.BUFFER_SIZE, int)

        if lexical.HAVE_I128:
            self.assertIsInstance(lexical.I128_FORMATTED_SIZE, int)
            self.assertIsInstance(lexical.U128_FORMATTED_SIZE, int)
            self.assertIsInstance(lexical.I128_FORMATTED_SIZE_DECIMAL, int)
            self.assertIsInstance(lexical.U128_FORMATTED_SIZE_DECIMAL, int)

class ErrorTests(unittest.TestCase):
    '''Test ErrorCode and Error structures.'''

    def setUp(self):
        self.overflow = lexical.Error(lexical.ErrorCode.Overflow.value, 0)
        self.underflow = lexical.Error(lexical.ErrorCode.Underflow.value, 0)
        self.invalid_digit = lexical.Error(lexical.ErrorCode.InvalidDigit.value, 0)
        self.empty = lexical.Error(lexical.ErrorCode.Empty.value, 0)
        self.empty_mantissa = lexical.Error(lexical.ErrorCode.EmptyMantissa.value, 0)
        self.empty_exponent = lexical.Error(lexical.ErrorCode.EmptyExponent.value, 0)
        self.empty_integer = lexical.Error(lexical.ErrorCode.EmptyInteger.value, 0)
        self.empty_fraction = lexical.Error(lexical.ErrorCode.EmptyFraction.value, 0)
        self.invalid_positive_mantissa_sign = lexical.Error(lexical.ErrorCode.InvalidPositiveMantissaSign.value, 0)
        self.missing_mantissa_sign = lexical.Error(lexical.ErrorCode.MissingMantissaSign.value, 0)
        self.invalid_exponent = lexical.Error(lexical.ErrorCode.InvalidExponent.value, 0)
        self.invalid_positive_exponent_sign = lexical.Error(lexical.ErrorCode.InvalidPositiveExponentSign.value, 0)
        self.missing_exponent_sign = lexical.Error(lexical.ErrorCode.MissingExponentSign.value, 0)
        self.exponent_without_fraction = lexical.Error(lexical.ErrorCode.ExponentWithoutFraction.value, 0)
        self.invalid_leading_zeros = lexical.Error(lexical.ErrorCode.InvalidLeadingZeros.value, 0)

    def test_is_overflow(self):
        self.assertTrue(self.overflow.is_overflow())
        self.assertFalse(self.underflow.is_overflow())

    def test_is_underflow(self):
        self.assertFalse(self.overflow.is_underflow())
        self.assertTrue(self.underflow.is_underflow())

    def test_is_invalid_digit(self):
        self.assertFalse(self.overflow.is_invalid_digit())
        self.assertTrue(self.invalid_digit.is_invalid_digit())

    def test_is_empty(self):
        self.assertFalse(self.overflow.is_empty())
        self.assertTrue(self.empty.is_empty())

    def test_is_empty_mantissa(self):
        self.assertFalse(self.overflow.is_empty_mantissa())
        self.assertTrue(self.empty_mantissa.is_empty_mantissa())

    def test_is_empty_exponent(self):
        self.assertFalse(self.overflow.is_empty_exponent())
        self.assertTrue(self.empty_exponent.is_empty_exponent())

    def test_is_empty_integer(self):
        self.assertFalse(self.overflow.is_empty_integer())
        self.assertTrue(self.empty_integer.is_empty_integer())

    def test_is_empty_fraction(self):
        self.assertFalse(self.overflow.is_empty_fraction())
        self.assertTrue(self.empty_fraction.is_empty_fraction())

    def test_is_invalid_positive_mantissa_sign(self):
        self.assertFalse(self.overflow.is_invalid_positive_mantissa_sign())
        self.assertTrue(self.invalid_positive_mantissa_sign.is_invalid_positive_mantissa_sign())

    def test_is_missing_mantissa_sign(self):
        self.assertFalse(self.overflow.is_missing_mantissa_sign())
        self.assertTrue(self.missing_mantissa_sign.is_missing_mantissa_sign())

    def test_is_invalid_exponent(self):
        self.assertFalse(self.overflow.is_invalid_exponent())
        self.assertTrue(self.invalid_exponent.is_invalid_exponent())

    def test_is_invalid_positive_exponent_sign(self):
        self.assertFalse(self.overflow.is_invalid_positive_exponent_sign())
        self.assertTrue(self.invalid_positive_exponent_sign.is_invalid_positive_exponent_sign())

    def test_is_missing_exponent_sign(self):
        self.assertFalse(self.overflow.is_missing_exponent_sign())
        self.assertTrue(self.missing_exponent_sign.is_missing_exponent_sign())

    def test_is_exponent_without_fraction(self):
        self.assertFalse(self.overflow.is_exponent_without_fraction())
        self.assertTrue(self.exponent_without_fraction.is_exponent_without_fraction())

    def test_is_invalid_leading_zeros(self):
        self.assertFalse(self.overflow.is_invalid_leading_zeros())
        self.assertTrue(self.invalid_leading_zeros.is_invalid_leading_zeros())


class ResultTests(unittest.TestCase):
    '''Test complete and partial result types.'''

    def setUp(self):
        self.error = lexical.Error(lexical.ErrorCode.Overflow.value, 0)

    def _complete_test(self, cls, value_type):
        success_union = cls.union_type(value=value_type(1))
        error_union = cls.union_type(error=self.error)
        success = cls(lexical.ResultTag.Ok.value, success_union)
        error = cls(lexical.ResultTag.Err.value, error_union)
        self.assertEqual(success.into(), value_type(1))
        with self.assertRaises(lexical.LexicalError):
            error.into()

    def _partial_test(self, cls, value_type):
        tuple_type = cls.union_type.value_type
        success_union = cls.union_type(value=tuple_type(value_type(1), 0))
        error_union = cls.union_type(error=self.error)
        success = cls(lexical.ResultTag.Ok.value, success_union)
        error = cls(lexical.ResultTag.Err.value, error_union)
        self.assertEqual(success.into(), (value_type(1), 0))
        with self.assertRaises(lexical.LexicalError):
            error.into()

    def test_result_i8(self):
        self._complete_test(lexical.ResultI8, int)

    def test_result_i16(self):
        self._complete_test(lexical.ResultI16, int)

    def test_result_i32(self):
        self._complete_test(lexical.ResultI32, int)

    def test_result_i64(self):
        self._complete_test(lexical.ResultI64, int)

    def test_result_isize(self):
        self._complete_test(lexical.ResultIsize, int)

    def test_result_u8(self):
        self._complete_test(lexical.ResultU8, int)

    def test_result_u16(self):
        self._complete_test(lexical.ResultU16, int)

    def test_result_u32(self):
        self._complete_test(lexical.ResultU32, int)

    def test_result_u64(self):
        self._complete_test(lexical.ResultU64, int)

    def test_result_usize(self):
        self._complete_test(lexical.ResultUsize, int)

    def test_result_f32(self):
        self._complete_test(lexical.ResultF32, float)

    def test_result_f64(self):
        self._complete_test(lexical.ResultF64, float)

    def test_partial_result_i8(self):
        self._partial_test(lexical.PartialResultI8, int)

    def test_partial_result_i16(self):
        self._partial_test(lexical.PartialResultI16, int)

    def test_partial_result_i32(self):
        self._partial_test(lexical.PartialResultI32, int)

    def test_partial_result_i64(self):
        self._partial_test(lexical.PartialResultI64, int)

    def test_partial_result_isize(self):
        self._partial_test(lexical.PartialResultIsize, int)

    def test_partial_result_u8(self):
        self._partial_test(lexical.PartialResultU8, int)

    def test_partial_result_u16(self):
        self._partial_test(lexical.PartialResultU16, int)

    def test_partial_result_u32(self):
        self._partial_test(lexical.PartialResultU32, int)

    def test_partial_result_u64(self):
        self._partial_test(lexical.PartialResultU64, int)

    def test_partial_result_usize(self):
        self._partial_test(lexical.PartialResultUsize, int)

    def test_partial_result_f32(self):
        self._partial_test(lexical.PartialResultF32, float)

    def test_partial_result_f64(self):
        self._partial_test(lexical.PartialResultF64, float)


class ToStringTests(unittest.TestCase):
    '''Test number-to-string conversion routines.'''

    def _test_integer(self, cb):
        self.assertEqual(cb(10), '10')

    def _test_integer_radix(self, cb):
        self.assertEqual(cb(10, 2), '1010')
        self.assertEqual(cb(10, 16), 'A')
        self.assertEqual(cb(10, 10), '10')

    def _test_float(self, cb):
        self.assertEqual(cb(10.5), '10.5')

    def _test_float_radix(self, cb):
        self.assertEqual(cb(10.5, 2), '1010.1')
        self.assertEqual(cb(10.5, 16), 'A.8')
        self.assertEqual(cb(10.5, 10), '10.5')

    def test_i8toa(self):
        self._test_integer(lexical.i8toa)

    def test_i16toa(self):
        self._test_integer(lexical.i16toa)

    def test_i32toa(self):
        self._test_integer(lexical.i32toa)

    def test_i64toa(self):
        self._test_integer(lexical.i64toa)

    def test_isizetoa(self):
        self._test_integer(lexical.isizetoa)

    def test_u8toa(self):
        self._test_integer(lexical.u8toa)

    def test_u16toa(self):
        self._test_integer(lexical.u16toa)

    def test_u32toa(self):
        self._test_integer(lexical.u32toa)

    def test_u64toa(self):
        self._test_integer(lexical.u64toa)

    def test_usizetoa(self):
        self._test_integer(lexical.usizetoa)

    def test_f32toa(self):
        self._test_float(lexical.f32toa)

    def test_f64toa(self):
        self._test_float(lexical.f64toa)

    def test_i8toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.i8toa_radix)

    def test_i16toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.i16toa_radix)

    def test_i32toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.i32toa_radix)

    def test_i64toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.i64toa_radix)

    def test_isizetoa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.isizetoa_radix)

    def test_u8toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.u8toa_radix)

    def test_u16toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.u16toa_radix)

    def test_u32toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.u32toa_radix)

    def test_u64toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.u64toa_radix)

    def test_usizetoa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_integer_radix(lexical.usizetoa_radix)

    def test_f32toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_float_radix(lexical.f32toa_radix)

    def test_f64toa_radix(self):
        if lexical.HAVE_RADIX:
            self._test_float_radix(lexical.f64toa_radix)


class ParseTests(unittest.TestCase):
    '''Test string-to-number conversion routines.'''

    def _complete_test(self, callback, value_type, *args):
        self.assertEqual(callback('10', *args), value_type(10))
        with self.assertRaises(lexical.LexicalError):
            callback('10a', *args)
        with self.assertRaises(lexical.LexicalError):
            callback('', *args)

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('10.5', *args), value_type(10.5))
            self.assertEqual(callback('10e5', *args), value_type(10e5))
            with self.assertRaises(lexical.LexicalError):
                callback('.', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', *args)

    def _complete_radix_test(self, callback, value_type, *args):
        self.assertEqual(callback('1010', 2, *args), value_type(10))
        self.assertEqual(callback('10', 10, *args), value_type(10))
        self.assertEqual(callback('A', 16, *args), value_type(10))
        with self.assertRaises(lexical.LexicalError):
            callback('10102', 2, *args)
        with self.assertRaises(lexical.LexicalError):
            callback('10a', 10, *args)
        with self.assertRaises(lexical.LexicalError):
            callback('AG', 16, *args)
        with self.assertRaises(lexical.LexicalError):
            callback('', 10, *args)

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('1010.1', 2, *args), value_type(10.5))
            self.assertEqual(callback('10.5', 10, *args), value_type(10.5))
            self.assertEqual(callback('A.8', 16, *args), value_type(10.5))
            self.assertEqual(callback('10e5', 10, *args), value_type(10e5))
            with self.assertRaises(lexical.LexicalError):
                callback('.', 10, *args)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', 10, *args)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', 10, *args)

    def _partial_test(self, callback, value_type, *args):
        self.assertEqual(callback('10', *args), (value_type(10), 2))
        self.assertEqual(callback('10a', *args), (value_type(10), 2))
        with self.assertRaises(lexical.LexicalError):
            callback('', *args)

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('10.5', *args), (value_type(10.5), 4))
            self.assertEqual(callback('10e5', *args), (value_type(10e5), 4))
            with self.assertRaises(lexical.LexicalError):
                callback('.', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', *args)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', *args)

    def _partial_radix_test(self, callback, value_type, *args):
        self.assertEqual(callback('1010', 2, *args), (value_type(10), 4))
        self.assertEqual(callback('10', 10, *args), (value_type(10), 2))
        self.assertEqual(callback('A', 16, *args), (value_type(10), 1))
        self.assertEqual(callback('10102', 2, *args), (value_type(10), 4))
        self.assertEqual(callback('10a', 10, *args), (value_type(10), 2))
        self.assertEqual(callback('AG', 16, *args), (value_type(10), 1))
        with self.assertRaises(lexical.LexicalError):
            callback('', 10, *args)

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('1010.1', 2, *args), (value_type(10.5), 6))
            self.assertEqual(callback('10.5', 10, *args), (value_type(10.5), 4))
            self.assertEqual(callback('A.8', 16, *args), (value_type(10.5), 3))
            self.assertEqual(callback('10e5', 10, *args), (value_type(10e5), 4))
            with self.assertRaises(lexical.LexicalError):
                callback('.', 10, *args)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', 10, *args)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', 10, *args)

    def test_atoi8(self):
        self._complete_test(lexical.atoi8, int)

    def test_atoi16(self):
        self._complete_test(lexical.atoi16, int)

    def test_atoi32(self):
        self._complete_test(lexical.atoi32, int)

    def test_atoi64(self):
        self._complete_test(lexical.atoi64, int)

    def test_atoisize(self):
        self._complete_test(lexical.atoisize, int)

    def test_atou8(self):
        self._complete_test(lexical.atou8, int)

    def test_atou16(self):
        self._complete_test(lexical.atou16, int)

    def test_atou32(self):
        self._complete_test(lexical.atou32, int)

    def test_atou64(self):
        self._complete_test(lexical.atou64, int)

    def test_atousize(self):
        self._complete_test(lexical.atousize, int)

    def test_atof32(self):
        self._complete_test(lexical.atof32, float)

    def test_atof64(self):
        self._complete_test(lexical.atof64, float)

    def test_atof32_lossy(self):
        self._complete_test(lexical.atof32_lossy, float)

    def test_atof64_lossy(self):
        self._complete_test(lexical.atof64_lossy, float)

    def test_atoi8_partial(self):
        self._partial_test(lexical.atoi8_partial, int)

    def test_atoi16_partial(self):
        self._partial_test(lexical.atoi16_partial, int)

    def test_atoi32_partial(self):
        self._partial_test(lexical.atoi32_partial, int)

    def test_atoi64_partial(self):
        self._partial_test(lexical.atoi64_partial, int)

    def test_atoisize_partial(self):
        self._partial_test(lexical.atoisize_partial, int)

    def test_atou8_partial(self):
        self._partial_test(lexical.atou8_partial, int)

    def test_atou16_partial(self):
        self._partial_test(lexical.atou16_partial, int)

    def test_atou32_partial(self):
        self._partial_test(lexical.atou32_partial, int)

    def test_atou64_partial(self):
        self._partial_test(lexical.atou64_partial, int)

    def test_atousize_partial(self):
        self._partial_test(lexical.atousize_partial, int)

    def test_atof32_partial(self):
        self._partial_test(lexical.atof32_partial, float)

    def test_atof64_partial(self):
        self._partial_test(lexical.atof64_partial, float)

    def test_atof32_partial_lossy(self):
        self._partial_test(lexical.atof32_partial_lossy, float)

    def test_atof64_partial_lossy(self):
        self._partial_test(lexical.atof64_partial_lossy, float)

    def test_atoi8_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atoi8_radix, int)

    def test_atoi16_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atoi16_radix, int)

    def test_atoi32_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atoi32_radix, int)

    def test_atoi64_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atoi64_radix, int)

    def test_atoisize_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atoisize_radix, int)

    def test_atou8_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atou8_radix, int)

    def test_atou16_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atou16_radix, int)

    def test_atou32_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atou32_radix, int)

    def test_atou64_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atou64_radix, int)

    def test_atousize_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atousize_radix, int)

    def test_atof32_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atof32_radix, float)

    def test_atof64_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atof64_radix, float)

    def test_atof32_lossy_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atof32_lossy_radix, float)

    def test_atof64_lossy_radix(self):
        if lexical.HAVE_RADIX:
            self._complete_radix_test(lexical.atof64_lossy_radix, float)

    def test_atoi8_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atoi8_partial_radix, int)

    def test_atoi16_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atoi16_partial_radix, int)

    def test_atoi32_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atoi32_partial_radix, int)

    def test_atoi64_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atoi64_partial_radix, int)

    def test_atoisize_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atoisize_partial_radix, int)

    def test_atou8_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atou8_partial_radix, int)

    def test_atou16_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atou16_partial_radix, int)

    def test_atou32_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atou32_partial_radix, int)

    def test_atou64_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atou64_partial_radix, int)

    def test_atousize_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atousize_partial_radix, int)

    def test_atof32_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atof32_partial_radix, float)

    def test_atof64_partial_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atof64_partial_radix, float)

    def test_atof32_partial_lossy_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atof32_partial_lossy_radix, float)

    def test_atof64_partial_lossy_radix(self):
        if lexical.HAVE_RADIX:
            self._partial_radix_test(lexical.atof64_partial_lossy_radix, float)

    def test_atoi8_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atoi8_format, int, lexical.NumberFormat.RustString)

    def test_atoi16_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atoi16_format, int, lexical.NumberFormat.RustString)

    def test_atoi32_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atoi32_format, int, lexical.NumberFormat.RustString)

    def test_atoi64_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atoi64_format, int, lexical.NumberFormat.RustString)

    def test_atoisize_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atoisize_format, int, lexical.NumberFormat.RustString)

    def test_atou8_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atou8_format, int, lexical.NumberFormat.RustString)

    def test_atou16_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atou16_format, int, lexical.NumberFormat.RustString)

    def test_atou32_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atou32_format, int, lexical.NumberFormat.RustString)

    def test_atou64_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atou64_format, int, lexical.NumberFormat.RustString)

    def test_atousize_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atousize_format, int, lexical.NumberFormat.RustString)

    def test_atof32_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atof32_format, float, lexical.NumberFormat.RustString)

    def test_atof64_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atof64_format, float, lexical.NumberFormat.RustString)

    def test_atof32_lossy_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atof32_lossy_format, float, lexical.NumberFormat.RustString)

    def test_atof64_lossy_format(self):
        if lexical.HAVE_FORMAT:
            self._complete_test(lexical.atof64_lossy_format, float, lexical.NumberFormat.RustString)

    def test_atoi8_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atoi8_partial_format, int, lexical.NumberFormat.RustString)

    def test_atoi16_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atoi16_partial_format, int, lexical.NumberFormat.RustString)

    def test_atoi32_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atoi32_partial_format, int, lexical.NumberFormat.RustString)

    def test_atoi64_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atoi64_partial_format, int, lexical.NumberFormat.RustString)

    def test_atoisize_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atoisize_partial_format, int, lexical.NumberFormat.RustString)

    def test_atou8_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atou8_partial_format, int, lexical.NumberFormat.RustString)

    def test_atou16_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atou16_partial_format, int, lexical.NumberFormat.RustString)

    def test_atou32_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atou32_partial_format, int, lexical.NumberFormat.RustString)

    def test_atou64_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atou64_partial_format, int, lexical.NumberFormat.RustString)

    def test_atousize_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atousize_partial_format, int, lexical.NumberFormat.RustString)

    def test_atof32_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atof32_partial_format, float, lexical.NumberFormat.RustString)

    def test_atof64_partial_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atof64_partial_format, float, lexical.NumberFormat.RustString)

    def test_atof32_partial_lossy_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atof32_partial_lossy_format, float, lexical.NumberFormat.RustString)

    def test_atof64_partial_lossy_format(self):
        if lexical.HAVE_FORMAT:
            self._partial_test(lexical.atof64_partial_lossy_format, float, lexical.NumberFormat.RustString)

    def test_atoi8_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atoi8_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoi16_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atoi16_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoi32_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atoi32_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoi64_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atoi64_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoisize_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atoisize_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou8_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atou8_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou16_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atou16_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou32_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atou32_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou64_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atou64_format_radix, int, lexical.NumberFormat.RustString)

    def test_atousize_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atousize_format_radix, int, lexical.NumberFormat.RustString)

    def test_atof32_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atof32_format_radix, float, lexical.NumberFormat.RustString)

    def test_atof64_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atof64_format_radix, float, lexical.NumberFormat.RustString)

    def test_atof32_lossy_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atof32_lossy_format_radix, float, lexical.NumberFormat.RustString)

    def test_atof64_lossy_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._complete_radix_test(lexical.atof64_lossy_format_radix, float, lexical.NumberFormat.RustString)

    def test_atoi8_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atoi8_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoi16_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atoi16_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoi32_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atoi32_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoi64_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atoi64_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atoisize_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atoisize_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou8_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atou8_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou16_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atou16_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou32_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atou32_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atou64_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atou64_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atousize_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atousize_partial_format_radix, int, lexical.NumberFormat.RustString)

    def test_atof32_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atof32_partial_format_radix, float, lexical.NumberFormat.RustString)

    def test_atof64_partial_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atof64_partial_format_radix, float, lexical.NumberFormat.RustString)

    def test_atof32_partial_lossy_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atof32_partial_lossy_format_radix, float, lexical.NumberFormat.RustString)

    def test_atof64_partial_lossy_format_radix(self):
        if lexical.HAVE_RADIX and lexical.HAVE_FORMAT:
            self._partial_radix_test(lexical.atof64_partial_lossy_format_radix, float, lexical.NumberFormat.RustString)


if __name__ == '__main__':
   unittest.main()
