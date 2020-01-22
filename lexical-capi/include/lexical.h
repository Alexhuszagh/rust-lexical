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
};

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

// CLEANUP
// -------

#undef lexical_static_assert
#undef lexical_type
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

#ifdef __cplusplus
}
#endif
#endif  /* !LEXICAL_H_ */
