/**
 *  Unittests for the C API to lexical-core.
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

#include <stdexcept>
#include <gtest/gtest.h>
#include "lexical.h"

// HELPERS
// -------

#define lexical_get_string(cb)                                                  \
    uint8_t* ptr;                                                               \
    size_t size;                                                                \
    if (::cb(&ptr, &size) != 0) {                                               \
        throw std::runtime_error("Unexpected runtime error.");                  \
    }                                                                           \
    auto value = std::string(reinterpret_cast<char*>(ptr), size)

// C++ wrapper for set_*_string.
#define lexical_set_string(cb)                                                  \
    auto* ptr = reinterpret_cast<uint8_t const*>(value.data());                 \
    auto size = value.length();                                                 \
    if (::cb(ptr, size) != 0) {                                                 \
        throw std::runtime_error("Unexpected runtime error.");                  \
    }

inline lexical_i8_result result_ok(lexical_i8 value)
{
    // Initialize the union.
    lexical_i8_result_union u;
    u.value = value;

    // Create the tagged union.
    lexical_i8_result r;
    r.tag = lexical_ok;
    r.data = u;

    return r;
}

inline lexical_i8_result result_err(int32_t code, size_t index)
{
    // Initialize the error.
    lexical_error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    lexical_i8_result_union u;
    u.error = e;

    // Create the tagged union.
    lexical_i8_result r;
    r.tag = lexical_err;
    r.data = u;

    return r;
}

inline lexical_i8_result result_overflow(size_t index)
{
    return result_err(lexical_overflow, index);
}

inline lexical_i8_result result_underflow(size_t index)
{
    return result_err(lexical_underflow, index);
}

inline lexical_i8_result result_invalid_digit(size_t index)
{
    return result_err(lexical_invalid_digit, index);
}

inline lexical_i8_result result_empty(size_t index)
{
    return result_err(lexical_empty, index);
}

inline lexical_i8_result result_empty_fraction(size_t index)
{
    return result_err(lexical_empty_fraction, index);
}

inline lexical_i8_result result_empty_exponent(size_t index)
{
    return result_err(lexical_empty_exponent, index);
}

inline lexical_i8_partial_result partial_result_ok(lexical_i8 value, size_t index)
{
    // Initialize the tuple.
    lexical_i8_partial_result_tuple t;
    t.x = value;
    t.y = index;

    // Initialize the union.
    lexical_i8_partial_result_union u;
    u.value = t;

    // Create the tagged union.
    lexical_i8_partial_result r;
    r.tag = lexical_ok;
    r.data = u;

    return r;
}

inline lexical_i8_partial_result partial_result_err(int32_t code, size_t index)
{
    // Initialize the error.
    lexical_error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    lexical_i8_partial_result_union u;
    u.error = e;

    // Create the tagged union.
    lexical_i8_partial_result r;
    r.tag = lexical_err;
    r.data = u;

    return r;
}

inline lexical_i8_partial_result partial_result_overflow(size_t index)
{
    return partial_result_err(lexical_overflow, index);
}

inline lexical_i8_partial_result partial_result_underflow(size_t index)
{
    return partial_result_err(lexical_underflow, index);
}

inline lexical_i8_partial_result partial_result_invalid_digit(size_t index)
{
    return partial_result_err(lexical_invalid_digit, index);
}

inline lexical_i8_partial_result partial_result_empty(size_t index)
{
    return partial_result_err(lexical_empty, index);
}

inline lexical_i8_partial_result partial_result_empty_fraction(size_t index)
{
    return partial_result_err(lexical_empty_fraction, index);
}

inline lexical_i8_partial_result partial_result_empty_exponent(size_t index)
{
    return partial_result_err(lexical_empty_exponent, index);
}

inline bool is_overflow(lexical_error error)
{
    return lexical_error_is_overflow(&error);
}

inline bool is_underflow(lexical_error error)
{
    return lexical_error_is_underflow(&error);
}

inline bool is_invalid_digit(lexical_error error)
{
    return lexical_error_is_invalid_digit(&error);
}

inline bool is_empty(lexical_error error)
{
    return lexical_error_is_empty(&error);
}

inline bool is_empty_fraction(lexical_error error)
{
    return lexical_error_is_empty_fraction(&error);
}

inline bool is_empty_exponent(lexical_error error)
{
    return lexical_error_is_empty_exponent(&error);
}

// CONFIG TESTS
// ------------

TEST(test_lexical_get_exponent_default_char, config_tests)
{
    EXPECT_EQ(lexical_get_exponent_default_char(), 'e');
}

TEST(test_lexical_set_exponent_default_char, config_tests)
{
    lexical_set_exponent_default_char('e');
}

#ifdef HAVE_RADIX

TEST(test_lexical_get_exponent_backup_char, config_tests)
{
    EXPECT_EQ(lexical_get_exponent_backup_char(), '^');
}

TEST(test_lexical_set_exponent_backup_char, config_tests)
{
    lexical_set_exponent_backup_char('^');
}

#endif  // HAVE_RADIX

#ifdef HAVE_ROUNDING

TEST(test_lexical_get_float_rounding, config_tests)
{
    EXPECT_EQ(lexical_get_float_rounding(), lexical_nearest_tie_even);
}

TEST(test_lexical_set_float_rounding, config_tests)
{
    lexical_set_float_rounding(lexical_nearest_tie_even);
}

#endif  // HAVE_ROUNDING

TEST(test_get_nan_string, config_tests)
{
    lexical_get_string(lexical_get_nan_string);
    EXPECT_EQ(value, "NaN");
}

TEST(test_set_nan_string, config_tests)
{
    std::string value = "NaN";
    lexical_set_string(lexical_set_nan_string);
}

TEST(test_get_inf_string, config_tests)
{
    lexical_get_string(lexical_get_inf_string);
    EXPECT_EQ(value, "inf");
}

TEST(test_set_inf_string, config_tests)
{
    std::string value = "inf";
    lexical_set_string(lexical_set_inf_string);
}

TEST(test_get_infinity_string, config_tests)
{
    lexical_get_string(lexical_get_infinity_string);
    EXPECT_EQ(value, "infinity");
}

TEST(test_set_infinity_string, config_tests)
{
    std::string value = "infinity";
    lexical_set_string(lexical_set_infinity_string);
}

// CONSTANT TESTS

TEST(test_size, constant_tests)
{
    // Check all the sizes can be used.
    EXPECT_GE(LEXICAL_I8_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I16_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I32_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I64_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_ISIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U8_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U16_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U32_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_U64_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_USIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_F32_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_F64_FORMATTED_SIZE, 0);
    EXPECT_GE(LEXICAL_I8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_I16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_I32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_I64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_U64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_F32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_F64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(LEXICAL_BUFFER_SIZE, 0);
}

// ERROR TESTS

TEST(test_is_overflow, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error underflow = { lexical_underflow, 0 };
    EXPECT_TRUE(lexical_error_is_overflow(&overflow));
    EXPECT_FALSE(lexical_error_is_overflow(&underflow));
}

TEST(test_is_underflow, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error underflow = { lexical_underflow, 0 };
    EXPECT_FALSE(lexical_error_is_underflow(&overflow));
    EXPECT_TRUE(lexical_error_is_underflow(&underflow));
}

TEST(test_is_invalid_digit, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_digit = { lexical_invalid_digit, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_digit(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_digit(&invalid_digit));
}

TEST(test_is_empty, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty = { lexical_empty, 0 };
    EXPECT_FALSE(lexical_error_is_empty(&overflow));
    EXPECT_TRUE(lexical_error_is_empty(&empty));
}

TEST(test_is_empty_fraction, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_fraction = { lexical_empty_fraction, 0 };
    EXPECT_FALSE(lexical_error_is_empty_fraction(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_fraction(&empty_fraction));
}

TEST(test_is_empty_exponent, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_exponent = { lexical_empty_exponent, 0 };
    EXPECT_FALSE(lexical_error_is_empty_exponent(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_exponent(&empty_exponent));
}

// RESULT TESTS

TEST(result, result_tests)
{
    auto ok = result_ok(0);
    auto overflow = result_overflow(0);
    auto underflow = result_underflow(0);
    auto invalid_digit = result_invalid_digit(0);
    auto empty = result_empty(0);
    auto empty_fraction = result_empty_fraction(0);
    auto empty_exponent = result_empty_exponent(0);

    EXPECT_TRUE(lexical_i8_result_is_ok(&ok));
    EXPECT_FALSE(lexical_i8_result_is_err(&ok));
    EXPECT_TRUE(lexical_i8_result_is_err(&overflow));
    EXPECT_TRUE(lexical_i8_result_is_err(&underflow));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_digit));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_fraction));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_exponent));

    EXPECT_EQ(lexical_i8_result_ok(ok), 0);
    EXPECT_TRUE(is_overflow(lexical_i8_result_err(overflow)));
    EXPECT_TRUE(is_underflow(lexical_i8_result_err(underflow)));
    EXPECT_TRUE(is_invalid_digit(lexical_i8_result_err(invalid_digit)));
    EXPECT_TRUE(is_empty(lexical_i8_result_err(empty)));
    EXPECT_TRUE(is_empty_fraction(lexical_i8_result_err(empty_fraction)));
    EXPECT_TRUE(is_empty_exponent(lexical_i8_result_err(empty_exponent)));
}

// PARTIAL RESULT TESTS

TEST(partial_result, partial_result_tests)
{
    auto ok = partial_result_ok(0, 1);
    auto overflow = partial_result_overflow(0);
    auto underflow = partial_result_underflow(0);
    auto invalid_digit = partial_result_invalid_digit(0);
    auto empty = partial_result_empty(0);
    auto empty_fraction = partial_result_empty_fraction(0);
    auto empty_exponent = partial_result_empty_exponent(0);

    EXPECT_TRUE(lexical_i8_partial_result_is_ok(&ok));
    EXPECT_FALSE(lexical_i8_partial_result_is_err(&ok));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&overflow));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&underflow));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_digit));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_fraction));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_exponent));

    EXPECT_EQ(lexical_i8_partial_result_ok(ok).x, 0);
    EXPECT_TRUE(is_overflow(lexical_i8_partial_result_err(overflow)));
    EXPECT_TRUE(is_underflow(lexical_i8_partial_result_err(underflow)));
    EXPECT_TRUE(is_invalid_digit(lexical_i8_partial_result_err(invalid_digit)));
    EXPECT_TRUE(is_empty(lexical_i8_partial_result_err(empty)));
    EXPECT_TRUE(is_empty_fraction(lexical_i8_partial_result_err(empty_fraction)));
    EXPECT_TRUE(is_empty_exponent(lexical_i8_partial_result_err(empty_exponent)));
}
