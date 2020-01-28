/**
 *  lexical_core
 *  ============
 *
 *  Access lexical-core functionality from C.
 *
 *  License
 *  -------
 *
 *  This is free and unencumbered software released into the public domain.
 *
 *  Anyone is free to copy, modify, publish, use, compile, sell, or
 *  distribute this software, either in source code form or as a compiled
 *  binary, for any purpose, commercial or non-commercial, and by any
 *  means.
 *
 *  In jurisdictions that recognize copyright laws, the author or authors
 *  of this software dedicate any and all copyright interest in the
 *  software to the public domain. We make this dedication for the benefit
 *  of the public at large and to the detriment of our heirs and
 *  successors. We intend this dedication to be an overt act of
 *  relinquishment in perpetuity of all present and future rights to this
 *  software under copyright law.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 *  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 *  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 *  IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 *  OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 *  ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 *  OTHER DEALINGS IN THE SOFTWARE.
 *
 *  For more information, please refer to <http://unlicense.org/>
 */

#ifndef LEXICAL_H_
#define LEXICAL_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Features must be enabled through the following macro definitions:
//  1. HAVE_RADIX
//  2. HAVE_ROUNDING

// STATIC ASSERT
// -------------

// Static assertion implementation for C.
#define lexical_static_assert(condition, message)                               \
        typedef char lexical_static_assertion_##message[(condition)?1:-1]

// CONFIG
// ------

extern uint8_t lexical_get_exponent_default_char();
extern void lexical_set_exponent_default_char(uint8_t ch);

#ifdef HAVE_RADIX
    extern uint8_t lexical_get_exponent_backup_char();
    extern void lexical_set_exponent_backup_char(uint8_t ch);
#endif  // HAVE_RADIX

#ifdef HAVE_ROUNDING
    // Rounding type for float-parsing.
    enum lexical_rounding_kind {
        // Round to the nearest, tie to even.
        lexical_nearest_tie_even = 0,
        // Round to the nearest, tie away from zero.
        lexical_nearest_tie_away_zero = 1,
        // Round toward positive infinity.
        lexical_toward_positive_infinity = 2,
        // Round toward negative infinity.
        lexical_toward_negative_infinity = 3,
        // Round toward zero.
        lexical_toward_zero = 4,
    };
    extern int32_t lexical_get_float_rounding();
    extern void lexical_set_float_rounding(int32_t rounding);
#endif  // HAVE_ROUNDING

extern int32_t lexical_get_nan_string(uint8_t** ptr, size_t* size);
extern int32_t lexical_set_nan_string(uint8_t const* ptr, size_t size);
extern int32_t lexical_get_inf_string(uint8_t** ptr, size_t* size);
extern int32_t lexical_set_inf_string(uint8_t const* ptr, size_t size);
extern int32_t lexical_get_infinity_string(uint8_t** ptr, size_t* size);
extern int32_t lexical_set_infinity_string(uint8_t const* ptr, size_t size);

// CONSTANTS
// ---------

extern const size_t LEXICAL_I8_FORMATTED_SIZE;
extern const size_t LEXICAL_I16_FORMATTED_SIZE;
extern const size_t LEXICAL_I32_FORMATTED_SIZE;
extern const size_t LEXICAL_I64_FORMATTED_SIZE;
extern const size_t LEXICAL_ISIZE_FORMATTED_SIZE;
extern const size_t LEXICAL_U8_FORMATTED_SIZE;
extern const size_t LEXICAL_U16_FORMATTED_SIZE;
extern const size_t LEXICAL_U32_FORMATTED_SIZE;
extern const size_t LEXICAL_U64_FORMATTED_SIZE;
extern const size_t LEXICAL_USIZE_FORMATTED_SIZE;
extern const size_t LEXICAL_F32_FORMATTED_SIZE;
extern const size_t LEXICAL_F64_FORMATTED_SIZE;

extern const size_t LEXICAL_I8_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_I16_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_I32_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_I64_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_U8_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_U16_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_U32_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_U64_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_F32_FORMATTED_SIZE_DECIMAL;
extern const size_t LEXICAL_F64_FORMATTED_SIZE_DECIMAL;

extern const size_t LEXICAL_BUFFER_SIZE;

// TYPES
// -----

// ALIAS

typedef int8_t lexical_i8;
typedef int16_t lexical_i16;
typedef int32_t lexical_i32;
typedef int64_t lexical_i64;
typedef ptrdiff_t lexical_isize;

typedef uint8_t lexical_u8;
typedef uint16_t lexical_u16;
typedef uint32_t lexical_u32;
typedef uint64_t lexical_u64;
typedef size_t lexical_usize;

typedef float lexical_f32;
typedef double lexical_f64;

// Assert lexical_isize and lexical_usize are the same size (otherwise this won't work.)
lexical_static_assert(sizeof(lexical_isize) == sizeof(lexical_usize), size_type);

// Get type from type name.
#define lexical_type(type) lexical_##type

// ERROR

// Error code, indicating failure type.
enum lexical_error_code {
    // Integral overflow occurred during numeric parsing.
    lexical_overflow = -1,
    // Integral underflow occurred during numeric parsing.
    lexical_underflow = -2,
    // Invalid digit found before string termination.
    lexical_invalid_digit = -3,
    // Empty byte array found.
    lexical_empty = -4,
    // Empty mantissa found.
    lexical_empty_mantissa = -5,
    // Empty exponent found.
    lexical_empty_exponent = -6,
    // Empty integer found.
    lexical_empty_integer = -7,
    // Empty fraction found.
    lexical_empty_fraction = -8,
    // Invalid positive mantissa sign was found.
    lexical_invalid_positive_mantissa_sign = -9,
    // Mantissa sign was required, but not found.
    lexical_missing_mantissa_sign = -10,
    // Exponent was present but not allowed.
    lexical_invalid_exponent = -11,
    // invalid positive exponent sign was found.
    lexical_invalid_positive_exponent_sign = -12,
    // Exponent sign was required, but not found.
    lexical_missing_exponent_sign = -13,
    // Exponent was present without fraction component.
    lexical_exponent_without_fraction = -14,
    // Integer had invalid leading zeros.
    lexical_invalid_leading_zeros = -15,
};

