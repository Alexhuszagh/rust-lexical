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
import math
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

    def test_features(self):
        self.assertIsInstance(lexical.HAVE_FORMAT, bool)
        self.assertIsInstance(lexical.HAVE_RADIX, bool)
        self.assertIsInstance(lexical.HAVE_ROUNDING, bool)
        self.assertIsInstance(lexical.HAVE_I128, bool)


class RoundingKindTests(unittest.TestCase):
    '''Test the rounding kind enumeration.'''

    def test_kind(self):
        self.assertEqual(lexical.RoundingKind.NearestTieEven.value, 0)


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


if lexical.HAVE_I128:
    class Int128Tests(unittest.TestCase):
        '''Test our wrappers for 128-bit integers.'''

        def test_c_uint128(self):
            i128 = lexical.c_uint128(128)
            self.assertEqual(i128.value, 128)

            i128 = lexical.c_uint128(2**128)
            self.assertEqual(i128.value, 0)

            i128 = lexical.c_uint128(2**127)
            self.assertEqual(i128.value, 2**127)

        def test_c_int128(self):
            i128 = lexical.c_int128(128)
            self.assertEqual(i128.value, 128)

            i128 = lexical.c_int128(2**128)
            self.assertEqual(i128.value, 0)

            i128 = lexical.c_int128(2**127)
            self.assertEqual(i128.value, -2**127)


class OptionTests(unittest.TestCase):
    '''Test the Option structure.'''

    def _complete_test(self, cls, value):
        some = cls(lexical.OptionTag.Some.value, value)
        nil = cls(lexical.OptionTag.Nil.value, value)
        self.assertEqual(some.into(), value)
        self.assertTrue(some.is_some)
        self.assertFalse(nil.is_some)
        with self.assertRaises(ValueError):
            nil.into()

        self.assertEqual(cls.of(value), some)
        self.assertEqual(cls.of(None), nil)
        self.assertNotEqual(some, nil)
        self.assertNotEqual(nil, some)

    def test_option_number_format(self):
        self._complete_test(lexical.OptionNumberFormat, lexical.NumberFormat.Permissive)


class NumberFormatTests(unittest.TestCase):
    '''Test NumberFormatFlags and the NumberFormat structure.'''

    def test_error(self):
        builder = lexical.NumberFormatBuilder()
        builder.decimal_point = b'e'
        builder.exponent_default = b'e'
        with self.assertRaises(ValueError):
            builder.build()

    def test_rebuild(self):
        standard = lexical.NumberFormat.Standard
        builder = standard.rebuild()
        builder.decimal_point = b','
        format = builder.build()
        self.assertEqual(format.flags, standard.flags)
        self.assertEqual(format.decimal_point, b',')
        self.assertEqual(format.exponent_default, b'e')
        self.assertEqual(format.exponent_backup, b'^')

    if lexical.HAVE_FORMAT:
        def test_json(self):
            json = lexical.NumberFormat.Json
            self.assertEqual(json.digit_separator, b'\x00')
            self.assertEqual(json.decimal_point, b'.')
            self.assertEqual(json.exponent_default, b'e')
            self.assertEqual(json.exponent_backup, b'^')
            self.assertTrue(json.required_integer_digits)
            self.assertTrue(json.required_fraction_digits)
            self.assertTrue(json.required_exponent_digits)
            self.assertTrue(json.required_digits)
            self.assertTrue(json.no_positive_mantissa_sign)
            self.assertFalse(json.no_exponent_notation)
            self.assertFalse(json.no_positive_exponent_sign)
            self.assertFalse(json.required_exponent_sign)
            self.assertFalse(json.no_exponent_without_fraction)
            self.assertTrue(json.no_special)
            self.assertFalse(json.case_sensitive_special)
            self.assertTrue(json.no_integer_leading_zeros)
            self.assertTrue(json.no_float_leading_zeros)
            self.assertFalse(json.integer_internal_digit_separator)
            self.assertFalse(json.fraction_internal_digit_separator)
            self.assertFalse(json.exponent_internal_digit_separator)
            self.assertFalse(json.internal_digit_separator)
            self.assertFalse(json.integer_leading_digit_separator)
            self.assertFalse(json.fraction_leading_digit_separator)
            self.assertFalse(json.exponent_leading_digit_separator)
            self.assertFalse(json.leading_digit_separator)
            self.assertFalse(json.integer_trailing_digit_separator)
            self.assertFalse(json.fraction_trailing_digit_separator)
            self.assertFalse(json.exponent_trailing_digit_separator)
            self.assertFalse(json.trailing_digit_separator)
            self.assertFalse(json.integer_consecutive_digit_separator)
            self.assertFalse(json.fraction_consecutive_digit_separator)
            self.assertFalse(json.exponent_consecutive_digit_separator)
            self.assertFalse(json.consecutive_digit_separator)
            self.assertFalse(json.special_digit_separator)


