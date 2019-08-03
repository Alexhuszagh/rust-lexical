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

#ifndef LEXICALCORE_H_
#define LEXICALCORE_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Features must be enabled through the following macro definitions:
//  1. HAVE_RADIX
//  1. HAVE_ROUNDING

// CONFIG
// ------

extern int32_t get_nan_string_ffi(uint8_t** ptr, size_t* size);
extern int32_t set_nan_string_ffi(uint8_t* ptr, size_t size);
extern int32_t get_inf_string_ffi(uint8_t** ptr, size_t* size);
extern int32_t set_inf_string_ffi(uint8_t* ptr, size_t size);
extern int32_t get_infinity_string_ffi(uint8_t** ptr, size_t* size);
extern int32_t set_infinity_string_ffi(uint8_t* ptr, size_t size);

// GLOBALS
// -------

// Rounding type for float-parsing.
enum RoundingKind {
    // Round to the nearest, tie to even.
    NearestTieEven = 0,
    // Round to the nearest, tie away from zero.
    NearestTieAwayZero = 1,
    // Round toward positive infinity.
    TowardPositiveInfinity = 2,
    // Round toward negative infinity.
    TowardNegativeInfinity = 3,
    // Round toward zero.
    TowardZero = 4,
};

// MUTABLE GLOBALS
extern uint8_t EXPONENT_DEFAULT_CHAR;
#ifdef HAVE_RADIX
    extern uint8_t EXPONENT_BACKUP_CHAR;
#endif
#ifdef HAVE_ROUNDING
    extern int32_t FLOAT_ROUNDING;
#endif

// CONSTANTS
extern const size_t MAX_I8_SIZE_FFI;
extern const size_t MAX_I16_SIZE_FFI;
extern const size_t MAX_I32_SIZE_FFI;
extern const size_t MAX_I64_SIZE_FFI;
extern const size_t MAX_ISIZE_SIZE_FFI;
extern const size_t MAX_U8_SIZE_FFI;
extern const size_t MAX_U16_SIZE_FFI;
extern const size_t MAX_U32_SIZE_FFI;
extern const size_t MAX_U64_SIZE_FFI;
extern const size_t MAX_USIZE_SIZE_FFI;
extern const size_t MAX_F32_SIZE_FFI;
extern const size_t MAX_F64_SIZE_FFI;

extern const size_t MAX_I8_SIZE_BASE10_FFI;
extern const size_t MAX_I16_SIZE_BASE10_FFI;
extern const size_t MAX_I32_SIZE_BASE10_FFI;
extern const size_t MAX_I64_SIZE_BASE10_FFI;
extern const size_t MAX_ISIZE_SIZE_BASE10_FFI;
extern const size_t MAX_U8_SIZE_BASE10_FFI;
extern const size_t MAX_U16_SIZE_BASE10_FFI;
extern const size_t MAX_U32_SIZE_BASE10_FFI;
extern const size_t MAX_U64_SIZE_BASE10_FFI;
extern const size_t MAX_USIZE_SIZE_BASE10_FFI;
extern const size_t MAX_F32_SIZE_BASE10_FFI;
extern const size_t MAX_F64_SIZE_BASE10_FFI;

extern const size_t BUFFER_SIZE_FFI;

// TYPES
// -----

// ERROR

// Error code, indicating failure type.
enum ErrorCode {
    // Integral overflow occurred during numeric parsing.
    Overflow = -1,
    // Integral underflow occurred during numeric parsing.
    Underflow = -2,
    // Invalid digit found before string termination.
    InvalidDigit = -3,
    // Empty byte array found.
    Empty = -4,
    // Empty fraction found.
    EmptyFraction = -5,
    // Empty exponent found.
    EmptyExponent = -6,
};

// C-compatible error for FFI.
struct Error {
    int32_t code;
    size_t index;
};

extern bool error_is_overflow(Error error);
extern bool error_is_underflow(Error error);
extern bool error_is_invalid_digit(Error error);
extern bool error_is_empty(Error error);
extern bool error_is_empty_fraction(Error error);
extern bool error_is_empty_exponent(Error error);

// UNIONS

union UnionI8 {
    int8_t value;
    Error error;
};

union UnionI16 {
    int16_t value;
    Error error;
};

union UnionI32 {
    int32_t value;
    Error error;
};

union UnionI64 {
    int64_t value;
    Error error;
};

union UnionIsize {
    // Assume ptrdiff_t and size_t are the same size.
    ptrdiff_t value;
    Error error;
};

union UnionU8 {
    uint8_t value;
    Error error;
};

union UnionU16 {
    uint16_t value;
    Error error;
};

