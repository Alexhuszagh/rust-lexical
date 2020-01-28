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

#ifdef HAVE_FORMAT
    // BITFLAGS

    // Bitflags for a serialized number format.
    enum class number_format: uint64_t {
        // FLAGS
        required_integer_digits = lexical_required_integer_digits,
        required_fraction_digits = lexical_required_fraction_digits,
        required_exponent_digits = lexical_required_exponent_digits,
        no_positive_mantissa_sign = lexical_no_positive_mantissa_sign,
        required_mantissa_sign = lexical_required_mantissa_sign,
        no_exponent_notation = lexical_no_exponent_notation,
        no_positive_exponent_sign = lexical_no_positive_exponent_sign,
        required_exponent_sign = lexical_required_exponent_sign,
        no_exponent_without_fraction = lexical_no_exponent_without_fraction,
        no_special = lexical_no_special,
        case_sensitive_special = lexical_case_sensitive_special,
        no_integer_leading_zeros = lexical_no_integer_leading_zeros,
        no_float_leading_zeros = lexical_no_float_leading_zeros,
        integer_internal_digit_separator = lexical_integer_internal_digit_separator,
        fraction_internal_digit_separator = lexical_fraction_internal_digit_separator,
        exponent_internal_digit_separator = lexical_exponent_internal_digit_separator,
        integer_leading_digit_separator = lexical_integer_leading_digit_separator,
        fraction_leading_digit_separator = lexical_fraction_leading_digit_separator,
        exponent_leading_digit_separator = lexical_exponent_leading_digit_separator,
        integer_trailing_digit_separator = lexical_integer_trailing_digit_separator,
        fraction_trailing_digit_separator = lexical_fraction_trailing_digit_separator,
        exponent_trailing_digit_separator = lexical_exponent_trailing_digit_separator,
        integer_consecutive_digit_separator = lexical_integer_consecutive_digit_separator,
        fraction_consecutive_digit_separator = lexical_fraction_consecutive_digit_separator,
        exponent_consecutive_digit_separator = lexical_exponent_consecutive_digit_separator,
        special_digit_separator = lexical_special_digit_separator,
        required_digits = lexical_required_digits,
        internal_digit_separator = lexical_internal_digit_separator,
        leading_digit_separator = lexical_leading_digit_separator,
        trailing_digit_separator = lexical_trailing_digit_separator,
        consecutive_digit_separator = lexical_consecutive_digit_separator,

        // MASKS
        digit_separator_flag_mask = lexical_digit_separator_flag_mask,
        integer_digit_separator_flag_mask = lexical_integer_digit_separator_flag_mask,
        fraction_digit_separator_flag_mask = lexical_fraction_digit_separator_flag_mask,
        exponent_digit_separator_flag_mask = lexical_exponent_digit_separator_flag_mask,
        exponent_flag_mask = lexical_exponent_flag_mask,
        flag_mask = lexical_flag_mask,

        // PRE-DEFINED
        // Note:
        //  The pre-defined enum definitions are the public API for
        //  lexical_number_format.
        rust_literal = lexical_rust_literal,
        rust_string = lexical_rust_string,
        rust_string_strict = lexical_rust_string_strict,
        python_literal = lexical_python_literal,
        python_string = lexical_python_string,
        cxx17_literal = lexical_cxx17_literal,
        cxx17_string = lexical_cxx17_string,
        cxx14_literal = lexical_cxx14_literal,
        cxx14_string = lexical_cxx14_string,
        cxx11_literal = lexical_cxx11_literal,
        cxx11_string = lexical_cxx11_string,
        cxx03_literal = lexical_cxx03_literal,
        cxx03_string = lexical_cxx03_string,
        cxx98_literal = lexical_cxx98_literal,
        cxx98_string = lexical_cxx98_string,
        c18_literal = lexical_c18_literal,
        c18_string = lexical_c18_string,
        c11_literal = lexical_c11_literal,
        c11_string = lexical_c11_string,
        c99_literal = lexical_c99_literal,
        c99_string = lexical_c99_string,
        c90_literal = lexical_c90_literal,
        c90_string = lexical_c90_string,
        c89_literal = lexical_c89_literal,
        c89_string = lexical_c89_string,
        ruby_literal = lexical_ruby_literal,
        ruby_string = lexical_ruby_string,
        swift_literal = lexical_swift_literal,
        swift_string = lexical_swift_string,
        go_literal = lexical_go_literal,
        go_string = lexical_go_string,
        haskell_literal = lexical_haskell_literal,
        haskell_string = lexical_haskell_string,
        javascript_literal = lexical_javascript_literal,
        javascript_string = lexical_javascript_string,
        perl_literal = lexical_perl_literal,
        perl_string = lexical_perl_string,
        php_literal = lexical_php_literal,
        php_string = lexical_php_string,
        java_literal = lexical_java_literal,
        java_string = lexical_java_string,
        r_literal = lexical_r_literal,
        r_string = lexical_r_string,
        kotlin_literal = lexical_kotlin_literal,
        kotlin_string = lexical_kotlin_string,
        julia_literal = lexical_julia_literal,
        julia_string = lexical_julia_string,
        csharp7_literal = lexical_csharp7_literal,
        csharp7_string = lexical_csharp7_string,
        csharp6_literal = lexical_csharp6_literal,
        csharp6_string = lexical_csharp6_string,
        csharp5_literal = lexical_csharp5_literal,
        csharp5_string = lexical_csharp5_string,
        csharp4_literal = lexical_csharp4_literal,
        csharp4_string = lexical_csharp4_string,
        csharp3_literal = lexical_csharp3_literal,
        csharp3_string = lexical_csharp3_string,
        csharp2_literal = lexical_csharp2_literal,
        csharp2_string = lexical_csharp2_string,
        csharp1_literal = lexical_csharp1_literal,
        csharp1_string = lexical_csharp1_string,
        kawa_literal = lexical_kawa_literal,
        kawa_string = lexical_kawa_string,
        gambitc_literal = lexical_gambitc_literal,
        gambitc_string = lexical_gambitc_string,
        guile_literal = lexical_guile_literal,
        guile_string = lexical_guile_string,
        clojure_literal = lexical_clojure_literal,
        clojure_string = lexical_clojure_string,
        erlang_literal = lexical_erlang_literal,
        erlang_string = lexical_erlang_string,
        elm_literal = lexical_elm_literal,
        elm_string = lexical_elm_string,
        scala_literal = lexical_scala_literal,
        scala_string = lexical_scala_string,
        elixir_literal = lexical_elixir_literal,
        elixir_string = lexical_elixir_string,
        fortran_literal = lexical_fortran_literal,
        fortran_string = lexical_fortran_string,
        d_literal = lexical_d_literal,
        d_string = lexical_d_string,
        coffeescript_literal = lexical_coffeescript_literal,
        coffeescript_string = lexical_coffeescript_string,
        cobol_literal = lexical_cobol_literal,
        cobol_string = lexical_cobol_string,
        fsharp_literal = lexical_fsharp_literal,
        fsharp_string = lexical_fsharp_string,
        vb_literal = lexical_vb_literal,
        vb_string = lexical_vb_string,
        ocaml_literal = lexical_ocaml_literal,
        ocaml_string = lexical_ocaml_string,
        objectivec_literal = lexical_objectivec_literal,
        objectivec_string = lexical_objectivec_string,
        reasonml_literal = lexical_reasonml_literal,
        reasonml_string = lexical_reasonml_string,
        octave_literal = lexical_octave_literal,
        octave_string = lexical_octave_string,
        matlab_literal = lexical_matlab_literal,
        matlab_string = lexical_matlab_string,
        zig_literal = lexical_zig_literal,
        zig_string = lexical_zig_string,
        sage_literal = lexical_sage_literal,
        sage_string = lexical_sage_string,
        json = lexical_json,
        toml = lexical_toml,
        yaml = lexical_yaml,
        xml = lexical_xml,
        sqlite = lexical_sqlite,
        postgresql = lexical_postgresql,
        mysql = lexical_mysql,
        mongodb = lexical_mongodb,

        // HIDDEN DEFAULTS
        permissive = lexical_permissive,
        standard = lexical_standard,
        ignore = lexical_ignore,
    };

    // OPTION TAG

    // Tag for the option type in the tagged enum.
    enum class option_tag: uint32_t {
        some = ::lexical_some,
        none = ::lexical_none,
    };

    // OPTION

    // Option type for number format compilation.
    template <typename T>
    struct option {
        option_tag tag;
        T data;

        // Safely convert from a C-style result to a C++ one.
        // This is to prevent layout differences from causing UB.
        template <typename ResultT>
        static inline option from(ResultT c_opt)
        {
            // Ensure we likely have a similar layout.
            // We're technically invoking UB, since the layout isn't
            // guaranteed to be the same, but it would take a
            // very malicious compiler to do so.
            // Pretty much any approach would option in UB, even the platform-
            // specific bindings, since the structs aren't **guaranteed**
            // to be the same as what we're using.
            static_assert(sizeof(ResultT) == sizeof(option), "Invalid sizes");
            static_assert(std::is_standard_layout<ResultT>::value, "Not std layout");
            static_assert(std::is_standard_layout<option>::value, "Not std layout");

            option cc_opt;
            std::memcpy(&cc_opt, &c_opt, sizeof(option));
            return cc_opt;
        }

        inline bool is_some()
        {
            return tag == option_tag::some;
        }

        inline bool is_none()
        {
            return tag == option_tag::none;
        }

        inline T unwrap()
        {
            assert(is_some());
            return std::move(data);
        }

        inline friend bool operator==(const option& lhs, const option& rhs)
        {
            if (lhs.tag != rhs.tag) {
                return false;
            } else if (lhs.tag == option_tag::some) {
                return lhs.data == rhs.data;
            } else {
                return true;
            }
        }

        inline friend bool operator!=(const option& lhs, const option& rhs)
        {
            return !(lhs == rhs);
        }
    };

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
    inline option<number_format> number_format_compile(
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
        return option<number_format>::from(::lexical_number_format_compile(
            digit_separator,
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
        ));
    }

    // Compile permissive number format.
    //
    // The permissive number format does not require any control
    // grammar, besides the presence of mantissa digits.
    inline option<number_format> number_format_permissive()
    {
        return option<number_format>::from(::lexical_number_format_permissive());
    }

    // Compile standard number format.
    //
    // The standard number format is guaranteed to be identical
    // to the format expected by Rust's string to number parsers.
    inline option<number_format> number_format_standard()
    {
        return option<number_format>::from(::lexical_number_format_standard());
    }

    // Compile ignore number format.
    //
    // The ignore number format ignores all digit separators,
    // and is permissive for all other control grammar, so
    // implements a fast parser.
    //
    // * `digit_separator`                         - Character to separate digits.
    inline option<number_format> number_format_ignore(uint8_t digit_separator)
    {
        return option<number_format>::from(::lexical_number_format_ignore(digit_separator));
    }

    // Get the flag bits from the compiled float format.
    inline uint64_t number_format_flags(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_flags(f);
    }

    // Get the digit separator from the compiled float format.
    inline uint8_t number_format_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_digit_separator(f);
    }

    // Get if digits are required before the decimal point.
    inline bool number_format_required_integer_digits(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_required_integer_digits(f);
    }

    // Get if digits are required after the decimal point.
    inline bool number_format_required_fraction_digits(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_required_fraction_digits(f);
    }

    // Get if digits are required after the exponent character.
    inline bool number_format_required_exponent_digits(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_required_exponent_digits(f);
    }

    // Get if digits are required before or after the decimal point.
    inline bool number_format_required_digits(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_required_digits(f);
    }

    // Get if positive sign before the mantissa is not allowed.
    inline bool number_format_no_positive_mantissa_sign(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_no_positive_mantissa_sign(f);
    }

    // Get if positive sign before the mantissa is required.
    inline bool number_format_required_mantissa_sign(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_required_mantissa_sign(f);
    }

    // Get if exponent notation is not allowed.
    inline bool number_format_no_exponent_notation(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_no_exponent_notation(f);
    }

    // Get if positive sign before the exponent is not allowed.
    inline bool number_format_no_positive_exponent_sign(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_no_positive_exponent_sign(f);
    }

    // Get if sign before the exponent is required.
    inline bool number_format_required_exponent_sign(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_required_exponent_sign(f);
    }

    // Get if exponent without fraction is not allowed.
    inline bool number_format_no_exponent_without_fraction(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_no_exponent_without_fraction(f);
    }

    // Get if special (non-finite) values are not allowed.
    inline bool number_format_no_special(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_no_special(f);
    }

    // Get if special (non-finite) values are case-sensitive.
    inline bool number_format_case_sensitive_special(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_case_sensitive_special(f);
    }

    // Get if leading zeros before an integer are not allowed.
    inline bool number_format_no_integer_leading_zeros(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_no_integer_leading_zeros(f);
    }

    // Get if leading zeros before a float are not allowed.
    inline bool number_format_no_float_leading_zeros(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_no_float_leading_zeros(f);
    }

    // Get if digit separators are allowed between integer digits.
    inline bool number_format_integer_internal_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_integer_internal_digit_separator(f);
    }

    // Get if digit separators are allowed between fraction digits.
    inline bool number_format_fraction_internal_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_fraction_internal_digit_separator(f);
    }

    // Get if digit separators are allowed between exponent digits.
    inline bool number_format_exponent_internal_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_exponent_internal_digit_separator(f);
    }

    // Get if digit separators are allowed between digits.
    inline bool number_format_internal_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_internal_digit_separator(f);
    }

    // Get if a digit separator is allowed before any integer digits.
    inline bool number_format_integer_leading_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_integer_leading_digit_separator(f);
    }

    // Get if a digit separator is allowed before any fraction digits.
    inline bool number_format_fraction_leading_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_fraction_leading_digit_separator(f);
    }

    // Get if a digit separator is allowed before any exponent digits.
    inline bool number_format_exponent_leading_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_exponent_leading_digit_separator(f);
    }

    // Get if a digit separator is allowed before any digits.
    inline bool number_format_leading_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_leading_digit_separator(f);
    }

    // Get if a digit separator is allowed after any integer digits.
    inline bool number_format_integer_trailing_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_integer_trailing_digit_separator(f);
    }

    // Get if a digit separator is allowed after any fraction digits.
    inline bool number_format_fraction_trailing_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_fraction_trailing_digit_separator(f);
    }

    // Get if a digit separator is allowed after any exponent digits.
    inline bool number_format_exponent_trailing_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_exponent_trailing_digit_separator(f);
    }

    // Get if a digit separator is allowed after any digits.
    inline bool number_format_trailing_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_trailing_digit_separator(f);
    }

    // Get if multiple consecutive integer digit separators are allowed.
    inline bool number_format_integer_consecutive_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_integer_consecutive_digit_separator(f);
    }

    // Get if multiple consecutive fraction digit separators are allowed.
    inline bool number_format_fraction_consecutive_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_fraction_consecutive_digit_separator(f);
    }

    // Get if multiple consecutive exponent digit separators are allowed.
    inline bool number_format_exponent_consecutive_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_exponent_consecutive_digit_separator(f);
    }

    // Get if multiple consecutive digit separators are allowed.
    inline bool number_format_consecutive_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_consecutive_digit_separator(f);
    }

    // Get if any digit separators are allowed in special (non-finite) values.
    inline bool number_format_special_digit_separator(number_format format)
    {
        auto f = static_cast<uint64_t>(format);
        return ::lexical_number_format_special_digit_separator(f);
    }