// Declare extern to lexical function definitions.
#ifdef HAVE_FORMAT
    // Convert digit separator to flags.
    #define lexical_digit_separator_to_flags(ch) (((uint64_t) ch) << 56)

    // Extract digit separator from flags.
    #define lexical_digit_separator_from_flags(flag) ((uint8_t) (flag >> 56))

    // Determine if character is valid ASCII.
    inline bool lexical_is_ascii(uint8_t ch)
    {
        return ch < 0x80;
    }

    // Determine if the digit separator is valid.
    inline bool lexical_is_valid_separator(uint8_t ch);

    #ifdef HAVE_RADIX
        inline bool lexical_is_valid_separator(uint8_t ch)
        {
            uint8_t a_upper = 'A';
            uint8_t z_upper = 'Z';
            uint8_t a_lower = 'a';
            uint8_t z_lower = 'z';
            uint8_t zero = '0';
            uint8_t nine = '9';
            uint8_t plus = '+';
            uint8_t period = '.';
            uint8_t minus = '-';
            if (ch >= a_upper && ch <= z_upper) {
                return false;
            } else if (ch >= a_lower && ch <= z_lower) {
                return false;
            } else if (ch >= zero && ch <= nine) {
                return false;
            } else if (ch == plus || ch == period || ch == minus) {
                return false;
            } else {
                return (
                    lexical_is_ascii(ch)
                    && ch != lexical_get_exponent_default_char()
                    && ch != lexical_get_exponent_backup_char()
                );
            }
        }
    #else   // HAVE_RADIX
        inline bool lexical_is_valid_separator(uint8_t ch)
        {
            uint8_t zero = '0';
            uint8_t nine = '9';
            uint8_t plus = '+';
            uint8_t period = '.';
            uint8_t minus = '-';
            if (ch >= zero && ch <= nine) {
                return false;
            } else if (ch == plus || ch == period || ch == minus) {
                return false;
            } else {
                return (
                    lexical_is_ascii(ch)
                    && ch != lexical_get_exponent_default_char()
                );
            }
        }
    #endif  // HAVE_RADIX

    // BITFLAGS

    // Bitflags for a serialized number format.
    // Use macros because we have no other choices for 64-bit values.
    //  We either need to have a compilation unit, which won't work,
    //  or an enumeration, which also won't work.
    // NON-DIGIT SEPARATOR FLAGS
    #define lexical_required_integer_digits                 0x1ull
    #define lexical_required_fraction_digits                0x2ull
    #define lexical_required_exponent_digits                0x4ull
    #define lexical_no_positive_mantissa_sign               0x8ull
    #define lexical_required_mantissa_sign                  0x10ull
    #define lexical_no_exponent_notation                    0x20ull
    #define lexical_no_positive_exponent_sign               0x40ull
    #define lexical_required_exponent_sign                  0x80ull
    #define lexical_no_exponent_without_fraction            0x100ull
    #define lexical_no_special                              0x200ull
    #define lexical_case_sensitive_special                  0x400ull
    #define lexical_no_integer_leading_zeros                0x800ull
    #define lexical_no_float_leading_zeros                  0x1000ull

    // DIGIT SEPARATOR FLAGS
    #define lexical_integer_internal_digit_separator        0x100000000ull
    #define lexical_integer_leading_digit_separator         0x200000000ull
    #define lexical_integer_trailing_digit_separator        0x400000000ull
    #define lexical_integer_consecutive_digit_separator     0x800000000ull
    #define lexical_fraction_internal_digit_separator       0x1000000000ull
    #define lexical_fraction_leading_digit_separator        0x2000000000ull
    #define lexical_fraction_trailing_digit_separator       0x4000000000ull
    #define lexical_fraction_consecutive_digit_separator    0x8000000000ull
    #define lexical_exponent_internal_digit_separator       0x10000000000ull
    #define lexical_exponent_leading_digit_separator        0x20000000000ull
    #define lexical_exponent_trailing_digit_separator       0x40000000000ull
    #define lexical_exponent_consecutive_digit_separator    0x80000000000ull
    #define lexical_special_digit_separator                 0x100000000000ull

    // MASKS
    #define lexical_required_digits (                                   \
        lexical_required_integer_digits                                 \
            | lexical_required_fraction_digits                          \
            | lexical_required_exponent_digits                          \
    )

    #define lexical_internal_digit_separator (                          \
        lexical_integer_internal_digit_separator                        \
        | lexical_fraction_internal_digit_separator                     \
        | lexical_exponent_internal_digit_separator                     \
    )

    #define lexical_leading_digit_separator (                           \
        lexical_integer_leading_digit_separator                         \
        | lexical_fraction_leading_digit_separator                      \
        | lexical_exponent_leading_digit_separator                      \
    )

    #define lexical_trailing_digit_separator (                          \
        lexical_integer_trailing_digit_separator                        \
        | lexical_fraction_trailing_digit_separator                     \
        | lexical_exponent_trailing_digit_separator                     \
    )

    #define lexical_consecutive_digit_separator (                       \
        lexical_integer_consecutive_digit_separator                     \
        | lexical_fraction_consecutive_digit_separator                  \
        | lexical_exponent_consecutive_digit_separator                  \
    )

    #define lexical_digit_separator_flag_mask (                         \
        lexical_internal_digit_separator                                \
        | lexical_leading_digit_separator                               \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
        | lexical_special_digit_separator                               \
    )

    #define lexical_integer_digit_separator_flag_mask (                 \
        lexical_integer_internal_digit_separator                        \
        | lexical_integer_leading_digit_separator                       \
        | lexical_integer_trailing_digit_separator                      \
        | lexical_integer_consecutive_digit_separator                   \
    )

    #define lexical_fraction_digit_separator_flag_mask (                \
        lexical_fraction_internal_digit_separator                       \
        | lexical_fraction_leading_digit_separator                      \
        | lexical_fraction_trailing_digit_separator                     \
        | lexical_fraction_consecutive_digit_separator                  \
    )

    #define lexical_exponent_digit_separator_flag_mask (                \
        lexical_exponent_internal_digit_separator                       \
        | lexical_exponent_leading_digit_separator                      \
        | lexical_exponent_trailing_digit_separator                     \
        | lexical_exponent_consecutive_digit_separator                  \
    )

    #define lexical_exponent_flag_mask (                                \
        lexical_required_exponent_digits                                \
        | lexical_no_positive_exponent_sign                             \
        | lexical_required_exponent_sign                                \
        | lexical_no_exponent_without_fraction                          \
        | lexical_exponent_internal_digit_separator                     \
        | lexical_exponent_leading_digit_separator                      \
        | lexical_exponent_trailing_digit_separator                     \
        | lexical_exponent_consecutive_digit_separator                  \
    )

    #define lexical_flag_mask (                                         \
        lexical_required_digits                                         \
        | lexical_no_positive_mantissa_sign                             \
        | lexical_required_mantissa_sign                                \
        | lexical_no_exponent_notation                                  \
        | lexical_no_positive_exponent_sign                             \
        | lexical_required_exponent_sign                                \
        | lexical_no_exponent_without_fraction                          \
        | lexical_no_special                                            \
        | lexical_case_sensitive_special                                \
        | lexical_no_integer_leading_zeros                              \
        | lexical_no_float_leading_zeros                                \
        | lexical_internal_digit_separator                              \
        | lexical_leading_digit_separator                               \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
        | lexical_special_digit_separator                               \
    )

    // PRE-DEFINED
    // Note:
    //  The pre-defined enum definitions are the public API for
    //  lexical_number_format.

    // Float format for a Rust literal floating-point number.
    #define lexical_rust_literal (                                      \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_digits                                       \
        | lexical_no_positive_mantissa_sign                             \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a Rust float from string.
    #define lexical_rust_string lexical_required_exponent_digits

    // `RUST_STRING`, but enforces strict equality for special values.
    #define lexical_rust_string_strict (                                \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a Python literal floating-point number.
    #define lexical_python_literal (                                    \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a Python float from string.
    #define lexical_python_string lexical_required_exponent_digits

    // Float format for a C++17 literal floating-point number.
    #define lexical_cxx17_literal (                                     \
        lexical_digit_separator_to_flags('\'')                          \
        | lexical_required_exponent_digits                              \
        | lexical_case_sensitive_special                                \
        | lexical_internal_digit_separator                              \
    )

    // Float format for a C++17 float from string.
    #define lexical_cxx17_string lexical_required_exponent_digits

    // Float format for a C++14 literal floating-point number.
    #define lexical_cxx14_literal (                                     \
        lexical_digit_separator_to_flags('\'')                          \
        | lexical_required_exponent_digits                              \
        | lexical_case_sensitive_special                                \
        | lexical_internal_digit_separator                              \
    )

    // Float format to parse a C++14 float from string.
    #define lexical_cxx14_string lexical_required_exponent_digits

    // Float format for a C++11 literal floating-point number.
    #define lexical_cxx11_literal (                                     \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a C++11 float from string.
    #define lexical_cxx11_string lexical_required_exponent_digits

    // Float format for a C++03 literal floating-point number.
    #define lexical_cxx03_literal (                                     \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a C++03 float from string.
    #define lexical_cxx03_string lexical_required_exponent_digits

    // Float format for a C++98 literal floating-point number.
    #define lexical_cxx98_literal (                                     \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a C++98 float from string.
    #define lexical_cxx98_string lexical_required_exponent_digits

    // Float format for a C18 literal floating-point number.
    #define lexical_c18_literal (                                       \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a C18 float from string.
    #define lexical_c18_string lexical_required_exponent_digits

    // Float format for a C11 literal floating-point number.
    #define lexical_c11_literal (                                       \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a C11 float from string.
    #define lexical_c11_string lexical_required_exponent_digits

    // Float format for a C99 literal floating-point number.
    #define lexical_c99_literal (                                       \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a C99 float from string.
    #define lexical_c99_string lexical_required_exponent_digits

    // Float format for a C90 literal floating-point number.
    #define lexical_c90_literal (                                       \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a C90 float from string.
    #define lexical_c90_string lexical_required_exponent_digits

    // Float format for a C89 literal floating-point number.
    #define lexical_c89_literal (                                       \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a C89 float from string.
    #define lexical_c89_string lexical_required_exponent_digits

    // Float format for a Ruby literal floating-point number.
    #define lexical_ruby_literal (                                      \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_digits                                       \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
    )

    // Float format to parse a Ruby float from string.
    #define lexical_ruby_string (                                       \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
    )

    // Float format for a Swift literal floating-point number.
    #define lexical_swift_literal (                                     \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_digits                                       \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a Swift float from string.
    #define lexical_swift_string lexical_required_fraction_digits

    // Float format for a Golang literal floating-point number.
    #define lexical_go_literal (                                        \
        lexical_required_fraction_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a Golang float from string.
    #define lexical_go_string lexical_required_fraction_digits

    // Float format for a Haskell literal floating-point number.
    #define lexical_haskell_literal (                                   \
        lexical_required_digits                                         \
        | lexical_no_positive_mantissa_sign                             \
        | lexical_no_special                                            \
    )

    // Float format to parse a Haskell float from string.
    #define lexical_haskell_string (                                    \
        lexical_required_digits                                         \
        | lexical_no_positive_mantissa_sign                             \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a Javascript literal floating-point number.
    #define lexical_javascript_literal (                                \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a Javascript float from string.
    #define lexical_javascript_string lexical_case_sensitive_special

    // Float format for a Perl literal floating-point number.
    #define lexical_perl_literal (                                      \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
        | lexical_fraction_leading_digit_separator                      \
        | lexical_exponent_leading_digit_separator                      \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a Perl float from string.
    #define lexical_perl_string 0ull

    // Float format for a PHP literal floating-point number.
    #define lexical_php_literal (                                       \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a PHP float from string.
    #define lexical_php_string lexical_no_special

    // Float format for a Java literal floating-point number.
    #define lexical_java_literal (                                      \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a Java float from string.
    #define lexical_java_string (                                       \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a R literal floating-point number.
    #define lexical_r_literal (                                         \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a R float from string.
    #define lexical_r_string 0ull

    // Float format for a Kotlin literal floating-point number.
    #define lexical_kotlin_literal (                                    \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a Kotlin float from string.
    #define lexical_kotlin_string (                                     \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a Julia literal floating-point number.
    #define lexical_julia_literal (                                     \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_case_sensitive_special                                \
        | lexical_integer_internal_digit_separator                      \
        | lexical_fraction_internal_digit_separator                     \
    )

    // Float format to parse a Julia float from string.
    #define lexical_julia_string lexical_required_exponent_digits

    // Float format for a C#7 literal floating-point number.
    #define lexical_csharp7_literal (                                   \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_fraction_digits                              \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a C#7 float from string.
    #define lexical_csharp7_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a C#6 literal floating-point number.
    #define lexical_csharp6_literal (                                   \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a C#6 float from string.
    #define lexical_csharp6_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a C#5 literal floating-point number.
    #define lexical_csharp5_literal (                                   \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a C#5 float from string.
    #define lexical_csharp5_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a C#4 literal floating-point number.
    #define lexical_csharp4_literal (                                   \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a C#4 float from string.
    #define lexical_csharp4_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a C#3 literal floating-point number.
    #define lexical_csharp3_literal (                                   \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a C#3 float from string.
    #define lexical_csharp3_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a C#2 literal floating-point number.
    #define lexical_csharp2_literal (                                   \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a C#2 float from string.
    #define lexical_csharp2_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a C#1 literal floating-point number.
    #define lexical_csharp1_literal (                                   \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a C#1 float from string.
    #define lexical_csharp1_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a Kawa literal floating-point number.
    #define lexical_kawa_literal (                                      \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a Kawa float from string.
    #define lexical_kawa_string (                                       \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format for a Gambit-C literal floating-point number.
    #define lexical_gambitc_literal (                                   \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a Gambit-C float from string.
    #define lexical_gambitc_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format for a Guile literal floating-point number.
    #define lexical_guile_literal (                                     \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a Guile float from string.
    #define lexical_guile_string (                                      \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format for a Clojure literal floating-point number.
    #define lexical_clojure_literal (                                   \
        lexical_required_integer_digits                                 \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a Clojure float from string.
    #define lexical_clojure_string (                                    \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for an Erlang literal floating-point number.
    #define lexical_erlang_literal (                                    \
        lexical_required_digits                                         \
        | lexical_no_exponent_without_fraction                          \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse an Erlang float from string.
    #define lexical_erlang_string (                                     \
        lexical_required_digits                                         \
        | lexical_no_exponent_without_fraction                          \
        | lexical_no_special                                            \
    )

    // Float format for an Elm literal floating-point number.
    #define lexical_elm_literal (                                       \
        lexical_required_digits                                         \
        | lexical_no_positive_mantissa_sign                             \
    )

    // Float format to parse an Elm float from string.
    #define lexical_elm_string (                                        \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for a Scala literal floating-point number.
    #define lexical_scala_literal (                                     \
        lexical_required_digits                                         \
        | lexical_no_special                                            \
    )

    // Float format to parse a Scala float from string.
    #define lexical_scala_string (                                      \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for an Elixir literal floating-point number.
    #define lexical_elixir_literal (                                    \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_digits                                       \
        | lexical_no_exponent_without_fraction                          \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
    )

    // Float format to parse an Elixir float from string.
    #define lexical_elixir_string (                                     \
        lexical_required_digits                                         \
        | lexical_no_exponent_without_fraction                          \
        | lexical_no_special                                            \
    )

    // Float format for a FORTRAN literal floating-point number.
    #define lexical_fortran_literal (                                   \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse a FORTRAN float from string.
    #define lexical_fortran_string lexical_required_exponent_digits

    // Float format for a D literal floating-point number.
    #define lexical_d_literal (                                         \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a D float from string.
    #define lexical_d_string (                                          \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_integer_internal_digit_separator                      \
        | lexical_fraction_internal_digit_separator                     \
        | lexical_integer_trailing_digit_separator                      \
        | lexical_fraction_trailing_digit_separator                     \
    )

    // Float format for a Coffeescript literal floating-point number.
    #define lexical_coffeescript_literal (                              \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a Coffeescript float from string.
    #define lexical_coffeescript_string lexical_case_sensitive_special

    // Float format for a Cobol literal floating-point number.
    #define lexical_cobol_literal (                                     \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_exponent_without_fraction                          \
        | lexical_no_special                                            \
    )

    // Float format to parse a Cobol float from string.
    #define lexical_cobol_string (                                      \
        lexical_required_exponent_sign                                  \
        | lexical_no_special                                            \
    )

    // Float format for a F# literal floating-point number.
    #define lexical_fsharp_literal (                                    \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_integer_digits                               \
        | lexical_required_exponent_digits                              \
        | lexical_case_sensitive_special                                \
        | lexical_internal_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a F# float from string.
    #define lexical_fsharp_string (                                     \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_internal_digit_separator                              \
        | lexical_case_sensitive_special                                \
        | lexical_leading_digit_separator                               \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
        | lexical_special_digit_separator                               \
    )

    // Float format for a Visual Basic literal floating-point number.
    #define lexical_vb_literal (                                        \
        lexical_required_fraction_digits                                \
        | lexical_required_exponent_digits                              \
        | lexical_no_special                                            \
    )

    // Float format to parse a Visual Basic float from string.
    #define lexical_vb_string (                                         \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format for an OCaml literal floating-point number.
    #define lexical_ocaml_literal (                                     \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_integer_digits                               \
        | lexical_required_exponent_digits                              \
        | lexical_no_positive_mantissa_sign                             \
        | lexical_case_sensitive_special                                \
        | lexical_internal_digit_separator                              \
        | lexical_fraction_leading_digit_separator                      \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse an OCaml float from string.
    #define lexical_ocaml_string (                                      \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_internal_digit_separator                              \
        | lexical_leading_digit_separator                               \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
        | lexical_special_digit_separator                               \
    )

    // Float format for an Objective-C literal floating-point number.
    #define lexical_objectivec_literal (                                \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format to parse an Objective-C float from string.
    #define lexical_objectivec_string (                                 \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format for a ReasonML literal floating-point number.
    #define lexical_reasonml_literal (                                  \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_integer_digits                               \
        | lexical_required_exponent_digits                              \
        | lexical_case_sensitive_special                                \
        | lexical_internal_digit_separator                              \
        | lexical_fraction_leading_digit_separator                      \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse a ReasonML float from string.
    #define lexical_reasonml_string (                                   \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_internal_digit_separator                              \
        | lexical_leading_digit_separator                               \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
        | lexical_special_digit_separator                               \
    )

    // Float format for an Octave literal floating-point number.
    #define lexical_octave_literal (                                    \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_case_sensitive_special                                \
        | lexical_internal_digit_separator                              \
        | lexical_fraction_leading_digit_separator                      \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse an Octave float from string.
    #define lexical_octave_string (                                     \
        lexical_digit_separator_to_flags(',')                           \
        | lexical_required_exponent_digits                              \
        | lexical_internal_digit_separator                              \
        | lexical_leading_digit_separator                               \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format for an Matlab literal floating-point number.
    #define lexical_matlab_literal (                                    \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_case_sensitive_special                                \
        | lexical_internal_digit_separator                              \
        | lexical_fraction_leading_digit_separator                      \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format to parse an Matlab float from string.
    #define lexical_matlab_string (                                     \
        lexical_digit_separator_to_flags(',')                           \
        | lexical_required_exponent_digits                              \
        | lexical_internal_digit_separator                              \
        | lexical_leading_digit_separator                               \
        | lexical_trailing_digit_separator                              \
        | lexical_consecutive_digit_separator                           \
    )

    // Float format for a Zig literal floating-point number.
    #define lexical_zig_literal (                                       \
        lexical_required_integer_digits                                 \
        | lexical_no_positive_mantissa_sign                             \
        | lexical_no_special                                            \
    )

    // Float format to parse a Zig float from string.
    #define lexical_zig_string 0ull

    // Float format for a Sage literal floating-point number.
    #define lexical_sage_literal (                                      \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // Float format to parse a Sage float from string.
    #define lexical_sage_string (                                       \
        lexical_digit_separator_to_flags('_')                           \
        | lexical_required_exponent_digits                              \
        | lexical_internal_digit_separator                              \
    )

    // Float format for a JSON literal floating-point number.
    #define lexical_json (                                              \
        lexical_required_digits                                         \
        | lexical_no_positive_mantissa_sign                             \
        | lexical_no_special                                            \
    )

    // Float format for a TOML literal floating-point number.
    #define lexical_toml (                                              \
        lexical_required_digits                                         \
        | lexical_no_special                                            \
        | lexical_internal_digit_separator                              \
    )

    // Float format for a YAML literal floating-point number.
    #define lexical_yaml lexical_json

    // Float format for a XML literal floating-point number.
    #define lexical_xml lexical_case_sensitive_special

    // Float format for a SQLite literal floating-point number.
    #define lexical_sqlite (                                             \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format for a PostgreSQL literal floating-point number.
    #define lexical_postgresql (                                        \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format for a MySQL literal floating-point number.
    #define lexical_mysql (                                             \
        lexical_required_exponent_digits                                \
        | lexical_no_special                                            \
    )

    // Float format for a MongoDB literal floating-point number.
    #define lexical_mongodb (                                           \
        lexical_required_exponent_digits                                \
        | lexical_case_sensitive_special                                \
    )

    // HIDDEN DEFAULTS

    // Float format when no flags are set.
    #define lexical_permissive 0ull

    // Standard float format.
    #define lexical_standard lexical_rust_string

    // Float format when all digit separator flags are set.
    #define lexical_ignore lexical_digit_separator_flag_mask

    // OPTION TAG

    // Tag for the option type in the tagged enum.
    enum lexical_option_tag {
        lexical_some = 0,
        lexical_none = 1,
    };

    // OPTION

    // Struct to define the number option.
    struct lexical_number_format_option {
        uint32_t tag;
        uint64_t data;
    };

    inline bool lexical_number_format_option_is_some(lexical_number_format_option* option)
    {
        return option->tag == lexical_some;
    }

    inline bool lexical_number_format_option_is_none(lexical_number_format_option* option)
    {
        return option->tag == lexical_none;
    }

    inline uint64_t lexical_number_format_option_unwrap(lexical_number_format_option option)
    {
        assert(lexical_number_format_option_is_some(&option));
        return option.data;
    }

    inline bool lexical_number_format_intersects(uint64_t x, uint64_t y)
    {
        return (x & y) != 0;
    }

    // COMPILER

    // Compile float format value from specifications.
    //
    // * `digit_separator`                         - Character to separate digits.
    // * `required_integer_digits`                 - If digits are required before the decimal point.
    // * `required_fraction_digits`                - If digits are required after the decimal point.
    // * `required_exponent_digits`                - If digits are required after the exponent character.
    // * `no_positive_mantissa_sign`               - If positive sign before the mantissa is not allowed.
    // * `required_mantissa_sign`                  - If positive sign before the mantissa is required.
    // * `no_exponent_notation`                    - If exponent notation is not allowed.
    // * `no_positive_exponent_sign`               - If positive sign before the exponent is not allowed.
    // * `required_exponent_sign`                  - If sign before the exponent is required.
    // * `no_exponent_without_fraction`            - If exponent without fraction is not allowed.
    // * `no_special`                              - If special (non-finite) values are not allowed.
    // * `case_sensitive_special`                  - If special (non-finite) values are case-sensitive.
    // * `no_integer_leading_zeros`                - If leading zeros before an integer are not allowed.
    // * `no_float_leading_zeros`                  - If leading zeros before a float are not allowed.
    // * `integer_internal_digit_separator`        - If digit separators are allowed between integer digits.
    // * `fraction_internal_digit_separator`       - If digit separators are allowed between fraction digits.
    // * `exponent_internal_digit_separator`       - If digit separators are allowed between exponent digits.
    // * `integer_leading_digit_separator`         - If a digit separator is allowed before any integer digits.
    // * `fraction_leading_digit_separator`        - If a digit separator is allowed before any fraction digits.
    // * `exponent_leading_digit_separator`        - If a digit separator is allowed before any exponent digits.
    // * `integer_trailing_digit_separator`        - If a digit separator is allowed after any integer digits.
    // * `fraction_trailing_digit_separator`       - If a digit separator is allowed after any fraction digits.
    // * `exponent_trailing_digit_separator`       - If a digit separator is allowed after any exponent digits.
    // * `integer_consecutive_digit_separator`     - If multiple consecutive integer digit separators are allowed.
    // * `fraction_consecutive_digit_separator`    - If multiple consecutive fraction digit separators are allowed.
    // * `special_digit_separator`                 - If any digit separators are allowed in special (non-finite) values.
    //
    // Returns the value if it was able to compile the format,
    // otherwise, returns None. Digit separators must not be
    // in the character group `[A-Za-z0-9+.-]`, nor be equal to
    // `get_exponent_default_char` or `get_exponent_backup_char`.
    inline lexical_number_format_option lexical_number_format_compile(
        char digit_separator = '_',
        bool required_integer_digits = false,
        bool required_fraction_digits = false,
        bool required_exponent_digits = false,
        bool no_positive_mantissa_sign = false,
        bool required_mantissa_sign = false,
        bool no_exponent_notation = false,
        bool no_positive_exponent_sign = false,
        bool required_exponent_sign = false,
        bool no_exponent_without_fraction = false,
        bool no_special = false,
        bool case_sensitive_special = false,
        bool no_integer_leading_zeros = false,
        bool no_float_leading_zeros = false,
        bool integer_internal_digit_separator = false,
        bool fraction_internal_digit_separator = false,
        bool exponent_internal_digit_separator = false,
        bool integer_leading_digit_separator = false,
        bool fraction_leading_digit_separator = false,
        bool exponent_leading_digit_separator = false,
        bool integer_trailing_digit_separator = false,
        bool fraction_trailing_digit_separator = false,
        bool exponent_trailing_digit_separator = false,
        bool integer_consecutive_digit_separator = false,
        bool fraction_consecutive_digit_separator = false,
        bool exponent_consecutive_digit_separator = false,
        bool special_digit_separator = false
    )
    {
        uint64_t flags = 0;
        // Generic flags.
        if (required_integer_digits) {
            flags |= lexical_required_integer_digits;
        }
        if (required_fraction_digits) {
            flags |= lexical_required_fraction_digits;
        }
        if (required_exponent_digits) {
            flags |= lexical_required_exponent_digits;
        }
        if (no_positive_mantissa_sign) {
            flags |= lexical_no_positive_mantissa_sign;
        }
        if (required_mantissa_sign) {
            flags |= lexical_required_mantissa_sign;
        }
        if (no_exponent_notation) {
            flags |= lexical_no_exponent_notation;
        }
        if (no_positive_exponent_sign) {
            flags |= lexical_no_positive_exponent_sign;
        }
        if (required_exponent_sign) {
            flags |= lexical_required_exponent_sign;
        }
        if (no_exponent_without_fraction) {
            flags |= lexical_no_exponent_without_fraction;
        }
        if (no_special) {
            flags |= lexical_no_special;
        }
        if (case_sensitive_special) {
            flags |= lexical_case_sensitive_special;
        }
        if (no_integer_leading_zeros) {
            flags |= lexical_no_integer_leading_zeros;
        }
        if (no_float_leading_zeros) {
            flags |= lexical_no_float_leading_zeros;
        }

        // Digit separator flags.
        if (integer_internal_digit_separator) {
            flags |= lexical_integer_internal_digit_separator;
        }
        if (fraction_internal_digit_separator) {
            flags |= lexical_fraction_internal_digit_separator;
        }
        if (exponent_internal_digit_separator) {
            flags |= lexical_exponent_internal_digit_separator;
        }
        if (integer_leading_digit_separator) {
            flags |= lexical_integer_leading_digit_separator;
        }
        if (fraction_leading_digit_separator) {
            flags |= lexical_fraction_leading_digit_separator;
        }
        if (exponent_leading_digit_separator) {
            flags |= lexical_exponent_leading_digit_separator;
        }
        if (integer_trailing_digit_separator) {
            flags |= lexical_integer_trailing_digit_separator;
        }
        if (fraction_trailing_digit_separator) {
            flags |= lexical_fraction_trailing_digit_separator;
        }
        if (exponent_trailing_digit_separator) {
            flags |= lexical_exponent_trailing_digit_separator;
        }
        if (integer_consecutive_digit_separator) {
            flags |= lexical_integer_consecutive_digit_separator;
        }
        if (fraction_consecutive_digit_separator) {
            flags |= lexical_fraction_consecutive_digit_separator;
        }
        if (exponent_consecutive_digit_separator) {
            flags |= lexical_exponent_consecutive_digit_separator;
        }
        if (special_digit_separator) {
            flags |= lexical_special_digit_separator;
        }

        // Digit separator.
        if (lexical_number_format_intersects(flags, lexical_digit_separator_flag_mask)) {
            flags |= lexical_digit_separator_to_flags(digit_separator);
        }

        // Validation.
        bool is_invalid = (
            !lexical_is_valid_separator(digit_separator)
            || (lexical_number_format_intersects(flags, lexical_no_exponent_notation) && lexical_number_format_intersects(flags, lexical_exponent_flag_mask))
            || (no_positive_mantissa_sign && required_mantissa_sign)
            || (no_positive_exponent_sign && required_exponent_sign)
            || (no_special && (case_sensitive_special || special_digit_separator))
            || (flags & lexical_integer_digit_separator_flag_mask) == lexical_integer_consecutive_digit_separator
            || (flags & lexical_fraction_digit_separator_flag_mask) == lexical_fraction_consecutive_digit_separator
            || (flags & lexical_exponent_digit_separator_flag_mask) == lexical_exponent_consecutive_digit_separator
        );

        // Return an option with the compiled format.
        lexical_number_format_option result;
        if (is_invalid) {
            result.tag = lexical_none;
        } else {
            result.tag = lexical_some;
            result.data = flags;
        }
        return result;
    }

    // Compile permissive number format.
    //
    // The permissive number format does not require any control
    // grammar, besides the presence of mantissa digits.
    inline lexical_number_format_option lexical_number_format_permissive()
    {
        lexical_number_format_option result;
        result.tag = lexical_some;
        result.data = lexical_permissive;

        return result;
    }

    // Compile standard number format.
    //
    // The standard number format is guaranteed to be identical
    // to the format expected by Rust's string to number parsers.
    inline lexical_number_format_option lexical_number_format_standard()
    {
        lexical_number_format_option result;
        result.tag = lexical_some;
        result.data = lexical_standard;

        return result;
    }

    // Compile ignore number format.
    //
    // The ignore number format ignores all digit separators,
    // and is permissive for all other control grammar, so
    // implements a fast parser.
    //
    // * `digit_separator`                         - Character to separate digits.
    inline lexical_number_format_option lexical_number_format_ignore(uint8_t digit_separator)
    {
        lexical_number_format_option result;
        if (!lexical_is_valid_separator(digit_separator)) {
            result.tag = lexical_none;
        } else {
            result.tag = lexical_some;
            result.data = (
                lexical_ignore
                | lexical_digit_separator_to_flags(digit_separator)
            );
        }

        return result;
    }

    // Get the flag bits from the compiled float format.
    inline uint64_t lexical_number_format_flags(uint64_t format)
    {
        return format & lexical_flag_mask;
    }

    // Get the digit separator from the compiled float format.
    inline uint8_t lexical_number_format_digit_separator(uint64_t format)
    {
        return lexical_digit_separator_from_flags(format);
    }

    // Get if digits are required before the decimal point.
    inline bool lexical_number_format_required_integer_digits(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_required_integer_digits);
    }

    // Get if digits are required after the decimal point.
    inline bool lexical_number_format_required_fraction_digits(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_required_fraction_digits);
    }

    // Get if digits are required after the exponent character.
    inline bool lexical_number_format_required_exponent_digits(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_required_exponent_digits);
    }

    // Get if digits are required before or after the decimal point.
    inline bool lexical_number_format_required_digits(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_required_digits);
    }

    // Get if positive sign before the mantissa is not allowed.
    inline bool lexical_number_format_no_positive_mantissa_sign(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_no_positive_mantissa_sign);
    }

    // Get if positive sign before the mantissa is required.
    inline bool lexical_number_format_required_mantissa_sign(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_required_mantissa_sign);
    }

    // Get if exponent notation is not allowed.
    inline bool lexical_number_format_no_exponent_notation(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_no_exponent_notation);
    }

    // Get if positive sign before the exponent is not allowed.
    inline bool lexical_number_format_no_positive_exponent_sign(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_no_positive_exponent_sign);
    }

    // Get if sign before the exponent is required.
    inline bool lexical_number_format_required_exponent_sign(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_required_exponent_sign);
    }

    // Get if exponent without fraction is not allowed.
    inline bool lexical_number_format_no_exponent_without_fraction(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_no_exponent_without_fraction);
    }

    // Get if special (non-finite) values are not allowed.
    inline bool lexical_number_format_no_special(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_no_special);
    }

    // Get if special (non-finite) values are case-sensitive.
    inline bool lexical_number_format_case_sensitive_special(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_case_sensitive_special);
    }

    // Get if leading zeros before an integer are not allowed.
    inline bool lexical_number_format_no_integer_leading_zeros(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_no_integer_leading_zeros);
    }

    // Get if leading zeros before a float are not allowed.
    inline bool lexical_number_format_no_float_leading_zeros(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_no_float_leading_zeros);
    }

    // Get if digit separators are allowed between integer digits.
    inline bool lexical_number_format_integer_internal_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_integer_internal_digit_separator);
    }

    // Get if digit separators are allowed between fraction digits.
    inline bool lexical_number_format_fraction_internal_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_fraction_internal_digit_separator);
    }

    // Get if digit separators are allowed between exponent digits.
    inline bool lexical_number_format_exponent_internal_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_exponent_internal_digit_separator);
    }

    // Get if digit separators are allowed between digits.
    inline bool lexical_number_format_internal_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_internal_digit_separator);
    }

    // Get if a digit separator is allowed before any integer digits.
    inline bool lexical_number_format_integer_leading_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_integer_leading_digit_separator);
    }

    // Get if a digit separator is allowed before any fraction digits.
    inline bool lexical_number_format_fraction_leading_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_fraction_leading_digit_separator);
    }

    // Get if a digit separator is allowed before any exponent digits.
    inline bool lexical_number_format_exponent_leading_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_exponent_leading_digit_separator);
    }

    // Get if a digit separator is allowed before any digits.
    inline bool lexical_number_format_leading_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_leading_digit_separator);
    }

    // Get if a digit separator is allowed after any integer digits.
    inline bool lexical_number_format_integer_trailing_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_integer_trailing_digit_separator);
    }

    // Get if a digit separator is allowed after any fraction digits.
    inline bool lexical_number_format_fraction_trailing_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_fraction_trailing_digit_separator);
    }

    // Get if a digit separator is allowed after any exponent digits.
    inline bool lexical_number_format_exponent_trailing_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_exponent_trailing_digit_separator);
    }

    // Get if a digit separator is allowed after any digits.
    inline bool lexical_number_format_trailing_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_trailing_digit_separator);
    }

    // Get if multiple consecutive integer digit separators are allowed.
    inline bool lexical_number_format_integer_consecutive_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_integer_consecutive_digit_separator);
    }

    // Get if multiple consecutive fraction digit separators are allowed.
    inline bool lexical_number_format_fraction_consecutive_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_fraction_consecutive_digit_separator);
    }

    // Get if multiple consecutive exponent digit separators are allowed.
    inline bool lexical_number_format_exponent_consecutive_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_exponent_consecutive_digit_separator);
    }

    // Get if multiple consecutive digit separators are allowed.
    inline bool lexical_number_format_consecutive_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_consecutive_digit_separator);
    }

    // Get if any digit separators are allowed in special (non-finite) values.
    inline bool lexical_number_format_special_digit_separator(uint64_t format)
    {
        return lexical_number_format_intersects(format, lexical_special_digit_separator);
    }
