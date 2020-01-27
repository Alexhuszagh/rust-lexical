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

// C++ wrapper for get_*_string.
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

#define lexical_result_error(type)                                              \
    inline lexical_i8_result result_##type(size_t index)                        \
    {                                                                           \
        return result_err(lexical_##type, index);                               \
    }

lexical_result_error(overflow);
lexical_result_error(underflow);
lexical_result_error(invalid_digit);
lexical_result_error(empty);
lexical_result_error(empty_mantissa);
lexical_result_error(empty_exponent);
lexical_result_error(empty_integer);
lexical_result_error(empty_fraction);
lexical_result_error(invalid_positive_mantissa_sign);
lexical_result_error(missing_mantissa_sign);
lexical_result_error(invalid_exponent);
lexical_result_error(invalid_positive_exponent_sign);
lexical_result_error(missing_exponent_sign);
lexical_result_error(exponent_without_fraction);
lexical_result_error(invalid_leading_zeros);

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

#define lexical_partial_result_error(type)                                      \
    inline lexical_i8_partial_result partial_result_##type(size_t index)        \
    {                                                                           \
        return partial_result_err(lexical_##type, index);                       \
    }

lexical_partial_result_error(overflow);
lexical_partial_result_error(underflow);
lexical_partial_result_error(invalid_digit);
lexical_partial_result_error(empty);
lexical_partial_result_error(empty_mantissa);
lexical_partial_result_error(empty_exponent);
lexical_partial_result_error(empty_integer);
lexical_partial_result_error(empty_fraction);
lexical_partial_result_error(invalid_positive_mantissa_sign);
lexical_partial_result_error(missing_mantissa_sign);
lexical_partial_result_error(invalid_exponent);
lexical_partial_result_error(invalid_positive_exponent_sign);
lexical_partial_result_error(missing_exponent_sign);
lexical_partial_result_error(exponent_without_fraction);
lexical_partial_result_error(invalid_leading_zeros);

#define lexical_is_error(type)                                                  \
    inline bool is_##type(lexical_error error)                                  \
    {                                                                           \
        return lexical_error_is_##type(&error);                                 \
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

TEST(test_is_empty_mantissa, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_mantissa = { lexical_empty_mantissa, 0 };
    EXPECT_FALSE(lexical_error_is_empty_mantissa(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_mantissa(&empty_mantissa));
}

TEST(test_is_empty_exponent, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_exponent = { lexical_empty_exponent, 0 };
    EXPECT_FALSE(lexical_error_is_empty_exponent(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_exponent(&empty_exponent));
}

TEST(test_is_empty_integer, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_integer = { lexical_empty_integer, 0 };
    EXPECT_FALSE(lexical_error_is_empty_integer(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_integer(&empty_integer));
}

TEST(test_is_empty_fraction, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error empty_fraction = { lexical_empty_fraction, 0 };
    EXPECT_FALSE(lexical_error_is_empty_fraction(&overflow));
    EXPECT_TRUE(lexical_error_is_empty_fraction(&empty_fraction));
}

TEST(test_is_invalid_positive_mantissa_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_positive_mantissa_sign = { lexical_invalid_positive_mantissa_sign, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_positive_mantissa_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_positive_mantissa_sign(&invalid_positive_mantissa_sign));
}

TEST(test_is_missing_mantissa_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error missing_mantissa_sign = { lexical_missing_mantissa_sign, 0 };
    EXPECT_FALSE(lexical_error_is_missing_mantissa_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_missing_mantissa_sign(&missing_mantissa_sign));
}

TEST(test_is_invalid_exponent, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_exponent = { lexical_invalid_exponent, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_exponent(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_exponent(&invalid_exponent));
}

TEST(test_is_invalid_positive_exponent_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_positive_exponent_sign = { lexical_invalid_positive_exponent_sign, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_positive_exponent_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_positive_exponent_sign(&invalid_positive_exponent_sign));
}

TEST(test_is_missing_exponent_sign, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error missing_exponent_sign = { lexical_missing_exponent_sign, 0 };
    EXPECT_FALSE(lexical_error_is_missing_exponent_sign(&overflow));
    EXPECT_TRUE(lexical_error_is_missing_exponent_sign(&missing_exponent_sign));
}

TEST(test_is_exponent_without_fraction, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error exponent_without_fraction = { lexical_exponent_without_fraction, 0 };
    EXPECT_FALSE(lexical_error_is_exponent_without_fraction(&overflow));
    EXPECT_TRUE(lexical_error_is_exponent_without_fraction(&exponent_without_fraction));
}

TEST(test_is_invalid_leading_zeros, error_tests)
{
    lexical_error overflow = { lexical_overflow, 0 };
    lexical_error invalid_leading_zeros = { lexical_invalid_leading_zeros, 0 };
    EXPECT_FALSE(lexical_error_is_invalid_leading_zeros(&overflow));
    EXPECT_TRUE(lexical_error_is_invalid_leading_zeros(&invalid_leading_zeros));
}

// RESULT TESTS

TEST(result, result_tests)
{
    auto ok = result_ok(0);
    auto overflow = result_overflow(0);
    auto underflow = result_underflow(0);
    auto invalid_digit = result_invalid_digit(0);
    auto empty = result_empty(0);
    auto empty_mantissa = result_empty_mantissa(0);
    auto empty_exponent = result_empty_exponent(0);
    auto empty_integer = result_empty_integer(0);
    auto empty_fraction = result_empty_fraction(0);
    auto invalid_positive_mantissa_sign = result_invalid_positive_mantissa_sign(0);
    auto missing_mantissa_sign = result_missing_mantissa_sign(0);
    auto invalid_exponent = result_invalid_exponent(0);
    auto invalid_positive_exponent_sign = result_invalid_positive_exponent_sign(0);
    auto missing_exponent_sign = result_missing_exponent_sign(0);
    auto exponent_without_fraction = result_exponent_without_fraction(0);
    auto invalid_leading_zeros = result_invalid_leading_zeros(0);

    EXPECT_TRUE(lexical_i8_result_is_ok(&ok));
    EXPECT_FALSE(lexical_i8_result_is_err(&ok));
    EXPECT_TRUE(lexical_i8_result_is_err(&overflow));
    EXPECT_TRUE(lexical_i8_result_is_err(&underflow));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_digit));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_mantissa));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_exponent));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_integer));
    EXPECT_TRUE(lexical_i8_result_is_err(&empty_fraction));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_positive_mantissa_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&missing_mantissa_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_exponent));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_positive_exponent_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&missing_exponent_sign));
    EXPECT_TRUE(lexical_i8_result_is_err(&exponent_without_fraction));
    EXPECT_TRUE(lexical_i8_result_is_err(&invalid_leading_zeros));

    EXPECT_EQ(lexical_i8_result_ok(ok), 0);
    EXPECT_TRUE(is_overflow(lexical_i8_result_err(overflow)));
    EXPECT_TRUE(is_underflow(lexical_i8_result_err(underflow)));
    EXPECT_TRUE(is_invalid_digit(lexical_i8_result_err(invalid_digit)));
    EXPECT_TRUE(is_empty(lexical_i8_result_err(empty)));
    EXPECT_TRUE(is_empty_mantissa(lexical_i8_result_err(empty_mantissa)));
    EXPECT_TRUE(is_empty_exponent(lexical_i8_result_err(empty_exponent)));
    EXPECT_TRUE(is_empty_integer(lexical_i8_result_err(empty_integer)));
    EXPECT_TRUE(is_empty_fraction(lexical_i8_result_err(empty_fraction)));
    EXPECT_TRUE(is_invalid_positive_mantissa_sign(lexical_i8_result_err(invalid_positive_mantissa_sign)));
    EXPECT_TRUE(is_missing_mantissa_sign(lexical_i8_result_err(missing_mantissa_sign)));
    EXPECT_TRUE(is_invalid_exponent(lexical_i8_result_err(invalid_exponent)));
    EXPECT_TRUE(is_invalid_positive_exponent_sign(lexical_i8_result_err(invalid_positive_exponent_sign)));
    EXPECT_TRUE(is_missing_exponent_sign(lexical_i8_result_err(missing_exponent_sign)));
    EXPECT_TRUE(is_exponent_without_fraction(lexical_i8_result_err(exponent_without_fraction)));
    EXPECT_TRUE(is_invalid_leading_zeros(lexical_i8_result_err(invalid_leading_zeros)));
}