#endif  // HAVE_FORMAT

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
    invalid_leading_zeros = ::lexical_invalid_leading_zeros,
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
    lexical_is_error(invalid_leading_zeros);

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

#ifdef HAVE_FORMAT
    // FORMAT DISPATCHER

    // Dispatch function for from_lexical_format.
    #define lexical_from_lexical_format(type)                                   \
        inline static                                                           \
        result<type>                                                            \
        from_lexical_format(                                                    \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using result_type = result<type>;                                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_format(first, last, f);              \
            return result_type::from(r);                                        \
        }

    // Dispatch function for from_lexical_partial_format.
    #define lexical_from_lexical_partial_format(type)                           \
        inline static                                                           \
        partial_result<type>                                                    \
        from_lexical_partial_format(                                            \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using partial_result_type = partial_result<type>;                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_partial_format(first, last, f);      \
            return partial_result_type::from(r);                                \
        }

    // Dispatch function for from_lexical_format_radix.
    #define lexical_from_lexical_format_radix(type)                             \
        inline static                                                           \
        result<type>                                                            \
        from_lexical_format_radix(                                              \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using result_type = result<type>;                                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_format_radix(first, last, radix, f); \
            return result_type::from(r);                                        \
        }

    // Dispatch function for from_lexical_partial_radix.
    #define lexical_from_lexical_partial_format_radix(type)                     \
        inline static                                                           \
        partial_result<type>                                                    \
        from_lexical_partial_format_radix(                                      \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using partial_result_type = partial_result<type>;                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_partial_format_radix(                \
                first, last, radix, f                                           \
            );                                                                  \
            return partial_result_type::from(r);                                \
        }

    // Get type name for lexical format dispatcher
    #define lexical_format_dispatcher_type(type) type##_format_dispatcher

    // Define a dispatcher for a given type. This
    // allows us to use std::conditional to get the proper
    // type (a type) from the type. Every single function will
    // be static.
    #ifdef HAVE_RADIX
        #define lexical_format_dispatcher(type)                                 \
            struct lexical_format_dispatcher_type(type)                         \
            {                                                                   \
                lexical_from_lexical_format(type)                               \
                lexical_from_lexical_partial_format(type)                       \
                lexical_from_lexical_format_radix(type)                         \
                lexical_from_lexical_partial_format_radix(type)                 \
            }
    #else   // !HAVE_RADIX
        #define lexical_format_dispatcher(type)                                 \
            struct lexical_format_dispatcher_type(type)                         \
            {                                                                   \
                lexical_from_lexical_format(type)                               \
                lexical_from_lexical_partial_format(type)                       \
            }
    #endif  // HAVE_RADIX

    lexical_format_dispatcher(i8);
    lexical_format_dispatcher(i16);
    lexical_format_dispatcher(i32);
    lexical_format_dispatcher(i64);
    lexical_format_dispatcher(isize);

    lexical_format_dispatcher(u8);
    lexical_format_dispatcher(u16);
    lexical_format_dispatcher(u32);
    lexical_format_dispatcher(u64);
    lexical_format_dispatcher(usize);

    lexical_format_dispatcher(f32);
    lexical_format_dispatcher(f64);

    // LOSSY FORMAT DISPATCHER

    // Dispatch function for from_lexical_lossy_format.
    #define lexical_from_lexical_lossy_format(type)                             \
        inline static                                                           \
        result<type>                                                            \
        from_lexical_lossy_format(                                              \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using result_type = result<type>;                                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_lossy_format(first, last, f);        \
            return result_type::from(r);                                        \
        }

    // Dispatch function for from_lexical_partial_lossy_format.
    #define lexical_from_lexical_partial_lossy_format(type)                     \
        inline static                                                           \
        partial_result<type>                                                    \
        from_lexical_partial_lossy_format(                                      \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using partial_result_type = partial_result<type>;                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_partial_lossy_format(first, last, f);\
            return partial_result_type::from(r);                                \
        }

    // Dispatch function for from_lexical_lossy_format_radix.
    #define lexical_from_lexical_lossy_format_radix(type)                       \
        inline static                                                           \
        result<type>                                                            \
        from_lexical_lossy_format_radix(                                        \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using result_type = result<type>;                                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_lossy_format_radix(                  \
                first, last, radix, f                                           \
            );                                                                  \
            return result_type::from(r);                                        \
        }

    // Dispatch function for from_lexical_partial_lossy_format_radix.
    #define lexical_from_lexical_partial_lossy_format_radix(type)               \
        inline static                                                           \
        partial_result<type>                                                    \
        from_lexical_partial_lossy_format_radix(                                \
            uint8_t const* first,                                               \
            uint8_t const* last,                                                \
            uint8_t radix,                                                      \
            number_format format                                                \
        )                                                                       \
        {                                                                       \
            using partial_result_type = partial_result<type>;                   \
            auto f = static_cast<uint64_t>(format);                             \
            auto r = ::lexical_ato##type##_partial_lossy_format_radix(          \
                first, last, radix, f                                           \
            );                                                                  \
            return partial_result_type::from(r);                                \
        }

    // Get type name for lexical lossy format dispatcher
    #define lexical_lossy_format_dispatcher_type(type) type##_lossy_format_dispatcher

    // Define a lossy, format dispatcher for a given type. This
    // allows us to use std::conditional to get the proper
    // type (a type) from the type. Every single function will
    // be static.
    #ifdef HAVE_RADIX
        #define lexical_lossy_format_dispatcher(type)                           \
            struct lexical_lossy_format_dispatcher_type(type)                   \
            {                                                                   \
                lexical_from_lexical_lossy_format(type)                         \
                lexical_from_lexical_partial_lossy_format(type)                 \
                lexical_from_lexical_lossy_format_radix(type)                   \
                lexical_from_lexical_partial_lossy_format_radix(type)           \
            }
    #else   // !HAVE_RADIX
        #define lexical_lossy_format_dispatcher(type)                           \
            struct lexical_lossy_format_dispatcher_type(type)                   \
            {                                                                   \
                lexical_from_lexical_lossy_format(type)                         \
                lexical_from_lexical_partial_lossy_format(type)                 \
            }
    #endif  // HAVE_RADIX

    lexical_lossy_format_dispatcher(f32);
    lexical_lossy_format_dispatcher(f64);