#endif  // HAVE_FORMAT

// C-compatible error for FFI.
struct lexical_error {
    int32_t code;
    size_t index;
};

// Determine if an error code matches the desired code.
#define lexical_is_error(type)                                          \
    inline bool lexical_error_is_##type(lexical_error* error)           \
    {                                                                   \
        return error->code == lexical_##type;                           \
    }

lexical_is_error(overflow);
lexical_is_error(underflow);
lexical_is_error(invalid_digit);
lexical_is_error(empty);
lexical_is_error(empty_mantissa);
lexical_is_error(empty_exponent);
lexical_is_error(empty_integer);
lexical_is_error(empty_fraction);
lexical_is_error(invalid_positive_mantissa_sign);
lexical_is_error(missing_mantissa_sign);
lexical_is_error(invalid_exponent);
lexical_is_error(invalid_positive_exponent_sign);
lexical_is_error(missing_exponent_sign);
lexical_is_error(exponent_without_fraction);
lexical_is_error(invalid_leading_zeros);

// RESULT TAG

// Tag for the result type in the tagged enum.
enum lexical_result_tag {
    lexical_ok = 0,
    lexical_err = 1,
};

// COMPLETE UNIONS

// Get result union type from type name.
#define lexical_result_union_type(type) lexical_##type##_result_union

