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

HAVE_FORMAT = hasattr(LIB, 'LEXICAL_HAS_FORMAT')
HAVE_RADIX = hasattr(LIB, 'LEXICAL_HAS_RADIX')
HAVE_ROUNDING = hasattr(LIB, 'LEXICAL_HAS_ROUNDING')
HAVE_I128 = hasattr(LIB, 'LEXICAL_HAS_I128')

# STRING
# ------

def _to_c_string(string):
    '''Get a pointer and size from a string.'''

    if isinstance(string, str):
        string = string.encode('ascii')
    if not isinstance(string, (bytes, bytearray)):
        raise TypeError("Must set string from bytes.")
    ptr = cast(string, POINTER(c_ubyte))
    size = len(string)

    return (ptr, size)


def _from_c_string(ptr, size):
    '''Get a string from a pointer and size.'''
    return string_at(ptr, size).decode('ascii')

# MAGIC
# -----

def _new_init(self):
    '''The default inits improperly work for ctypes initialized on the Rust end.'''

    # If we don't do this, we get default values such as empty
    # strings, and improper types.
    inst = self.__class__.new()
    for field, _ in self._fields_:
        setattr(self, field, getattr(inst, field))

def _struct_eq(self, other):
    '''Check if two structs are equal.'''

    # Do a reciprocal check because we don't know
    # if we have trivial subclasses.
    if not isinstance(self, other.__class__) and not isinstance(other, self.__class__):
        return False

    fields = [i[0] for i in self._fields_]
    x = [getattr(self, i) for i in fields]
    y = [getattr(other, i) for i in fields]
    return x == y


# ROUNDING
# --------

class RoundingKind(enum.Enum):
    '''Rounding type for float-parsing.'''

    NearestTieEven = 0
    NearestTieAwayZero = 1
    TowardPositiveInfinity = 2
    TowardNegativeInfinity = 3
    TowardZero = 4

# OPTION
# ------

# RESULT TAG

class OptionTag(enum.Enum):
    '''Tag for the option tagged enum.'''

    Some = 0
    Nil = 1

def _option(cls, name):
    class Option(Structure):
        value_type = cls
        _fields_ = [
            ("_tag", c_uint32),
            ("_value", cls)
        ]

        def __repr__(self):
            if self.tag == OptionTag.Nil:
                return 'Option(Nil)'
            return f'Option(Some({repr(self._value)}))'

        def __eq__(self, other):
            return _struct_eq(self, other)

        @property
        def tag(self):
            return OptionTag(self._tag)

        @property
        def is_some(self):
            return self.tag == OptionTag.Some

        @property
        def is_nil(self):
            return self.tag == OptionTag.Nil

        def into(self):
            '''Extract value from structure.'''

            if self.tag == OptionTag.Nil:
                raise ValueError(f'Expected value of type {cls.__name__}, got None.')
            return self._value

    Option.__name__ = name
    return Option

# NUMBER FORMAT
# -------------

def _to_flags(character, shift, mask):
    '''Convert a character, shift and mask to flags.'''
    return (ord(character) & mask) << shift

def _from_flags(flag, shift, mask):
    '''Convert a character, shift and mask to flags.'''
    return chr((flag >> shift) & mask).encode('ascii')

def _digit_separator_to_flags(digit_separator):
    '''Convert digit separator byte to 32-bit flags.'''
    return _to_flags(digit_separator, 57, 0x7F)

def _digit_separator_from_flags(flags):
    '''Extract digit separator byte from 32-bit flags.'''
    return _from_flags(flags, 57, 0x7F)

def _exponent_default_to_flags(exponent_default):
    '''Convert exponent default byte to 64-bit flags.'''
    return _to_flags(exponent_default, 18, 0x7F)

def _exponent_default_from_flags(flags):
    '''Extract exponent default byte from 64-bit flags.'''
    return _from_flags(flags, 18, 0x7F)

def _exponent_backup_to_flags(exponent_backup):
    '''Convert exponent backup byte to 64-bit flags.'''
    return _to_flags(exponent_backup, 25, 0x7F)

def _exponent_backup_from_flags(flags):
    '''Extract exponent backup byte from 64-bit flags.'''
    return _from_flags(flags, 25, 0x7F)

def _decimal_point_to_flags(decimal_point):
    '''Convert decimal point byte to 64-bit flags.'''
    return _to_flags(decimal_point, 50, 0x7F)

def _decimal_point_from_flags(flags):
    '''Extract decimal point byte from 64-bit flags.'''
    return _from_flags(flags, 50, 0x7F)


if HAVE_FORMAT:
    class NumberFormatFlags(enum.Flag):
        '''
        Bitflags for a serialized number format.

        See lexical-core/src/util/format/feature_format.rs for
        a more in-depth description.
        '''

        # NON-DIGIT SEPARATOR FLAGS
        RequiredIntegerDigits               = 0b0000000000000000000000000000000000000000000000000000000000000001
        RequiredFractionDigits              = 0b0000000000000000000000000000000000000000000000000000000000000010
        RequiredExponentDigits              = 0b0000000000000000000000000000000000000000000000000000000000000100
        NoPositiveMantissaSign              = 0b0000000000000000000000000000000000000000000000000000000000001000
        RequiredMantissaSign                = 0b0000000000000000000000000000000000000000000000000000000000010000
        NoExponentNotation                  = 0b0000000000000000000000000000000000000000000000000000000000100000
        NoPositiveExponentSign              = 0b0000000000000000000000000000000000000000000000000000000001000000
        RequiredExponentSign                = 0b0000000000000000000000000000000000000000000000000000000010000000
        NoExponentWithoutFraction           = 0b0000000000000000000000000000000000000000000000000000000100000000
        NoSpecial                           = 0b0000000000000000000000000000000000000000000000000000001000000000
        CaseSensitiveSpecial                = 0b0000000000000000000000000000000000000000000000000000010000000000
        NoIntegerLeadingZeros               = 0b0000000000000000000000000000000000000000000000000000100000000000
        NoFloatLeadingZeros                 = 0b0000000000000000000000000000000000000000000000000001000000000000

        # DIGIT SEPARATOR FLAGS
        IntegerInternalDigitSeparator       = 0b0000000000000000000000000000000100000000000000000000000000000000
        IntegerLeadingDigitSeparator        = 0b0000000000000000000000000000001000000000000000000000000000000000
        IntegerTrailingDigitSeparator       = 0b0000000000000000000000000000010000000000000000000000000000000000
        IntegerConsecutiveDigitSeparator    = 0b0000000000000000000000000000100000000000000000000000000000000000
        FractionInternalDigitSeparator      = 0b0000000000000000000000000001000000000000000000000000000000000000
        FractionLeadingDigitSeparator       = 0b0000000000000000000000000010000000000000000000000000000000000000
        FractionTrailingDigitSeparator      = 0b0000000000000000000000000100000000000000000000000000000000000000
        FractionConsecutiveDigitSeparator   = 0b0000000000000000000000001000000000000000000000000000000000000000
        ExponentInternalDigitSeparator      = 0b0000000000000000000000010000000000000000000000000000000000000000
        ExponentLeadingDigitSeparator       = 0b0000000000000000000000100000000000000000000000000000000000000000
        ExponentTrailingDigitSeparator      = 0b0000000000000000000001000000000000000000000000000000000000000000
        ExponentConsecutiveDigitSeparator   = 0b0000000000000000000010000000000000000000000000000000000000000000
        SpecialDigitSeparator               = 0b0000000000000000000100000000000000000000000000000000000000000000

        # MASKS

        RequiredDigits = (
            RequiredIntegerDigits
            | RequiredFractionDigits
            | RequiredExponentDigits
        )

        InternalDigitSeparator = (
            IntegerInternalDigitSeparator
            | FractionInternalDigitSeparator
            | ExponentInternalDigitSeparator
        )

        LeadingDigitSeparator = (
            IntegerLeadingDigitSeparator
            | FractionLeadingDigitSeparator
            | ExponentLeadingDigitSeparator
        )

        TrailingDigitSeparator = (
            IntegerTrailingDigitSeparator
            | FractionTrailingDigitSeparator
            | ExponentTrailingDigitSeparator
        )

        ConsecutiveDigitSeparator = (
            IntegerConsecutiveDigitSeparator
            | FractionConsecutiveDigitSeparator
            | ExponentConsecutiveDigitSeparator
        )

        DigitSeparatorFlagMask = (
            InternalDigitSeparator
            | LeadingDigitSeparator
            | TrailingDigitSeparator
            | ConsecutiveDigitSeparator
            | SpecialDigitSeparator
        )

        IntegerDigitSeparatorFlagMask = (
            IntegerInternalDigitSeparator
            | IntegerLeadingDigitSeparator
            | IntegerTrailingDigitSeparator
            | IntegerConsecutiveDigitSeparator
        )

        FractionDigitSeparatorFlagMask = (
            FractionInternalDigitSeparator
            | FractionLeadingDigitSeparator
            | FractionTrailingDigitSeparator
            | FractionConsecutiveDigitSeparator
        )

        ExponentDigitSeparatorFlagMask = (
            ExponentInternalDigitSeparator
            | ExponentLeadingDigitSeparator
            | ExponentTrailingDigitSeparator
            | ExponentConsecutiveDigitSeparator
        )

        ExponentFlagMask = (
            RequiredExponentDigits
            | NoPositiveExponentSign
            | RequiredExponentSign
            | NoExponentWithoutFraction
            | ExponentInternalDigitSeparator
            | ExponentLeadingDigitSeparator
            | ExponentTrailingDigitSeparator
            | ExponentConsecutiveDigitSeparator
        )

        FlagMask = (
            RequiredDigits
            | NoPositiveMantissaSign
            | RequiredMantissaSign
            | NoExponentNotation
            | NoPositiveExponentSign
            | RequiredExponentSign
            | NoExponentWithoutFraction
            | NoSpecial
            | CaseSensitiveSpecial
            | NoIntegerLeadingZeros
            | NoFloatLeadingZeros
            | InternalDigitSeparator
            | LeadingDigitSeparator
            | TrailingDigitSeparator
            | ConsecutiveDigitSeparator
            | SpecialDigitSeparator
        )