#endif  // HAVE_FORMAT

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

#ifdef HAVE_FORMAT
    // GET FORMAT DISPATCHER

    // Conditional to simplify long recursive statements.
    #define lexical_format_conditional(name, fallback)                          \
        typename std::conditional<                                              \
            lexical_is_same(name),                                              \
            lexical_format_dispatcher_type(name),                               \
            fallback                                                            \
        >::type

    // Create a single template that resolves to our dispatcher **or**
    // evaluates to void.
    template <typename T>
    using format_dispatcher = lexical_format_conditional(
        i8, lexical_format_conditional(i16, lexical_format_conditional(i32,
            lexical_format_conditional(i64, lexical_format_conditional(isize,
                lexical_format_conditional(u8, lexical_format_conditional(u16,
                    lexical_format_conditional(u32, lexical_format_conditional(u64,
                        lexical_format_conditional(usize, lexical_format_conditional(f32,
                            lexical_format_conditional(f64, void)
                        ))
                    ))
                ))
            ))
        ))
    );

    // GET LOSSY FORMAT DISPATCHER

    // Conditional to simplify long recursive statements.
    #define lexical_lossy_format_conditional(name, fallback)                    \
        typename std::conditional<                                              \
            lexical_is_same(name),                                              \
            lexical_lossy_format_dispatcher_type(name),                         \
            fallback                                                            \
        >::type

    // Create a single template that resolves to our lossy format dispatcher
    // **or** evaluates to void.
    template <typename T>
    using lossy_format_dispatcher = lexical_lossy_format_conditional(f32,
        lexical_lossy_format_conditional(f64, void)
    );
