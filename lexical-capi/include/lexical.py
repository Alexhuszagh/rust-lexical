"""
    lexical
    =======

    Access lexical-capi functionality from Python.

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

from ctypes.util import find_library
from ctypes import *
import contextlib
import enum
import os
import sys

# LOADING
# -------

# Identify the shared lib suffix on the platform.
# Allow the user to specify `SHARED_LIBRARY_SUFFIX` in the shell
# environment to override the default.
if 'SHARED_LIBRARY_SUFFIX' in os.environ:
    SHARED_LIBRARY_SUFFIX = os.environ['SHARED_LIBRARY_SUFFIX']
elif os.name == 'nt':
    SHARED_LIBRARY_SUFFIX = 'dll'
elif sys.platform == 'darwin':
    # Path can be either.
    SHARED_LIBRARY_SUFFIX = 'dylib,so'
else:
    SHARED_LIBRARY_SUFFIX = 'so'

# Wrap the dynlib. Find's the path to an installed lexical-capi library,
# otherwise, assumes it's in the working directory.
# You can modify this code to change how liblexical_capi is loaded for
# your application.
PATH = find_library('lexical_capi')
LIB = None
if PATH is not None:
    LIB = CDLL(PATH)
else:
    for suffix in SHARED_LIBRARY_SUFFIX.split(','):
        path = os.path.join(os.getcwd(), 'liblexical_capi.{}'.format(suffix))
        with contextlib.suppress(OSError):
            LIB = CDLL(path)
if LIB is None:
    raise OSError("Unavailable to find path to the liblexical_capi shared library.")

# FEATURES
# --------

HAVE_RADIX = hasattr(LIB, 'lexical_get_exponent_backup_char')
HAVE_ROUNDING = hasattr(LIB, 'lexical_get_float_rounding')
HAVE_I128 = hasattr(LIB, 'LEXICAL_I128_FORMATTED_SIZE')

# CONFIG
# ------

LIB.lexical_get_nan_string.restype = c_int
LIB.lexical_set_nan_string.restype = c_int
LIB.lexical_get_inf_string.restype = c_int
LIB.lexical_set_inf_string.restype = c_int
LIB.lexical_get_infinity_string.restype = c_int
LIB.lexical_set_infinity_string.restype = c_int
LIB.lexical_get_exponent_default_char.restype = c_ubyte

def _get_string(name):
    cb = getattr(LIB, name)
    ptr = POINTER(c_ubyte)()
    size = c_size_t()
    if cb(byref(ptr), byref(size)) != 0:
        raise OSError('Unexpected error in lexical_capi.{}'.format(name))
    return string_at(ptr, size.value).decode('ascii')

def _set_string(name, data):
    if isinstance(data, str):
        data = data.encode('ascii')
    if not isinstance(data, (bytes, bytearray)):
        raise TypeError("Must set string from bytes.")
    cb = getattr(LIB, name)
    ptr = cast(data, POINTER(c_ubyte))
    size = c_size_t(len(data))
    if cb(ptr, size) != 0:
        raise OSError('Unexpected error in lexical_capi.{}'.format(name))

def get_nan_string():
    '''Get string representation of Not a Number as a byte slice.'''
    return _get_string('lexical_get_nan_string')

def set_nan_string(data):
    '''Set string representation of Not a Number from a byte slice.'''
    _set_string('lexical_set_nan_string', data)

def get_inf_string():
    '''Get the short representation of an Infinity literal as a byte slice.'''
    return _get_string('lexical_get_inf_string')

def set_inf_string(data):
    '''Set the short representation of an Infinity literal from a byte slice.'''
    _set_string('lexical_set_inf_string', data)

def get_infinity_string():
    '''Get the long representation of an Infinity literal as a byte slice.'''
    return _get_string('lexical_get_infinity_string')

def set_infinity_string(data):
    '''Set the long representation of an Infinity literal from a byte slice.'''
    _set_string('lexical_set_infinity_string', data)

def get_exponent_default_char():
    '''Get the default exponent character.'''
    return chr(LIB.lexical_get_exponent_default_char())

def set_exponent_default_char(character):
    '''Set the default exponent character.'''
    LIB.lexical_set_exponent_default_char(c_ubyte(ord(character)))

if HAVE_RADIX:
    def get_exponent_backup_char():
        '''Get the backup exponent character.'''
        return chr(LIB.lexical_get_exponent_backup_char())

    def set_exponent_backup_char(character):
        '''Set the backup exponent character.'''
        LIB.lexical_set_exponent_backup_char(c_ubyte(ord(character)))

if HAVE_ROUNDING:
    LIB.lexical_get_float_rounding.restype = c_int

    class RoundingKind(enum.Enum):
        '''Rounding type for float-parsing.'''

        NearestTieEven = 0
        NearestTieAwayZero = 1
        TowardPositiveInfinity = 2
        TowardNegativeInfinity = 3
        TowardZero = 4

    def get_float_rounding():
        '''Get the default rounding scheme.'''
        return RoundingKind(LIB.lexical_get_float_rounding())

    def set_float_rounding(rounding):
        '''Set the default rounding scheme.'''
        LIB.lexical_set_float_rounding(c_int(rounding.value))

# GLOBALS
# -------

# CONSTANTS
I8_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I8_FORMATTED_SIZE').value
I16_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I16_FORMATTED_SIZE').value
I32_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I32_FORMATTED_SIZE').value
I64_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I64_FORMATTED_SIZE').value
ISIZE_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_ISIZE_FORMATTED_SIZE').value
U8_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U8_FORMATTED_SIZE').value
U16_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U16_FORMATTED_SIZE').value
U32_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U32_FORMATTED_SIZE').value
U64_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U64_FORMATTED_SIZE').value
USIZE_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_USIZE_FORMATTED_SIZE').value
F32_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_F32_FORMATTED_SIZE').value
F64_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_F64_FORMATTED_SIZE').value

I8_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I8_FORMATTED_SIZE_DECIMAL').value
I16_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I16_FORMATTED_SIZE_DECIMAL').value
I32_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I32_FORMATTED_SIZE_DECIMAL').value
I64_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I64_FORMATTED_SIZE_DECIMAL').value
ISIZE_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL').value
U8_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U8_FORMATTED_SIZE_DECIMAL').value
U16_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U16_FORMATTED_SIZE_DECIMAL').value
U32_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U32_FORMATTED_SIZE_DECIMAL').value
U64_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U64_FORMATTED_SIZE_DECIMAL').value
USIZE_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL').value
F32_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_F32_FORMATTED_SIZE_DECIMAL').value
F64_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_F64_FORMATTED_SIZE_DECIMAL').value

BUFFER_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_BUFFER_SIZE').value

if HAVE_I128:
    I128_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_I128_FORMATTED_SIZE').value
    U128_FORMATTED_SIZE = c_size_t.in_dll(LIB, 'LEXICAL_U128_FORMATTED_SIZE').value
    I128_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_I128_FORMATTED_SIZE_DECIMAL').value
    U128_FORMATTED_SIZE_DECIMAL = c_size_t.in_dll(LIB, 'LEXICAL_U128_FORMATTED_SIZE_DECIMAL').value

# TYPES
# -----

# ERROR

class ErrorCode(enum.Enum):
    '''Error code, indicating failure type.'''

    Overflow = -1
    Underflow = -2
    InvalidDigit = -3
    Empty = -4
    EmptyMantissa = -5
    EmptyExponent = -6
    EmptyInteger = -7
    EmptyFraction = -8
    InvalidPositiveMantissaSign = -9
    MissingMantissaSign = -10
    InvalidExponent = -11
    InvalidPositiveExponentSign = -12
    MissingExponentSign = -13
    ExponentWithoutFraction = -14

class Error(Structure):
    '''C-compatible error for FFI.'''

    _fields_ = [
        ("_code", c_int),
        ("index", c_size_t)
    ]

    @property
    def code(self):
        return ErrorCode(self._code)

    @code.setter
    def code(self, value):
        if not isinstance(value, ErrorCode):
            raise TypeError('Expected ErrorCode')
        self._code = value.value

    def is_overflow(self):
        return self.code == ErrorCode.Overflow

    def is_underflow(self):
        return self.code == ErrorCode.Underflow

    def is_invalid_digit(self):
        return self.code == ErrorCode.InvalidDigit

    def is_empty(self):
        return self.code == ErrorCode.Empty

    def is_empty_mantissa(self):
        return self.code == ErrorCode.EmptyMantissa

    def is_empty_exponent(self):
        return self.code == ErrorCode.EmptyExponent

    def is_empty_integer(self):
        return self.code == ErrorCode.EmptyInteger

    def is_empty_fraction(self):
        return self.code == ErrorCode.EmptyFraction

    def is_invalid_positive_mantissa_sign(self):
        return self.code == ErrorCode.InvalidPositiveMantissaSign

    def is_missing_mantissa_sign(self):
        return self.code == ErrorCode.MissingMantissaSign

    def is_invalid_exponent(self):
        return self.code == ErrorCode.InvalidExponent

    def is_invalid_positive_exponent_sign(self):
        return self.code == ErrorCode.InvalidPositiveExponentSign

    def is_missing_exponent_sign(self):
        return self.code == ErrorCode.MissingExponentSign

    def is_exponent_without_fraction(self):
        return self.code == ErrorCode.ExponentWithoutFraction

class LexicalError(Exception):
    '''Python-native exception raised during errors in lexical parsing.'''

    def __init__(self, error):
        self.error = error

    def __repr__(self):
        code = self.error.code
        if code == ErrorCode.Overflow:
            return 'Numeric overflow occurred at index {}'.format(self.error.index)
        elif code == ErrorCode.Underflow:
            return 'Numeric underflow occurred at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidDigit:
            return 'Invalid digit found at index {}'.format(self.error.index)
        elif code == ErrorCode.Empty:
            return 'Empty input found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyMantissa:
            return 'Empty mantissa found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyExponent:
            return 'Empty exponent found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyInteger:
            return 'Empty integer found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.EmptyFraction:
            return 'Empty fraction found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidPositiveMantissaSign:
            return 'Invalid "+" sign found for mantissa, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.MissingMantissaSign:
            return 'Missing required sign for mantissa, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidExponent:
            return 'Disallowed exponent was found, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.InvalidPositiveExponentSign:
            return 'Invalid "+" sign found for exponent, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.MissingExponentSign:
            return 'Missing required sign for exponent, starting at index {}'.format(self.error.index)
        elif code == ErrorCode.ExponentWithoutFraction:
            return 'Exponent found without fraction, starting at index {}'.format(self.error.index)
        else:
            raise ValueError('Invalid ErrorCode for lexical error.')

# RESULT TAG

class ResultTag(enum.Enum):
    '''Tag for the result tagged enum.'''

    Ok = 0
    Err = 1

# COMPLETE UNIONS

def _union(cls, name):
    class ResultUnion(Union):
        value_type = cls
        _fields_ = [
            ("value", cls),
            ("error", Error)
        ]

    ResultUnion.__name__ = name
    return ResultUnion

UnionI8 = _union(c_int8, 'UnionI8')
UnionI16 = _union(c_int16, 'UnionI16')
UnionI32 = _union(c_int32, 'UnionI32')
UnionI64 = _union(c_int64, 'UnionI64')
UnionIsize = _union(c_ssize_t, 'UnionIsize')
UnionU8 = _union(c_uint8, 'UnionU8')
UnionU16 = _union(c_uint16, 'UnionU16')
UnionU32 = _union(c_uint32, 'UnionU32')
UnionU64 = _union(c_uint64, 'UnionU64')
UnionUsize = _union(c_size_t, 'UnionUsize')
UnionF32 = _union(c_float, 'UnionF32')
UnionF64 = _union(c_double, 'UnionF64')

# COMPLETE RESULTS

def _result(cls, prefix, name):
    class Result(Structure):
        union_type = cls
        _fields_ = [
            ("_tag", c_uint),
            ("data", cls)
        ]

        @property
        def tag(self):
            return ResultTag(self._tag)

        @tag.setter
        def tag(self, value):
            if not isinstance(value, ResultTag):
                raise TypeError('Expected ResultTag')
            self._tag = value.value

        def into(self):
            '''Extract value from structure.'''

            if self.tag == ResultTag.Err:
                raise LexicalError(self.data.error)
            return self.data.value

    Result.__name__ = name
    return Result

ResultI8 = _result(UnionI8, 'i8', 'ResultI8')
ResultI16 = _result(UnionI16, 'i16', 'ResultI16')
ResultI32 = _result(UnionI32, 'i32', 'ResultI32')
ResultI64 = _result(UnionI64, 'i64', 'ResultI64')
ResultIsize = _result(UnionIsize, 'isize', 'ResultIsize')
ResultU8 = _result(UnionU8, 'u8', 'ResultU8')
ResultU16 = _result(UnionU16, 'u16', 'ResultU16')
ResultU32 = _result(UnionU32, 'u32', 'ResultU32')
ResultU64 = _result(UnionU64, 'u64', 'ResultU64')
ResultUsize = _result(UnionUsize, 'usize', 'ResultUsize')
ResultF32 = _result(UnionF32, 'f32', 'ResultF32')
ResultF64 = _result(UnionF64, 'f64', 'ResultF64')

# PARTIAL TUPLES

def _partial_tuple(cls, name):
    class Tuple(Structure):
        _fields_ = [
            ("x", cls),
            ("y", c_size_t)
        ]

        def into(self):
            '''Extract Python tuple from structure.'''
            return (self.x, self.y)

    Tuple.__name__ = name
    return Tuple

PartialTupleI8 = _partial_tuple(c_int8, 'PartialTupleI8')
PartialTupleI16 = _partial_tuple(c_int16, 'PartialTupleI16')
PartialTupleI32 = _partial_tuple(c_int32, 'PartialTupleI32')
PartialTupleI64 = _partial_tuple(c_int64, 'PartialTupleI64')
PartialTupleIsize = _partial_tuple(c_ssize_t, 'PartialTupleIsize')
PartialTupleU8 = _partial_tuple(c_uint8, 'PartialTupleU8')
PartialTupleU16 = _partial_tuple(c_uint16, 'PartialTupleU16')
PartialTupleU32 = _partial_tuple(c_uint32, 'PartialTupleU32')
PartialTupleU64 = _partial_tuple(c_uint64, 'PartialTupleU64')
PartialTupleUsize = _partial_tuple(c_size_t, 'PartialTupleUsize')
PartialTupleF32 = _partial_tuple(c_float, 'PartialTupleF32')
PartialTupleF64 = _partial_tuple(c_double, 'PartialTupleF64')

# PARTIAL UNIONS

def _partial_union(cls, name):
    return _union(cls, name)

PartialUnionI8 = _partial_union(PartialTupleI8, 'PartialUnionI8')
PartialUnionI16 = _partial_union(PartialTupleI16, 'PartialUnionI16')
PartialUnionI32 = _partial_union(PartialTupleI32, 'PartialUnionI32')
PartialUnionI64 = _partial_union(PartialTupleI64, 'PartialUnionI64')
PartialUnionIsize = _partial_union(PartialTupleIsize, 'PartialUnionIsize')
PartialUnionU8 = _partial_union(PartialTupleU8, 'PartialUnionU8')
PartialUnionU16 = _partial_union(PartialTupleU16, 'PartialUnionU16')
PartialUnionU32 = _partial_union(PartialTupleU32, 'PartialUnionU32')
PartialUnionU64 = _partial_union(PartialTupleU64, 'PartialUnionU64')
PartialUnionUsize = _partial_union(PartialTupleUsize, 'PartialUnionUsize')
PartialUnionF32 = _partial_union(PartialTupleF32, 'PartialUnionF32')
PartialUnionF64 = _partial_union(PartialTupleF64, 'PartialUnionF64')

# PARTIAL RESULTS

def _partial_result(cls, prefix, name):
    class PartialResult(Structure):
        union_type = cls
        _fields_ = [
            ("_tag", c_uint),
            ("data", cls)
        ]

        @property
        def tag(self):
            return ResultTag(self._tag)

        @tag.setter
        def tag(self, value):
            if not isinstance(value, ResultTag):
                raise TypeError('Expected ResultTag')
            self._tag = value.value

        def into(self):
            '''Extract value from structure.'''

            if self.tag == ResultTag.Err:
                raise LexicalError(self.data.error)
            return self.data.value.into()

    PartialResult.__name__ = name
    return PartialResult

PartialResultI8 = _partial_result(PartialUnionI8, 'i8', 'PartialResultI8')
PartialResultI16 = _partial_result(PartialUnionI16, 'i16', 'PartialResultI16')
PartialResultI32 = _partial_result(PartialUnionI32, 'i32', 'PartialResultI32')
PartialResultI64 = _partial_result(PartialUnionI64, 'i64', 'PartialResultI64')
PartialResultIsize = _partial_result(PartialUnionIsize, 'isize', 'PartialResultIsize')
PartialResultU8 = _partial_result(PartialUnionU8, 'u8', 'PartialResultU8')
PartialResultU16 = _partial_result(PartialUnionU16, 'u16', 'PartialResultU16')
PartialResultU32 = _partial_result(PartialUnionU32, 'u32', 'PartialResultU32')
PartialResultU64 = _partial_result(PartialUnionU64, 'u64', 'PartialResultU64')
PartialResultUsize = _partial_result(PartialUnionUsize, 'usize', 'PartialResultUsize')
PartialResultF32 = _partial_result(PartialUnionF32, 'f32', 'PartialResultF32')
PartialResultF64 = _partial_result(PartialUnionF64, 'f64', 'PartialResultF64')

# API
# ---

# HELPERS

def _to_address(ptr):
    return cast(ptr, c_voidp).value

def _to_u8_ptr(address):
    return cast(address, POINTER(c_ubyte))

def _distance(first, last):
    return _to_address(last) - _to_address(first)

# TO_STRING

def _to_string(name, max_size, type, value):
    buffer_type = c_ubyte * max_size
    buffer = buffer_type()
    if not isinstance(value, type):
        value = type(value)
    cb = getattr(LIB, name)
    first = _to_u8_ptr(buffer)
    last = _to_u8_ptr(_to_address(first) + len(buffer))
    ptr = cb(value, first, last)
    length = _distance(first, ptr)
    return string_at(buffer, length).decode('ascii')

LIB.lexical_i8toa.restype = POINTER(c_ubyte)
LIB.lexical_i16toa.restype = POINTER(c_ubyte)
LIB.lexical_i32toa.restype = POINTER(c_ubyte)
LIB.lexical_i64toa.restype = POINTER(c_ubyte)
LIB.lexical_isizetoa.restype = POINTER(c_ubyte)
LIB.lexical_u8toa.restype = POINTER(c_ubyte)
LIB.lexical_u16toa.restype = POINTER(c_ubyte)
LIB.lexical_u32toa.restype = POINTER(c_ubyte)
LIB.lexical_u64toa.restype = POINTER(c_ubyte)
LIB.lexical_usizetoa.restype = POINTER(c_ubyte)
LIB.lexical_f32toa.restype = POINTER(c_ubyte)
LIB.lexical_f64toa.restype = POINTER(c_ubyte)

def i8toa(value):
    '''Format 8-bit signed integer to bytes'''
    return _to_string('lexical_i8toa', I8_FORMATTED_SIZE_DECIMAL, c_int8, value)

def i16toa(value):
    '''Format 16-bit signed integer to bytes'''
    return _to_string('lexical_i16toa', I16_FORMATTED_SIZE_DECIMAL, c_int16, value)

def i32toa(value):
    '''Format 32-bit signed integer to bytes'''
    return _to_string('lexical_i32toa', I32_FORMATTED_SIZE_DECIMAL, c_int32, value)

def i64toa(value):
    '''Format 64-bit signed integer to bytes'''
    return _to_string('lexical_i64toa', I64_FORMATTED_SIZE_DECIMAL, c_int64, value)

def isizetoa(value):
    '''Format ssize_t to bytes'''
    return _to_string('lexical_isizetoa', ISIZE_FORMATTED_SIZE_DECIMAL, c_ssize_t, value)

def u8toa(value):
    '''Format 8-bit unsigned integer to bytes'''
    return _to_string('lexical_u8toa', U8_FORMATTED_SIZE_DECIMAL, c_uint8, value)

def u16toa(value):
    '''Format 16-bit unsigned integer to bytes'''
    return _to_string('lexical_u16toa', U16_FORMATTED_SIZE_DECIMAL, c_uint16, value)

def u32toa(value):
    '''Format 32-bit unsigned integer to bytes'''
    return _to_string('lexical_u32toa', U32_FORMATTED_SIZE_DECIMAL, c_uint32, value)

def u64toa(value):
    '''Format 64-bit unsigned integer to bytes'''
    return _to_string('lexical_u64toa', U64_FORMATTED_SIZE_DECIMAL, c_uint64, value)

def usizetoa(value):
    '''Format size_t to bytes'''
    return _to_string('lexical_usizetoa', USIZE_FORMATTED_SIZE_DECIMAL, c_size_t, value)

def f32toa(value):
    '''Format 32-bit float to bytes'''
    return _to_string('lexical_f32toa', F32_FORMATTED_SIZE_DECIMAL, c_float, value)

def f64toa(value):
    '''Format 64-bit float to bytes'''
    return _to_string('lexical_f64toa', F64_FORMATTED_SIZE_DECIMAL, c_double, value)

if HAVE_RADIX:
    # TO_STRING_RADIX

    def _to_string_radix(name, max_size, type, value, radix):
        buffer_type = c_ubyte * max_size
        buffer = buffer_type()
        if not isinstance(value, type):
            value = type(value)
        if not isinstance(radix, c_uint8):
            radix = c_uint8(radix)
        cb = getattr(LIB, name)
        first = _to_u8_ptr(buffer)
        last = _to_u8_ptr(_to_address(first) + len(buffer))
        ptr = cb(value, radix, first, last)
        length = _distance(first, ptr)
        return string_at(buffer, length).decode('ascii')

    LIB.lexical_i8toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_i16toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_i32toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_i64toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_isizetoa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_u8toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_u16toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_u32toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_u64toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_usizetoa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_f32toa_radix.restype = POINTER(c_ubyte)
    LIB.lexical_f64toa_radix.restype = POINTER(c_ubyte)

    def i8toa_radix(value, radix):
        '''Format 8-bit signed integer to bytes'''
        return _to_string_radix('lexical_i8toa_radix', I8_FORMATTED_SIZE, c_int8, value, radix)

    def i16toa_radix(value, radix):
        '''Format 16-bit signed integer to bytes'''
        return _to_string_radix('lexical_i16toa_radix', I16_FORMATTED_SIZE, c_int16, value, radix)

    def i32toa_radix(value, radix):
        '''Format 32-bit signed integer to bytes'''
        return _to_string_radix('lexical_i32toa_radix', I32_FORMATTED_SIZE, c_int32, value, radix)

    def i64toa_radix(value, radix):
        '''Format 64-bit signed integer to bytes'''
        return _to_string_radix('lexical_i64toa_radix', I64_FORMATTED_SIZE, c_int64, value, radix)

    def isizetoa_radix(value, radix):
        '''Format ssize_t to bytes'''
        return _to_string_radix('lexical_isizetoa_radix', ISIZE_FORMATTED_SIZE, c_ssize_t, value, radix)

    def u8toa_radix(value, radix):
        '''Format 8-bit unsigned integer to bytes'''
        return _to_string_radix('lexical_u8toa_radix', U8_FORMATTED_SIZE, c_uint8, value, radix)

    def u16toa_radix(value, radix):
        '''Format 16-bit unsigned integer to bytes'''
        return _to_string_radix('lexical_u16toa_radix', U16_FORMATTED_SIZE, c_uint16, value, radix)

    def u32toa_radix(value, radix):
        '''Format 32-bit unsigned integer to bytes'''
        return _to_string_radix('lexical_u32toa_radix', U32_FORMATTED_SIZE, c_uint32, value, radix)

    def u64toa_radix(value, radix):
        '''Format 64-bit unsigned integer to bytes'''
        return _to_string_radix('lexical_u64toa_radix', U64_FORMATTED_SIZE, c_uint64, value, radix)

    def usizetoa_radix(value, radix):
        '''Format size_t to bytes'''
        return _to_string_radix('lexical_usizetoa_radix', USIZE_FORMATTED_SIZE, c_size_t, value, radix)

    def f32toa_radix(value, radix):
        '''Format 32-bit float to bytes'''
        return _to_string_radix('lexical_f32toa_radix', F32_FORMATTED_SIZE, c_float, value, radix)

    def f64toa_radix(value, radix):
        '''Format 64-bit float to bytes'''
        return _to_string_radix('lexical_f64toa_radix', F64_FORMATTED_SIZE, c_double, value, radix)


# PARSE

def _parse(name, data):
    if isinstance(data, str):
        data = data.encode('ascii')
    if not isinstance(data, (bytes, bytearray)):
        raise TypeError("Must parse from bytes.")
    cb = getattr(LIB, name)
    first = _to_u8_ptr(data)
    last = _to_u8_ptr(_to_address(first) + len(data))
    result = cb(first, last)
    return result.into()

# COMPLETE PARSE

LIB.lexical_atoi8.restype = ResultI8
LIB.lexical_atoi16.restype = ResultI16
LIB.lexical_atoi32.restype = ResultI32
LIB.lexical_atoi64.restype = ResultI64
LIB.lexical_atoisize.restype = ResultIsize
LIB.lexical_atou8.restype = ResultU8
LIB.lexical_atou16.restype = ResultU16
LIB.lexical_atou32.restype = ResultU32
LIB.lexical_atou64.restype = ResultU64
LIB.lexical_atousize.restype = ResultUsize
LIB.lexical_atof32.restype = ResultF32
LIB.lexical_atof64.restype = ResultF64

def atoi8(data):
    '''Parse 8-bit signed integer from input data.'''
    return _parse('lexical_atoi8', data)

def atoi16(data):
    '''Parse 16-bit signed integer from input data.'''
    return _parse('lexical_atoi16', data)

def atoi32(data):
    '''Parse 32-bit signed integer from input data.'''
    return _parse('lexical_atoi32', data)

def atoi64(data):
    '''Parse 64-bit signed integer from input data.'''
    return _parse('lexical_atoi64', data)

def atoisize(data):
    '''Parse ssize_t from input data.'''
    return _parse('lexical_atoisize', data)

def atou8(data):
    '''Parse 8-bit unsigned integer from input data.'''
    return _parse('lexical_atou8', data)

def atou16(data):
    '''Parse 16-bit unsigned integer from input data.'''
    return _parse('lexical_atou16', data)

def atou32(data):
    '''Parse 32-bit unsigned integer from input data.'''
    return _parse('lexical_atou32', data)

def atou64(data):
    '''Parse 64-bit unsigned integer from input data.'''
    return _parse('lexical_atou64', data)

def atousize(data):
    '''Parse size_t from input data.'''
    return _parse('lexical_atousize', data)

def atof32(data):
    '''Parse 32-bit float from input data.'''
    return _parse('lexical_atof32', data)

def atof64(data):
    '''Parse 64-bit float from input data.'''
    return _parse('lexical_atof64', data)

# COMPLETE PARSE_LOSSY

LIB.lexical_atof32_lossy.restype = ResultF32
LIB.lexical_atof64_lossy.restype = ResultF64

def atof32_lossy(data):
    '''Lossily parse 32-bit float from input data.'''
    return _parse('lexical_atof32_lossy', data)

def atof64_lossy(data):
    '''Lossily parse 64-bit float from input data.'''
    return _parse('lexical_atof64_lossy', data)

# PARTIAL PARSE

LIB.lexical_atoi8_partial.restype = PartialResultI8
LIB.lexical_atoi16_partial.restype = PartialResultI16
LIB.lexical_atoi32_partial.restype = PartialResultI32
LIB.lexical_atoi64_partial.restype = PartialResultI64
LIB.lexical_atoisize_partial.restype = PartialResultIsize
LIB.lexical_atou8_partial.restype = PartialResultU8
LIB.lexical_atou16_partial.restype = PartialResultU16
LIB.lexical_atou32_partial.restype = PartialResultU32
LIB.lexical_atou64_partial.restype = PartialResultU64
LIB.lexical_atousize_partial.restype = PartialResultUsize
LIB.lexical_atof32_partial.restype = PartialResultF32
LIB.lexical_atof64_partial.restype = PartialResultF64

def atoi8_partial(data):
    '''Parse 8-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi8_partial', data)

def atoi16_partial(data):
    '''Parse 16-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi16_partial', data)

def atoi32_partial(data):
    '''Parse 32-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi32_partial', data)

def atoi64_partial(data):
    '''Parse 64-bit signed integer and the number of processed bytes from input data.'''
    return _parse('lexical_atoi64_partial', data)

def atoisize_partial(data):
    '''Parse ssize_t and the number of processed bytes from input data.'''
    return _parse('lexical_atoisize_partial', data)

def atou8_partial(data):
    '''Parse 8-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou8_partial', data)

def atou16_partial(data):
    '''Parse 16-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou16_partial', data)

def atou32_partial(data):
    '''Parse 32-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou32_partial', data)

def atou64_partial(data):
    '''Parse 64-bit unsigned integer and the number of processed bytes from input data.'''
    return _parse('lexical_atou64_partial', data)

def atousize_partial(data):
    '''Parse size_t and the number of processed bytes from input data.'''
    return _parse('lexical_atousize_partial', data)

def atof32_partial(data):
    '''Parse 32-bit float and the number of processed bytes from bytes.'''
    return _parse('lexical_atof32_partial', data)

def atof64_partial(data):
    '''Parse 64-bit float and the number of processed bytes from bytes.'''
    return _parse('lexical_atof64_partial', data)

# PARTIAL PARSE_LOSSY

LIB.lexical_atof32_partial_lossy.restype = PartialResultF32
LIB.lexical_atof64_partial_lossy.restype = PartialResultF64

def atof32_partial_lossy(data):
    '''Lossily parse 32-bit float and the number of processed bytes from input data.'''
    return _parse('lexical_atof32_partial_lossy', data)

def atof64_partial_lossy(data):
    '''Lossily parse 64-bit float and the number of processed bytes from input data.'''
    return _parse('lexical_atof64_partial_lossy', data)

if HAVE_RADIX:
    # PARSE_RADIX

    def _parse_radix(name, data, radix):
        if isinstance(data, str):
            data = data.encode('ascii')
        if not isinstance(data, (bytes, bytearray)):
            raise TypeError("Must parse from bytes.")
        if not isinstance(radix, c_uint8):
            radix = c_uint8(radix)
        cb = getattr(LIB, name)
        first = _to_u8_ptr(data)
        last = _to_u8_ptr(_to_address(first) + len(data))
        result = cb(first, last, radix)
        return result.into()

    # COMPLETE PARSE RADIX

    LIB.lexical_atoi8_radix.restype = ResultI8
    LIB.lexical_atoi16_radix.restype = ResultI16
    LIB.lexical_atoi32_radix.restype = ResultI32
    LIB.lexical_atoi64_radix.restype = ResultI64
    LIB.lexical_atoisize_radix.restype = ResultIsize
    LIB.lexical_atou8_radix.restype = ResultU8
    LIB.lexical_atou16_radix.restype = ResultU16
    LIB.lexical_atou32_radix.restype = ResultU32
    LIB.lexical_atou64_radix.restype = ResultU64
    LIB.lexical_atousize_radix.restype = ResultUsize
    LIB.lexical_atof32_radix.restype = ResultF32
    LIB.lexical_atof64_radix.restype = ResultF64

    def atoi8_radix(data, radix):
        '''Parse 8-bit signed integer from bytes.'''
        return _parse_radix('lexical_atoi8_radix', data, radix)

    def atoi16_radix(data, radix):
        '''Parse 16-bit signed integer from bytes.'''
        return _parse_radix('lexical_atoi16_radix', data, radix)

    def atoi32_radix(data, radix):
        '''Parse 32-bit signed integer from bytes.'''
        return _parse_radix('lexical_atoi32_radix', data, radix)

    def atoi64_radix(data, radix):
        '''Parse 64-bit signed integer from bytes.'''
        return _parse_radix('lexical_atoi64_radix', data, radix)

    def atoisize_radix(data, radix):
        '''Parse ssize_t from bytes.'''
        return _parse_radix('lexical_atoisize_radix', data, radix)

    def atou8_radix(data, radix):
        '''Parse 8-bit unsigned integer from bytes.'''
        return _parse_radix('lexical_atou8_radix', data, radix)

    def atou16_radix(data, radix):
        '''Parse 16-bit unsigned integer from bytes.'''
        return _parse_radix('lexical_atou16_radix', data, radix)

    def atou32_radix(data, radix):
        '''Parse 32-bit unsigned integer from bytes.'''
        return _parse_radix('lexical_atou32_radix', data, radix)

    def atou64_radix(data, radix):
        '''Parse 64-bit unsigned integer from bytes.'''
        return _parse_radix('lexical_atou64_radix', data, radix)

    def atousize_radix(data, radix):
        '''Parse size_t from bytes.'''
        return _parse_radix('lexical_atousize_radix', data, radix)

    def atof32_radix(data, radix):
        '''Parse 32-bit float from bytes.'''
        return _parse_radix('lexical_atof32_radix', data, radix)

    def atof64_radix(data, radix):
        '''Parse 64-bit float from bytes.'''
        return _parse_radix('lexical_atof64_radix', data, radix)

    # COMPLETE PARSE LOSSY RADIX

    LIB.lexical_atof32_lossy_radix.restype = ResultF32
    LIB.lexical_atof64_lossy_radix.restype = ResultF64

    def atof32_lossy_radix(data, radix):
        '''Lossily parse 32-bit float from bytes.'''
        return _parse_radix('lexical_atof32_lossy_radix', data, radix)

    def atof64_lossy_radix(data, radix):
        '''Lossily parse 64-bit float from bytes.'''
        return _parse_radix('lexical_atof64_lossy_radix', data, radix)

    # PARTIAL PARSE RADIX

    LIB.lexical_atoi8_partial_radix.restype = PartialResultI8
    LIB.lexical_atoi16_partial_radix.restype = PartialResultI16
    LIB.lexical_atoi32_partial_radix.restype = PartialResultI32
    LIB.lexical_atoi64_partial_radix.restype = PartialResultI64
    LIB.lexical_atoisize_partial_radix.restype = PartialResultIsize
    LIB.lexical_atou8_partial_radix.restype = PartialResultU8
    LIB.lexical_atou16_partial_radix.restype = PartialResultU16
    LIB.lexical_atou32_partial_radix.restype = PartialResultU32
    LIB.lexical_atou64_partial_radix.restype = PartialResultU64
    LIB.lexical_atousize_partial_radix.restype = PartialResultUsize
    LIB.lexical_atof32_partial_radix.restype = PartialResultF32
    LIB.lexical_atof64_partial_radix.restype = PartialResultF64

    def atoi8_partial_radix(data, radix):
        '''Parse 8-bit signed integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atoi8_partial_radix', data, radix)

    def atoi16_partial_radix(data, radix):
        '''Parse 16-bit signed integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atoi16_partial_radix', data, radix)

    def atoi32_partial_radix(data, radix):
        '''Parse 32-bit signed integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atoi32_partial_radix', data, radix)

    def atoi64_partial_radix(data, radix):
        '''Parse 64-bit signed integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atoi64_partial_radix', data, radix)

    def atoisize_partial_radix(data, radix):
        '''Parse ssize_t and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atoisize_partial_radix', data, radix)

    def atou8_partial_radix(data, radix):
        '''Parse 8-bit unsigned integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atou8_partial_radix', data, radix)

    def atou16_partial_radix(data, radix):
        '''Parse 16-bit unsigned integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atou16_partial_radix', data, radix)

    def atou32_partial_radix(data, radix):
        '''Parse 32-bit unsigned integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atou32_partial_radix', data, radix)

    def atou64_partial_radix(data, radix):
        '''Parse 64-bit unsigned integer and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atou64_partial_radix', data, radix)

    def atousize_partial_radix(data, radix):
        '''Parse size_t and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atousize_partial_radix', data, radix)

    def atof32_partial_radix(data, radix):
        '''Parse 32-bit float and the number of processed bytes from bytes.'''
        return _parse_radix('lexical_atof32_partial_radix', data, radix)

    def atof64_partial_radix(data, radix):
        '''Parse 64-bit float and the number of processed bytes from bytes.'''
        return _parse_radix('lexical_atof64_partial_radix', data, radix)

    # PARTIAL PARSE_LOSSY RADIX

    LIB.lexical_atof32_partial_lossy_radix.restype = PartialResultF32
    LIB.lexical_atof64_partial_lossy_radix.restype = PartialResultF64

    def atof32_partial_lossy_radix(data, radix):
        '''Lossily parse 32-bit float and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atof32_partial_lossy_radix', data, radix)

    def atof64_partial_lossy_radix(data, radix):
        '''Lossily parse 64-bit float and the number of processed bytes from input data.'''
        return _parse_radix('lexical_atof64_partial_lossy_radix', data, radix)
