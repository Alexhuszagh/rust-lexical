/**
 *  lexical_core
 *  ============
 *
 *  Access lexical-core functionality from C++.
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

#ifndef LEXICAL_HPP_
#define LEXICAL_HPP_

#include "lexical.h"
#include <cassert>
#include <cstdint>
#include <cstring>
#include <iterator>
#include <stdexcept>
#include <string>
#include <tuple>
#include <type_traits>

#if __cplusplus >= 201703L
#   include <string_view>
#endif

namespace lexical {

// Features must be enabled through the following macro definitions:
//  1. HAVE_RADIX
//  2. HAVE_ROUNDING

// CONFIG
// ------

inline uint8_t get_exponent_default_char()
{
    return ::lexical_get_exponent_default_char();
}

inline void set_exponent_default_char(uint8_t ch)
{
    ::lexical_set_exponent_default_char(ch);
}

#ifdef HAVE_RADIX
    inline uint8_t get_exponent_backup_char()
    {
        return ::lexical_get_exponent_backup_char();
    }

    inline void set_exponent_backup_char(uint8_t ch)
    {
        ::lexical_set_exponent_backup_char(ch);
    }
#endif  // HAVE_RADIX

#ifdef HAVE_ROUNDING
    // Rounding type for float-parsing.
    enum class rounding_kind: uint32_t {
        nearest_tie_even = ::lexical_nearest_tie_even,
        nearest_tie_away_zero = ::lexical_nearest_tie_away_zero,
        toward_positive_infinity = ::lexical_toward_positive_infinity,
        toward_negative_infinity = ::lexical_toward_negative_infinity,
        toward_zero = ::lexical_toward_zero,
    };

    inline rounding_kind get_float_rounding()
    {
        return static_cast<rounding_kind>(::lexical_get_float_rounding());
    }

    inline void set_float_rounding(rounding_kind rounding)
    {
        ::lexical_set_float_rounding(static_cast<uint32_t>(rounding));
    }
#endif  // HAVE_ROUNDING

// C++ wrapper for get_*_string.
#define lexical_get_string(cb)                                                  \
    uint8_t* ptr;                                                               \
    size_t size;                                                                \
    if (::cb(&ptr, &size) != 0) {                                               \
        throw std::runtime_error("Unexpected runtime error.");                  \
    }                                                                           \
    return std::string(reinterpret_cast<char*>(ptr), size);

// C++ wrapper for set_*_string.
#define lexical_set_string(cb)                                                  \
    auto* ptr = reinterpret_cast<uint8_t const*>(string.data());                \
    auto size = string.length();                                                \
    if (::cb(ptr, size) != 0) {                                                 \
        throw std::runtime_error("Unexpected runtime error.");                  \
    }

inline std::string get_nan_string()
{
    lexical_get_string(lexical_get_nan_string)
}

inline void set_nan_string(const std::string& string)
{
    lexical_set_string(lexical_set_nan_string)
}

inline std::string get_inf_string()
{
    lexical_get_string(lexical_get_inf_string)
}

inline void set_inf_string(const std::string& string)
{
    lexical_set_string(lexical_set_inf_string)
}

inline std::string get_infinity_string()
{
    lexical_get_string(lexical_get_infinity_string)
}

inline void set_infinity_string(const std::string& string)
{
    lexical_set_string(lexical_set_infinity_string)
}

// CONSTANTS
// ---------

static const size_t I8_FORMATTED_SIZE = ::LEXICAL_I8_FORMATTED_SIZE;
static const size_t I16_FORMATTED_SIZE = ::LEXICAL_I16_FORMATTED_SIZE;
static const size_t I32_FORMATTED_SIZE = ::LEXICAL_I32_FORMATTED_SIZE;
static const size_t I64_FORMATTED_SIZE = ::LEXICAL_I64_FORMATTED_SIZE;
static const size_t ISIZE_FORMATTED_SIZE = ::LEXICAL_ISIZE_FORMATTED_SIZE;
static const size_t U8_FORMATTED_SIZE = ::LEXICAL_U8_FORMATTED_SIZE;
static const size_t U16_FORMATTED_SIZE = ::LEXICAL_U16_FORMATTED_SIZE;
static const size_t U32_FORMATTED_SIZE = ::LEXICAL_U32_FORMATTED_SIZE;
static const size_t U64_FORMATTED_SIZE = ::LEXICAL_U64_FORMATTED_SIZE;
static const size_t USIZE_FORMATTED_SIZE = ::LEXICAL_USIZE_FORMATTED_SIZE;
static const size_t F32_FORMATTED_SIZE = ::LEXICAL_F32_FORMATTED_SIZE;
static const size_t F64_FORMATTED_SIZE = ::LEXICAL_F64_FORMATTED_SIZE;

static const size_t I8_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I8_FORMATTED_SIZE_DECIMAL;
static const size_t I16_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I16_FORMATTED_SIZE_DECIMAL;
static const size_t I32_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I32_FORMATTED_SIZE_DECIMAL;
static const size_t I64_FORMATTED_SIZE_DECIMAL = ::LEXICAL_I64_FORMATTED_SIZE_DECIMAL;
static const size_t ISIZE_FORMATTED_SIZE_DECIMAL = ::LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL;
static const size_t U8_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U8_FORMATTED_SIZE_DECIMAL;
static const size_t U16_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U16_FORMATTED_SIZE_DECIMAL;
static const size_t U32_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U32_FORMATTED_SIZE_DECIMAL;
static const size_t U64_FORMATTED_SIZE_DECIMAL = ::LEXICAL_U64_FORMATTED_SIZE_DECIMAL;
static const size_t USIZE_FORMATTED_SIZE_DECIMAL = ::LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL;
static const size_t F32_FORMATTED_SIZE_DECIMAL = ::LEXICAL_F32_FORMATTED_SIZE_DECIMAL;
static const size_t F64_FORMATTED_SIZE_DECIMAL = ::LEXICAL_F64_FORMATTED_SIZE_DECIMAL;

static const size_t BUFFER_SIZE = ::LEXICAL_BUFFER_SIZE;

// Buffer size used internally for the to_string implementations.
// Avoid malloc whenever possible.
static constexpr size_t WRITE_SIZE = 256;

// TYPES
// -----

// ALIAS
using u8 = ::lexical_u8;
using u16 = ::lexical_u16;
using u32 = ::lexical_u32;
using u64 = ::lexical_u64;
using usize = ::lexical_usize;

using i8 = ::lexical_i8;
using i16 = ::lexical_i16;
using i32 = ::lexical_i32;
using i64 = ::lexical_i64;
using isize = ::lexical_isize;

using f32 = ::lexical_f32;
using f64 = ::lexical_f64;

// Internal typedef for the parsers.
// If we have C++17, we can use `std::string_view` by value.
// Otherwise, use a const ref to `std::string`.
#if __cplusplus >= 201703L
    using string_type = std::string_view;
#else   // !CPP_17
    using string_type = const std::string&;
#endif  // CPP_17

// ERROR

// Error code, indicating failure type.
enum class error_code: int32_t {
    overflow = ::lexical_overflow,
    underflow = ::lexical_underflow,
    invalid_digit = ::lexical_invalid_digit,
    empty = ::lexical_empty,
    empty_mantissa = ::lexical_empty_mantissa,
    empty_exponent = ::lexical_empty_exponent,
    empty_integer = ::lexical_empty_integer,
    empty_fraction = ::lexical_empty_fraction,
    invalid_positive_mantissa_sign = ::lexical_invalid_positive_mantissa_sign,
    missing_mantissa_sign = ::lexical_missing_mantissa_sign,
    invalid_exponent = ::lexical_invalid_exponent,
    invalid_positive_exponent_sign = ::lexical_invalid_positive_exponent_sign,
    missing_exponent_sign = ::lexical_missing_exponent_sign,
    exponent_without_fraction = ::lexical_exponent_without_fraction,
};

// Determine if an error code matches the desired code.
#define lexical_is_error(type)                                                  \
    inline bool is_##type()                                                     \
    {                                                                           \
        return code == error_code::type;                                        \
    }

// C-compatible error type.
struct error
{
    error_code code;
    size_t index;

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

    inline friend bool operator==(const error& lhs, const error& rhs)
    {
        return std::make_tuple(lhs.code, lhs.index) == std::make_tuple(rhs.code, rhs.index);
    }

    inline friend bool operator!=(const error& lhs, const error& rhs)
    {
        return !(lhs == rhs);
    }
};

// RESULT TAG

// Tag for the result type in the tagged enum.
enum class result_tag: uint32_t {
    ok = ::lexical_ok,
    err = ::lexical_err,
};

// COMPLETE UNIONS

// Union for the lexical result type.
template <typename T>
union result_union {
    T value;
    struct error error;
};

// COMPLETE RESULTS

// Result type for parser functions.
template <typename T>
struct result {
    result_tag tag;
    result_union<T> data;

    // Safely convert from a C-style result to a C++ one.
    // This is to prevent layout differences from causing UB.
    template <typename ResultT>
    static inline result from(ResultT c_res)
    {
        // Ensure we likely have a similar layout.
        // We're technically invoking UB, since the layout isn't
        // guaranteed to be the same, but it would take a
        // very malicious compiler to do so.
        // Pretty much any approach would result in UB, even the platform-
        // specific bindings, since the structs aren't **guaranteed**
        // to be the same as what we're using.
        static_assert(sizeof(ResultT) == sizeof(result), "Invalid sizes");
        static_assert(std::is_standard_layout<ResultT>::value, "Not std layout");
        static_assert(std::is_standard_layout<result>::value, "Not std layout");

        result cc_res;
        std::memcpy(&cc_res, &c_res, sizeof(result));
        return cc_res;
    }

    inline bool is_ok()
    {
        return tag == result_tag::ok;
    }

    inline bool is_err()
    {
        return tag == result_tag::err;
    }

    inline T ok()
    {
        assert(is_ok());
        return std::move(data.value);
    }

    inline error err()
    {
        assert(is_err());
        return std::move(data.error);
    }

    inline friend bool operator==(const result& lhs, const result& rhs)
    {
        if (lhs.tag != rhs.tag) {
            return false;
        } else if (lhs.tag == result_tag::ok) {
            return lhs.data.value == rhs.data.value;
        } else {
            return lhs.data.error == rhs.data.error;
        }
    }

    inline friend bool operator!=(const result& lhs, const result& rhs)
    {
        return !(lhs == rhs);
    }
};

// PARTIAL RESULT TUPLES

// Result value type for the partial parsers.
template <typename T>
struct partial_result_tuple {
    T x;
    size_t y;

    inline friend bool operator==(const partial_result_tuple& lhs, const partial_result_tuple& rhs)
    {
        return std::make_tuple(lhs.x, lhs.y) == std::make_tuple(rhs.x, rhs.y);
    }

    inline friend bool operator!=(const partial_result_tuple& lhs, const partial_result_tuple& rhs)
    {
        return !(lhs == rhs);
    }
};

// PARTIAL RESULT UNIONS

// Union for the lexical partial result type.
template <typename T>
union partial_result_union {
    partial_result_tuple<T> value;
    struct error error;
};

// PARTIAL RESULTS

// Result type for partial parser functions.
template <typename T>
struct partial_result {
    result_tag tag;
    partial_result_union<T> data;

    // Safely convert from a C-style result to a C++ one.
    // This is to prevent layout differences from causing UB.
    template <typename ResultT>
    static inline partial_result from(ResultT c_res)
    {
        // Ensure we likely have a similar layout.
        // We're technically invoking UB, since the layout isn't
        // guaranteed to be the same, but it would take a
        // very malicious compiler to do so.
        // Pretty much any approach would result in UB, even the platform-
        // specific bindings, since the structs aren't **guaranteed**
        // to be the same as what we're using.
        static_assert(sizeof(ResultT) == sizeof(partial_result), "Invalid sizes");
        static_assert(std::is_standard_layout<ResultT>::value, "Not std layout");
        static_assert(std::is_standard_layout<partial_result>::value, "Not std layout");

        partial_result cc_res;
        std::memcpy(&cc_res, &c_res, sizeof(partial_result));
        return cc_res;
    }

    inline bool is_ok()
    {
        return tag == result_tag::ok;
    }

    inline bool is_err()
    {
        return tag == result_tag::err;
    }

    inline std::tuple<T, size_t> ok()
    {
        assert(is_ok());
        auto value = std::move(data.value);
        return std::make_tuple(std::move(value.x), std::move(value.y));
    }

    inline error err()
    {
        assert(is_err());
        return std::move(data.error);
    }

    inline friend bool operator==(const partial_result& lhs, const partial_result& rhs)
    {
        if (lhs.tag != rhs.tag) {
            return false;
        } else if (lhs.tag == result_tag::ok) {
            return lhs.data.value == rhs.data.value;
        } else {
            return lhs.data.error == rhs.data.error;
        }
    }

    inline friend bool operator!=(const partial_result& lhs, const partial_result& rhs)
    {
        return !(lhs == rhs);
    }
};

// API
// ---

// DISPATCHER

// Dispatch function for to_lexical.
#define lexical_to_lexical(type)                                                \
    inline static                                                               \
    uint8_t*                                                                    \
    to_lexical(                                                                 \
        type value,                                                             \
        uint8_t* first,                                                         \
        uint8_t* last                                                           \
    )                                                                           \
    {                                                                           \
        return ::lexical_##type##toa(value, first, last);                       \
    }

// Dispatch function for to_lexical_radix.
#define lexical_to_lexical_radix(type)                                          \
    inline static                                                               \
    uint8_t*                                                                    \
    to_lexical_radix(                                                           \
        type value,                                                             \
        uint8_t radix,                                                          \
        uint8_t* first,                                                         \
        uint8_t* last                                                           \
    )                                                                           \
    {                                                                           \
        return ::lexical_##type##toa_radix(value, radix, first, last);          \
    }

// Dispatch function for from_lexical.
#define lexical_from_lexical(type)                                              \
    inline static                                                               \
    result<type>                                                                \
    from_lexical(                                                               \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )                                                                           \
    {                                                                           \
        using result_type = result<type>;                                       \
        auto r = ::lexical_ato##type(first, last);                              \
        return result_type::from(r);                                            \
    }

// Dispatch function for from_lexical_partial.
#define lexical_from_lexical_partial(type)                                      \
    inline static                                                               \
    partial_result<type>                                                        \
    from_lexical_partial(                                                       \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )                                                                           \
    {                                                                           \
        using partial_result_type = partial_result<type>;                       \
        auto r = ::lexical_ato##type##_partial(first, last);                    \
        return partial_result_type::from(r);                                    \
    }

// Dispatch function for from_lexical_radix.
#define lexical_from_lexical_radix(type)                                        \
    inline static                                                               \
    result<type>                                                                \
    from_lexical_radix(                                                         \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )                                                                           \
    {                                                                           \
        using result_type = result<type>;                                       \
        auto r = ::lexical_ato##type##_radix(first, last, radix);               \
        return result_type::from(r);                                            \
    }

// Dispatch function for from_lexical_partial_radix.
#define lexical_from_lexical_partial_radix(type)                                \
    inline static                                                               \
    partial_result<type>                                                        \
    from_lexical_partial_radix(                                                 \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )                                                                           \
    {                                                                           \
        using partial_result_type = partial_result<type>;                       \
        auto r = ::lexical_ato##type##_partial_radix(first, last, radix);       \
        return partial_result_type::from(r);                                    \
    }

// Get type name for lexical dispatcher
#define lexical_dispatcher_type(type) type##_dispatcher

// Define a dispatcher for a given type. This
// allows us to use std::conditional to get the proper
// type (a type) from the type. Every single function will
// be static.
#ifdef HAVE_RADIX
    #define lexical_dispatcher(type)                                            \
        struct lexical_dispatcher_type(type)                                    \
        {                                                                       \
            lexical_to_lexical(type)                                            \
            lexical_to_lexical_radix(type)                                      \
            lexical_from_lexical(type)                                          \
            lexical_from_lexical_partial(type)                                  \
            lexical_from_lexical_radix(type)                                    \
            lexical_from_lexical_partial_radix(type)                            \
        }
#else   // !HAVE_RADIX
    #define lexical_dispatcher(type)                                            \
        struct lexical_dispatcher_type(type)                                    \
        {                                                                       \
            lexical_to_lexical(type)                                            \
            lexical_from_lexical(type)                                          \
            lexical_from_lexical_partial(type)                                  \
        }
#endif  // HAVE_RADIX

lexical_dispatcher(i8);
lexical_dispatcher(i16);
lexical_dispatcher(i32);
lexical_dispatcher(i64);
lexical_dispatcher(isize);

lexical_dispatcher(u8);
lexical_dispatcher(u16);
lexical_dispatcher(u32);
lexical_dispatcher(u64);
lexical_dispatcher(usize);

lexical_dispatcher(f32);
lexical_dispatcher(f64);

// LOSSY DISPATCHER

// Dispatch function for from_lexical_lossy.
#define lexical_from_lexical_lossy(type)                                        \
    inline static                                                               \
    result<type>                                                                \
    from_lexical_lossy(                                                         \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )                                                                           \
    {                                                                           \
        using result_type = result<type>;                                       \
        auto r = ::lexical_ato##type##_lossy(first, last);                      \
        return result_type::from(r);                                            \
    }


// Dispatch function for from_lexical_partial_lossy.
#define lexical_from_lexical_partial_lossy(type)                                \
    inline static                                                               \
    partial_result<type>                                                        \
    from_lexical_partial_lossy(                                                 \
        uint8_t const* first,                                                   \
        uint8_t const* last                                                     \
    )                                                                           \
    {                                                                           \
        using partial_result_type = partial_result<type>;                       \
        auto r = ::lexical_ato##type##_partial_lossy(first, last);              \
        return partial_result_type::from(r);                                    \
    }

// Dispatch function for from_lexical_lossy_radix.
#define lexical_from_lexical_lossy_radix(type)                                  \
    inline static                                                               \
    result<type>                                                                \
    from_lexical_lossy_radix(                                                   \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )                                                                           \
    {                                                                           \
        using result_type = result<type>;                                       \
        auto r = ::lexical_ato##type##_lossy_radix(first, last, radix);         \
        return result_type::from(r);                                            \
    }

// Dispatch function for from_lexical_partial_lossy_radix.
#define lexical_from_lexical_partial_lossy_radix(type)                          \
    inline static                                                               \
    partial_result<type>                                                        \
    from_lexical_partial_lossy_radix(                                           \
        uint8_t const* first,                                                   \
        uint8_t const* last,                                                    \
        uint8_t radix                                                           \
    )                                                                           \
    {                                                                           \
        using partial_result_type = partial_result<type>;                       \
        auto r = ::lexical_ato##type##_partial_lossy_radix(first, last, radix); \
        return partial_result_type::from(r);                                    \
    }

// Get type name for lexical lossy dispatcher
#define lexical_lossy_dispatcher_type(type) type##_lossy_dispatcher

// Define a lossy dispatcher for a given type. This
// allows us to use std::conditional to get the proper
// type (a type) from the type. Every single function will
// be static.
#ifdef HAVE_RADIX
    #define lexical_lossy_dispatcher(type)                                      \
        struct lexical_lossy_dispatcher_type(type)                              \
        {                                                                       \
            lexical_from_lexical_lossy(type)                                    \
            lexical_from_lexical_partial_lossy(type)                            \
            lexical_from_lexical_lossy_radix(type)                              \
            lexical_from_lexical_partial_lossy_radix(type)                      \
        }
#else   // !HAVE_RADIX
    #define lexical_lossy_dispatcher(type)                                      \
        struct lexical_lossy_dispatcher_type(type)                              \
        {                                                                       \
            lexical_from_lexical_lossy(type)                                    \
            lexical_from_lexical_partial_lossy(type)                            \
        }
#endif  // HAVE_RADIX

lexical_lossy_dispatcher(f32);
lexical_lossy_dispatcher(f64);

// GET DISPATCHER

// Check if value is same as type parameter.
#define lexical_is_same(type) std::is_same<T, type>::value

// Conditional to simplify long recursive statements.
// This is to prevent a lot of super ugly code from being written
// (not that it's very pretty regardless).
#define lexical_conditional(name, fallback)                                     \
    typename std::conditional<                                                  \
        lexical_is_same(name),                                                  \
        lexical_dispatcher_type(name),                                          \
        fallback                                                                \
    >::type

// Create a single template that resolves to our dispatcher **or**
// evaluates to void.
template <typename T>
using dispatcher = lexical_conditional(
    i8, lexical_conditional(i16, lexical_conditional(i32,
        lexical_conditional(i64, lexical_conditional(isize,
            lexical_conditional(u8, lexical_conditional(u16,
                lexical_conditional(u32, lexical_conditional(u64,
                    lexical_conditional(usize, lexical_conditional(f32,
                        lexical_conditional(f64, void)
                    ))
                ))
            ))
        ))
    ))
);

// GET LOSSY DISPATCHER

// Conditional to simplify long recursive statements.
#define lexical_lossy_conditional(name, fallback)                               \
    typename std::conditional<                                                  \
        lexical_is_same(name),                                                  \
        lexical_lossy_dispatcher_type(name),                                    \
        fallback                                                                \
    >::type

// Create a single template that resolves to our lossy dispatcher **or**
// evaluates to void.
template <typename T>
using lossy_dispatcher = lexical_lossy_conditional(f32,
    lexical_lossy_conditional(f64, void)
);

// TO STRING

// Write directly to a pointer range and
// return a pointer to the last written element.
template <typename T>
inline uint8_t* write(T value, uint8_t* first, uint8_t* last)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to write.");
    return disp::to_lexical(value, first, last);
}

// High-level function to serialize a value to a string.
template <typename T>
inline std::string to_string(T value)
{
    assert(WRITE_SIZE >= BUFFER_SIZE);

    uint8_t array[WRITE_SIZE];
    uint8_t* first = array;
    uint8_t* last = first + WRITE_SIZE;
    auto ptr = write(value, first, last);
    return std::string(reinterpret_cast<char*>(first), std::distance(first, ptr));
}

#ifdef HAVE_RADIX
    // Write directly to a pointer range and
    // return a pointer to the last written element.
    template <typename T>
    inline uint8_t* write_radix(T value, uint8_t radix, uint8_t* first, uint8_t* last)
    {
        using disp = dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to write_radix.");
        return disp::to_lexical_radix(value, radix, first, last);
    }

    // High-level function to serialize a value to a string with a custom radix.
    template <typename T>
    inline std::string to_string_radix(T value, uint8_t radix)
    {
        assert(WRITE_SIZE >= BUFFER_SIZE);

        uint8_t array[WRITE_SIZE];
        uint8_t* first = array;
        uint8_t* last = first + WRITE_SIZE;
        auto ptr = write_radix(value, radix, first, last);
        return std::string(reinterpret_cast<char*>(first), std::distance(first, ptr));
    }
#endif  // HAVE_RADIX

// PARSE

// High-level function to parse a value from string.
template <typename T>
inline result<T> parse(string_type string)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical(first, last);
}

// High-level function to partially parse a value from string.
template <typename T>
inline partial_result<T> parse_partial(string_type string)
{
    using disp = dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical_partial(first, last);
}

// High-level function to lossily parse a value from string.
template <typename T>
inline result<T> parse_lossy(string_type string)
{
    using disp = lossy_dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical_lossy(first, last);
}

// High-level function to lossily, partially parse a value from string.
template <typename T>
inline partial_result<T> parse_partial_lossy(string_type string)
{
    using disp = lossy_dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical_partial_lossy(first, last);
}

#ifdef HAVE_RADIX
    // High-level function to parse a value from string with a custom radix.
    template <typename T>
    inline result<T> parse_radix(string_type string, uint8_t radix)
    {
        using disp = dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_radix(first, last, radix);
    }

    // High-level function to partially parse a value from string with a custom radix.
    template <typename T>
    inline partial_result<T> parse_partial_radix(string_type string, uint8_t radix)
    {
        using disp = dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_radix(first, last, radix);
    }

    // High-level function to lossily parse a value from string with a custom radix.
    template <typename T>
    inline result<T> parse_lossy_radix(string_type string, uint8_t radix)
    {
        using disp = lossy_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_lossy_radix(first, last, radix);
    }

    // High-level function to lossily, partially parse a value from string with a custom radix.
    template <typename T>
    inline partial_result<T> parse_partial_lossy_radix(string_type string, uint8_t radix)
    {
        using disp = lossy_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_lossy_radix(first, last, radix);
    }
#endif  // HAVE_RADIX

// CLEANUP
// -------

#undef lexical_get_string
#undef lexical_set_string
#undef lexical_is_error
#undef lexical_to_lexical
#undef lexical_to_lexical_radix
#undef lexical_from_lexical
#undef lexical_from_lexical_partial
#undef lexical_from_lexical_radix
#undef lexical_from_lexical_partial_radix
#undef lexical_from_lexical_lossy
#undef lexical_from_lexical_partial_lossy
#undef lexical_from_lexical_lossy_radix
#undef lexical_from_lexical_partial_lossy_radix
#undef lexical_dispatcher
#undef lexical_dispatcher_type
#undef lexical_lossy_dispatcher
#undef lexical_lossy_dispatcher_type
#undef lexical_is_same
#undef lexical_conditional
#undef lexical_lossy_conditional

}   // lexical

#endif  /* !LEXICAL_HPP_ */