// Define a union for the lexical result type.
#define lexical_result_union(type)                                              \
    union lexical_result_union_type(type) {                                     \
        lexical_type(type) value;                                               \
        lexical_error error;                                                    \
    }

lexical_result_union(i8);
lexical_result_union(i16);
lexical_result_union(i32);
lexical_result_union(i64);
lexical_result_union(isize);

lexical_result_union(u8);
lexical_result_union(u16);
lexical_result_union(u32);
lexical_result_union(u64);
lexical_result_union(usize);

lexical_result_union(f32);
lexical_result_union(f64);

// COMPLETE RESULTS

// Get result type from type name.
#define lexical_result_type(type) lexical_##type##_result

// Define a struct for the lexical result type.
#define lexical_result(type)                                                    \
    struct lexical_result_type(type) {                                          \
        uint32_t tag;                                                           \
        lexical_result_union_type(type) data;                                   \
    };                                                                          \
                                                                                \
    inline                                                                      \
    bool                                                                        \
    lexical_##type##_result_is_ok(                                              \
        lexical_result_type(type)* result                                       \
    )                                                                           \
    {                                                                           \
        return result->tag == lexical_ok;                                       \
    }                                                                           \
                                                                                \
    inline                                                                      \
    bool                                                                        \
    lexical_##type##_result_is_err(                                             \
        lexical_result_type(type)* result                                       \
    )                                                                           \
    {                                                                           \
        return result->tag == lexical_err;                                      \
    }                                                                           \
                                                                                \
    inline                                                                      \
    lexical_type(type)                                                          \
    lexical_##type##_result_ok(                                                 \
        lexical_result_type(type) result                                        \
    )                                                                           \
    {                                                                           \
        assert(lexical_##type##_result_is_ok(&result));                         \
        return result.data.value;                                               \
    }                                                                           \
                                                                                \
    inline                                                                      \
    lexical_error                                                               \
    lexical_##type##_result_err(                                                \
        lexical_result_type(type) result                                        \
    )                                                                           \
    {                                                                           \
        assert(lexical_##type##_result_is_err(&result));                        \
        return result.data.error;                                               \
    }

lexical_result(i8);
lexical_result(i16);
lexical_result(i32);
lexical_result(i64);
lexical_result(isize);

lexical_result(u8);
lexical_result(u16);
lexical_result(u32);
lexical_result(u64);
lexical_result(usize);

lexical_result(f32);
lexical_result(f64);

// PARTIAL RESULT TUPLES

// Get partial result tuple type from type name.
#define lexical_partial_result_tuple_type(type) lexical_##type##_partial_result_tuple

// Define a tuple for the lexical partial result type.
#define lexical_partial_result_tuple(type)                                      \
    struct lexical_partial_result_tuple_type(type) {                            \
        lexical_type(type) x;                                                   \
        size_t y;                                                               \
    }

lexical_partial_result_tuple(i8);
lexical_partial_result_tuple(i16);
lexical_partial_result_tuple(i32);
lexical_partial_result_tuple(i64);
lexical_partial_result_tuple(isize);

lexical_partial_result_tuple(u8);
lexical_partial_result_tuple(u16);
lexical_partial_result_tuple(u32);
lexical_partial_result_tuple(u64);
lexical_partial_result_tuple(usize);

lexical_partial_result_tuple(f32);
lexical_partial_result_tuple(f64);

// PARTIAL RESULT UNIONS

// Get partial result union type from type name.
#define lexical_partial_result_union_type(type) lexical_##type##_partial_result_union

// Define a union for the lexical partial result type.
#define lexical_partial_result_union(type)                                      \
    union lexical_partial_result_union_type(type) {                             \
        lexical_partial_result_tuple_type(type) value;                          \
        lexical_error error;                                                    \
    }

lexical_partial_result_union(i8);
lexical_partial_result_union(i16);
lexical_partial_result_union(i32);
lexical_partial_result_union(i64);
lexical_partial_result_union(isize);

lexical_partial_result_union(u8);
lexical_partial_result_union(u16);
lexical_partial_result_union(u32);
lexical_partial_result_union(u64);
lexical_partial_result_union(usize);

lexical_partial_result_union(f32);
lexical_partial_result_union(f64);

// PARTIAL RESULTS

// Get partial result type from type name.
#define lexical_partial_result_type(type) lexical_##type##_partial_result

// Define a struct for the lexical partial result type.
#define lexical_partial_result(type)                                            \
    struct lexical_partial_result_type(type) {                                  \
        uint32_t tag;                                                           \
        lexical_partial_result_union_type(type) data;                           \
    };                                                                          \
                                                                                \
    inline                                                                      \
    bool                                                                        \
    lexical_##type##_partial_result_is_ok(                                      \
        lexical_partial_result_type(type)* result                               \
    )                                                                           \
    {                                                                           \
        return result->tag == lexical_ok;                                       \
    }                                                                           \
                                                                                \
    inline                                                                      \
    bool                                                                        \
    lexical_##type##_partial_result_is_err(                                     \
        lexical_partial_result_type(type)* result                               \
    )                                                                           \
    {                                                                           \
        return result->tag == lexical_err;                                      \
    }                                                                           \
                                                                                \
    inline                                                                      \
    lexical_partial_result_tuple_type(type)                                     \
    lexical_##type##_partial_result_ok(                                         \
        lexical_partial_result_type(type) result                                \
    )                                                                           \
    {                                                                           \
        assert(lexical_##type##_partial_result_is_ok(&result));                 \
        return result.data.value;                                               \
    }                                                                           \
                                                                                \
    inline                                                                      \
    lexical_error                                                               \
    lexical_##type##_partial_result_err(                                        \
        lexical_partial_result_type(type) result                                \
    )                                                                           \
    {                                                                           \
        assert(lexical_##type##_partial_result_is_err(&result));                \
        return result.data.error;                                               \
    }

lexical_partial_result(i8);
lexical_partial_result(i16);
lexical_partial_result(i32);
lexical_partial_result(i64);
lexical_partial_result(isize);

lexical_partial_result(u8);
lexical_partial_result(u16);
lexical_partial_result(u32);
lexical_partial_result(u64);
lexical_partial_result(usize);

lexical_partial_result(f32);
lexical_partial_result(f64);

// API
// ---

// TO LEXICAL

// Declare extern to lexical function definitions for type.
#define lexical_decimal_to_range(type)                                          \
    extern uint8_t* lexical_##type##toa(                                        \
        lexical_type(type) value,                                               \
        uint8_t* first,                                                         \
        uint8_t* last                                                           \
    )