// PARTIAL RESULT TESTS

TEST(partial_result, partial_result_tests)
{
    auto ok = partial_result_ok(0, 1);
    auto overflow = partial_result_overflow(0);
    auto underflow = partial_result_underflow(0);
    auto invalid_digit = partial_result_invalid_digit(0);
    auto empty = partial_result_empty(0);
    auto empty_mantissa = partial_result_empty_mantissa(0);
    auto empty_exponent = partial_result_empty_exponent(0);
    auto empty_integer = partial_result_empty_integer(0);
    auto empty_fraction = partial_result_empty_fraction(0);
    auto invalid_positive_mantissa_sign = partial_result_invalid_positive_mantissa_sign(0);
    auto missing_mantissa_sign = partial_result_missing_mantissa_sign(0);
    auto invalid_exponent = partial_result_invalid_exponent(0);
    auto invalid_positive_exponent_sign = partial_result_invalid_positive_exponent_sign(0);
    auto missing_exponent_sign = partial_result_missing_exponent_sign(0);
    auto exponent_without_fraction = partial_result_exponent_without_fraction(0);
    auto invalid_leading_zeros = partial_result_invalid_leading_zeros(0);

    EXPECT_TRUE(lexical_i8_partial_result_is_ok(&ok));
    EXPECT_FALSE(lexical_i8_partial_result_is_err(&ok));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&overflow));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&underflow));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_digit));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_mantissa));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_exponent));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_integer));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&empty_fraction));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_positive_mantissa_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&missing_mantissa_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_exponent));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_positive_exponent_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&missing_exponent_sign));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&exponent_without_fraction));
    EXPECT_TRUE(lexical_i8_partial_result_is_err(&invalid_leading_zeros));

    EXPECT_EQ(lexical_i8_partial_result_ok(ok).x, 0);
    EXPECT_TRUE(is_overflow(lexical_i8_partial_result_err(overflow)));
    EXPECT_TRUE(is_underflow(lexical_i8_partial_result_err(underflow)));
    EXPECT_TRUE(is_invalid_digit(lexical_i8_partial_result_err(invalid_digit)));
    EXPECT_TRUE(is_empty(lexical_i8_partial_result_err(empty)));
    EXPECT_TRUE(is_empty_mantissa(lexical_i8_partial_result_err(empty_mantissa)));
    EXPECT_TRUE(is_empty_exponent(lexical_i8_partial_result_err(empty_exponent)));
    EXPECT_TRUE(is_empty_integer(lexical_i8_partial_result_err(empty_integer)));
    EXPECT_TRUE(is_empty_fraction(lexical_i8_partial_result_err(empty_fraction)));
    EXPECT_TRUE(is_invalid_positive_mantissa_sign(lexical_i8_partial_result_err(invalid_positive_mantissa_sign)));
    EXPECT_TRUE(is_missing_mantissa_sign(lexical_i8_partial_result_err(missing_mantissa_sign)));
    EXPECT_TRUE(is_invalid_exponent(lexical_i8_partial_result_err(invalid_exponent)));
    EXPECT_TRUE(is_invalid_positive_exponent_sign(lexical_i8_partial_result_err(invalid_positive_exponent_sign)));
    EXPECT_TRUE(is_missing_exponent_sign(lexical_i8_partial_result_err(missing_exponent_sign)));
    EXPECT_TRUE(is_exponent_without_fraction(lexical_i8_partial_result_err(exponent_without_fraction)));
    EXPECT_TRUE(is_invalid_leading_zeros(lexical_i8_partial_result_err(invalid_leading_zeros)));
}