union UnionU32 {
    uint32_t value;
    Error error;
};

union UnionU64 {
    uint64_t value;
    Error error;
};

union UnionUsize {
    size_t value;
    Error error;
};

union UnionF32 {
    float value;
    Error error;
};

union UnionF64 {
    double value;
    Error error;
};

// RESULTS

struct ResultI8 {
    uint32_t tag;
    UnionI8 data;
};

extern bool i8_result_ffi_is_ok(ResultI8 result);
extern bool i8_result_ffi_is_err(ResultI8 result);
extern int8_t i8_result_ffi_ok(ResultI8 result);
extern Error i8_result_ffi_err(ResultI8 result);

struct ResultI16 {
    uint32_t tag;
    UnionI16 data;
};

extern bool i16_result_ffi_is_ok(ResultI16 result);
extern bool i16_result_ffi_is_err(ResultI16 result);
extern int16_t i16_result_ffi_ok(ResultI16 result);
extern Error i16_result_ffi_err(ResultI16 result);

struct ResultI32 {
    uint32_t tag;
    UnionI32 data;
};

extern bool i32_result_ffi_is_ok(ResultI32 result);
extern bool i32_result_ffi_is_err(ResultI32 result);
extern int32_t i32_result_ffi_ok(ResultI32 result);
extern Error i32_result_ffi_err(ResultI32 result);

struct ResultI64 {
    uint32_t tag;
    UnionI64 data;
};

extern bool i64_result_ffi_is_ok(ResultI64 result);
extern bool i64_result_ffi_is_err(ResultI64 result);
extern int64_t i64_result_ffi_ok(ResultI64 result);
extern Error i64_result_ffi_err(ResultI64 result);

struct ResultIsize {
    uint32_t tag;
    UnionIsize data;
};

extern bool isize_result_ffi_is_ok(ResultIsize result);
extern bool isize_result_ffi_is_err(ResultIsize result);
extern ptrdiff_t isize_result_ffi_ok(ResultIsize result);
extern Error isize_result_ffi_err(ResultIsize result);

struct ResultU8 {
    uint32_t tag;
    UnionU8 data;
};

extern bool u8_result_ffi_is_ok(ResultU8 result);
extern bool u8_result_ffi_is_err(ResultU8 result);
extern uint8_t u8_result_ffi_ok(ResultU8 result);
extern Error u8_result_ffi_err(ResultU8 result);

struct ResultU16 {
    uint32_t tag;
    UnionU16 data;
};

extern bool u16_result_ffi_is_ok(ResultU16 result);
extern bool u16_result_ffi_is_err(ResultU16 result);
extern uint16_t u16_result_ffi_ok(ResultU16 result);
extern Error u16_result_ffi_err(ResultU16 result);

struct ResultU32 {
    uint32_t tag;
    UnionU32 data;
};

extern bool u32_result_ffi_is_ok(ResultU32 result);
extern bool u32_result_ffi_is_err(ResultU32 result);
extern uint32_t u32_result_ffi_ok(ResultU32 result);
extern Error u32_result_ffi_err(ResultU32 result);

struct ResultU64 {
    uint32_t tag;
    UnionU64 data;
};

extern bool u64_result_ffi_is_ok(ResultU64 result);
extern bool u64_result_ffi_is_err(ResultU64 result);
extern uint64_t u64_result_ffi_ok(ResultU64 result);
extern Error u64_result_ffi_err(ResultU64 result);

struct ResultUsize {
    uint32_t tag;
    UnionUsize data;
};

extern bool usize_result_ffi_is_ok(ResultUsize result);
extern bool usize_result_ffi_is_err(ResultUsize result);
extern size_t usize_result_ffi_ok(ResultUsize result);
extern Error usize_result_ffi_err(ResultUsize result);

struct ResultF32 {
    uint32_t tag;
    UnionF32 data;
};

extern bool f32_result_ffi_is_ok(ResultF32 result);
extern bool f32_result_ffi_is_err(ResultF32 result);
extern float f32_result_ffi_ok(ResultF32 result);
extern Error f32_result_ffi_err(ResultF32 result);

struct ResultF64 {
    uint32_t tag;
    UnionF64 data;
};

extern bool f64_result_ffi_is_ok(ResultF64 result);
extern bool f64_result_ffi_is_err(ResultF64 result);
extern double f64_result_ffi_ok(ResultF64 result);
extern Error f64_result_ffi_err(ResultF64 result);

// TODO(ahuszagh) Finish the definitions.

#ifdef __cplusplus
}
#endif
#endif  /* !LEXICALCORE_H_ */