#endif  // HAVE_FORMAT

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
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_lossy.");

    auto* first = reinterpret_cast<uint8_t const*>(string.data());
    auto* last = first + string.length();
    return disp::from_lexical_lossy(first, last);
}

// High-level function to lossily, partially parse a value from string.
template <typename T>
inline partial_result<T> parse_partial_lossy(string_type string)
{
    using disp = lossy_dispatcher<T>;
    static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_lossy.");

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
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_radix(first, last, radix);
    }

    // High-level function to partially parse a value from string with a custom radix.
    template <typename T>
    inline partial_result<T> parse_partial_radix(string_type string, uint8_t radix)
    {
        using disp = dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_radix(first, last, radix);
    }

    // High-level function to lossily parse a value from string with a custom radix.
    template <typename T>
    inline result<T> parse_lossy_radix(string_type string, uint8_t radix)
    {
        using disp = lossy_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_lossy_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_lossy_radix(first, last, radix);
    }

    // High-level function to lossily, partially parse a value from string with a custom radix.
    template <typename T>
    inline partial_result<T> parse_partial_lossy_radix(string_type string, uint8_t radix)
    {
        using disp = lossy_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_lossy_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_lossy_radix(first, last, radix);
    }
#endif  // HAVE_RADIX

#ifdef HAVE_FORMAT
    // High-level function to parse a value from string.
    template <typename T>
    inline result<T> parse_format(string_type string, number_format format)
    {
        using disp = format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_format.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_format(first, last, format);
    }

    // High-level function to partially parse a value from string.
    template <typename T>
    inline partial_result<T> parse_partial_format(string_type string, number_format format)
    {
        using disp = format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_format.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_format(first, last, format);
    }

    // High-level function to lossily parse a value from string.
    template <typename T>
    inline result<T> parse_lossy_format(string_type string, number_format format)
    {
        using disp = lossy_format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_lossy_format.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_lossy_format(first, last, format);
    }

    // High-level function to lossily, partially parse a value from string.
    template <typename T>
    inline partial_result<T> parse_partial_lossy_format(string_type string, number_format format)
    {
        using disp = lossy_format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_lossy_format.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_lossy_format(first, last, format);
    }