else:
    class NumberFormatFlags(enum.Flag):
        '''
        Bitflags for a serialized number format.

        See lexical-core/src/util/format/not_feature_format.rs for
        a more in-depth description.
        '''

class NumberFormat(Structure):
    '''Immutable wrapper around bitflags for a serialized number format.'''

    _fields_ = [
        ("_value", c_uint64)
    ]

    @property
    def _digit_separator(self):
        return _digit_separator_from_flags(self._value)

    @property
    def _exponent_default(self):
        return _exponent_default_from_flags(self._value)

    @property
    def _exponent_backup(self):
        return _exponent_backup_from_flags(self._value)

    @property
    def _decimal_point(self):
        return _decimal_point_from_flags(self._value)

    @property
    def _flags(self):
        return NumberFormatFlags(self._value & NumberFormatFlags.FlagMask.value)

    # MAGIC

    def __repr__(self):
        return f'NumberFormat(flags={self.flags}, digit_separator={self.digit_separator}, exponent_default={self.exponent_default}, exponent_backup={self.exponent_backup}, decimal_point={self.decimal_point})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    # FUNCTIONS

    def intersects(self, flags):
        '''Determine if a flag'''
        return self._value & flags.value != 0

    def exponent(self, radix):
        '''Get the exponent character based on the radix.'''
        if HAVE_RADIX and radix >= 14:
            return self.exponent_backup
        else:
            return self.exponent_default

    # GETTERS

    @property
    def flags(self):
        '''Get the flag bits from the compiled float format.'''
        return self._flags

    @property
    def digit_separator(self):
        '''Get the digit separator from the compiled float format.'''
        return self._digit_separator

    @property
    def exponent_default(self):
        '''Get the default exponent from the compiled float format.'''
        return self._exponent_default

    @property
    def exponent_backup(self):
        '''Get the backup exponent from the compiled float format.'''
        return self._exponent_backup

    @property
    def decimal_point(self):
        '''Get the decimal point from the compiled float format.'''
        return self._decimal_point

    @property
    def required_integer_digits(self):
        '''Get if digits are required before the decimal point.'''
        return self.intersects(NumberFormatFlags.RequiredIntegerDigits)

    @property
    def required_fraction_digits(self):
        '''Get if digits are required after the decimal point.'''
        return self.intersects(NumberFormatFlags.RequiredFractionDigits)

    @property
    def required_exponent_digits(self):
        '''Get if digits are required after the exponent character.'''
        return self.intersects(NumberFormatFlags.RequiredExponentDigits)

    @property
    def required_digits(self):
        '''Get if digits are required before or after the decimal point.'''
        return self.intersects(NumberFormatFlags.RequiredDigits)

    @property
    def no_positive_mantissa_sign(self):
        '''Get if positive sign before the mantissa is not allowed.'''
        return self.intersects(NumberFormatFlags.NoPositiveMantissaSign)

    @property
    def required_mantissa_sign(self):
        '''Get if positive sign before the mantissa is required.'''
        return self.intersects(NumberFormatFlags.RequiredMantissaSign)

    @property
    def no_exponent_notation(self):
        '''Get if exponent notation is not allowed.'''
        return self.intersects(NumberFormatFlags.NoExponentNotation)

    @property
    def no_positive_exponent_sign(self):
        '''Get if positive sign before the exponent is not allowed.'''
        return self.intersects(NumberFormatFlags.NoPositiveExponentSign)

    @property
    def required_exponent_sign(self):
        '''Get if sign before the exponent is required.'''
        return self.intersects(NumberFormatFlags.RequiredExponentSign)

    @property
    def no_exponent_without_fraction(self):
        '''Get if exponent without fraction is not allowed.'''
        return self.intersects(NumberFormatFlags.NoExponentWithoutFraction)

    @property
    def no_special(self):
        '''Get if special (non-finite) values are not allowed.'''
        return self.intersects(NumberFormatFlags.NoSpecial)

    @property
    def case_sensitive_special(self):
        '''Get if special (non-finite) values are case-sensitive.'''
        return self.intersects(NumberFormatFlags.CaseSensitiveSpecial)

    @property
    def no_integer_leading_zeros(self):
        '''Get if leading zeros before an integer are not allowed.'''
        return self.intersects(NumberFormatFlags.NoIntegerLeadingZeros)

    @property
    def no_float_leading_zeros(self):
        '''Get if leading zeros before a float are not allowed.'''
        return self.intersects(NumberFormatFlags.NoFloatLeadingZeros)

    @property
    def integer_internal_digit_separator(self):
        '''Get if digit separators are allowed between integer digits.'''
        return self.intersects(NumberFormatFlags.IntegerInternalDigitSeparator)

    @property
    def fraction_internal_digit_separator(self):
        '''Get if digit separators are allowed between fraction digits.'''
        return self.intersects(NumberFormatFlags.FractionInternalDigitSeparator)

    @property
    def exponent_internal_digit_separator(self):
        '''Get if digit separators are allowed between exponent digits.'''
        return self.intersects(NumberFormatFlags.ExponentInternalDigitSeparator)

    @property
    def internal_digit_separator(self):
        '''Get if digit separators are allowed between digits.'''
        return self.intersects(NumberFormatFlags.InternalDigitSeparator)

    @property
    def integer_leading_digit_separator(self):
        '''Get if a digit separator is allowed before any integer digits.'''
        return self.intersects(NumberFormatFlags.IntegerLeadingDigitSeparator)

    @property
    def fraction_leading_digit_separator(self):
        '''Get if a digit separator is allowed before any fraction digits.'''
        return self.intersects(NumberFormatFlags.FractionLeadingDigitSeparator)

    @property
    def exponent_leading_digit_separator(self):
        '''Get if a digit separator is allowed before any exponent digits.'''
        return self.intersects(NumberFormatFlags.ExponentLeadingDigitSeparator)

    @property
    def leading_digit_separator(self):
        '''Get if a digit separator is allowed before any digits.'''
        return self.intersects(NumberFormatFlags.LeadingDigitSeparator)

    @property
    def integer_trailing_digit_separator(self):
        '''Get if a digit separator is allowed after any integer digits.'''
        return self.intersects(NumberFormatFlags.IntegerTrailingDigitSeparator)

    @property
    def fraction_trailing_digit_separator(self):
        '''Get if a digit separator is allowed after any fraction digits.'''
        return self.intersects(NumberFormatFlags.FractionTrailingDigitSeparator)

    @property
    def exponent_trailing_digit_separator(self):
        '''Get if a digit separator is allowed after any exponent digits.'''
        return self.intersects(NumberFormatFlags.ExponentTrailingDigitSeparator)

    @property
    def trailing_digit_separator(self):
        '''Get if a digit separator is allowed after any digits.'''
        return self.intersects(NumberFormatFlags.TrailingDigitSeparator)

    @property
    def integer_consecutive_digit_separator(self):
        '''Get if multiple consecutive integer digit separators are allowed.'''
        return self.intersects(NumberFormatFlags.IntegerConsecutiveDigitSeparator)

    @property
    def fraction_consecutive_digit_separator(self):
        '''Get if multiple consecutive fraction digit separators are allowed.'''
        return self.intersects(NumberFormatFlags.FractionConsecutiveDigitSeparator)

    @property
    def exponent_consecutive_digit_separator(self):
        '''Get if multiple consecutive exponent digit separators are allowed.'''
        return self.intersects(NumberFormatFlags.ExponentConsecutiveDigitSeparator)

    @property
    def consecutive_digit_separator(self):
        '''Get if multiple consecutive digit separators are allowed.'''
        return self.intersects(NumberFormatFlags.ConsecutiveDigitSeparator)

    @property
    def special_digit_separator(self):
        '''Get if any digit separators are allowed in special (non-finite) values.'''
        return self.intersects(NumberFormatFlags.SpecialDigitSeparator)

    # BUILDERS

    @staticmethod
    def builder():
        '''Create new builder with default arguments from the Rust API.'''
        return NumberFormatBuilder.new()

    def rebuild(self):
        '''Create NumberFormatBuilder using existing values.'''
        return LIB.lexical_number_format_rebuild(self)