class ParseIntegerOptionsTests(unittest.TestCase):
    '''Test ParseIntegerOptions and ParseIntegerOptionsBuilder.'''

    def test_builder(self):
        builder = lexical.ParseIntegerOptionsBuilder()
        self.assertEqual(builder.radix, 10)
        self.assertEqual(builder.format, None)
        options = builder.build()
        self.assertEqual(options.radix, 10)
        self.assertEqual(options.format, None)

        standard = lexical.NumberFormat.Standard
        builder.format = standard
        self.assertEqual(builder.format, standard)
        options = builder.build()
        self.assertEqual(options.radix, 10)
        self.assertEqual(options.format, standard)

        if lexical.HAVE_RADIX:
            builder.radix = 2
            self.assertEqual(builder.radix, 2)
            self.assertEqual(builder.build().radix, 2)

            builder.radix = 37
            self.assertEqual(builder.radix, 37)
            with self.assertRaises(ValueError):
                builder.build()

        builder = options.rebuild()
        self.assertEqual(builder.build(), options)


class ParseFloatOptionsTests(unittest.TestCase):
    '''Test ParseFloatOptions and ParseFloatOptionsBuilder.'''

    def test_builder(self):
        standard = lexical.NumberFormat.Standard
        builder = lexical.ParseFloatOptionsBuilder()
        self.assertEqual(builder.radix, 10)
        options = builder.build()
        self.assertEqual(options.radix, 10)
        self.assertEqual(builder.format, standard)
        self.assertEqual(options.nan_string, 'NaN')

        builder.nan_string = 'nan'
        options = builder.build()
        self.assertEqual(options.nan_string, 'nan')

        with self.assertRaises(TypeError):
            builder.format = None

        builder.inf_string = 'INF'
        options = builder.build()
        self.assertEqual(options.inf_string, 'INF')

        builder.infinity_string = 'INF'
        options = builder.build()
        self.assertEqual(options.infinity_string, 'INF')

        builder.nan_string = 'i'
        with self.assertRaises(ValueError):
            builder.build()

        builder = options.rebuild()
        self.assertEqual(builder.build(), options)
        builder.infinity_string = 'i'
        with self.assertRaises(ValueError):
            builder.build()

        builder = options.rebuild()
        self.assertEqual(builder.build(), options)
        builder.inf_string = 'i'
        self.assertEqual(builder.build().inf_string, 'i')

        if lexical.HAVE_RADIX:
            builder.radix = 2
            self.assertEqual(builder.radix, 2)
            self.assertEqual(builder.build().radix, 2)

            builder.radix = 37
            self.assertEqual(builder.radix, 37)
            with self.assertRaises(ValueError):
                builder.build()


class WriteIntegerOptionsTests(unittest.TestCase):
    '''Test WriteIntegerOptions and WriteIntegerOptionsBuilder.'''

    def test_builder(self):
        builder = lexical.WriteIntegerOptionsBuilder()
        self.assertEqual(builder.radix, 10)
        options = builder.build()
        self.assertEqual(options.radix, 10)

        if lexical.HAVE_RADIX:
            builder.radix = 2
            self.assertEqual(builder.radix, 2)
            self.assertEqual(builder.build().radix, 2)

            builder.radix = 37
            self.assertEqual(builder.radix, 37)
            with self.assertRaises(ValueError):
                builder.build()

        builder = options.rebuild()
        self.assertEqual(builder.build(), options)