// Declare extern to lexical radix function definitions for type.
#define lexical_radix_to_range(type)                                            \
    extern uint8_t* lexical_##type##toa_radix(                                  \
        lexical_type(type) value,                                               \
        uint8_t radix,                                                          \
        uint8_t* first,                                                         \
        uint8_t* last                                                           \
    )

// Declare extern to lexical function definitions.
#ifdef HAVE_RADIX
    #define lexical_to_lexical(type)                                            \
        lexical_decimal_to_range(type);                                         \
        lexical_radix_to_range(type)
#else   // !HAVE_RADIX
    #define lexical_to_lexical(type)                                            \
        lexical_decimal_to_range(type)
#endif  // HAVE_RADIX

// FROM LEXICAL

// Declare extern from lexical function definitions for type.
#define lexical_decimal_from_range(type)                                        \
    extern                                                                      \
    lexical_result_type(type)                                                   \
    lexical_ato##type(                                                          \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )

// Declare extern partial from lexical function definitions for type.
#define lexical_partial_decimal_from_range(type)                                \
    extern                                                                      \
    lexical_partial_result_type(type)                                           \
    lexical_ato##type##_partial(                                                \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )

// Declare extern from lexical function definitions for type.
#define lexical_radix_from_range(type)                                          \
    extern                                                                      \
    lexical_result_type(type)                                                   \
    lexical_ato##type##_radix(                                                  \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )

// Declare extern partial from lexical function definitions for type.
#define lexical_partial_radix_from_range(type)                                  \
    extern                                                                      \
    lexical_partial_result_type(type)                                           \
    lexical_ato##type##_partial_radix(                                          \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )

// Declare extern from lexical function definitions.
#ifdef HAVE_RADIX
    #define lexical_from_lexical(type)                                          \
        lexical_decimal_from_range(type);                                       \
        lexical_partial_decimal_from_range(type);                               \
        lexical_radix_from_range(type);                                         \
        lexical_partial_radix_from_range(type)
#else   // !HAVE_RADIX
    #define lexical_from_lexical(type)                                          \
        lexical_decimal_from_range(type);                                       \
        lexical_partial_decimal_from_range(type)
#endif  // HAVE_RADIX

// FROM LEXICAL LOSSY

// Declare extern lossy from lexical function definitions for type.
#define lexical_lossy_decimal_from_range(type)                                  \
    extern                                                                      \
    lexical_result_type(type)                                                   \
    lexical_ato##type##_lossy(                                                  \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )

// Declare extern lossy, partial from lexical function definitions for type.
#define lexical_lossy_partial_decimal_from_range(type)                          \
    extern                                                                      \
    lexical_partial_result_type(type)                                           \
    lexical_ato##type##_partial_lossy(                                          \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )

// Declare extern lossy from lexical function definitions for type.
#define lexical_lossy_radix_from_range(type)                                    \
    extern                                                                      \
    lexical_result_type(type)                                                   \
    lexical_ato##type##_lossy_radix(                                            \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )

// Declare extern lossy, partial from lexical function definitions for type.
#define lexical_lossy_partial_radix_from_range(type)                            \
    extern                                                                      \
    lexical_partial_result_type(type)                                           \
    lexical_ato##type##_partial_lossy_radix(                                    \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )

// Declare extern from lexical lossy function definitions.
#ifdef HAVE_RADIX
    #define lexical_from_lexical_lossy(type)                                    \
        lexical_lossy_decimal_from_range(type);                                 \
        lexical_lossy_partial_decimal_from_range(type);                         \
        lexical_lossy_radix_from_range(type);                                   \
        lexical_lossy_partial_radix_from_range(type)
#else   // !HAVE_RADIX
    #define lexical_from_lexical_lossy(type)                                    \
        lexical_lossy_decimal_from_range(type);                                 \
        lexical_lossy_partial_decimal_from_range(type)
#endif  // HAVE_RADIX

#ifdef HAVE_FORMAT
    // FROM LEXICAL FORMAT

    // Declare extern format from lexical function definitions for type.
    #define lexical_format_decimal_from_range(type)                             \
        extern                                                                  \
        lexical_result_type(type)                                               \
        lexical_ato##type##_format(                                             \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint64_t format                                                     \
        )

    // Declare extern partial, format from lexical function definitions for type.
    #define lexical_partial_format_decimal_from_range(type)                     \
        extern                                                                  \
        lexical_partial_result_type(type)                                       \
        lexical_ato##type##_partial_format(                                     \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint64_t format                                                     \
        )

    // Declare extern format from lexical function definitions for type.
    #define lexical_format_radix_from_range(type)                               \
        extern                                                                  \
        lexical_result_type(type)                                               \
        lexical_ato##type##_format_radix(                                       \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            uint64_t format                                                     \
        )

    // Declare extern partial, format from lexical function definitions for type.
    #define lexical_partial_format_radix_from_range(type)                       \
        extern                                                                  \
        lexical_partial_result_type(type)                                       \
        lexical_ato##type##_partial_format_radix(                               \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            uint64_t format                                                     \
        )

    // Declare extern format from lexical function definitions.
    #ifdef HAVE_RADIX
        #define lexical_from_lexical_format(type)                               \
            lexical_format_decimal_from_range(type);                            \
            lexical_partial_format_decimal_from_range(type);                    \
            lexical_format_radix_from_range(type);                              \
            lexical_partial_format_radix_from_range(type)
    #else   // !HAVE_RADIX
        #define lexical_from_lexical_format(type)                               \
            lexical_format_decimal_from_range(type);                            \
            lexical_partial_format_decimal_from_range(type)
    #endif  // HAVE_RADIX

    // FROM LEXICAL LOSSY FORMAT

    // Declare extern lossy, format from lexical function definitions for type.
    #define lexical_lossy_format_decimal_from_range(type)                       \
        extern                                                                  \
        lexical_result_type(type)                                               \
        lexical_ato##type##_lossy_format(                                       \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint64_t format                                                     \
        )

    // Declare extern lossy, partial, format from lexical function definitions for type.
    #define lexical_lossy_partial_format_decimal_from_range(type)               \
        extern                                                                  \
        lexical_partial_result_type(type)                                       \
        lexical_ato##type##_partial_lossy_format(                               \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint64_t format                                                     \
        )

    // Declare extern lossy, format from lexical function definitions for type.
    #define lexical_lossy_format_radix_from_range(type)                         \
        extern                                                                  \
        lexical_result_type(type)                                               \
        lexical_ato##type##_lossy_format_radix(                                 \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            uint64_t format                                                     \
        )

    // Declare extern lossy, partial, format from lexical function definitions for type.
    #define lexical_lossy_partial_format_radix_from_range(type)                 \
        extern                                                                  \
        lexical_partial_result_type(type)                                       \
        lexical_ato##type##_partial_lossy_format_radix(                         \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            uint64_t format                                                     \
        )

    // Declare extern from lexical lossy function definitions.
    #ifdef HAVE_RADIX
        #define lexical_from_lexical_lossy_format(type)                         \
            lexical_lossy_format_decimal_from_range(type);                      \
            lexical_lossy_partial_format_decimal_from_range(type);              \
            lexical_lossy_format_radix_from_range(type);                        \
            lexical_lossy_partial_format_radix_from_range(type)
    #else   // !HAVE_RADIX
        #define lexical_from_lexical_lossy_format(type)                         \
            lexical_lossy_format_decimal_from_range(type);                      \
            lexical_lossy_partial_format_decimal_from_range(type)
    #endif  // HAVE_RADIX