if HAVE_FORMAT:
    # HIDDEN DEFAULTS
    NumberFormat.Permissive = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
    )

    NumberFormat.Ignore = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.DigitSeparatorFlagMask.value
    )

    # PRE-DEFINED CONSTANTS

    # Float format for a Rust literal floating-point number.
    NumberFormat.RustLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Rust float from string.
    NumberFormat.RustString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # `RustString`, but enforces strict equality for special values.
    NumberFormat.RustStringStrict = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Python3 literal floating-point number.
    NumberFormat.Python3Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.NoIntegerLeadingZeros.value
    )

    # Float format to parse a Python3 float from string.
    NumberFormat.Python3String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a Python2 literal floating-point number.
    NumberFormat.Python2Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Python2 float from string.
    NumberFormat.Python2String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for the latest Python literal floating-point number.
    NumberFormat.PythonLiteral = NumberFormat.Python3Literal

    # Float format to parse the latest Python float from string.
    NumberFormat.PythonString = NumberFormat.Python3String

    # Float format for a C++17 literal floating-point number.
    NumberFormat.Cxx17Literal = NumberFormat(
        _digit_separator_to_flags(b'\'')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format to parse a C++17 float from string.
    NumberFormat.Cxx17String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++14 literal floating-point number.
    NumberFormat.Cxx14Literal = NumberFormat(
        _digit_separator_to_flags(b'\'')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format to parse a C++14 float from string.
    NumberFormat.Cxx14String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++11 literal floating-point number.
    NumberFormat.Cxx11Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C++11 float from string.
    NumberFormat.Cxx11String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++03 literal floating-point number.
    NumberFormat.Cxx03Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C++03 float from string.
    NumberFormat.Cxx03String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C++98 literal floating-point number.
    NumberFormat.Cxx98Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C++98 float from string.
    NumberFormat.Cxx98String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for the latest C++ literal floating-point number.
    NumberFormat.CxxLiteral = NumberFormat.Cxx17Literal

    # Float format to parse the latest C++ float from string.
    NumberFormat.CxxString = NumberFormat.Cxx17String

    # Float format for a C18 literal floating-point number.
    NumberFormat.C18Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C18 float from string.
    NumberFormat.C18String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C11 literal floating-point number.
    NumberFormat.C11Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C11 float from string.
    NumberFormat.C11String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C99 literal floating-point number.
    NumberFormat.C99Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a C99 float from string.
    NumberFormat.C99String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C90 literal floating-point number.
    NumberFormat.C90Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C90 float from string.
    NumberFormat.C90String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C89 literal floating-point number.
    NumberFormat.C89Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C89 float from string.
    NumberFormat.C89String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for the latest C literal floating-point number.
    NumberFormat.CLiteral = NumberFormat.C18Literal

    # Float format to parse the latest C float from string.
    NumberFormat.CString = NumberFormat.C18String

    # Float format for a Ruby literal floating-point number.
    NumberFormat.RubyLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.NoFloatLeadingZeros.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format to parse a Ruby float from string.
    NumberFormat.RubyString = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format for a Swift literal floating-point number.
    NumberFormat.SwiftLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Swift float from string.
    NumberFormat.SwiftString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
    )

    # Float format for a Golang literal floating-point number.
    NumberFormat.GoLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Golang float from string.
    NumberFormat.GoString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
    )

    # Float format for a Haskell literal floating-point number.
    NumberFormat.HaskellLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Haskell float from string.
    NumberFormat.HaskellString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Javascript literal floating-point number.
    NumberFormat.JavascriptLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.NoFloatLeadingZeros.value
    )

    # Float format to parse a Javascript float from string.
    NumberFormat.JavascriptString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Perl literal floating-point number.
    NumberFormat.PerlLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | _digit_separator_to_flags(b'_')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.ExponentLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Perl float from string.
    NumberFormat.PerlString = NumberFormat.Permissive

    # Float format for a PHP literal floating-point number.
    NumberFormat.PhpLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a PHP float from string.
    NumberFormat.PhpString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Java literal floating-point number.
    NumberFormat.JavaLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Java float from string.
    NumberFormat.JavaString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a R literal floating-point number.
    NumberFormat.RLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a R float from string.
    NumberFormat.RString = NumberFormat.Permissive

    # Float format for a Kotlin literal floating-point number.
    NumberFormat.KotlinLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.NoIntegerLeadingZeros.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a Kotlin float from string.
    NumberFormat.KotlinString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Julia literal floating-point number.
    NumberFormat.JuliaLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.IntegerInternalDigitSeparator.value
        | NumberFormatFlags.FractionInternalDigitSeparator.value
    )

    # Float format to parse a Julia float from string.
    NumberFormat.JuliaString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a C#7 literal floating-point number.
    NumberFormat.Csharp7Literal = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a C#7 float from string.
    NumberFormat.Csharp7String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#6 literal floating-point number.
    NumberFormat.Csharp6Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#6 float from string.
    NumberFormat.Csharp6String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#5 literal floating-point number.
    NumberFormat.Csharp5Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#5 float from string.
    NumberFormat.Csharp5String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#4 literal floating-point number.
    NumberFormat.Csharp4Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#4 float from string.
    NumberFormat.Csharp4String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#3 literal floating-point number.
    NumberFormat.Csharp3Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#3 float from string.
    NumberFormat.Csharp3String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#2 literal floating-point number.
    NumberFormat.Csharp2Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#2 float from string.
    NumberFormat.Csharp2String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a C#1 literal floating-point number.
    NumberFormat.Csharp1Literal = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a C#1 float from string.
    NumberFormat.Csharp1String = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for the latest C# literal floating-point number.
    NumberFormat.CsharpLiteral = NumberFormat.Csharp7Literal

    # Float format to parse the latest C# float from string.
    NumberFormat.CsharpString = NumberFormat.Csharp7String

    # Float format for a Kawa literal floating-point number.
    NumberFormat.KawaLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Kawa float from string.
    NumberFormat.KawaString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Gambit-C literal floating-point number.
    NumberFormat.GambitcLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Gambit-C float from string.
    NumberFormat.GambitcString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Guile literal floating-point number.
    NumberFormat.GuileLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Guile float from string.
    NumberFormat.GuileString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a Clojure literal floating-point number.
    NumberFormat.ClojureLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Clojure float from string.
    NumberFormat.ClojureString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for an Erlang literal floating-point number.
    NumberFormat.ErlangLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse an Erlang float from string.
    NumberFormat.ErlangString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for an Elm literal floating-point number.
    NumberFormat.ElmLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoIntegerLeadingZeros.value
        | NumberFormatFlags.NoFloatLeadingZeros.value
    )

    # Float format to parse an Elm float from string.
    NumberFormat.ElmString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Scala literal floating-point number.
    NumberFormat.ScalaLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.NoFloatLeadingZeros.value
    )

    # Float format to parse a Scala float from string.
    NumberFormat.ScalaString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for an Elixir literal floating-point number.
    NumberFormat.ElixirLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format to parse an Elixir float from string.
    NumberFormat.ElixirString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a FORTRAN literal floating-point number.
    NumberFormat.FortranLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a FORTRAN float from string.
    NumberFormat.FortranString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
    )

    # Float format for a D literal floating-point number.
    NumberFormat.DLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.NoIntegerLeadingZeros.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a D float from string.
    NumberFormat.DString = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.IntegerInternalDigitSeparator.value
        | NumberFormatFlags.FractionInternalDigitSeparator.value
        | NumberFormatFlags.IntegerTrailingDigitSeparator.value
        | NumberFormatFlags.FractionTrailingDigitSeparator.value
    )

    # Float format for a Coffeescript literal floating-point number.
    NumberFormat.CoffeescriptLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.NoIntegerLeadingZeros.value
        | NumberFormatFlags.NoFloatLeadingZeros.value
    )

    # Float format to parse a Coffeescript float from string.
    NumberFormat.CoffeescriptString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a Cobol literal floating-point number.
    NumberFormat.CobolLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoExponentWithoutFraction.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Cobol float from string.
    NumberFormat.CobolString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentSign.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a F# literal floating-point number.
    NumberFormat.FsharpLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a F# float from string.
    NumberFormat.FsharpString = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
        | NumberFormatFlags.SpecialDigitSeparator.value
    )

    # Float format for a Visual Basic literal floating-point number.
    NumberFormat.VbLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredFractionDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Visual Basic float from string.
    NumberFormat.VbString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for an OCaml literal floating-point number.
    NumberFormat.OcamlLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse an OCaml float from string.
    NumberFormat.OcamlString = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
        | NumberFormatFlags.SpecialDigitSeparator.value
    )

    # Float format for an Objective-C literal floating-point number.
    NumberFormat.ObjectivecLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse an Objective-C float from string.
    NumberFormat.ObjectivecString = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a ReasonML literal floating-point number.
    NumberFormat.ReasonmlLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse a ReasonML float from string.
    NumberFormat.ReasonmlString = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
        | NumberFormatFlags.SpecialDigitSeparator.value
    )

    # Float format for an Octave literal floating-point number.
    NumberFormat.OctaveLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse an Octave float from string.
    NumberFormat.OctaveString = NumberFormat(
        _digit_separator_to_flags(b',')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format for an Matlab literal floating-point number.
    NumberFormat.MatlabLiteral = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.FractionLeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format to parse an Matlab float from string.
    NumberFormat.MatlabString = NumberFormat(
        _digit_separator_to_flags(b',')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.LeadingDigitSeparator.value
        | NumberFormatFlags.TrailingDigitSeparator.value
        | NumberFormatFlags.ConsecutiveDigitSeparator.value
    )

    # Float format for a Zig literal floating-point number.
    NumberFormat.ZigLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredIntegerDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format to parse a Zig float from string.
    NumberFormat.ZigString = NumberFormat.Permissive

    # Float format for a Sage literal floating-point number.
    NumberFormat.SageLiteral = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format to parse a Sage float from string.
    NumberFormat.SageString = NumberFormat(
        _digit_separator_to_flags(b'_')
        | _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.InternalDigitSeparator.value
    )

    # Float format for a JSON literal floating-point number.
    NumberFormat.Json = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoPositiveMantissaSign.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.NoIntegerLeadingZeros.value
        | NumberFormatFlags.NoFloatLeadingZeros.value
    )

    # Float format for a TOML literal floating-point number.
    NumberFormat.Toml = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredDigits.value
        | NumberFormatFlags.NoSpecial.value
        | NumberFormatFlags.InternalDigitSeparator.value
        | NumberFormatFlags.NoFloatLeadingZeros.value
    )

    # Float format for a YAML literal floating-point number.
    NumberFormat.Yaml = NumberFormat.Json

    # Float format for a XML literal floating-point number.
    NumberFormat.Xml = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # Float format for a SQLite literal floating-point number.
    NumberFormat.Sqlite = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a PostgreSQL literal floating-point number.
    NumberFormat.Postgresql = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a MySQL literal floating-point number.
    NumberFormat.Mysql = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.NoSpecial.value
    )

    # Float format for a MongoDB literal floating-point number.
    NumberFormat.Mongodb = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
        | NumberFormatFlags.RequiredExponentDigits.value
        | NumberFormatFlags.CaseSensitiveSpecial.value
    )

    # HIDDEN DEFAULTS
    NumberFormat.Standard = NumberFormat.RustString