class WriteFloatOptionsTests(unittest.TestCase):
    '''Test WriteFloatOptions and WriteFloatOptionsBuilder.'''

    def test_builder(self):
        standard = lexical.NumberFormat.Standard
        builder = lexical.WriteFloatOptionsBuilder()
        self.assertEqual(builder.radix, 10)
        self.assertEqual(builder.format, None)
        options = builder.build()
        self.assertEqual(options.radix, 10)
        self.assertEqual(options.nan_string, 'NaN')

        builder.nan_string = 'nan'
        options = builder.build()
        self.assertEqual(options.nan_string, 'nan')

        builder.format = standard
        options = builder.build()
        self.assertEqual(builder.format, standard)

        builder.format = None
        options = builder.build()
        self.assertEqual(builder.format, None)

        builder.inf_string = 'INF'
        options = builder.build()
        self.assertEqual(options.inf_string, 'INF')

        builder.nan_string = 'i'
        with self.assertRaises(ValueError):
            builder.build()

        builder = options.rebuild()
        self.assertEqual(builder.build(), options)

        if lexical.HAVE_RADIX:
            builder.radix = 2
            self.assertEqual(builder.radix, 2)
            self.assertEqual(builder.build().radix, 2)

            builder.radix = 37
            self.assertEqual(builder.radix, 37)
            with self.assertRaises(ValueError):
                builder.build()


class TupleTests(unittest.TestCase):
    '''Test the pair-wise Tuple structure.'''

    def test_tuple(self):
        value = lexical.PartialTupleI8(8, 255)
        self.assertEqual(value.into(), (8, 255))
        self.assertEqual(lexical.PartialTupleI8.of((8, 255)), value)


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

    def _complete_test(self, cls):
        success_union = cls.union_type(_value=1)
        error_union = cls.union_type(_error=self.error)
        success = cls(lexical.ResultTag.Ok.value, success_union)
        error = cls(lexical.ResultTag.Err.value, error_union)
        self.assertEqual(success.into(), 1)
        self.assertTrue(success.is_ok)
        self.assertFalse(error.is_ok)
        with self.assertRaises(lexical.LexicalError):
            error.into()

        self.assertEqual(cls.of(1), success)
        self.assertEqual(cls.of(self.error), error)
        self.assertNotEqual(success, error)
        self.assertNotEqual(error, success)

    def _partial_test(self, cls):
        tuple_type = cls.union_type.value_type
        success_union = cls.union_type(_value=tuple_type(1, 0))
        error_union = cls.union_type(_error=self.error)
        success = cls(lexical.ResultTag.Ok.value, success_union)
        error = cls(lexical.ResultTag.Err.value, error_union)
        self.assertEqual(success.into(), (1, 0))
        self.assertTrue(success.is_ok)
        self.assertFalse(error.is_ok)
        with self.assertRaises(lexical.LexicalError):
            error.into()

        self.assertEqual(cls.of((1, 0)), success)
        self.assertEqual(cls.of(self.error), error)
        self.assertNotEqual(success, error)
        self.assertNotEqual(error, success)

    def test_result_i8(self):
        self._complete_test(lexical.ResultI8)

    def test_result_i16(self):
        self._complete_test(lexical.ResultI16)

    def test_result_i32(self):
        self._complete_test(lexical.ResultI32)

    def test_result_i64(self):
        self._complete_test(lexical.ResultI64)

    def test_result_isize(self):
        self._complete_test(lexical.ResultIsize)

    def test_result_u8(self):
        self._complete_test(lexical.ResultU8)

    def test_result_u16(self):
        self._complete_test(lexical.ResultU16)

    def test_result_u32(self):
        self._complete_test(lexical.ResultU32)

    def test_result_u64(self):
        self._complete_test(lexical.ResultU64)

    def test_result_usize(self):
        self._complete_test(lexical.ResultUsize)

    def test_result_f32(self):
        self._complete_test(lexical.ResultF32)

    def test_result_f64(self):
        self._complete_test(lexical.ResultF64)

    def test_partial_result_i8(self):
        self._partial_test(lexical.PartialResultI8)

    def test_partial_result_i16(self):
        self._partial_test(lexical.PartialResultI16)

    def test_partial_result_i32(self):
        self._partial_test(lexical.PartialResultI32)

    def test_partial_result_i64(self):
        self._partial_test(lexical.PartialResultI64)

    def test_partial_result_isize(self):
        self._partial_test(lexical.PartialResultIsize)

    def test_partial_result_u8(self):
        self._partial_test(lexical.PartialResultU8)

    def test_partial_result_u16(self):
        self._partial_test(lexical.PartialResultU16)

    def test_partial_result_u32(self):
        self._partial_test(lexical.PartialResultU32)

    def test_partial_result_u64(self):
        self._partial_test(lexical.PartialResultU64)

    def test_partial_result_usize(self):
        self._partial_test(lexical.PartialResultUsize)

    def test_partial_result_f32(self):
        self._partial_test(lexical.PartialResultF32)

    def test_partial_result_f64(self):
        self._partial_test(lexical.PartialResultF64)