#endif  // HAVE_FORMAT

// ATOF
lexical_from_lexical(f32);
lexical_from_lexical(f64);
lexical_from_lexical_lossy(f32);
lexical_from_lexical_lossy(f64);

// ATOI
lexical_from_lexical(i8);
lexical_from_lexical(i16);
lexical_from_lexical(i32);
lexical_from_lexical(i64);
lexical_from_lexical(isize);

lexical_from_lexical(u8);
lexical_from_lexical(u16);
lexical_from_lexical(u32);
lexical_from_lexical(u64);
lexical_from_lexical(usize);

// FTOA
lexical_to_lexical(f32);
lexical_to_lexical(f64);

// ITOA
lexical_to_lexical(i8);
lexical_to_lexical(i16);
lexical_to_lexical(i32);
lexical_to_lexical(i64);
lexical_to_lexical(isize);

lexical_to_lexical(u8);
lexical_to_lexical(u16);
lexical_to_lexical(u32);
lexical_to_lexical(u64);
lexical_to_lexical(usize);

#ifdef HAVE_FORMAT
    // ATOF
    lexical_from_lexical_format(f32);
    lexical_from_lexical_format(f64);
    lexical_from_lexical_lossy_format(f32);
    lexical_from_lexical_lossy_format(f64);

    // ATOI
    lexical_from_lexical_format(i8);
    lexical_from_lexical_format(i16);
    lexical_from_lexical_format(i32);
    lexical_from_lexical_format(i64);
    lexical_from_lexical_format(isize);

    lexical_from_lexical_format(u8);
    lexical_from_lexical_format(u16);
    lexical_from_lexical_format(u32);
    lexical_from_lexical_format(u64);
    lexical_from_lexical_format(usize);
#endif  // HAVE_FORMAT

// CLEANUP
// -------

#undef lexical_static_assert
#undef lexical_type
#undef lexical_digit_separator_from_flags
#undef lexical_is_error
#undef lexical_result_union_type
#undef lexical_result_union
#undef lexical_result_type
#undef lexical_result
#undef lexical_partial_result_tuple_type
#undef lexical_partial_result_tuple
#undef lexical_partial_result_union_type
#undef lexical_partial_result_union
#undef lexical_partial_result_type
#undef lexical_partial_result
#undef lexical_decimal_to_range
#undef lexical_radix_to_range
#undef lexical_to_lexical
#undef lexical_decimal_from_range
#undef lexical_partial_decimal_from_range
#undef lexical_radix_from_range
#undef lexical_partial_radix_from_range
#undef lexical_from_lexical
#undef lexical_lossy_decimal_from_range
#undef lexical_lossy_partial_decimal_from_range
#undef lexical_lossy_radix_from_range
#undef lexical_lossy_partial_radix_from_range
#undef lexical_from_lexical_lossy
#undef lexical_format_decimal_from_range
#undef lexical_partial_format_decimal_from_range
#undef lexical_format_radix_from_range
#undef lexical_partial_format_radix_from_range
#undef lexical_from_lexical_format
#undef lexical_lossy_format_decimal_from_range
#undef lexical_lossy_partial_format_decimal_from_range
#undef lexical_lossy_radix_from_range
#undef lexical_lossy_partial_radix_from_range
#undef lexical_from_lexical_lossy_format

#ifdef __cplusplus
}
#endif
#endif  /* !LEXICAL_H_ */