#endif  // HAVE_FORMAT

#if defined(HAVE_RADIX) && defined(HAVE_FORMAT)
    // High-level function to parse a value from string with a custom radix.
    template <typename T>
    inline result<T> parse_format_radix(string_type string, uint8_t radix, number_format format)
    {
        using disp = format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_format_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_format_radix(first, last, radix, format);
    }

    // High-level function to partially parse a value from string with a custom radix.
    template <typename T>
    inline partial_result<T> parse_partial_format_radix(string_type string, uint8_t radix, number_format format)
    {
        using disp = format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_format_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_format_radix(first, last, radix, format);
    }

    // High-level function to lossily parse a value from string with a custom radix.
    template <typename T>
    inline result<T> parse_lossy_format_radix(string_type string, uint8_t radix, number_format format)
    {
        using disp = lossy_format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_lossy_format_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_lossy_format_radix(first, last, radix, format);
    }

    // High-level function to lossily, partially parse a value from string with a custom radix.
    template <typename T>
    inline partial_result<T> parse_partial_lossy_format_radix(string_type string, uint8_t radix, number_format format)
    {
        using disp = lossy_format_dispatcher<T>;
        static_assert(!std::is_void<disp>::value, "Invalid type passed to parse_partial_lossy_format_radix.");

        auto* first = reinterpret_cast<uint8_t const*>(string.data());
        auto* last = first + string.length();
        return disp::from_lexical_partial_lossy_format_radix(first, last, radix, format);
    }
#endif  // HAVE_RADIX && HAVE_FORMAT

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
#undef lexical_from_lexical_format
#undef lexical_from_lexical_partial_format
#undef lexical_from_lexical_format_radix
#undef lexical_from_lexical_partial_format_radix
#undef lexical_format_dispatcher_type
#undef lexical_format_dispatcher
#undef lexical_from_lexical_lossy_format
#undef lexical_from_lexical_partial_lossy_format
#undef lexical_from_lexical_lossy_format_radix
#undef lexical_from_lexical_partial_lossy_format_radix
#undef lexical_lossy_format_dispatcher_type
#undef lexical_lossy_format_dispatcher
#undef lexical_format_conditional
#undef lexical_lossy_format_conditional

}   // lexical

#endif  /* !LEXICAL_HPP_ */