else:
    # HIDDEN DEFAULTS
    NumberFormat.Standard = NumberFormat(
        _exponent_default_to_flags(b'e')
        | _exponent_backup_to_flags(b'^')
        | _decimal_point_to_flags(b'.')
    )

OptionNumberFormat = _option(NumberFormat, 'OptionNumberFormat')

# NUMBER FORMAT BUILDER
# ---------------------

if HAVE_FORMAT:
    class NumberFormatBuilder(Structure):
        '''Build number format value from specifications.'''

        _fields_ = [
            ("digit_separator", c_char),
            ("decimal_point", c_char),
            ("exponent_default", c_char),
            ("exponent_backup", c_char),
            ("required_integer_digits", c_bool),
            ("required_fraction_digits", c_bool),
            ("required_exponent_digits", c_bool),
            ("no_positive_mantissa_sign", c_bool),
            ("required_mantissa_sign", c_bool),
            ("no_exponent_notation", c_bool),
            ("no_positive_exponent_sign", c_bool),
            ("required_exponent_sign", c_bool),
            ("no_exponent_without_fraction", c_bool),
            ("no_special", c_bool),
            ("case_sensitive_special", c_bool),
            ("no_integer_leading_zeros", c_bool),
            ("no_float_leading_zeros", c_bool),
            ("integer_internal_digit_separator", c_bool),
            ("fraction_internal_digit_separator", c_bool),
            ("exponent_internal_digit_separator", c_bool),
            ("integer_leading_digit_separator", c_bool),
            ("fraction_leading_digit_separator", c_bool),
            ("exponent_leading_digit_separator", c_bool),
            ("integer_trailing_digit_separator", c_bool),
            ("fraction_trailing_digit_separator", c_bool),
            ("exponent_trailing_digit_separator", c_bool),
            ("integer_consecutive_digit_separator", c_bool),
            ("fraction_consecutive_digit_separator", c_bool),
            ("exponent_consecutive_digit_separator", c_bool),
            ("special_digit_separator", c_bool),
        ]

        def __init__(self):
            _new_init(self)

        def __repr__(self):
            fields = [i[0] for i in self._fields_]
            data = ', '.join([f'{i}={getattr(self, i)}' for i in fields])
            return f'NumberFormatBuilder({data})'

        def __eq__(self, other):
            return _struct_eq(self, other)

        @staticmethod
        def new():
            '''Create new builder with default arguments from the Rust API.'''
            return LIB.lexical_number_format_builder_new()

        def build(self):
            '''Build the NumberFormat from the current values.'''
            return LIB.lexical_number_format_builder_build(self)

else:
    class NumberFormatBuilder(Structure):
        '''Build number format value from specifications.'''

        _fields_ = [
            ("decimal_point", c_char),
            ("exponent_default", c_char),
            ("exponent_backup", c_char),
        ]

        def __init__(self):
            _new_init(self)

        def __repr__(self):
            fields = [i[0] for i in self._fields_]
            data = ', '.join([f'{i}={getattr(self, i)}' for i in fields])
            return f'NumberFormatBuilder({data})'

        def __eq__(self, other):
            return _struct_eq(self, other)

        @staticmethod
        def new():
            '''Create new builder with default arguments from the Rust API.'''
            return LIB.lexical_number_format_builder_new()

        def build(self):
            '''Build the NumberFormat from the current values.'''
            return LIB.lexical_number_format_builder_build(self)

LIB.lexical_number_format_rebuild.restype = NumberFormatBuilder
LIB.lexical_number_format_builder_new.restype = NumberFormatBuilder
LIB.lexical_number_format_builder_build.restype = OptionNumberFormat

# OPTIONS API
# -----------

# PARSE INTEGER OPTIONS