class ToStringTests(unittest.TestCase):
    '''Test number-to-string conversion routines.'''

    def _test_integer(self, cb):
        self.assertEqual(cb(10), '10')

    def _test_integer_options(self, cb):
        opt10 = lexical.WriteIntegerOptions.decimal()
        self.assertEqual(cb(10, opt10), '10')

        if lexical.HAVE_RADIX:
            opt2 = lexical.WriteIntegerOptions.binary()
            self.assertEqual(cb(10, opt2), '1010')

            opt16 = lexical.WriteIntegerOptions.hexadecimal()
            self.assertEqual(cb(10, opt16), 'A')

    def _test_float(self, cb):
        self.assertEqual(cb(10.5), '10.5')

    def _test_float_options(self, cb):
        opt10 = lexical.WriteFloatOptions.decimal()
        self.assertEqual(cb(10.0, opt10), '10.0')
        self.assertEqual(cb(10.5, opt10), '10.5')

        builder = opt10.rebuild()
        builder.trim_floats = True
        opt_trim = builder.build()
        self.assertEqual(cb(10.0, opt_trim), '10')
        self.assertEqual(cb(10.5, opt_trim), '10.5')

        if lexical.HAVE_RADIX:
            opt2 = lexical.WriteFloatOptions.binary()
            self.assertEqual(cb(10.5, opt2), '1010.1')

            opt16 = lexical.WriteFloatOptions.hexadecimal()
            self.assertEqual(cb(10.5, opt16), 'A.8')

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

    if lexical.HAVE_I128:
        def test_i128toa(self):
            self._test_integer(lexical.i128toa)

        def test_u128toa(self):
            self._test_integer(lexical.u128toa)

    def test_i8toa_options(self):
        self._test_integer_options(lexical.i8toa_with_options)

    def test_i16toa_options(self):
        self._test_integer_options(lexical.i16toa_with_options)

    def test_i32toa_options(self):
        self._test_integer_options(lexical.i32toa_with_options)

    def test_i64toa_options(self):
        self._test_integer_options(lexical.i64toa_with_options)

    def test_isizetoa_options(self):
        self._test_integer_options(lexical.isizetoa_with_options)

    def test_u8toa_options(self):
        self._test_integer_options(lexical.u8toa_with_options)

    def test_u16toa_options(self):
        self._test_integer_options(lexical.u16toa_with_options)

    def test_u32toa_options(self):
        self._test_integer_options(lexical.u32toa_with_options)

    def test_u64toa_options(self):
        self._test_integer_options(lexical.u64toa_with_options)

    def test_usizetoa_options(self):
        self._test_integer_options(lexical.usizetoa_with_options)

    def test_f32toa_options(self):
        self._test_float_options(lexical.f32toa_with_options)

    def test_f64toa_options(self):
        self._test_float_options(lexical.f64toa_with_options)

    if lexical.HAVE_I128:
        def test_i128toa_options(self):
            self._test_integer_options(lexical.i128toa_with_options)

        def test_u128toa_options(self):
            self._test_integer_options(lexical.u128toa_with_options)

