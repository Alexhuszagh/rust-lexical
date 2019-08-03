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

#ifndef LEXICALCORE_HPP_
#define LEXICALCORE_HPP_

#include "lexical.h"
#include <cstdint>
#include <string>

// CONFIG
// ------

inline std::string get_nan_string() {
    uint8_t* ptr;
    size_t size;
    if (get_nan_string_ffi(&ptr, &size) != 0) {
        throw std::runtime_error("Unexpected runtime error.");
    }
    return std::string(reinterpret_cast<char*>(ptr), size);
}

inline void set_nan_string(const std::string& string) {
    uint8_t* ptr = reinterpret_cast<uint8_t*>(string.data());
    size_t size = string.length();
    if (set_nan_string_ffi(ptr, size) != 0) {
        throw std::runtime_error("Unexpected runtime error.");
    }
}

inline std::string get_inf_string() {
    uint8_t* ptr;
    size_t size;
    if (get_inf_string_ffi(&ptr, &size) != 0) {
        throw std::runtime_error("Unexpected runtime error.");
    }
    return std::string(reinterpret_cast<char*>(ptr), size);
}

inline void set_inf_string(const std::string& string) {
    uint8_t* ptr = reinterpret_cast<uint8_t*>(string.data());
    size_t size = string.length();
    if (set_inf_string_ffi(ptr, size) != 0) {
        throw std::runtime_error("Unexpected runtime error.");
    }
}

inline std::string get_infinity_string() {
    uint8_t* ptr;
    size_t size;
    if (get_infinity_string_ffi(&ptr, &size) != 0) {
        throw std::runtime_error("Unexpected runtime error.");
    }
    return std::string(reinterpret_cast<char*>(ptr), size);
}

inline void set_infinity_string(const std::string& string) {
    uint8_t* ptr = reinterpret_cast<uint8_t*>(string.data());
    size_t size = string.length();
    if (set_infinity_string_ffi(ptr, size) != 0) {
        throw std::runtime_error("Unexpected runtime error.");
    }
}

// TODO(ahuszagh) Finish the definitions.


#endif  /* !LEXICALCORE_HPP_ */