class ParseIntegerOptionsBuilder(Structure):
    '''Builder for `ParseIntegerOptions`.'''

    _fields_ = [
        ('_radix', c_uint8),
        ('_format', OptionNumberFormat),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'ParseIntegerOptions(radix={self.radix}, format={repr(self.format)})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def format(self):
        '''Get the number format.'''
        return self._format

    @format.setter
    def format(self, value):
        '''Set the number format.'''
        self._format = value

    @property
    def radix(self):
        '''Get the radix.'''
        return self._radix

    if HAVE_RADIX:
        @radix.setter
        def radix(self, value):
            '''Set the radix.'''
            self._radix = value

    @staticmethod
    def new():
        '''Create new builder with default arguments from the Rust API.'''
        return LIB.lexical_parse_integer_options_builder_new()

    def build(self):
        '''Build the NumberFormat from the current values.'''
        return LIB.lexical_parse_integer_options_builder_build(self)


class ParseIntegerOptions(Structure):
    '''Options to customize parsing integers.'''

    _fields_ = [
        ('_radix', c_uint32),
        ('_format', OptionNumberFormat),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'ParseIntegerOptions(radix={self.radix}, format={repr(self.format)})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def radix(self):
        '''Get the radix.'''
        return self._radix

    @property
    def format(self):
        '''Get the number format.'''
        return self._format

    @staticmethod
    def new():
        '''Create options with default values.'''
        return LIB.lexical_parse_integer_options_new()

    @staticmethod
    def decimal():
        '''Create new options to parse the default decimal format.'''

        options = ParseIntegerOptions.new()
        options._radix = 10
        return options

    if HAVE_RADIX:
        @staticmethod
        def binary():
            '''Create new options to parse the default binary format.'''

            options = ParseIntegerOptions.new()
            options._radix = 2
            return options

        @staticmethod
        def hexadecimal():
            '''Create new options to parse the default hexadecimal format.'''

            options = ParseIntegerOptions.new()
            options._radix = 16
            return options

    @staticmethod
    def builder():
        '''Get ParseIntegerOptionsBuilder as a static function.'''
        return LIB.lexical_parse_integer_options_builder()

    def rebuild(self):
        '''Create ParseIntegerOptionsBuilder using existing values.'''
        return LIB.lexical_parse_integer_options_rebuild(self)

OptionParseIntegerOptions = _option(ParseIntegerOptions, 'OptionParseIntegerOptions')
LIB.lexical_parse_integer_options_builder_new.restype = ParseIntegerOptionsBuilder
LIB.lexical_parse_integer_options_builder_build.restype = OptionParseIntegerOptions
LIB.lexical_parse_integer_options_new.restype = ParseIntegerOptions
LIB.lexical_parse_integer_options_builder.restype = ParseIntegerOptionsBuilder
LIB.lexical_parse_integer_options_rebuild.restype = ParseIntegerOptionsBuilder

# PARSE FLOAT OPTIONS

class ParseFloatOptionsBuilder(Structure):
    '''Builder for `ParseFloatOptions`.'''

    _fields_ = [
        ('_radix', c_uint8),
        ('_format', NumberFormat),
        ('_rounding', c_uint32),
        ('_incorrect', c_bool),
        ('_lossy', c_bool),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_size', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_size', c_size_t),
        ('_infinity_string_ptr', POINTER(c_ubyte)),
        ('_infinity_string_size', c_size_t),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'ParseFloatOptionsBuilder(radix={self.radix}, format={repr(self.format)}, rounding=repr({self.rounding}), incorrect={self.incorrect}, lossy={self.lossy}, nan_string={self.nan_string}, inf_string={self.inf_string}, infinity_string={self.infinity_string})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def format(self):
        '''Get the number format.'''
        return self._format

    @format.setter
    def format(self, value):
        '''Set the number format.'''
        self._format = value

    @property
    def incorrect(self):
        '''Get if we use the incorrect, fast parser.'''
        return self._incorrect

    @incorrect.setter
    def incorrect(self, value):
        '''Set if we use the incorrect, fast parser.'''
        self._incorrect = value

    @property
    def lossy(self):
        '''Get if we use the lossy, fast parser.'''
        return self._lossy

    @lossy.setter
    def lossy(self, value):
        '''Set if we use the lossy, fast parser.'''
        self._lossy = value

    @property
    def nan_string(self):
        '''Get the string representation for `NaN`.'''
        return _from_c_string(self._nan_string_ptr, self._nan_string_size)

    @nan_string.setter
    def nan_string(self, value):
        '''Set the string representation for `NaN`.'''

        ptr, size = _to_c_string(value)
        self._nan_string_ptr = ptr
        self._nan_string_size = size

    @property
    def inf_string(self):
        '''Get the short string representation for `Infinity`.'''
        return _from_c_string(self._inf_string_ptr, self._inf_string_size)

    @inf_string.setter
    def inf_string(self, value):
        '''Set the short string representation for `Infinity`.'''

        ptr, size = _to_c_string(value)
        self._inf_string_ptr = ptr
        self._inf_string_size = size

    @property
    def infinity_string(self):
        '''Get the long string representation for `Infinity`.'''
        return _from_c_string(self._infinity_string_ptr, self._infinity_string_size)

    @infinity_string.setter
    def infinity_string(self, value):
        '''Set the long string representation for `Infinity`.'''

        ptr, size = _to_c_string(value)
        self._infinity_string_ptr = ptr
        self._infinity_string_size = size

    @property
    def radix(self):
        '''Get the radix.'''
        return self._radix

    @property
    def rounding(self):
        '''Get the rounding kind.'''
        return RoundingKind(self._rounding)

    if HAVE_RADIX:
        @radix.setter
        def radix(self, value):
            '''Set the radix.'''
            self._radix = value

    if HAVE_ROUNDING:
        @rounding.setter
        def rounding(self, value):
            '''Set the rounding kind.'''
            if not isinstance(value, RoundingKind):
                raise TypeError('Expected RoundingKind')
            self._rounding = value.value

    @staticmethod
    def new():
        '''Create new builder with default arguments from the Rust API.'''
        return LIB.lexical_parse_float_options_builder_new()

    def build(self):
        '''Build the NumberFormat from the current values.'''
        return LIB.lexical_parse_float_options_builder_build(self)


class ParseFloatOptions(Structure):
    '''Options to customize parsing floats.'''

    _fields_ = [
        ('_compressed', c_uint32),
        ('_format', NumberFormat),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_size', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_size', c_size_t),
        ('_infinity_string_ptr', POINTER(c_ubyte)),
        ('_infinity_string_size', c_size_t),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'ParseFloatOptions(radix={self.radix}, format={repr(self.format)}, rounding=repr({self.rounding}), incorrect={self.incorrect}, lossy={self.lossy}, nan_string={self.nan_string}, inf_string={self.inf_string}, infinity_string={self.infinity_string})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def radix(self):
        '''Get the radix.'''
        return self._compressed & 0xFF

    @property
    def rounding(self):
        '''Get the rounding kind.'''
        return RoundingKind((self._compressed & 0xFF00) >> 8)

    @property
    def incorrect(self):
        '''Get if we use the incorrect, fast parser.'''
        return self._compressed & 0x10000 != 0

    @property
    def lossy(self):
        '''Get if we use the lossy, fast parser.'''
        return self._compressed & 0x20000 != 0

    @property
    def format(self):
        '''Get the number format.'''
        return self._format

    @property
    def nan_string(self):
        '''Get the string representation for `NaN`.'''
        return _from_c_string(self._nan_string_ptr, self._nan_string_size)

    @property
    def inf_string(self):
        '''Get the short string representation for `Infinity`.'''
        return _from_c_string(self._inf_string_ptr, self._inf_string_size)

    @property
    def infinity_string(self):
        '''Get the long string representation for `Infinity`.'''
        return _from_c_string(self._infinity_string_ptr, self._infinity_string_size)

    @staticmethod
    def new():
        '''Create options with default values.'''
        return LIB.lexical_parse_float_options_new()

    @staticmethod
    def decimal():
        '''Create new options to parse the default decimal format.'''

        options = ParseFloatOptions.new()
        options._compressed &= 0xFFFFFF00
        options._compressed |= 10
        return options

    if HAVE_RADIX:
        @staticmethod
        def binary():
            '''Create new options to parse the default binary format.'''

            options = ParseFloatOptions.new()
            options._compressed &= 0xFFFFFF00
            options._compressed |= 2
            return options

        @staticmethod
        def hexadecimal():
            '''Create new options to parse the default hexadecimal format.'''

            options = ParseFloatOptions.new()
            options._compressed &= 0xFFFFFF00
            options._compressed |= 16
            return options

    @staticmethod
    def builder():
        '''Get ParseFloatOptionsBuilder as a static function.'''
        return LIB.lexical_parse_float_options_builder()

    def rebuild(self):
        '''Create ParseFloatOptionsBuilder using existing values.'''
        return LIB.lexical_parse_float_options_rebuild(self)

OptionParseFloatOptions = _option(ParseFloatOptions, 'OptionParseFloatOptions')
LIB.lexical_parse_float_options_builder_new.restype = ParseFloatOptionsBuilder
LIB.lexical_parse_float_options_builder_build.restype = OptionParseFloatOptions
LIB.lexical_parse_float_options_new.restype = ParseFloatOptions
LIB.lexical_parse_float_options_builder.restype = ParseFloatOptionsBuilder
LIB.lexical_parse_float_options_rebuild.restype = ParseFloatOptionsBuilder

# WRITE INTEGER OPTIONS

class WriteIntegerOptionsBuilder(Structure):
    '''Builder for `WriteIntegerOptions`.'''

    _fields_ = [
        ('_radix', c_uint8),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'WriteIntegerOptions(radix={self.radix})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def radix(self):
        '''Get the radix.'''
        return self._radix

    if HAVE_RADIX:
        @radix.setter
        def radix(self, value):
            '''Set the radix.'''
            self._radix = value

    @staticmethod
    def new():
        '''Create new builder with default arguments from the Rust API.'''
        return LIB.lexical_write_integer_options_builder_new()

    def build(self):
        '''Build the NumberFormat from the current values.'''
        return LIB.lexical_write_integer_options_builder_build(self)


class WriteIntegerOptions(Structure):
    '''Options to customize parsing integers.'''

    _fields_ = [
        ('_radix', c_uint32),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'WriteIntegerOptions(radix={self.radix})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def radix(self):
        '''Get the radix.'''
        return self._radix

    @staticmethod
    def new():
        '''Create options with default values.'''
        return LIB.lexical_write_integer_options_new()

    @staticmethod
    def decimal():
        '''Create new options to write the default decimal format.'''

        options = WriteIntegerOptions.new()
        options._radix = 10
        return options

    if HAVE_RADIX:
        @staticmethod
        def binary():
            '''Create new options to write the default binary format.'''

            options = WriteIntegerOptions.new()
            options._radix = 2
            return options

        @staticmethod
        def hexadecimal():
            '''Create new options to write the default hexadecimal format.'''

            options = WriteIntegerOptions.new()
            options._radix = 16
            return options

    @staticmethod
    def builder():
        '''Get WriteIntegerOptionsBuilder as a static function.'''
        return LIB.lexical_write_integer_options_builder()

    def rebuild(self):
        '''Create WriteIntegerOptionsBuilder using existing values.'''
        return LIB.lexical_write_integer_options_rebuild(self)

OptionWriteIntegerOptions = _option(WriteIntegerOptions, 'OptionWriteIntegerOptions')
LIB.lexical_write_integer_options_builder_new.restype = WriteIntegerOptionsBuilder
LIB.lexical_write_integer_options_builder_build.restype = OptionWriteIntegerOptions
LIB.lexical_write_integer_options_new.restype = WriteIntegerOptions
LIB.lexical_write_integer_options_builder.restype = WriteIntegerOptionsBuilder
LIB.lexical_write_integer_options_rebuild.restype = WriteIntegerOptionsBuilder

# WRITE FLOAT OPTIONS

class WriteFloatOptionsBuilder(Structure):
    '''Builder for `WriteFloatOptions`.'''

    _fields_ = [
        ('_radix', c_uint8),
        ('_format', OptionNumberFormat),
        ('_trim_floats', c_bool),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_size', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_size', c_size_t),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'WriteFloatOptionsBuilder(radix={self.radix}, format={repr(self.format)}, trim_floats={self.trim_floats}, nan_string={self.nan_string}, inf_string={self.inf_string})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def format(self):
        '''Get the number format.'''
        return self._format

    @format.setter
    def format(self, value):
        '''Set the number format.'''
        self._format = value

    @property
    def trim_floats(self):
        '''Get if we should trim a trailing `".0"` from floats.'''
        return self._trim_floats

    @trim_floats.setter
    def trim_floats(self, value):
        '''Set if we should trim a trailing `".0"` from floats.'''
        self._trim_floats = value

    @property
    def nan_string(self):
        '''Get the string representation for `NaN`.'''
        return _from_c_string(self._nan_string_ptr, self._nan_string_size)

    @nan_string.setter
    def nan_string(self, value):
        '''Set the string representation for `NaN`.'''

        ptr, size = _to_c_string(value)
        self._nan_string_ptr = ptr
        self._nan_string_size = size

    @property
    def inf_string(self):
        '''Get the short string representation for `Infinity`.'''
        return _from_c_string(self._inf_string_ptr, self._inf_string_size)

    @inf_string.setter
    def inf_string(self, value):
        '''Set the short string representation for `Infinity`.'''

        ptr, size = _to_c_string(value)
        self._inf_string_ptr = ptr
        self._inf_string_size = size

    @property
    def radix(self):
        '''Get the radix.'''
        return self._radix

    @property
    def rounding(self):
        '''Get the rounding kind.'''
        return RoundingKind(self._rounding)

    if HAVE_RADIX:
        @radix.setter
        def radix(self, value):
            '''Set the radix.'''
            self._radix = value

    @staticmethod
    def new():
        '''Create new builder with default arguments from the Rust API.'''
        return LIB.lexical_write_float_options_builder_new()

    def build(self):
        '''Build the NumberFormat from the current values.'''
        return LIB.lexical_write_float_options_builder_build(self)


class WriteFloatOptions(Structure):
    '''Options to customize parsing floats.'''

    _fields_ = [
        ('_compressed', c_uint32),
        ('_format', OptionNumberFormat),
        ('_nan_string_ptr', POINTER(c_ubyte)),
        ('_nan_string_size', c_size_t),
        ('_inf_string_ptr', POINTER(c_ubyte)),
        ('_inf_string_size', c_size_t),
    ]

    def __init__(self):
        _new_init(self)

    def __repr__(self):
        return f'WriteFloatOptions(radix={self.radix}, format={repr(self.format)}, trim_floats={self.trim_floats}, nan_string={self.nan_string}, inf_string={self.inf_string})'

    def __eq__(self, other):
        return _struct_eq(self, other)

    @property
    def radix(self):
        '''Get the radix.'''
        return self._compressed & 0xFF

    @property
    def trim_floats(self):
        '''Get the radix.'''
        return self._compressed & 0x100 != 0

    @property
    def format(self):
        '''Get the number format.'''
        return self._format

    @property
    def nan_string(self):
        '''Get the string representation for `NaN`.'''
        return _from_c_string(self._nan_string_ptr, self._nan_string_size)

    @property
    def inf_string(self):
        '''Get the short string representation for `Infinity`.'''
        return _from_c_string(self._inf_string_ptr, self._inf_string_size)

    @staticmethod
    def new():
        '''Create options with default values.'''
        return LIB.lexical_write_float_options_new()

    @staticmethod
    def decimal():
        '''Create new options to write the default decimal format.'''

        options = WriteFloatOptions.new()
        options._compressed &= 0xFFFFFF00
        options._compressed |= 10
        return options

    if HAVE_RADIX:
        @staticmethod
        def binary():
            '''Create new options to write the default binary format.'''

            options = WriteFloatOptions.new()
            options._compressed &= 0xFFFFFF00
            options._compressed |= 2
            return options

        @staticmethod
        def hexadecimal():
            '''Create new options to write the default hexadecimal format.'''

            options = WriteFloatOptions.new()
            options._compressed &= 0xFFFFFF00
            options._compressed |= 16
            return options

    @staticmethod
    def builder():
        '''Get WriteFloatOptionsBuilder as a static function.'''
        return LIB.lexical_write_float_options_builder()

    def rebuild(self):
        '''Create WriteFloatOptionsBuilder using existing values.'''
        return LIB.lexical_write_float_options_rebuild(self)

OptionWriteFloatOptions = _option(WriteFloatOptions, 'OptionWriteFloatOptions')
LIB.lexical_write_float_options_builder_new.restype = WriteFloatOptionsBuilder
LIB.lexical_write_float_options_builder_build.restype = OptionWriteFloatOptions
LIB.lexical_write_float_options_new.restype = WriteFloatOptions
LIB.lexical_write_float_options_builder.restype = WriteFloatOptionsBuilder
LIB.lexical_write_float_options_rebuild.restype = WriteFloatOptionsBuilder

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

# 128-BIT INTS
# ------------

if HAVE_I128:
    class c_uint128(Structure):
        '''Wrapper for a 128-bit, unsigned integer.'''

        _fields_ = [
            ('_value', c_char * 16)
        ]

        def __init__(self, value=0):
            if not isinstance(value, int):
                raise TypeError(f'an integer is required (got type {type(value)})')
            # Need to ensure the value is from the range [0, 2**128).
            value = value % (2**128)
            self._value = value.to_bytes(16, sys.byteorder)

        def __repr__(self):
            return f'c_uint128({self.value})'

        def __eq__(self, other):
            return _struct_eq(self, other)

        @property
        def value(self):
            return int.from_bytes(bytes(self._value), sys.byteorder, signed=False)

    class c_int128(Structure):
        '''Wrapper for a 128-bit, signed integer.'''

        _fields_ = [
            ('_value', c_char * 16)
        ]

        def __init__(self, value=0):
            if not isinstance(value, int):
                raise TypeError(f'an integer is required (got type {type(value)})')
            # Need to ensure the value is from the range [0, 2**128).
            # We assume 2's complement, so we use a wrapping behavior
            # and just export the bytes as-is.
            value = value % (2**128)
            self._value = value.to_bytes(16, sys.byteorder)

        def __repr__(self):
            return f'c_int128({self.value})'

        def __eq__(self, other):
            return _struct_eq(self, other)

        @property
        def value(self):
            return int.from_bytes(bytes(self._value), sys.byteorder, signed=True)

# ERROR
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
    InvalidLeadingZeros = -15

class Error(Structure):
    '''C-compatible error for FFI.'''

    _fields_ = [
        ("_code", c_int32),
        ("index", c_size_t)
    ]

    def __repr__(self):
        return f'Error(code={self.code}, index={self.index})'

    def __eq__(self, other):
        return _struct_eq(self, other)

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

    def is_invalid_leading_zeros(self):
        return self.code == ErrorCode.InvalidLeadingZeros

class LexicalError(Exception):
    '''Python-native exception raised during errors in lexical parsing.'''

    def __init__(self, error):
        self.error = error

    def __eq__(self, other):
        if not isinstance(other, LexicalError):
            return False
        return self.error == other.error

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
        elif code == ErrorCode.InvalidPositiveExponentSign:
            return 'Number was found with invalid leading zeros at index {}'.format(self.error.index)
        else:
            raise ValueError('Invalid ErrorCode for lexical error.')

# RESULT
# ------

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
            ("_value", cls),
            ("_error", Error)
        ]

        def __repr__(self):
            return f'{name}(value={repr(self._value)}, index={repr(self._error)})'

        def __eq__(self, other):
            return _struct_eq(self, other)

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

if HAVE_I128:
    UnionI128 = _union(c_int128, 'UnionI128')
    UnionU128 = _union(c_uint128, 'UnionU128')

# COMPLETE RESULTS

def _result(cls, name):
    class Result(Structure):
        union_type = cls
        _fields_ = [
            ("_tag", c_uint32),
            ("_data", cls)
        ]

        def __repr__(self):
            if self.tag == ResultTag.Err:
                return f'Result(Err({self._data._error}))'
            return f'Result(Ok({repr(self._data._value)}))'

        def __eq__(self, other):
            return _struct_eq(self, other)

        @property
        def tag(self):
            return ResultTag(self._tag)

        @property
        def is_ok(self):
            return self.tag == ResultTag.Ok

        @property
        def is_err(self):
            return self.tag == ResultTag.Err

        def into(self):
            '''Extract value from structure.'''

            if self.is_err:
                raise LexicalError(self._data._error)
            # Use this sugar to handle c_uint128/c_int128
            value = self._data._value
            value = getattr(value, 'value', value)
            return value

    Result.__name__ = name
    return Result

ResultI8 = _result(UnionI8, 'ResultI8')
ResultI16 = _result(UnionI16, 'ResultI16')
ResultI32 = _result(UnionI32, 'ResultI32')
ResultI64 = _result(UnionI64, 'ResultI64')
ResultIsize = _result(UnionIsize, 'ResultIsize')
ResultU8 = _result(UnionU8, 'ResultU8')
ResultU16 = _result(UnionU16, 'ResultU16')
ResultU32 = _result(UnionU32, 'ResultU32')
ResultU64 = _result(UnionU64, 'ResultU64')
ResultUsize = _result(UnionUsize, 'ResultUsize')
ResultF32 = _result(UnionF32, 'ResultF32')
ResultF64 = _result(UnionF64, 'ResultF64')

if HAVE_I128:
    ResultI128 = _result(UnionI128, 'ResultI128')
    ResultU128 = _result(UnionU128, 'ResultU128')

# PARTIAL TUPLES

def _partial_tuple(cls, name):
    class Tuple(Structure):
        _fields_ = [
            ("_x", cls),
            ("_y", c_size_t)
        ]

        def __repr__(self):
            return f'Tuple(({repr(self._x)}, {self._y}))'

        def __eq__(self, other):
            return _struct_eq(self, other)

        def into(self):
            '''Extract Python tuple from structure.'''
            # Use this sugar to handle c_uint128/c_int128
            x = getattr(self._x, 'value', self._x)
            return (x, self._y)

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

if HAVE_I128:
    PartialTupleI128 = _partial_tuple(c_int128, 'PartialTupleI128')
    PartialTupleU128 = _partial_tuple(c_uint128, 'PartialTupleU128')

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

if HAVE_I128:
    PartialUnionI128 = _partial_union(PartialTupleI128, 'PartialUnionI128')
    PartialUnionU128 = _partial_union(PartialTupleU128, 'PartialUnionU128')

# PARTIAL RESULTS

def _partial_result(cls, name):
    class PartialResult(Structure):
        union_type = cls
        _fields_ = [
            ("_tag", c_uint32),
            ("_data", cls)
        ]

        def __repr__(self):
            if self._tag == Result.Err:
                return 'Result(Err)'
            return f'Result(Ok({repr(self._data._value.into())}))'

        def __eq__(self, other):
            return _struct_eq(self, other)

        @property
        def tag(self):
            return ResultTag(self._tag)

        @property
        def is_ok(self):
            return self.tag == ResultTag.Ok

        @property
        def is_err(self):
            return self.tag == ResultTag.Err

        def into(self):
            '''Extract value from structure.'''

            if self.tag == ResultTag.Err:
                raise LexicalError(self._data._error)
            return self._data._value.into()

    PartialResult.__name__ = name
    return PartialResult

PartialResultI8 = _partial_result(PartialUnionI8, 'PartialResultI8')
PartialResultI16 = _partial_result(PartialUnionI16, 'PartialResultI16')
PartialResultI32 = _partial_result(PartialUnionI32, 'PartialResultI32')
PartialResultI64 = _partial_result(PartialUnionI64, 'PartialResultI64')
PartialResultIsize = _partial_result(PartialUnionIsize, 'PartialResultIsize')
PartialResultU8 = _partial_result(PartialUnionU8, 'PartialResultU8')
PartialResultU16 = _partial_result(PartialUnionU16, 'PartialResultU16')
PartialResultU32 = _partial_result(PartialUnionU32, 'PartialResultU32')
PartialResultU64 = _partial_result(PartialUnionU64, 'PartialResultU64')
PartialResultUsize = _partial_result(PartialUnionUsize, 'PartialResultUsize')
PartialResultF32 = _partial_result(PartialUnionF32, 'PartialResultF32')
PartialResultF64 = _partial_result(PartialUnionF64, 'PartialResultF64')

if HAVE_I128:
    PartialResultI128 = _partial_result(PartialUnionI128, 'PartialResultI128')
    PartialResultU128 = _partial_result(PartialUnionU128, 'PartialResultU128')

# API
# ---

# HELPERS

def _to_address(ptr):
    return cast(ptr, c_voidp).value

def _to_u8_ptr(address):
    return cast(address, POINTER(c_ubyte))

def _distance(first, last):
    return _to_address(last) - _to_address(first)

# TOSTRING
# ---------

def _to_string(name, max_size, type, value):
    '''Handles all the magic to convert the C-API writers to return Python strings.'''

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

if HAVE_I128:
    LIB.lexical_i128toa.restype = POINTER(c_ubyte)
    LIB.lexical_u128toa.restype = POINTER(c_ubyte)

def i8toa(value):
    '''Format 8-bit signed integer to string.'''
    return _to_string('lexical_i8toa', I8_FORMATTED_SIZE_DECIMAL, c_int8, value)

def i16toa(value):
    '''Format 16-bit signed integer to string.'''
    return _to_string('lexical_i16toa', I16_FORMATTED_SIZE_DECIMAL, c_int16, value)

def i32toa(value):
    '''Format 32-bit signed integer to string.'''
    return _to_string('lexical_i32toa', I32_FORMATTED_SIZE_DECIMAL, c_int32, value)

def i64toa(value):
    '''Format 64-bit signed integer to string.'''
    return _to_string('lexical_i64toa', I64_FORMATTED_SIZE_DECIMAL, c_int64, value)

def isizetoa(value):
    '''Format ssize_t to string.'''
    return _to_string('lexical_isizetoa', ISIZE_FORMATTED_SIZE_DECIMAL, c_ssize_t, value)

def u8toa(value):
    '''Format 8-bit unsigned integer to string.'''
    return _to_string('lexical_u8toa', U8_FORMATTED_SIZE_DECIMAL, c_uint8, value)

def u16toa(value):
    '''Format 16-bit unsigned integer to string.'''
    return _to_string('lexical_u16toa', U16_FORMATTED_SIZE_DECIMAL, c_uint16, value)

def u32toa(value):
    '''Format 32-bit unsigned integer to string.'''
    return _to_string('lexical_u32toa', U32_FORMATTED_SIZE_DECIMAL, c_uint32, value)

def u64toa(value):
    '''Format 64-bit unsigned integer to string.'''
    return _to_string('lexical_u64toa', U64_FORMATTED_SIZE_DECIMAL, c_uint64, value)

def usizetoa(value):
    '''Format size_t to string.'''
    return _to_string('lexical_usizetoa', USIZE_FORMATTED_SIZE_DECIMAL, c_size_t, value)

def f32toa(value):
    '''Format 32-bit float to string.'''
    return _to_string('lexical_f32toa', F32_FORMATTED_SIZE_DECIMAL, c_float, value)

def f64toa(value):
    '''Format 64-bit float to string.'''
    return _to_string('lexical_f64toa', F64_FORMATTED_SIZE_DECIMAL, c_double, value)

if HAVE_I128:
    def i128toa(value):
        '''Format 128-bit signed integer to string.'''
        return _to_string('lexical_i128toa', I128_FORMATTED_SIZE_DECIMAL, c_int128, value)

    def u128toa(value):
        '''Format 128-bit unsigned integer to string.'''
        return _to_string('lexical_u128toa', U128_FORMATTED_SIZE_DECIMAL, c_uint128, value)

# TO STRING OPTIONS
# -----------------

def _to_string_options(name, max_size, type, value, options, options_type):
    '''Handles all the magic to convert the C-API writers to return Python strings.'''

    buffer_type = c_ubyte * max_size
    buffer = buffer_type()
    if not isinstance(value, type):
        value = type(value)
    if not isinstance(options, options_type):
        raise TypeError(f'Expected options of type {options_type.__name__}, got {type(options)}.')
    cb = getattr(LIB, name)
    first = _to_u8_ptr(buffer)
    last = _to_u8_ptr(_to_address(first) + len(buffer))
    ptr = cb(value, first, last, options)
    length = _distance(first, ptr)
    return string_at(buffer, length).decode('ascii')

LIB.lexical_i8toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_i16toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_i32toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_i64toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_isizetoa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u8toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u16toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u32toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_u64toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_usizetoa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_f32toa_with_options.restype = POINTER(c_ubyte)
LIB.lexical_f64toa_with_options.restype = POINTER(c_ubyte)

if HAVE_I128:
    LIB.lexical_i128toa_with_options.restype = POINTER(c_ubyte)
    LIB.lexical_u128toa_with_options.restype = POINTER(c_ubyte)

def i8toa_with_options(value, options):
    '''Format 8-bit signed integer to string with custom writing options.'''
    return _to_string_options('lexical_i8toa_with_options', I8_FORMATTED_SIZE, c_int8, value, options, WriteIntegerOptions)

def i16toa_with_options(value, options):
    '''Format 16-bit signed integer to string with custom writing options.'''
    return _to_string_options('lexical_i16toa_with_options', I16_FORMATTED_SIZE, c_int16, value, options, WriteIntegerOptions)

def i32toa_with_options(value, options):
    '''Format 32-bit signed integer to string with custom writing options.'''
    return _to_string_options('lexical_i32toa_with_options', I32_FORMATTED_SIZE, c_int32, value, options, WriteIntegerOptions)

def i64toa_with_options(value, options):
    '''Format 64-bit signed integer to string with custom writing options.'''
    return _to_string_options('lexical_i64toa_with_options', I64_FORMATTED_SIZE, c_int64, value, options, WriteIntegerOptions)

def isizetoa_with_options(value, options):
    '''Format ssize_t to string with custom writing options.'''
    return _to_string_options('lexical_isizetoa_with_options', ISIZE_FORMATTED_SIZE, c_ssize_t, value, options, WriteIntegerOptions)

def u8toa_with_options(value, options):
    '''Format 8-bit unsigned integer to string with custom writing options.'''
    return _to_string_options('lexical_u8toa_with_options', U8_FORMATTED_SIZE, c_uint8, value, options, WriteIntegerOptions)

def u16toa_with_options(value, options):
    '''Format 16-bit unsigned integer to string with custom writing options.'''
    return _to_string_options('lexical_u16toa_with_options', U16_FORMATTED_SIZE, c_uint16, value, options, WriteIntegerOptions)

def u32toa_with_options(value, options):
    '''Format 32-bit unsigned integer to string with custom writing options.'''
    return _to_string_options('lexical_u32toa_with_options', U32_FORMATTED_SIZE, c_uint32, value, options, WriteIntegerOptions)

def u64toa_with_options(value, options):
    '''Format 64-bit unsigned integer to string with custom writing options.'''
    return _to_string_options('lexical_u64toa_with_options', U64_FORMATTED_SIZE, c_uint64, value, options, WriteIntegerOptions)

def usizetoa_with_options(value, options):
    '''Format size_t to string with custom writing options.'''
    return _to_string_options('lexical_usizetoa_with_options', USIZE_FORMATTED_SIZE, c_size_t, value, options, WriteIntegerOptions)

def f32toa_with_options(value, options):
    '''Format 32-bit float to string with custom writing options.'''
    return _to_string_options('lexical_f32toa_with_options', F32_FORMATTED_SIZE, c_float, value, options, WriteFloatOptions)

def f64toa_with_options(value, options):
    '''Format 64-bit float to string with custom writing options.'''
    return _to_string_options('lexical_f64toa_with_options', F64_FORMATTED_SIZE, c_double, value, options, WriteFloatOptions)

if HAVE_I128:
    def i128toa_with_options(value, options):
        '''Format 128-bit signed integer to string with custom writing options.'''
        return _to_string_options('lexical_i128toa_with_options', I128_FORMATTED_SIZE, c_int128, value, options, WriteIntegerOptions)

    def u128toa_with_options(value, options):
        '''Format 128-bit unsigned integer to string with custom writing options.'''
        return _to_string_options('lexical_u128toa_with_options', U128_FORMATTED_SIZE, c_uint128, value, options, WriteIntegerOptions)

# PARSE
# -----

# PARSE

def _parse(name, data):
    '''Converts a string or bytes-like object to a native Python integer.'''

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

if HAVE_I128:
    LIB.lexical_atoi128.restype = ResultI128
    LIB.lexical_atou128.restype = ResultU128

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

if HAVE_I128:
    def atoi128(data):
        '''Parse 128-bit signed integer from input data.'''
        return _parse('lexical_atoi128', data)

    def atou128(data):
        '''Parse 128-bit unsigned integer from input data.'''
        return _parse('lexical_atou128', data)

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

if HAVE_I128:
    LIB.lexical_atoi128_partial.restype = PartialResultI128
    LIB.lexical_atou128_partial.restype = PartialResultU128

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

if HAVE_I128:
    def atoi128_partial(data):
        '''Parse 128-bit signed integer and the number of processed bytes from input data.'''
        return _parse('lexical_atoi128_partial', data)

    def atou128_partial(data):
        '''Parse 128-bit unsigned integer and the number of processed bytes from input data.'''
        return _parse('lexical_atou128_partial', data)

# PARSE WITH OPTIONS
# ------------------

# PARSE

def _parse_options(name, data, options, options_type):
    '''Converts a string or bytes-like object to a native Python integer.'''

    if isinstance(data, str):
        data = data.encode('ascii')
    if not isinstance(data, (bytes, bytearray)):
        raise TypeError("Must parse from bytes.")
    if not isinstance(options, options_type):
        raise TypeError(f'Expected options of type {options_type.__name__}, got {type(options)}.')
    cb = getattr(LIB, name)
    first = _to_u8_ptr(data)
    last = _to_u8_ptr(_to_address(first) + len(data))
    result = cb(first, last, options)
    return result.into()

# COMPLETE PARSE

LIB.lexical_atoi8_with_options.restype = ResultI8
LIB.lexical_atoi16_with_options.restype = ResultI16
LIB.lexical_atoi32_with_options.restype = ResultI32
LIB.lexical_atoi64_with_options.restype = ResultI64
LIB.lexical_atoisize_with_options.restype = ResultIsize
LIB.lexical_atou8_with_options.restype = ResultU8
LIB.lexical_atou16_with_options.restype = ResultU16
LIB.lexical_atou32_with_options.restype = ResultU32
LIB.lexical_atou64_with_options.restype = ResultU64
LIB.lexical_atousize_with_options.restype = ResultUsize
LIB.lexical_atof32_with_options.restype = ResultF32
LIB.lexical_atof64_with_options.restype = ResultF64

if HAVE_I128:
    LIB.lexical_atoi128_with_options.restype = ResultI128
    LIB.lexical_atou128_with_options.restype = ResultU128

def atoi8_with_options(data, options):
    '''Parse 8-bit signed integer from input data with parsing options.'''
    return _parse_options('lexical_atoi8_with_options', data, options, ParseIntegerOptions)

def atoi16_with_options(data, options):
    '''Parse 16-bit signed integer from input data with parsing options.'''
    return _parse_options('lexical_atoi16_with_options', data, options, ParseIntegerOptions)

def atoi32_with_options(data, options):
    '''Parse 32-bit signed integer from input data with parsing options.'''
    return _parse_options('lexical_atoi32_with_options', data, options, ParseIntegerOptions)

def atoi64_with_options(data, options):
    '''Parse 64-bit signed integer from input data with parsing options.'''
    return _parse_options('lexical_atoi64_with_options', data, options, ParseIntegerOptions)

def atoisize_with_options(data, options):
    '''Parse ssize_t from input data with parsing options.'''
    return _parse_options('lexical_atoisize_with_options', data, options, ParseIntegerOptions)

def atou8_with_options(data, options):
    '''Parse 8-bit unsigned integer from input data with parsing options.'''
    return _parse_options('lexical_atou8_with_options', data, options, ParseIntegerOptions)

def atou16_with_options(data, options):
    '''Parse 16-bit unsigned integer from input data with parsing options.'''
    return _parse_options('lexical_atou16_with_options', data, options, ParseIntegerOptions)

def atou32_with_options(data, options):
    '''Parse 32-bit unsigned integer from input data with parsing options.'''
    return _parse_options('lexical_atou32_with_options', data, options, ParseIntegerOptions)

def atou64_with_options(data, options):
    '''Parse 64-bit unsigned integer from input data with parsing options.'''
    return _parse_options('lexical_atou64_with_options', data, options, ParseIntegerOptions)

def atousize_with_options(data, options):
    '''Parse size_t from input data with parsing options.'''
    return _parse_options('lexical_atousize_with_options', data, options, ParseIntegerOptions)

def atof32_with_options(data, options):
    '''Parse 32-bit float from input data with parsing options.'''
    return _parse_options('lexical_atof32_with_options', data, options, ParseFloatOptions)

def atof64_with_options(data, options):
    '''Parse 64-bit float from input data with parsing options.'''
    return _parse_options('lexical_atof64_with_options', data, options, ParseFloatOptions)

if HAVE_I128:
    def atoi128_with_options(data, options):
        '''Parse 128-bit signed integer from input data with parsing options.'''
        return _parse_options('lexical_atoi128_with_options', data, options, ParseIntegerOptions)

    def atou128_with_options(data, options):
        '''Parse 128-bit unsigned integer from input data with parsing options.'''
        return _parse_options('lexical_atou128_with_options', data, options, ParseIntegerOptions)

# PARTIAL PARSE

LIB.lexical_atoi8_partial_with_options.restype = PartialResultI8
LIB.lexical_atoi16_partial_with_options.restype = PartialResultI16
LIB.lexical_atoi32_partial_with_options.restype = PartialResultI32
LIB.lexical_atoi64_partial_with_options.restype = PartialResultI64
LIB.lexical_atoisize_partial_with_options.restype = PartialResultIsize
LIB.lexical_atou8_partial_with_options.restype = PartialResultU8
LIB.lexical_atou16_partial_with_options.restype = PartialResultU16
LIB.lexical_atou32_partial_with_options.restype = PartialResultU32
LIB.lexical_atou64_partial_with_options.restype = PartialResultU64
LIB.lexical_atousize_partial_with_options.restype = PartialResultUsize
LIB.lexical_atof32_partial_with_options.restype = PartialResultF32
LIB.lexical_atof64_partial_with_options.restype = PartialResultF64

if HAVE_I128:
    LIB.lexical_atoi128_partial_with_options.restype = PartialResultI128
    LIB.lexical_atou128_partial_with_options.restype = PartialResultU128

def atoi8_partial_with_options(data, options):
    '''Parse 8-bit signed integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atoi8_partial_with_options', data, options, ParseIntegerOptions)

def atoi16_partial_with_options(data, options):
    '''Parse 16-bit signed integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atoi16_partial_with_options', data, options, ParseIntegerOptions)

def atoi32_partial_with_options(data, options):
    '''Parse 32-bit signed integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atoi32_partial_with_options', data, options, ParseIntegerOptions)

def atoi64_partial_with_options(data, options):
    '''Parse 64-bit signed integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atoi64_partial_with_options', data, options, ParseIntegerOptions)

def atoisize_partial_with_options(data, options):
    '''Parse ssize_t and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atoisize_partial_with_options', data, options, ParseIntegerOptions)

def atou8_partial_with_options(data, options):
    '''Parse 8-bit unsigned integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atou8_partial_with_options', data, options, ParseIntegerOptions)

def atou16_partial_with_options(data, options):
    '''Parse 16-bit unsigned integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atou16_partial_with_options', data, options, ParseIntegerOptions)

def atou32_partial_with_options(data, options):
    '''Parse 32-bit unsigned integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atou32_partial_with_options', data, options, ParseIntegerOptions)

def atou64_partial_with_options(data, options):
    '''Parse 64-bit unsigned integer and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atou64_partial_with_options', data, options, ParseIntegerOptions)

def atousize_partial_with_options(data, options):
    '''Parse size_t and the number of processed bytes from input data with parsing options.'''
    return _parse_options('lexical_atousize_partial_with_options', data, options, ParseIntegerOptions)

def atof32_partial_with_options(data, options):
    '''Parse 32-bit float and the number of processed bytes from bytes with parsing options.'''
    return _parse_options('lexical_atof32_partial_with_options', data, options, ParseFloatOptions)

def atof64_partial_with_options(data, options):
    '''Parse 64-bit float and the number of processed bytes from bytes with parsing options.'''
    return _parse_options('lexical_atof64_partial_with_options', data, options, ParseFloatOptions)

if HAVE_I128:
    def atoi128_partial_with_options(data, options):
        '''Parse 128-bit signed integer and the number of processed bytes from input data with parsing options.'''
        return _parse_options('lexical_atoi128_partial_with_options', data, options, ParseIntegerOptions)

    def atou128_partial_with_options(data, options):
        '''Parse 128-bit unsigned integer and the number of processed bytes from input data with parsing options.'''
        return _parse_options('lexical_atou128_partial_with_options', data, options, ParseIntegerOptions)