class ParseTests(unittest.TestCase):
    '''Test string-to-number conversion routines.'''

    def _complete_test(self, callback, value_type):
        self.assertEqual(callback('10'), 10)
        with self.assertRaises(lexical.LexicalError):
            callback('10a')
        with self.assertRaises(lexical.LexicalError):
            callback('')

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('10.5'), 10.5)
            self.assertEqual(callback('10e5'), 10e5)
            with self.assertRaises(lexical.LexicalError):
                callback('.')
            with self.assertRaises(lexical.LexicalError):
                callback('e5')
            with self.assertRaises(lexical.LexicalError):
                callback('10e+')

    def _complete_options_test(self, callback, value_type, options_type):
        opt10 = options_type.decimal()
        self.assertEqual(callback('10', opt10), 10)
        with self.assertRaises(lexical.LexicalError):
            callback('10a', opt10)
        with self.assertRaises(lexical.LexicalError):
            callback('', opt10)

        if lexical.HAVE_RADIX:
            opt2 = options_type.binary()
            self.assertEqual(callback('1010', opt2), 10)
            with self.assertRaises(lexical.LexicalError):
                callback('10102', opt2)

            opt16 = options_type.hexadecimal()
            self.assertEqual(callback('A', opt16), 10)
            with self.assertRaises(lexical.LexicalError):
                callback('AG', opt2)

        if issubclass(value_type, float):
            # Specialized tests for floats
            self.assertTrue(math.isnan(callback('nan', opt10)))
            self.assertTrue(math.isinf(callback('inf', opt10)))
            self.assertTrue(math.isinf(callback('Infinity', opt10)))
            self.assertEqual(callback('10.5', opt10), 10.5)
            self.assertEqual(callback('10e5', opt10), 10e5)
            with self.assertRaises(lexical.LexicalError):
                callback('.', opt10)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', opt10)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', opt10)

            if lexical.HAVE_RADIX:
                self.assertEqual(callback('1010.1', opt2), 10.5)
                self.assertEqual(callback('A.8', opt16), 10.5)

    def _partial_test(self, callback, value_type):
        self.assertEqual(callback('10'), (10, 2))
        self.assertEqual(callback('10a'), (10, 2))
        with self.assertRaises(lexical.LexicalError):
            callback('')

        if issubclass(value_type, float):
            # Specialized tests for floats.
            self.assertEqual(callback('10.5'), (10.5, 4))
            self.assertEqual(callback('10e5'), (10e5, 4))
            with self.assertRaises(lexical.LexicalError):
                callback('.')
            with self.assertRaises(lexical.LexicalError):
                callback('e5')
            with self.assertRaises(lexical.LexicalError):
                callback('10e+')

    def _partial_options_test(self, callback, value_type, options_type):
        opt10 = options_type.decimal()
        self.assertEqual(callback('10', opt10), (10, 2))
        self.assertEqual(callback('10a', opt10), (10, 2))
        with self.assertRaises(lexical.LexicalError):
            callback('', opt10)

        if lexical.HAVE_RADIX:
            opt2 = options_type.binary()
            self.assertEqual(callback('1010', opt2), (10, 4))
            self.assertEqual(callback('10102', opt2), (10, 4))

            opt16 = options_type.hexadecimal()
            self.assertEqual(callback('A', opt16), (10, 1))
            self.assertEqual(callback('AG', opt16), (10, 1))

        if issubclass(value_type, float):
            # Specialized tests for floats
            self.assertTrue(math.isnan(callback('nan', opt10)[0]))
            self.assertTrue(math.isinf(callback('inf', opt10)[0]))
            self.assertTrue(math.isinf(callback('Infinity', opt10)[0]))
            self.assertEqual(callback('10.5', opt10), (10.5, 4))
            self.assertEqual(callback('10e5', opt10), (10e5, 4))
            with self.assertRaises(lexical.LexicalError):
                callback('.', opt10)
            with self.assertRaises(lexical.LexicalError):
                callback('e5', opt10)
            with self.assertRaises(lexical.LexicalError):
                callback('10e+', opt10)

            if lexical.HAVE_RADIX:
                self.assertEqual(callback('1010.1', opt2), (10.5, 6))
                self.assertEqual(callback('A.8', opt16), (10.5, 3))

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

    if lexical.HAVE_I128:
        def test_atoi128(self):
            self._complete_test(lexical.atoi128, int)

        def test_atou128(self):
            self._complete_test(lexical.atou128, int)

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

    if lexical.HAVE_I128:
        def test_atoi128_partial(self):
            self._partial_test(lexical.atoi128_partial, int)

        def test_atou128_partial(self):
            self._partial_test(lexical.atou128_partial, int)

    def test_atoi8_with_options(self):
        self._complete_options_test(lexical.atoi8_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi16_with_options(self):
        self._complete_options_test(lexical.atoi16_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi32_with_options(self):
        self._complete_options_test(lexical.atoi32_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi64_with_options(self):
        self._complete_options_test(lexical.atoi64_with_options, int, lexical.ParseIntegerOptions)

    def test_atoisize_with_options(self):
        self._complete_options_test(lexical.atoisize_with_options, int, lexical.ParseIntegerOptions)

    def test_atou8_with_options(self):
        self._complete_options_test(lexical.atou8_with_options, int, lexical.ParseIntegerOptions)

    def test_atou16_with_options(self):
        self._complete_options_test(lexical.atou16_with_options, int, lexical.ParseIntegerOptions)

    def test_atou32_with_options(self):
        self._complete_options_test(lexical.atou32_with_options, int, lexical.ParseIntegerOptions)

    def test_atou64_with_options(self):
        self._complete_options_test(lexical.atou64_with_options, int, lexical.ParseIntegerOptions)

    def test_atousize_with_options(self):
        self._complete_options_test(lexical.atousize_with_options, int, lexical.ParseIntegerOptions)

    def test_atof32_with_options(self):
        self._complete_options_test(lexical.atof32_with_options, float, lexical.ParseFloatOptions)

    def test_atof64_with_options(self):
        self._complete_options_test(lexical.atof64_with_options, float, lexical.ParseFloatOptions)

    if lexical.HAVE_I128:
        def test_atoi128_with_options(self):
            self._complete_options_test(lexical.atoi128_with_options, int, lexical.ParseIntegerOptions)

        def test_atou128_with_options(self):
            self._complete_options_test(lexical.atou128_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi8_partial_with_options(self):
        self._partial_options_test(lexical.atoi8_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi16_partial_with_options(self):
        self._partial_options_test(lexical.atoi16_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi32_partial_with_options(self):
        self._partial_options_test(lexical.atoi32_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoi64_partial_with_options(self):
        self._partial_options_test(lexical.atoi64_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atoisize_partial_with_options(self):
        self._partial_options_test(lexical.atoisize_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou8_partial_with_options(self):
        self._partial_options_test(lexical.atou8_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou16_partial_with_options(self):
        self._partial_options_test(lexical.atou16_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou32_partial_with_options(self):
        self._partial_options_test(lexical.atou32_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atou64_partial_with_options(self):
        self._partial_options_test(lexical.atou64_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atousize_partial_with_options(self):
        self._partial_options_test(lexical.atousize_partial_with_options, int, lexical.ParseIntegerOptions)

    def test_atof32_partial_with_options(self):
        self._partial_options_test(lexical.atof32_partial_with_options, float, lexical.ParseFloatOptions)

    def test_atof64_partial_with_options(self):
        self._partial_options_test(lexical.atof64_partial_with_options, float, lexical.ParseFloatOptions)

    if lexical.HAVE_I128:
        def test_atoi128_partial_with_options(self):
            self._partial_options_test(lexical.atoi128_partial_with_options, int, lexical.ParseIntegerOptions)

        def test_atou128_partial_with_options(self):
            self._partial_options_test(lexical.atou128_partial_with_options, int, lexical.ParseIntegerOptions)


if __name__ == '__main__':
   unittest.main()
