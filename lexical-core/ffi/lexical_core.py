"""
    lexical_core
    ============

    Access lexical-core functionality from Python.

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

# Wrap the dynlib. Find's the path to an installed lexical-core library,
# otherwise, assumes it's in the working directory.
# You can modify this code to change how liblexical_core is loaded for
# your application.
PATH = find_library('lexical_core')
LIB = None
if PATH is not None:
    LIB = CDLL(PATH)
else:
    for suffix in SHARED_LIBRARY_SUFFIX.split(','):
        path = os.path.join(os.getcwd(), f'liblexical_core.{suffix}')
        with contextlib.suppress(OSError):
            LIB = CDLL(path)
if LIB is None:
    raise OSError("Unavailable to find path to the liblexical_core shared library.")

# CONFIG
# ------

LIB.get_nan_string_ffi.restype = c_int
LIB.set_nan_string_ffi.restype = c_int
LIB.get_inf_string_ffi.restype = c_int
LIB.set_inf_string_ffi.restype = c_int
LIB.get_infinity_string_ffi.restype = c_int
LIB.set_infinity_string_ffi.restype = c_int

def _get_string(name):
    cb = getattr(LIB, name)
    ptr = POINTER(c_ubyte)()
    size = c_size_t()
    if cb(byref(ptr), byref(size)) != 0:
        raise OSError(f"Unexpected error in lexical_core.{name}")
    return string_at(ptr, size.value)

def _set_string(name, data):
    if not isinstance(data, (bytes, bytearray)):
        raise TypeError("Must set string from bytes.")
    cb = getattr(LIB, name)
    ptr = cast(data, POINTER(c_ubyte))
    size = c_size_t(len(data))
    if cb(ptr, size) != 0:
        raise OSError(f"Unexpected error in lexical_core.{name}")

def get_nan_string():
    '''Get string representation of Not a Number as a byte slice.'''
    return _get_string('get_nan_string_ffi')

def set_nan_string(data):
    '''Set string representation of Not a Number from a byte slice.'''
    return _set_string('set_nan_string_ffi', data)

def get_inf_string():
    '''Get the short representation of an Infinity literal as a byte slice.'''
    return _get_string('get_inf_string_ffi')

def set_inf_string(data):
    '''Set the short representation of an Infinity literal from a byte slice.'''
    return _set_string('set_inf_string_ffi', data)

def get_infinity_string():
    '''Get the long representation of an Infinity literal as a byte slice.'''
    return _get_string('get_infinity_string_ffi')

def set_infinity_string(data):
    '''Set the long representation of an Infinity literal from a byte slice.'''
    return _set_string('set_infinity_string_ffi', data)


# GLOBALS
# -------

# MUTABLE GLOBALS
EXPONENT_DEFAULT_CHAR = c_ubyte.in_dll(LIB, "EXPONENT_DEFAULT_CHAR")

with contextlib.suppress(ValueError):
    # May or may not have been compiled with `radix` support, so allow
    # any of these symbols to fail.
    EXPONENT_BACKUP_CHAR = c_ubyte.in_dll(LIB, "EXPONENT_BACKUP_CHAR")

with contextlib.suppress(ValueError):
    # May or may not have been compiled with `rounding` support, so allow
    # any of these symbols to fail.
    FLOAT_ROUNDING = c_int.in_dll(LIB, 'FLOAT_ROUNDING')

# CONSTANTS
MAX_I8_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_I8_SIZE_FFI").value
MAX_I16_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_I16_SIZE_FFI").value
MAX_I32_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_I32_SIZE_FFI").value
MAX_I64_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_I64_SIZE_FFI").value
MAX_ISIZE_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_ISIZE_SIZE_FFI").value
MAX_U8_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_U8_SIZE_FFI").value
MAX_U16_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_U16_SIZE_FFI").value
MAX_U32_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_U32_SIZE_FFI").value
MAX_U64_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_U64_SIZE_FFI").value
MAX_USIZE_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_USIZE_SIZE_FFI").value
MAX_F32_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_F32_SIZE_FFI").value
MAX_F64_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_F64_SIZE_FFI").value

MAX_I8_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_I8_SIZE_BASE10_FFI").value
MAX_I16_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_I16_SIZE_BASE10_FFI").value
MAX_I32_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_I32_SIZE_BASE10_FFI").value
MAX_I64_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_I64_SIZE_BASE10_FFI").value
MAX_ISIZE_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_ISIZE_SIZE_BASE10_FFI").value
MAX_U8_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_U8_SIZE_BASE10_FFI").value
MAX_U16_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_U16_SIZE_BASE10_FFI").value
MAX_U32_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_U32_SIZE_BASE10_FFI").value
MAX_U64_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_U64_SIZE_BASE10_FFI").value
MAX_USIZE_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_USIZE_SIZE_BASE10_FFI").value
MAX_F32_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_F32_SIZE_BASE10_FFI").value
MAX_F64_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_F64_SIZE_BASE10_FFI").value

BUFFER_SIZE_FFI = c_size_t.in_dll(LIB, "BUFFER_SIZE_FFI").value

with contextlib.suppress(ValueError):
    # May or may not have been compiled with `has_i128` support, so allow
    # any of these symbols to fail.
    MAX_I128_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_I128_SIZE_FFI").value
    MAX_U128_SIZE_FFI = c_size_t.in_dll(LIB, "MAX_U128_SIZE_FFI").value
    MAX_I128_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_I128_SIZE_BASE10_FFI").value
    MAX_U128_SIZE_BASE10_FFI = c_size_t.in_dll(LIB, "MAX_U128_SIZE_BASE10_FFI").value

# TYPES
# -----

# ERROR
LIB.error_is_overflow.restype = c_bool
LIB.error_is_underflow.restype = c_bool
LIB.error_is_invalid_digit.restype = c_bool
LIB.error_is_empty.restype = c_bool
LIB.error_is_empty_fraction.restype = c_bool
LIB.error_is_empty_exponent.restype = c_bool

class ErrorCode(enum.Enum):
    '''Error code, indicating failure type.'''

    Overflow = -1
    Underflow = -2
    InvalidDigit = -3
    Empty = -4
    EmptyFraction = -5
    EmptyExponent = -6

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
        return LIB.error_is_overflow(self).value

    def is_underflow(self):
        return LIB.error_is_underflow(self).value

    def is_invalid_digit(self):
        return LIB.error_is_invalid_digit(self).value

    def is_empty(self):
        return LIB.error_is_empty(self).value

    def is_empty_fraction(self):
        return LIB.error_is_empty_fraction(self).value

    def is_empty_exponent(self):
        return LIB.error_is_empty_exponent(self).value

# UNIONS

class UnionI8(Union):
    _fields_ = [
        ("value", c_int8),
        ("error", Error)
    ]

class UnionI16(Union):
    _fields_ = [
        ("value", c_int16),
        ("error", Error)
    ]

class UnionI32(Union):
    _fields_ = [
        ("value", c_int32),
        ("error", Error)
    ]

class UnionI64(Union):
    _fields_ = [
        ("value", c_int64),
        ("error", Error)
    ]

class UnionIsize(Union):
    _fields_ = [
        ("value", c_ssize_t),
        ("error", Error)
    ]

class UnionU8(Union):
    _fields_ = [
        ("value", c_uint8),
        ("error", Error)
    ]

class UnionU16(Union):
    _fields_ = [
        ("value", c_uint16),
        ("error", Error)
    ]

class UnionU32(Union):
    _fields_ = [
        ("value", c_uint32),
        ("error", Error)
    ]

class UnionU64(Union):
    _fields_ = [
        ("value", c_uint64),
        ("error", Error)
    ]

class UnionUsize(Union):
    _fields_ = [
        ("value", c_size_t),
        ("error", Error)
    ]

class UnionF32(Union):
    _fields_ = [
        ("value", c_float),
        ("error", Error)
    ]

class UnionF64(Union):
    _fields_ = [
        ("value", c_float),
        ("error", Error)
    ]

# RESULTS

LIB.i8_result_ffi_is_ok.restype = c_bool
LIB.i8_result_ffi_is_err.restype = c_bool
LIB.i8_result_ffi_ok.restype = c_int8
LIB.i8_result_ffi_err.restype = Error

class ResultI8(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionI8)
    ]

    def is_ok(self):
        return LIB.i8_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.i8_result_ffi_is_err(self)

    def ok(self):
        return LIB.i8_result_ffi_ok(self)

    def err(self):
        return LIB.i8_result_ffi_err(self)

LIB.i16_result_ffi_is_ok.restype = c_bool
LIB.i16_result_ffi_is_err.restype = c_bool
LIB.i16_result_ffi_ok.restype = c_int16
LIB.i16_result_ffi_err.restype = Error

class ResultI16(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionI16)
    ]

    def is_ok(self):
        return LIB.i16_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.i16_result_ffi_is_err(self)

    def ok(self):
        return LIB.i16_result_ffi_ok(self)

    def err(self):
        return LIB.i16_result_ffi_err(self)

LIB.i32_result_ffi_is_ok.restype = c_bool
LIB.i32_result_ffi_is_err.restype = c_bool
LIB.i32_result_ffi_ok.restype = c_int32
LIB.i32_result_ffi_err.restype = Error

class ResultI32(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionI32)
    ]

    def is_ok(self):
        return LIB.i32_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.i32_result_ffi_is_err(self)

    def ok(self):
        return LIB.i32_result_ffi_ok(self)

    def err(self):
        return LIB.i32_result_ffi_err(self)

LIB.i64_result_ffi_is_ok.restype = c_bool
LIB.i64_result_ffi_is_err.restype = c_bool
LIB.i64_result_ffi_ok.restype = c_int64
LIB.i64_result_ffi_err.restype = Error

class ResultI64(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionI64)
    ]

    def is_ok(self):
        return LIB.i64_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.i64_result_ffi_is_err(self)

    def ok(self):
        return LIB.i64_result_ffi_ok(self)

    def err(self):
        return LIB.i64_result_ffi_err(self)

LIB.isize_result_ffi_is_ok.restype = c_bool
LIB.isize_result_ffi_is_err.restype = c_bool
LIB.isize_result_ffi_ok.restype = c_ssize_t
LIB.isize_result_ffi_err.restype = Error

class ResultIsize(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionIsize)
    ]

    def is_ok(self):
        return LIB.isize_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.isize_result_ffi_is_err(self)

    def ok(self):
        return LIB.isize_result_ffi_ok(self)

    def err(self):
        return LIB.isize_result_ffi_err(self)

LIB.u8_result_ffi_is_ok.restype = c_bool
LIB.u8_result_ffi_is_err.restype = c_bool
LIB.u8_result_ffi_ok.restype = c_uint8
LIB.u8_result_ffi_err.restype = Error

class ResultU8(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionU8)
    ]

    def is_ok(self):
        return LIB.u8_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.u8_result_ffi_is_err(self)

    def ok(self):
        return LIB.u8_result_ffi_ok(self)

    def err(self):
        return LIB.u8_result_ffi_err(self)

LIB.u16_result_ffi_is_ok.restype = c_bool
LIB.u16_result_ffi_is_err.restype = c_bool
LIB.u16_result_ffi_ok.restype = c_uint16
LIB.u16_result_ffi_err.restype = Error

class ResultU16(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionU16)
    ]

    def is_ok(self):
        return LIB.u16_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.u16_result_ffi_is_err(self)

    def ok(self):
        return LIB.u16_result_ffi_ok(self)

    def err(self):
        return LIB.u16_result_ffi_err(self)

LIB.u32_result_ffi_is_ok.restype = c_bool
LIB.u32_result_ffi_is_err.restype = c_bool
LIB.u32_result_ffi_ok.restype = c_uint32
LIB.u32_result_ffi_err.restype = Error

class ResultU32(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionU32)
    ]

    def is_ok(self):
        return LIB.u32_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.u32_result_ffi_is_err(self)

    def ok(self):
        return LIB.u32_result_ffi_ok(self)

    def err(self):
        return LIB.u32_result_ffi_err(self)

LIB.u64_result_ffi_is_ok.restype = c_bool
LIB.u64_result_ffi_is_err.restype = c_bool
LIB.u64_result_ffi_ok.restype = c_uint64
LIB.u64_result_ffi_err.restype = Error

class ResultU64(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionU64)
    ]

    def is_ok(self):
        return LIB.u64_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.u64_result_ffi_is_err(self)

    def ok(self):
        return LIB.u64_result_ffi_ok(self)

    def err(self):
        return LIB.u64_result_ffi_err(self)

LIB.usize_result_ffi_is_ok.restype = c_bool
LIB.usize_result_ffi_is_err.restype = c_bool
LIB.usize_result_ffi_ok.restype = c_size_t
LIB.usize_result_ffi_err.restype = Error

class ResultUsize(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionUsize)
    ]

    def is_ok(self):
        return LIB.usize_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.usize_result_ffi_is_err(self)

    def ok(self):
        return LIB.usize_result_ffi_ok(self)

    def err(self):
        return LIB.usize_result_ffi_err(self)

LIB.f32_result_ffi_is_ok.restype = c_bool
LIB.f32_result_ffi_is_err.restype = c_bool
LIB.f32_result_ffi_ok.restype = c_float
LIB.f32_result_ffi_err.restype = Error

class ResultF32(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionF32)
    ]

    def is_ok(self):
        return LIB.f32_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.f32_result_ffi_is_err(self)

    def ok(self):
        return LIB.f32_result_ffi_ok(self)

    def err(self):
        return LIB.f32_result_ffi_err(self)

LIB.f64_result_ffi_is_ok.restype = c_bool
LIB.f64_result_ffi_is_err.restype = c_bool
LIB.f64_result_ffi_ok.restype = c_double
LIB.f64_result_ffi_err.restype = Error

class ResultF64(Structure):
    _fields_ = [
        ("tag", c_uint),
        ("data", UnionF64)
    ]

    def is_ok(self):
        return LIB.f64_result_ffi_is_ok(self)

    def is_err(self):
        return LIB.f64_result_ffi_is_err(self)

    def ok(self):
        return LIB.f64_result_ffi_ok(self)

    def err(self):
        return LIB.f64_result_ffi_err(self)

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
    return string_at(buffer, length)

LIB.i8toa_range.restype = POINTER(c_ubyte)
LIB.i16toa_range.restype = POINTER(c_ubyte)
LIB.i32toa_range.restype = POINTER(c_ubyte)
LIB.i64toa_range.restype = POINTER(c_ubyte)
LIB.isizetoa_range.restype = POINTER(c_ubyte)
LIB.u8toa_range.restype = POINTER(c_ubyte)
LIB.u16toa_range.restype = POINTER(c_ubyte)
LIB.u32toa_range.restype = POINTER(c_ubyte)
LIB.u64toa_range.restype = POINTER(c_ubyte)
LIB.usizetoa_range.restype = POINTER(c_ubyte)
LIB.f32toa_range.restype = POINTER(c_ubyte)
LIB.f64toa_range.restype = POINTER(c_ubyte)

def i8toa(value):
    '''Format 8-bit signed integer to bytes'''
    return _to_string('i8toa_range', MAX_I8_SIZE_BASE10_FFI, c_int8, value)

def i16toa(value):
    '''Format 16-bit signed integer to bytes'''
    return _to_string('i16toa_range', MAX_I16_SIZE_BASE10_FFI, c_int16, value)

def i32toa(value):
    '''Format 32-bit signed integer to bytes'''
    return _to_string('i32toa_range', MAX_I32_SIZE_BASE10_FFI, c_int32, value)

def i64toa(value):
    '''Format 64-bit signed integer to bytes'''
    return _to_string('i64toa_range', MAX_I64_SIZE_BASE10_FFI, c_int64, value)

def isizetoa(value):
    '''Format ssize_t to bytes'''
    return _to_string('isizetoa_range', MAX_ISIZE_SIZE_BASE10_FFI, c_ssize_t, value)

def u8toa(value):
    '''Format 8-bit unsigned integer to bytes'''
    return _to_string('u8toa_range', MAX_U8_SIZE_BASE10_FFI, c_uint8, value)

def u16toa(value):
    '''Format 16-bit unsigned integer to bytes'''
    return _to_string('u16toa_range', MAX_U16_SIZE_BASE10_FFI, c_uint16, value)

def u32toa(value):
    '''Format 32-bit unsigned integer to bytes'''
    return _to_string('u32toa_range', MAX_U32_SIZE_BASE10_FFI, c_uint32, value)

def u64toa(value):
    '''Format 64-bit unsigned integer to bytes'''
    return _to_string('u64toa_range', MAX_U64_SIZE_BASE10_FFI, c_uint64, value)

def usizetoa(value):
    '''Format size_t to bytes'''
    return _to_string('usizetoa_range', MAX_USIZE_SIZE_BASE10_FFI, c_size_t, value)

def f32toa(value):
    '''Format 32-bit float to bytes'''
    return _to_string('f32toa_range', MAX_F32_SIZE_BASE10_FFI, c_float, value)

def f64toa(value):
    '''Format 64-bit float to bytes'''
    return _to_string('f64toa_range', MAX_F64_SIZE_BASE10_FFI, c_double, value)

# PARSE

def _parse(name, data):
    if not isinstance(data, (bytes, bytearray)):
        raise TypeError("Must parse from bytes.")
    cb = getattr(LIB, name)
    first = _to_u8_ptr(data)
    last = _to_u8_ptr(_to_address(first) + len(data))
    result = cb(first, last)
    if result.is_ok():
        return result.ok()
    else:
        return result.err()

LIB.atoi8_range.restype = ResultI8
LIB.atoi16_range.restype = ResultI16
LIB.atoi32_range.restype = ResultI32
LIB.atoi64_range.restype = ResultI64
LIB.atoisize_range.restype = ResultIsize
LIB.atou8_range.restype = ResultU8
LIB.atou16_range.restype = ResultU16
LIB.atou32_range.restype = ResultU32
LIB.atou64_range.restype = ResultU64
LIB.atousize_range.restype = ResultUsize
LIB.atof32_range.restype = ResultF32
LIB.atof64_range.restype = ResultF64

def atoi8(data):
    '''Parse 8-bit signed integer from bytes.'''
    return _parse('atoi8_range', data)

def atoi16(data):
    '''Parse 16-bit signed integer from bytes.'''
    return _parse('atoi16_range', data)

def atoi32(data):
    '''Parse 32-bit signed integer from bytes.'''
    return _parse('atoi32_range', data)

def atoi64(data):
    '''Parse 64-bit signed integer from bytes.'''
    return _parse('atoi64_range', data)

def atoisize(data):
    '''Parse ssize_t from bytes.'''
    return _parse('atoisize_range', data)

def atou8(data):
    '''Parse 8-bit unsigned integer from bytes.'''
    return _parse('atou8_range', data)

def atou16(data):
    '''Parse 16-bit unsigned integer from bytes.'''
    return _parse('atou16_range', data)

def atou32(data):
    '''Parse 32-bit unsigned integer from bytes.'''
    return _parse('atou32_range', data)

def atou64(data):
    '''Parse 64-bit unsigned integer from bytes.'''
    return _parse('atou64_range', data)

def atousize(data):
    '''Parse size_t from bytes.'''
    return _parse('atousize_range', data)

def atof32(data):
    '''Parse 32-bit float from bytes.'''
    return _parse('atof32_range', data)

def atof64(data):
    '''Parse 64-bit float from bytes.'''
    return _parse('atof64_range', data)

# PARSE_LOSSY

LIB.atof32_lossy_range.restype = ResultF32
LIB.atof64_lossy_range.restype = ResultF64

def atof32_lossy(data):
    '''Parse 32-bit float from bytes.'''
    return _parse('atof32_lossy_range', data)

def atof64_lossy(data):
    '''Parse 64-bit float from bytes.'''
    return _parse('atof64_lossy_range', data)

if hasattr(LIB, 'atof32_radix_range'):
    # Have radix support.

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
        return string_at(buffer, length)

    LIB.i8toa_radix_range.restype = POINTER(c_ubyte)
    LIB.i16toa_radix_range.restype = POINTER(c_ubyte)
    LIB.i32toa_radix_range.restype = POINTER(c_ubyte)
    LIB.i64toa_radix_range.restype = POINTER(c_ubyte)
    LIB.isizetoa_radix_range.restype = POINTER(c_ubyte)
    LIB.u8toa_radix_range.restype = POINTER(c_ubyte)
    LIB.u16toa_radix_range.restype = POINTER(c_ubyte)
    LIB.u32toa_radix_range.restype = POINTER(c_ubyte)
    LIB.u64toa_radix_range.restype = POINTER(c_ubyte)
    LIB.usizetoa_radix_range.restype = POINTER(c_ubyte)
    LIB.f32toa_radix_range.restype = POINTER(c_ubyte)
    LIB.f64toa_radix_range.restype = POINTER(c_ubyte)

    def i8toa_radix(value, radix):
        '''Format 8-bit signed integer to bytes'''
        return _to_string_radix('i8toa_radix_range', MAX_I8_SIZE_BASE10_FFI, c_int8, value, radix)

    def i16toa_radix(value, radix):
        '''Format 16-bit signed integer to bytes'''
        return _to_string_radix('i16toa_radix_range', MAX_I16_SIZE_BASE10_FFI, c_int16, value, radix)

    def i32toa_radix(value, radix):
        '''Format 32-bit signed integer to bytes'''
        return _to_string_radix('i32toa_radix_range', MAX_I32_SIZE_BASE10_FFI, c_int32, value, radix)

    def i64toa_radix(value, radix):
        '''Format 64-bit signed integer to bytes'''
        return _to_string_radix('i64toa_radix_range', MAX_I64_SIZE_BASE10_FFI, c_int64, value, radix)

    def isizetoa_radix(value, radix):
        '''Format ssize_t to bytes'''
        return _to_string_radix('isizetoa_radix_range', MAX_ISIZE_SIZE_BASE10_FFI, c_ssize_t, value, radix)

    def u8toa_radix(value, radix):
        '''Format 8-bit unsigned integer to bytes'''
        return _to_string_radix('u8toa_radix_range', MAX_U8_SIZE_BASE10_FFI, c_uint8, value, radix)

    def u16toa_radix(value, radix):
        '''Format 16-bit unsigned integer to bytes'''
        return _to_string_radix('u16toa_radix_range', MAX_U16_SIZE_BASE10_FFI, c_uint16, value, radix)

    def u32toa_radix(value, radix):
        '''Format 32-bit unsigned integer to bytes'''
        return _to_string_radix('u32toa_radix_range', MAX_U32_SIZE_BASE10_FFI, c_uint32, value, radix)

    def u64toa_radix(value, radix):
        '''Format 64-bit unsigned integer to bytes'''
        return _to_string_radix('u64toa_radix_range', MAX_U64_SIZE_BASE10_FFI, c_uint64, value, radix)

    def usizetoa_radix(value, radix):
        '''Format size_t to bytes'''
        return _to_string_radix('usizetoa_radix_range', MAX_USIZE_SIZE_BASE10_FFI, c_size_t, value, radix)

    def f32toa_radix(value, radix):
        '''Format 32-bit float to bytes'''
        return _to_string_radix('f32toa_radix_range', MAX_F32_SIZE_BASE10_FFI, c_float, value, radix)

    def f64toa_radix(value, radix):
        '''Format 64-bit float to bytes'''
        return _to_string_radix('f64toa_radix_range', MAX_F64_SIZE_BASE10_FFI, c_double, value, radix)

    # PARSE_RADIX
    def _parse_radix(name, radix, data):
        if not isinstance(data, (bytes, bytearray)):
            raise TypeError("Must parse from bytes.")
        if not isinstance(radix, c_uint8):
            radix = c_uint8(radix)
        cb = getattr(LIB, name)
        first = _to_u8_ptr(data)
        last = _to_u8_ptr(_to_address(first) + len(data))
        result = cb(radix, first, last)
        if result.is_ok():
            return result.ok()
        else:
            return result.err()

    LIB.atoi8_radix_range.restype = ResultI8
    LIB.atoi16_radix_range.restype = ResultI16
    LIB.atoi32_radix_range.restype = ResultI32
    LIB.atoi64_radix_range.restype = ResultI64
    LIB.atoisize_radix_range.restype = ResultIsize
    LIB.atou8_radix_range.restype = ResultU8
    LIB.atou16_radix_range.restype = ResultU16
    LIB.atou32_radix_range.restype = ResultU32
    LIB.atou64_radix_range.restype = ResultU64
    LIB.atousize_radix_range.restype = ResultUsize
    LIB.atof32_radix_range.restype = ResultF32
    LIB.atof64_radix_range.restype = ResultF64

    def atoi8_radix(radix, data):
        '''Parse 8-bit signed integer from bytes.'''
        return _parse('atoi8_radix_range', radix, data)

    def atoi16_radix(radix, data):
        '''Parse 16-bit signed integer from bytes.'''
        return _parse('atoi16_radix_range', radix, data)

    def atoi32_radix(radix, data):
        '''Parse 32-bit signed integer from bytes.'''
        return _parse('atoi32_radix_range', radix, data)

    def atoi64_radix(radix, data):
        '''Parse 64-bit signed integer from bytes.'''
        return _parse('atoi64_radix_range', radix, data)

    def atoisize_radix(radix, data):
        '''Parse ssize_t from bytes.'''
        return _parse('atoisize_radix_range', radix, data)

    def atou8_radix(radix, data):
        '''Parse 8-bit unsigned integer from bytes.'''
        return _parse('atou8_radix_range', radix, data)

    def atou16_radix(radix, data):
        '''Parse 16-bit unsigned integer from bytes.'''
        return _parse('atou16_radix_range', radix, data)

    def atou32_radix(radix, data):
        '''Parse 32-bit unsigned integer from bytes.'''
        return _parse('atou32_radix_range', radix, data)

    def atou64_radix(radix, data):
        '''Parse 64-bit unsigned integer from bytes.'''
        return _parse('atou64_radix_range', radix, data)

    def atousize_radix(radix, data):
        '''Parse size_t from bytes.'''
        return _parse('atousize_radix_range', radix, data)

    def atof32_radix(radix, data):
        '''Parse 32-bit float from bytes.'''
        return _parse('atof32_radix_range', radix, data)

    def atof64_radix(radix, data):
        '''Parse 64-bit float from bytes.'''
        return _parse('atof64_radix_range', radix, data)

    # PARSE_LOSSY_RADIX

    LIB.atof32_lossy_radix_range.restype = ResultF32
    LIB.atof64_lossy_radix_range.restype = ResultF64

    def atof32_lossy_radix(radix, data):
        '''Parse 32-bit float from bytes.'''
        return _parse('atof32_lossy_radix_range', radix, data)

    def atof64_lossy_radix(radix, data):
        '''Parse 64-bit float from bytes.'''
        return _parse('atof64_lossy_radix_range', radix, data)
