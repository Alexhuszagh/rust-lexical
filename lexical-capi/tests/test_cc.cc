/**
 *  Unittests for the C++ API to lexical-core.
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

#include <gtest/gtest.h>
#include "lexical.hpp"

using namespace lexical;

// HELPERS
// -------

template <typename T>
inline result<T> result_ok(T value)
{
    // Initialize the union.
    result_union<T> u;
    u.value = value;

    // Create the tagged union.
    result<T> r;
    r.tag = result_tag::ok;
    r.data = u;

    return r;
}

template <typename T>
inline result<T> result_err(error_code code, size_t index)
{
    // Initialize the error.
    error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    result_union<T> u;
    u.error = e;

    // Create the tagged union.
    result<T> r;
    r.tag = result_tag::err;
    r.data = u;

    return r;
}

#define lexical_result_error(type)                                              \
    template <typename T>                                                       \
    inline result<T> result_##type(size_t index)                                \
    {                                                                           \
        return result_err<T>(error_code::type, index);                          \
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

template <typename T>
inline partial_result<T> partial_result_ok(T value, size_t index)
{
    // Initialize the tuple.
    partial_result_tuple<T> t;
    t.x = value;
    t.y = index;

    // Initialize the union.
    partial_result_union<T> u;
    u.value = t;

    // Create the tagged union.
    partial_result<T> r;
    r.tag = result_tag::ok;
    r.data = u;

    return r;
}

template <typename T>
inline partial_result<T> partial_result_err(error_code code, size_t index)
{
    // Initialize the error.
    error e;
    e.code = code;
    e.index = index;

    // Initialize the union.
    partial_result_union<T> u;
    u.error = e;

    // Create the tagged union.
    partial_result<T> r;
    r.tag = result_tag::err;
    r.data = u;

    return r;
}

#define lexical_partial_result_error(type)                                      \
    template <typename T>                                                       \
    inline partial_result<T> partial_result_##type(size_t index)                \
    {                                                                           \
        return partial_result_err<T>(error_code::type, index);                  \
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

// CONFIG TESTS
// ------------

TEST(test_get_exponent_default_char, config_tests)
{
    EXPECT_EQ(get_exponent_default_char(), 'e');
}

TEST(test_set_exponent_default_char, config_tests)
{
    set_exponent_default_char('e');
}

#ifdef HAVE_RADIX
    TEST(test_get_exponent_backup_char, config_tests)
    {
        EXPECT_EQ(get_exponent_backup_char(), '^');
    }

    TEST(test_set_exponent_backup_char, config_tests)
    {
        set_exponent_backup_char('^');
    }
#endif  // HAVE_RADIX

#ifdef HAVE_ROUNDING
    TEST(test_get_float_rounding, config_tests)
    {
        EXPECT_EQ(get_float_rounding(), rounding_kind::nearest_tie_even);
    }

    TEST(test_set_float_rounding, config_tests)
    {
        set_float_rounding(rounding_kind::nearest_tie_even);
    }
#endif  // HAVE_ROUNDING

#ifdef HAVE_FORMAT
    TEST(number_format_compile, config_tests)
    {
        bool required_integer_digits = false;
        bool required_fraction_digits = false;
        bool required_exponent_digits = false;
        bool no_positive_mantissa_sign = false;
        bool required_mantissa_sign = false;
        bool no_exponent_notation = false;
        bool no_positive_exponent_sign = false;
        bool required_exponent_sign = false;
        bool no_exponent_without_fraction = false;
        bool no_special = true;
        bool case_sensitive_special = false;
        bool no_integer_leading_zeros = false;
        bool no_float_leading_zeros = false;
        bool integer_internal_digit_separator = true;
        bool fraction_internal_digit_separator = false;
        bool exponent_internal_digit_separator = false;
        bool integer_leading_digit_separator = false;
        bool fraction_leading_digit_separator = false;
        bool exponent_leading_digit_separator = false;
        bool integer_trailing_digit_separator = false;
        bool fraction_trailing_digit_separator = false;
        bool exponent_trailing_digit_separator = false;
        bool integer_consecutive_digit_separator = false;
        bool fraction_consecutive_digit_separator = false;
        bool exponent_consecutive_digit_separator = false;
        bool special_digit_separator = false;
        auto format = number_format_compile(
            '_',
            required_integer_digits,
            required_fraction_digits,
            required_exponent_digits,
            no_positive_mantissa_sign,
            required_mantissa_sign,
            no_exponent_notation,
            no_positive_exponent_sign,
            required_exponent_sign,
            no_exponent_without_fraction,
            no_special,
            case_sensitive_special,
            no_integer_leading_zeros,
            no_float_leading_zeros,
            integer_internal_digit_separator,
            fraction_internal_digit_separator,
            exponent_internal_digit_separator,
            integer_leading_digit_separator,
            fraction_leading_digit_separator,
            exponent_leading_digit_separator,
            integer_trailing_digit_separator,
            fraction_trailing_digit_separator,
            exponent_trailing_digit_separator,
            integer_consecutive_digit_separator,
            fraction_consecutive_digit_separator,
            exponent_consecutive_digit_separator,
            special_digit_separator
        ).unwrap();
        EXPECT_EQ(number_format_digit_separator(format), '_');
        EXPECT_TRUE(number_format_no_special(format));
        EXPECT_TRUE(number_format_integer_internal_digit_separator(format));
        EXPECT_FALSE(number_format_special_digit_separator(format));
    }

    TEST(number_format_permissive, config_tests)
    {
        auto format = number_format_permissive().unwrap();
        EXPECT_EQ(number_format_flags(format), 0);
        EXPECT_EQ(number_format_digit_separator(format), '\x00');
    }

    TEST(number_format_standard, config_tests)
    {
        auto format = number_format_standard().unwrap();
        EXPECT_EQ(format, number_format::required_exponent_digits);
        EXPECT_EQ(number_format_digit_separator(format), '\x00');
    }

    TEST(number_format_ignore, config_tests)
    {
        auto format = number_format_ignore('_').unwrap();
        EXPECT_EQ(number_format_flags(format), uint64_t(number_format::ignore));
        EXPECT_EQ(number_format_digit_separator(format), '_');
    }
#endif  // HAVE_FORMAT

TEST(test_get_nan_string, config_tests)
{
    EXPECT_EQ(get_nan_string(), "NaN");
}

TEST(test_set_nan_string, config_tests)
{
    set_nan_string("NaN");
}

TEST(test_get_inf_string, config_tests)
{
    EXPECT_EQ(get_inf_string(), "inf");
}

TEST(test_set_inf_string, config_tests)
{
    set_inf_string("inf");
}

TEST(test_get_infinity_string, config_tests)
{
    EXPECT_EQ(get_infinity_string(), "infinity");
}

TEST(test_set_infinity_string, config_tests)
{
    set_infinity_string("infinity");
}

// CONSTANT TESTS

TEST(test_size, constant_tests)
{
    // Check all the sizes can be used.
    EXPECT_GE(I8_FORMATTED_SIZE, 0);
    EXPECT_GE(I16_FORMATTED_SIZE, 0);
    EXPECT_GE(I32_FORMATTED_SIZE, 0);
    EXPECT_GE(I64_FORMATTED_SIZE, 0);
    EXPECT_GE(ISIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(U8_FORMATTED_SIZE, 0);
    EXPECT_GE(U16_FORMATTED_SIZE, 0);
    EXPECT_GE(U32_FORMATTED_SIZE, 0);
    EXPECT_GE(U64_FORMATTED_SIZE, 0);
    EXPECT_GE(USIZE_FORMATTED_SIZE, 0);
    EXPECT_GE(F32_FORMATTED_SIZE, 0);
    EXPECT_GE(F64_FORMATTED_SIZE, 0);
    EXPECT_GE(I8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(I64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(ISIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U8_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U16_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(U64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(USIZE_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(F32_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(F64_FORMATTED_SIZE_DECIMAL, 0);
    EXPECT_GE(BUFFER_SIZE, 0);
}

// ERROR TESTS

TEST(test_is_overflow, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error underflow = { error_code::underflow, 0 };
    EXPECT_TRUE(overflow.is_overflow());
    EXPECT_FALSE(underflow.is_overflow());
}

TEST(test_is_underflow, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error underflow = { error_code::underflow, 0 };
    EXPECT_FALSE(overflow.is_underflow());
    EXPECT_TRUE(underflow.is_underflow());
}

TEST(test_is_invalid_digit, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_digit = { error_code::invalid_digit, 0 };
    EXPECT_FALSE(overflow.is_invalid_digit());
    EXPECT_TRUE(invalid_digit.is_invalid_digit());
}

TEST(test_is_empty, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty = { error_code::empty, 0 };
    EXPECT_FALSE(overflow.is_empty());
    EXPECT_TRUE(empty.is_empty());
}

TEST(test_is_empty_mantissa, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_mantissa = { error_code::empty_mantissa, 0 };
    EXPECT_FALSE(overflow.is_empty_mantissa());
    EXPECT_TRUE(empty_mantissa.is_empty_mantissa());
}

TEST(test_is_empty_exponent, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_exponent = { error_code::empty_exponent, 0 };
    EXPECT_FALSE(overflow.is_empty_exponent());
    EXPECT_TRUE(empty_exponent.is_empty_exponent());
}

TEST(test_is_empty_integer, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_integer = { error_code::empty_integer, 0 };
    EXPECT_FALSE(overflow.is_empty_integer());
    EXPECT_TRUE(empty_integer.is_empty_integer());
}

TEST(test_is_empty_fraction, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error empty_fraction = { error_code::empty_fraction, 0 };
    EXPECT_FALSE(overflow.is_empty_fraction());
    EXPECT_TRUE(empty_fraction.is_empty_fraction());
}

TEST(test_is_invalid_positive_mantissa_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_positive_mantissa_sign = { error_code::invalid_positive_mantissa_sign, 0 };
    EXPECT_FALSE(overflow.is_invalid_positive_mantissa_sign());
    EXPECT_TRUE(invalid_positive_mantissa_sign.is_invalid_positive_mantissa_sign());
}

TEST(test_is_missing_mantissa_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error missing_mantissa_sign = { error_code::missing_mantissa_sign, 0 };
    EXPECT_FALSE(overflow.is_missing_mantissa_sign());
    EXPECT_TRUE(missing_mantissa_sign.is_missing_mantissa_sign());
}

TEST(test_is_invalid_exponent, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_exponent = { error_code::invalid_exponent, 0 };
    EXPECT_FALSE(overflow.is_invalid_exponent());
    EXPECT_TRUE(invalid_exponent.is_invalid_exponent());
}

TEST(test_is_invalid_positive_exponent_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_positive_exponent_sign = { error_code::invalid_positive_exponent_sign, 0 };
    EXPECT_FALSE(overflow.is_invalid_positive_exponent_sign());
    EXPECT_TRUE(invalid_positive_exponent_sign.is_invalid_positive_exponent_sign());
}

TEST(test_is_missing_exponent_sign, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error missing_exponent_sign = { error_code::missing_exponent_sign, 0 };
    EXPECT_FALSE(overflow.is_missing_exponent_sign());
    EXPECT_TRUE(missing_exponent_sign.is_missing_exponent_sign());
}

TEST(test_is_exponent_without_fraction, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error exponent_without_fraction = { error_code::exponent_without_fraction, 0 };
    EXPECT_FALSE(overflow.is_exponent_without_fraction());
    EXPECT_TRUE(exponent_without_fraction.is_exponent_without_fraction());
}

TEST(test_is_invalid_leading_zeros, error_tests)
{
    error overflow = { error_code::overflow, 0 };
    error invalid_leading_zeros = { error_code::invalid_leading_zeros, 0 };
    EXPECT_FALSE(overflow.is_invalid_leading_zeros());
    EXPECT_TRUE(invalid_leading_zeros.is_invalid_leading_zeros());
}

// RESULT TESTS

TEST(result, result_tests)
{
    auto ok = result_ok<u8>(0);
    auto overflow = result_overflow<u8>(0);
    auto underflow = result_underflow<u8>(0);
    auto invalid_digit = result_invalid_digit<u8>(0);
    auto empty = result_empty<u8>(0);
    auto empty_mantissa = result_empty_mantissa<u8>(0);
    auto empty_exponent = result_empty_exponent<u8>(0);
    auto empty_integer = result_empty_integer<u8>(0);
    auto empty_fraction = result_empty_fraction<u8>(0);
    auto invalid_positive_mantissa_sign = result_invalid_positive_mantissa_sign<u8>(0);
    auto missing_mantissa_sign = result_missing_mantissa_sign<u8>(0);
    auto invalid_exponent = result_invalid_exponent<u8>(0);
    auto invalid_positive_exponent_sign = result_invalid_positive_exponent_sign<u8>(0);
    auto missing_exponent_sign = result_missing_exponent_sign<u8>(0);
    auto exponent_without_fraction = result_exponent_without_fraction<u8>(0);
    auto invalid_leading_zeros = result_invalid_leading_zeros<u8>(0);

    EXPECT_TRUE(ok.is_ok());
    EXPECT_FALSE(ok.is_err());
    EXPECT_TRUE(overflow.is_err());
    EXPECT_TRUE(underflow.is_err());
    EXPECT_TRUE(invalid_digit.is_err());
    EXPECT_TRUE(empty.is_err());
    EXPECT_TRUE(empty_mantissa.is_err());
    EXPECT_TRUE(empty_exponent.is_err());
    EXPECT_TRUE(empty_integer.is_err());
    EXPECT_TRUE(empty_fraction.is_err());
    EXPECT_TRUE(invalid_positive_mantissa_sign.is_err());
    EXPECT_TRUE(missing_mantissa_sign.is_err());
    EXPECT_TRUE(invalid_exponent.is_err());
    EXPECT_TRUE(invalid_positive_exponent_sign.is_err());
    EXPECT_TRUE(missing_exponent_sign.is_err());
    EXPECT_TRUE(exponent_without_fraction.is_err());
    EXPECT_TRUE(invalid_leading_zeros.is_err());

    EXPECT_EQ(ok.ok(), 0);
    EXPECT_TRUE(overflow.err().is_overflow());
    EXPECT_TRUE(underflow.err().is_underflow());
    EXPECT_TRUE(invalid_digit.err().is_invalid_digit());
    EXPECT_TRUE(empty.err().is_empty());
    EXPECT_TRUE(empty_mantissa.err().is_empty_mantissa());
    EXPECT_TRUE(empty_exponent.err().is_empty_exponent());
    EXPECT_TRUE(empty_integer.err().is_empty_integer());
    EXPECT_TRUE(empty_fraction.err().is_empty_fraction());
    EXPECT_TRUE(invalid_positive_mantissa_sign.err().is_invalid_positive_mantissa_sign());
    EXPECT_TRUE(missing_mantissa_sign.err().is_missing_mantissa_sign());
    EXPECT_TRUE(invalid_exponent.err().is_invalid_exponent());
    EXPECT_TRUE(invalid_positive_exponent_sign.err().is_invalid_positive_exponent_sign());
    EXPECT_TRUE(missing_exponent_sign.err().is_missing_exponent_sign());
    EXPECT_TRUE(exponent_without_fraction.err().is_exponent_without_fraction());
    EXPECT_TRUE(invalid_leading_zeros.err().is_invalid_leading_zeros());
}

// PARTIAL RESULT TESTS

TEST(partial_result, partial_result_tests)
{
    auto ok = partial_result_ok<u8>(0, 1);
    auto overflow = partial_result_overflow<u8>(0);
    auto underflow = partial_result_underflow<u8>(0);
    auto invalid_digit = partial_result_invalid_digit<u8>(0);
    auto empty = partial_result_empty<u8>(0);
    auto empty_mantissa = partial_result_empty_mantissa<u8>(0);
    auto empty_exponent = partial_result_empty_exponent<u8>(0);
    auto empty_integer = partial_result_empty_integer<u8>(0);
    auto empty_fraction = partial_result_empty_fraction<u8>(0);
    auto invalid_positive_mantissa_sign = partial_result_invalid_positive_mantissa_sign<u8>(0);
    auto missing_mantissa_sign = partial_result_missing_mantissa_sign<u8>(0);
    auto invalid_exponent = partial_result_invalid_exponent<u8>(0);
    auto invalid_positive_exponent_sign = partial_result_invalid_positive_exponent_sign<u8>(0);
    auto missing_exponent_sign = partial_result_missing_exponent_sign<u8>(0);
    auto exponent_without_fraction = partial_result_exponent_without_fraction<u8>(0);
    auto invalid_leading_zeros = partial_result_invalid_leading_zeros<u8>(0);

    EXPECT_TRUE(ok.is_ok());
    EXPECT_FALSE(ok.is_err());
    EXPECT_TRUE(overflow.is_err());
    EXPECT_TRUE(underflow.is_err());
    EXPECT_TRUE(invalid_digit.is_err());
    EXPECT_TRUE(empty.is_err());
    EXPECT_TRUE(empty_mantissa.is_err());
    EXPECT_TRUE(empty_exponent.is_err());
    EXPECT_TRUE(empty_integer.is_err());
    EXPECT_TRUE(empty_fraction.is_err());
    EXPECT_TRUE(invalid_positive_mantissa_sign.is_err());
    EXPECT_TRUE(missing_mantissa_sign.is_err());
    EXPECT_TRUE(invalid_exponent.is_err());
    EXPECT_TRUE(invalid_positive_exponent_sign.is_err());
    EXPECT_TRUE(missing_exponent_sign.is_err());
    EXPECT_TRUE(exponent_without_fraction.is_err());
    EXPECT_TRUE(invalid_leading_zeros.is_err());

    EXPECT_EQ(ok.ok(), std::make_tuple(0, 1));
    EXPECT_TRUE(overflow.err().is_overflow());
    EXPECT_TRUE(underflow.err().is_underflow());
    EXPECT_TRUE(invalid_digit.err().is_invalid_digit());
    EXPECT_TRUE(empty.err().is_empty());
    EXPECT_TRUE(empty_mantissa.err().is_empty_mantissa());
    EXPECT_TRUE(empty_exponent.err().is_empty_exponent());
    EXPECT_TRUE(empty_integer.err().is_empty_integer());
    EXPECT_TRUE(empty_fraction.err().is_empty_fraction());
    EXPECT_TRUE(invalid_positive_mantissa_sign.err().is_invalid_positive_mantissa_sign());
    EXPECT_TRUE(missing_mantissa_sign.err().is_missing_mantissa_sign());
    EXPECT_TRUE(invalid_exponent.err().is_invalid_exponent());
    EXPECT_TRUE(invalid_positive_exponent_sign.err().is_invalid_positive_exponent_sign());
    EXPECT_TRUE(missing_exponent_sign.err().is_missing_exponent_sign());
    EXPECT_TRUE(exponent_without_fraction.err().is_exponent_without_fraction());
    EXPECT_TRUE(invalid_leading_zeros.err().is_invalid_leading_zeros());
}

// TO STRING TESTS

#define TO_STRING_TEST(t)                                                       \
    EXPECT_EQ("10", to_string<t>(10))

#define TO_STRING_FLOAT_TEST(t)                                                 \
    EXPECT_EQ("10.5", to_string<t>(10.5))

TEST(to_string, api_tests)
{
    TO_STRING_TEST(u8);
    TO_STRING_TEST(u16);
    TO_STRING_TEST(u32);
    TO_STRING_TEST(u64);
    TO_STRING_TEST(usize);
    TO_STRING_TEST(i8);
    TO_STRING_TEST(i16);
    TO_STRING_TEST(i32);
    TO_STRING_TEST(i64);
    TO_STRING_TEST(isize);
    TO_STRING_FLOAT_TEST(f32);
    TO_STRING_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define TO_STRING_RADIX_TEST(t)                                             \
        EXPECT_EQ("1010", to_string_radix<t>(10, 2));                           \
        EXPECT_EQ("A", to_string_radix<t>(10, 16));                             \
        EXPECT_EQ("10", to_string_radix<t>(10, 10))

    #define TO_STRING_RADIX_FLOAT_TEST(t)                                       \
        EXPECT_EQ("1010.1", to_string_radix<t>(10.5, 2));                       \
        EXPECT_EQ("A.8", to_string_radix<t>(10.5, 16));                         \
        EXPECT_EQ("10.5", to_string_radix<t>(10.5, 10))

    TEST(to_string_radix, api_tests)
    {
        TO_STRING_RADIX_TEST(u8);
        TO_STRING_RADIX_TEST(u16);
        TO_STRING_RADIX_TEST(u32);
        TO_STRING_RADIX_TEST(u64);
        TO_STRING_RADIX_TEST(usize);
        TO_STRING_RADIX_TEST(i8);
        TO_STRING_RADIX_TEST(i16);
        TO_STRING_RADIX_TEST(i32);
        TO_STRING_RADIX_TEST(i64);
        TO_STRING_RADIX_TEST(isize);
        TO_STRING_RADIX_FLOAT_TEST(f32);
        TO_STRING_RADIX_FLOAT_TEST(f64);
    }
#endif  // HAVE_RADIX

// PARSE TESTS

#define PARSE_TEST(t)                                                           \
    EXPECT_EQ(result_ok<t>(10), parse<t>("10"));                                \
    EXPECT_EQ(result_invalid_digit<t>(2), parse<t>("10a"));                     \
    EXPECT_EQ(result_empty<t>(0), parse<t>(""))

#define PARSE_FLOAT_TEST(t)                                                     \
    PARSE_TEST(t);                                                              \
    EXPECT_EQ(result_ok<t>(10.5), parse<t>("10.5"));                            \
    EXPECT_EQ(result_ok<t>(10e5), parse<t>("10e5"));                            \
    EXPECT_EQ(result_empty_mantissa<t>(0), parse<t>("."));                      \
    EXPECT_EQ(result_empty_mantissa<t>(0), parse<t>("e5"));                     \
    EXPECT_EQ(result_empty_exponent<t>(3), parse<t>("10e+"))

TEST(parse, api_tests)
{
    PARSE_TEST(u8);
    PARSE_TEST(u16);
    PARSE_TEST(u32);
    PARSE_TEST(u64);
    PARSE_TEST(usize);
    PARSE_TEST(i8);
    PARSE_TEST(i16);
    PARSE_TEST(i32);
    PARSE_TEST(i64);
    PARSE_TEST(isize);
    PARSE_FLOAT_TEST(f32);
    PARSE_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_RADIX_TEST(t)                                                 \
        EXPECT_EQ(result_ok<t>(10), parse_radix<t>("1010", 2));                 \
        EXPECT_EQ(result_ok<t>(10), parse_radix<t>("10", 10));                  \
        EXPECT_EQ(result_ok<t>(10), parse_radix<t>("A", 16));                   \
        EXPECT_EQ(result_invalid_digit<t>(4), parse_radix<t>("10102", 2));      \
        EXPECT_EQ(result_invalid_digit<t>(2), parse_radix<t>("10a", 10));       \
        EXPECT_EQ(result_invalid_digit<t>(1), parse_radix<t>("AG", 16));        \
        EXPECT_EQ(result_empty<t>(0), parse_radix<t>("", 10))

    #define PARSE_RADIX_FLOAT_TEST(t)                                           \
        PARSE_RADIX_TEST(t);                                                    \
        EXPECT_EQ(result_ok<t>(10.5), parse_radix<t>("1010.1", 2));             \
        EXPECT_EQ(result_ok<t>(10.5), parse_radix<t>("10.5", 10));              \
        EXPECT_EQ(result_ok<t>(10.5), parse_radix<t>("A.8", 16));               \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_radix<t>(".", 10));        \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_radix<t>("e5", 10));       \
        EXPECT_EQ(result_empty_exponent<t>(3), parse_radix<t>("10e+", 10))

    TEST(parse_radix, api_tests)
    {
        PARSE_RADIX_TEST(u8);
        PARSE_RADIX_TEST(u16);
        PARSE_RADIX_TEST(u32);
        PARSE_RADIX_TEST(u64);
        PARSE_RADIX_TEST(usize);
        PARSE_RADIX_TEST(i8);
        PARSE_RADIX_TEST(i16);
        PARSE_RADIX_TEST(i32);
        PARSE_RADIX_TEST(i64);
        PARSE_RADIX_TEST(isize);
        PARSE_RADIX_FLOAT_TEST(f32);
        PARSE_RADIX_FLOAT_TEST(f64);
    }
#endif  // HAVE_RADIX

#ifdef HAVE_FORMAT
    const number_format FORMAT = number_format::standard;

    #define PARSE_FORMAT_TEST(t)                                                \
        EXPECT_EQ(result_ok<t>(10), parse_format<t>("10", FORMAT));             \
        EXPECT_EQ(result_invalid_digit<t>(2), parse_format<t>("10a", FORMAT));  \
        EXPECT_EQ(result_empty<t>(0), parse_format<t>("", FORMAT))

    #define PARSE_FORMAT_FLOAT_TEST(t)                                          \
        PARSE_FORMAT_TEST(t);                                                   \
        EXPECT_EQ(result_ok<t>(10.5), parse_format<t>("10.5", FORMAT));         \
        EXPECT_EQ(result_ok<t>(10e5), parse_format<t>("10e5", FORMAT));         \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_format<t>(".", FORMAT));   \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_format<t>("e5", FORMAT));  \
        EXPECT_EQ(result_empty_exponent<t>(3), parse_format<t>("10e+", FORMAT))

    TEST(parse_format, api_tests)
    {
        PARSE_FORMAT_TEST(u8);
        PARSE_FORMAT_TEST(u16);
        PARSE_FORMAT_TEST(u32);
        PARSE_FORMAT_TEST(u64);
        PARSE_FORMAT_TEST(usize);
        PARSE_FORMAT_TEST(i8);
        PARSE_FORMAT_TEST(i16);
        PARSE_FORMAT_TEST(i32);
        PARSE_FORMAT_TEST(i64);
        PARSE_FORMAT_TEST(isize);
        PARSE_FORMAT_FLOAT_TEST(f32);
        PARSE_FORMAT_FLOAT_TEST(f64);
    }

    #ifdef HAVE_RADIX
        #define PARSE_FORMAT_RADIX_TEST(t)                                                      \
            EXPECT_EQ(result_ok<t>(10), parse_format_radix<t>("1010", 2, FORMAT));              \
            EXPECT_EQ(result_ok<t>(10), parse_format_radix<t>("10", 10, FORMAT));               \
            EXPECT_EQ(result_ok<t>(10), parse_format_radix<t>("A", 16, FORMAT));                \
            EXPECT_EQ(result_invalid_digit<t>(4), parse_format_radix<t>("10102", 2, FORMAT));   \
            EXPECT_EQ(result_invalid_digit<t>(2), parse_format_radix<t>("10a", 10, FORMAT));    \
            EXPECT_EQ(result_invalid_digit<t>(1), parse_format_radix<t>("AG", 16, FORMAT));     \
            EXPECT_EQ(result_empty<t>(0), parse_format_radix<t>("", 10, FORMAT))

        #define PARSE_FORMAT_RADIX_FLOAT_TEST(t)                                                \
            PARSE_FORMAT_RADIX_TEST(t);                                                         \
            EXPECT_EQ(result_ok<t>(10.5), parse_format_radix<t>("1010.1", 2, FORMAT));          \
            EXPECT_EQ(result_ok<t>(10.5), parse_format_radix<t>("10.5", 10, FORMAT));           \
            EXPECT_EQ(result_ok<t>(10.5), parse_format_radix<t>("A.8", 16, FORMAT));            \
            EXPECT_EQ(result_empty_mantissa<t>(0), parse_format_radix<t>(".", 10, FORMAT));     \
            EXPECT_EQ(result_empty_mantissa<t>(0), parse_format_radix<t>("e5", 10, FORMAT));    \
            EXPECT_EQ(result_empty_exponent<t>(3), parse_format_radix<t>("10e+", 10, FORMAT))

        TEST(parse_format_radix, api_tests)
        {
            PARSE_FORMAT_RADIX_TEST(u8);
            PARSE_FORMAT_RADIX_TEST(u16);
            PARSE_FORMAT_RADIX_TEST(u32);
            PARSE_FORMAT_RADIX_TEST(u64);
            PARSE_FORMAT_RADIX_TEST(usize);
            PARSE_FORMAT_RADIX_TEST(i8);
            PARSE_FORMAT_RADIX_TEST(i16);
            PARSE_FORMAT_RADIX_TEST(i32);
            PARSE_FORMAT_RADIX_TEST(i64);
            PARSE_FORMAT_RADIX_TEST(isize);
            PARSE_FORMAT_RADIX_FLOAT_TEST(f32);
            PARSE_FORMAT_RADIX_FLOAT_TEST(f64);
        }
    #endif  // HAVE_RADIX
#endif  // HAVE_FORMAT

#define PARSE_PARTIAL_TEST(t)                                                   \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial<t>("10"));             \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial<t>("10a"));            \
    EXPECT_EQ(partial_result_empty<t>(0), parse_partial<t>(""))

#define PARSE_PARTIAL_FLOAT_TEST(t)                                             \
    PARSE_PARTIAL_TEST(t);                                                      \
    EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial<t>("10.5"));         \
    EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial<t>("10e5"));         \
    EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial<t>("."));      \
    EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial<t>("e5"));     \
    EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial<t>("10e+"))

TEST(parse_partial, api_tests)
{
    PARSE_PARTIAL_TEST(u8);
    PARSE_PARTIAL_TEST(u16);
    PARSE_PARTIAL_TEST(u32);
    PARSE_PARTIAL_TEST(u64);
    PARSE_PARTIAL_TEST(usize);
    PARSE_PARTIAL_TEST(i8);
    PARSE_PARTIAL_TEST(i16);
    PARSE_PARTIAL_TEST(i32);
    PARSE_PARTIAL_TEST(i64);
    PARSE_PARTIAL_TEST(isize);
    PARSE_PARTIAL_FLOAT_TEST(f32);
    PARSE_PARTIAL_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_PARTIAL_RADIX_TEST(t)                                                     \
        EXPECT_EQ(partial_result_ok<t>(10, 4), parse_partial_radix<t>("1010", 2));          \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_radix<t>("10", 10));           \
        EXPECT_EQ(partial_result_ok<t>(10, 1), parse_partial_radix<t>("A", 16));            \
        EXPECT_EQ(partial_result_ok<t>(10, 4), parse_partial_radix<t>("10102", 2));         \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_radix<t>("10a", 10));          \
        EXPECT_EQ(partial_result_ok<t>(10, 1), parse_partial_radix<t>("AG", 16));           \
        EXPECT_EQ(partial_result_empty<t>(0), parse_partial_radix<t>("", 10))

    #define PARSE_PARTIAL_RADIX_FLOAT_TEST(t)                                               \
        PARSE_PARTIAL_RADIX_TEST(t);                                                        \
        EXPECT_EQ(partial_result_ok<t>(10.5, 6), parse_partial_radix<t>("1010.1", 2));      \
        EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_radix<t>("10.5", 10));       \
        EXPECT_EQ(partial_result_ok<t>(10.5, 3), parse_partial_radix<t>("A.8", 16));        \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_radix<t>(".", 10));    \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_radix<t>("e5", 10));   \
        EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial_radix<t>("10e+", 10))

    TEST(parse_partial_radix, api_tests)
    {
        PARSE_PARTIAL_RADIX_TEST(u8);
        PARSE_PARTIAL_RADIX_TEST(u16);
        PARSE_PARTIAL_RADIX_TEST(u32);
        PARSE_PARTIAL_RADIX_TEST(u64);
        PARSE_PARTIAL_RADIX_TEST(usize);
        PARSE_PARTIAL_RADIX_TEST(i8);
        PARSE_PARTIAL_RADIX_TEST(i16);
        PARSE_PARTIAL_RADIX_TEST(i32);
        PARSE_PARTIAL_RADIX_TEST(i64);
        PARSE_PARTIAL_RADIX_TEST(isize);
        PARSE_PARTIAL_RADIX_FLOAT_TEST(f32);
        PARSE_PARTIAL_RADIX_FLOAT_TEST(f64);
    }
#endif  // HAVE_RADIX

#ifdef HAVE_FORMAT
    #define PARSE_PARTIAL_FORMAT_TEST(t)                                                        \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_format<t>("10", FORMAT));          \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_format<t>("10a", FORMAT));         \
        EXPECT_EQ(partial_result_empty<t>(0), parse_partial_format<t>("", FORMAT))

    #define PARSE_PARTIAL_FORMAT_FLOAT_TEST(t)                                                  \
        PARSE_PARTIAL_FORMAT_TEST(t);                                                           \
        EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_format<t>("10.5", FORMAT));      \
        EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial_format<t>("10e5", FORMAT));      \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_format<t>(".", FORMAT));   \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_format<t>("e5", FORMAT));  \
        EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial_format<t>("10e+", FORMAT))

    TEST(parse_partial_format, api_tests)
    {
        PARSE_PARTIAL_FORMAT_TEST(u8);
        PARSE_PARTIAL_FORMAT_TEST(u16);
        PARSE_PARTIAL_FORMAT_TEST(u32);
        PARSE_PARTIAL_FORMAT_TEST(u64);
        PARSE_PARTIAL_FORMAT_TEST(usize);
        PARSE_PARTIAL_FORMAT_TEST(i8);
        PARSE_PARTIAL_FORMAT_TEST(i16);
        PARSE_PARTIAL_FORMAT_TEST(i32);
        PARSE_PARTIAL_FORMAT_TEST(i64);
        PARSE_PARTIAL_FORMAT_TEST(isize);
        PARSE_PARTIAL_FORMAT_FLOAT_TEST(f32);
        PARSE_PARTIAL_FORMAT_FLOAT_TEST(f64);
    }
    #ifdef HAVE_RADIX
        #define PARSE_PARTIAL_FORMAT_RADIX_TEST(t)                                                              \
            EXPECT_EQ(partial_result_ok<t>(10, 4), parse_partial_format_radix<t>("1010", 2, FORMAT));           \
            EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_format_radix<t>("10", 10, FORMAT));            \
            EXPECT_EQ(partial_result_ok<t>(10, 1), parse_partial_format_radix<t>("A", 16, FORMAT));             \
            EXPECT_EQ(partial_result_ok<t>(10, 4), parse_partial_format_radix<t>("10102", 2, FORMAT));          \
            EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_format_radix<t>("10a", 10, FORMAT));           \
            EXPECT_EQ(partial_result_ok<t>(10, 1), parse_partial_format_radix<t>("AG", 16, FORMAT));            \
            EXPECT_EQ(partial_result_empty<t>(0), parse_partial_format_radix<t>("", 10, FORMAT))

        #define PARSE_PARTIAL_FORMAT_RADIX_FLOAT_TEST(t)                                                        \
            PARSE_PARTIAL_FORMAT_RADIX_TEST(t);                                                                 \
            EXPECT_EQ(partial_result_ok<t>(10.5, 6), parse_partial_format_radix<t>("1010.1", 2, FORMAT));       \
            EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_format_radix<t>("10.5", 10, FORMAT));        \
            EXPECT_EQ(partial_result_ok<t>(10.5, 3), parse_partial_format_radix<t>("A.8", 16, FORMAT));         \
            EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_format_radix<t>(".", 10, FORMAT));     \
            EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_format_radix<t>("e5", 10, FORMAT));    \
            EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial_format_radix<t>("10e+", 10, FORMAT))

        TEST(parse_partial_format_radix, api_tests)
        {
            PARSE_PARTIAL_FORMAT_RADIX_TEST(u8);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(u16);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(u32);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(u64);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(usize);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(i8);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(i16);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(i32);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(i64);
            PARSE_PARTIAL_FORMAT_RADIX_TEST(isize);
            PARSE_PARTIAL_FORMAT_RADIX_FLOAT_TEST(f32);
            PARSE_PARTIAL_FORMAT_RADIX_FLOAT_TEST(f64);
        }
    #endif  // HAVE_RADIX
#endif  // HAVE_FORMAT

// PARSE LOSSY TESTS

#define PARSE_LOSSY_TEST(t)                                                     \
    EXPECT_EQ(result_ok<t>(10), parse_lossy<t>("10"));                          \
    EXPECT_EQ(result_invalid_digit<t>(2), parse_lossy<t>("10a"));               \
    EXPECT_EQ(result_empty<t>(0), parse_lossy<t>(""))

#define PARSE_LOSSY_FLOAT_TEST(t)                                               \
    PARSE_LOSSY_TEST(t);                                                        \
    EXPECT_EQ(result_ok<t>(10.5), parse_lossy<t>("10.5"));                      \
    EXPECT_EQ(result_ok<t>(10e5), parse_lossy<t>("10e5"));                      \
    EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy<t>("."));                \
    EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy<t>("e5"));               \
    EXPECT_EQ(result_empty_exponent<t>(3), parse_lossy<t>("10e+"))

TEST(parse_lossy, api_tests)
{
    PARSE_LOSSY_FLOAT_TEST(f32);
    PARSE_LOSSY_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_LOSSY_RADIX_TEST(t)                                               \
        EXPECT_EQ(result_ok<t>(10), parse_lossy_radix<t>("10", 10));                \
        EXPECT_EQ(result_invalid_digit<t>(2), parse_lossy_radix<t>("10a", 10));     \
        EXPECT_EQ(result_empty<t>(0), parse_lossy_radix<t>("", 10))

    #define PARSE_LOSSY_RADIX_FLOAT_TEST(t)                                         \
        PARSE_LOSSY_TEST(t);                                                        \
        EXPECT_EQ(result_ok<t>(10.5), parse_lossy_radix<t>("10.5", 10));            \
        EXPECT_EQ(result_ok<t>(10e5), parse_lossy_radix<t>("10e5", 10));            \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy_radix<t>(".", 10));      \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy_radix<t>("e5", 10));     \
        EXPECT_EQ(result_empty_exponent<t>(3), parse_lossy_radix<t>("10e+", 10))

    TEST(parse_lossy_radix, api_tests)
    {
        PARSE_LOSSY_RADIX_FLOAT_TEST(f32);
        PARSE_LOSSY_RADIX_FLOAT_TEST(f64);
    }
#endif  // HAVE_RADIX

#ifdef HAVE_FORMAT
    #define PARSE_LOSSY_FORMAT_TEST(t)                                                  \
        EXPECT_EQ(result_ok<t>(10), parse_lossy_format<t>("10", FORMAT));               \
        EXPECT_EQ(result_invalid_digit<t>(2), parse_lossy_format<t>("10a", FORMAT));    \
        EXPECT_EQ(result_empty<t>(0), parse_lossy_format<t>("", FORMAT))

    #define PARSE_LOSSY_FORMAT_FLOAT_TEST(t)                                            \
        PARSE_LOSSY_FORMAT_TEST(t);                                                     \
        EXPECT_EQ(result_ok<t>(10.5), parse_lossy_format<t>("10.5", FORMAT));           \
        EXPECT_EQ(result_ok<t>(10e5), parse_lossy_format<t>("10e5", FORMAT));           \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy_format<t>(".", FORMAT));     \
        EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy_format<t>("e5", FORMAT));    \
        EXPECT_EQ(result_empty_exponent<t>(3), parse_lossy_format<t>("10e+", FORMAT))

    TEST(parse_lossy_format, api_tests)
    {
        PARSE_LOSSY_FORMAT_FLOAT_TEST(f32);
        PARSE_LOSSY_FORMAT_FLOAT_TEST(f64);
    }

    #ifdef HAVE_RADIX
        #define PARSE_LOSSY_FORMAT_RADIX_TEST(t)                                                    \
            EXPECT_EQ(result_ok<t>(10), parse_lossy_format_radix<t>("10", 10, FORMAT));             \
            EXPECT_EQ(result_invalid_digit<t>(2), parse_lossy_format_radix<t>("10a", 10, FORMAT));  \
            EXPECT_EQ(result_empty<t>(0), parse_lossy_format_radix<t>("", 10, FORMAT))

        #define PARSE_LOSSY_FORMAT_RADIX_FLOAT_TEST(t)                                              \
            PARSE_LOSSY_FORMAT_TEST(t);                                                             \
            EXPECT_EQ(result_ok<t>(10.5), parse_lossy_format_radix<t>("10.5", 10, FORMAT));         \
            EXPECT_EQ(result_ok<t>(10e5), parse_lossy_format_radix<t>("10e5", 10, FORMAT));         \
            EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy_format_radix<t>(".", 10, FORMAT));   \
            EXPECT_EQ(result_empty_mantissa<t>(0), parse_lossy_format_radix<t>("e5", 10, FORMAT));  \
            EXPECT_EQ(result_empty_exponent<t>(3), parse_lossy_format_radix<t>("10e+", 10, FORMAT))

        TEST(parse_lossy_format_radix, api_tests)
        {
            PARSE_LOSSY_FORMAT_RADIX_FLOAT_TEST(f32);
            PARSE_LOSSY_FORMAT_RADIX_FLOAT_TEST(f64);
        }
    #endif // HAVE_RADIX
#endif  // HAVE_FORMAT

#define PARSE_PARTIAL_LOSSY_TEST(t)                                                         \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy<t>("10"));                   \
    EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy<t>("10a"));                  \
    EXPECT_EQ(partial_result_empty<t>(0), parse_partial_lossy<t>(""))

#define PARSE_PARTIAL_LOSSY_FLOAT_TEST(t)                                                   \
    PARSE_PARTIAL_LOSSY_TEST(t);                                                            \
    EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_lossy<t>("10.5"));               \
    EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial_lossy<t>("10e5"));               \
    EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy<t>("."));            \
    EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy<t>("e5"));           \
    EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial_lossy<t>("10e+"))

TEST(parse_partial_lossy, api_tests)
{
    PARSE_PARTIAL_LOSSY_FLOAT_TEST(f32);
    PARSE_PARTIAL_LOSSY_FLOAT_TEST(f64);
}

#ifdef HAVE_RADIX
    #define PARSE_PARTIAL_LOSSY_RADIX_TEST(t)                                                   \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_radix<t>("10", 10));         \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_radix<t>("10a", 10));        \
        EXPECT_EQ(partial_result_empty<t>(0), parse_partial_lossy_radix<t>("", 10))

    #define PARSE_PARTIAL_LOSSY_RADIX_FLOAT_TEST(t)                                             \
        PARSE_PARTIAL_LOSSY_RADIX_TEST(t);                                                      \
        EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_lossy_radix<t>("10.5", 10));     \
        EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial_lossy_radix<t>("10e5", 10));     \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy_radix<t>(".", 10));  \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy_radix<t>("e5", 10)); \
        EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial_lossy_radix<t>("10e+", 10))

    TEST(parse_partial_lossy_radix, api_tests)
    {
        PARSE_PARTIAL_LOSSY_RADIX_FLOAT_TEST(f32);
        PARSE_PARTIAL_LOSSY_RADIX_FLOAT_TEST(f64);
    }
#endif // HAVE_RADIX

#if HAVE_FORMAT
    #define PARSE_PARTIAL_LOSSY_FORMAT_TEST(t)                                                          \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_format<t>("10", FORMAT));            \
        EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_format<t>("10a", FORMAT));           \
        EXPECT_EQ(partial_result_empty<t>(0), parse_partial_lossy_format<t>("", FORMAT))

    #define PARSE_PARTIAL_LOSSY_FORMAT_FLOAT_TEST(t)                                                    \
        PARSE_PARTIAL_LOSSY_FORMAT_TEST(t);                                                             \
        EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_lossy_format<t>("10.5", FORMAT));        \
        EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial_lossy_format<t>("10e5", FORMAT));        \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy_format<t>(".", FORMAT));     \
        EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy_format<t>("e5", FORMAT));    \
        EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial_lossy_format<t>("10e+", FORMAT))

    TEST(parse_partial_lossy_format, api_tests)
    {
        PARSE_PARTIAL_LOSSY_FORMAT_FLOAT_TEST(f32);
        PARSE_PARTIAL_LOSSY_FORMAT_FLOAT_TEST(f64);
    }

    #ifdef HAVE_RADIX
        #define PARSE_PARTIAL_LOSSY_FORMAT_RADIX_TEST(t)                                                            \
            EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_format_radix<t>("10", 10, FORMAT));          \
            EXPECT_EQ(partial_result_ok<t>(10, 2), parse_partial_lossy_format_radix<t>("10a", 10, FORMAT));         \
            EXPECT_EQ(partial_result_empty<t>(0), parse_partial_lossy_format_radix<t>("", 10, FORMAT))

        #define PARSE_PARTIAL_LOSSY_FORMAT_RADIX_FLOAT_TEST(t)                                                      \
            PARSE_PARTIAL_LOSSY_FORMAT_RADIX_TEST(t);                                                               \
            EXPECT_EQ(partial_result_ok<t>(10.5, 4), parse_partial_lossy_format_radix<t>("10.5", 10, FORMAT));      \
            EXPECT_EQ(partial_result_ok<t>(10e5, 4), parse_partial_lossy_format_radix<t>("10e5", 10, FORMAT));      \
            EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy_format_radix<t>(".", 10, FORMAT));   \
            EXPECT_EQ(partial_result_empty_mantissa<t>(0), parse_partial_lossy_format_radix<t>("e5", 10, FORMAT));  \
            EXPECT_EQ(partial_result_empty_exponent<t>(3), parse_partial_lossy_format_radix<t>("10e+", 10, FORMAT))

        TEST(parse_partial_lossy_format_radix, api_tests)
        {
            PARSE_PARTIAL_LOSSY_FORMAT_RADIX_FLOAT_TEST(f32);
            PARSE_PARTIAL_LOSSY_FORMAT_RADIX_FLOAT_TEST(f64);
        }
    #endif  // HAVE_RADIX
#endif  // HAVE_FORMAT
