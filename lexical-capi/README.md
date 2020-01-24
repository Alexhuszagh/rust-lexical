lexical-capi
============

[![Build Status](https://api.travis-ci.org/Alexhuszagh/rust-lexical.svg?branch=master)](https://travis-ci.org/Alexhuszagh/rust-lexical)
[![Rustc Version 1.24+](https://img.shields.io/badge/rustc-1.24+-lightgray.svg)](https://blog.rust-lang.org/2018/02/15/Rust-1.24.html)

**Table of Contents**

- [Getting Started](#getting-started)
- [Python](#python)
- [CMake](#cmake)
- [C++](#c)
- [C](#c-1)

# Getting Started

Lexical-capi is a crate to generate a foreign function interface for lexical conversion routines, and has no Rust API. After building (and optionally installing) a lexical-capi native library, you may use the the headers/scripts in the include directory to simplify interfacing with the exported symbols. For extensive documentation on the caveats of certain features, see the documentation for [lexical-core](https://docs.rs/lexical-core).

To build lexical-capi, simply run the following snippet from the project root:

```bash
cargo build --release
```

# Python

The Python script, [lexical.py](include/lexical.py) will attempt to find the lexical_capi shared library relative to the directory the script is run from. Since Python does not have strong-typing, and lexical is based off fixed-width integer and floating point types, the type is included in the function signature of each parser and formatter. Simple usage of the can be summarized by the following example, for more advanced usage, please see the API [definitions](include/lexical.py) and the [test suite](tests/test_py.py).

```python
import lexical

# SERIALIZERS

# Serialize 64-bit integer to string, `'10'`.
print(lexical.atoi64(10))

# Serialize 64-bit float to string, `'10.5'`.
print(lexical.f64toa(10.5))

if lexical.HAVE_RADIX:
    # Have radix support in the library, can use non-decimal radixes.

    # Serialize 64-bit integer to string in binary, `'1010'`.
    print(lexical.i64toa_radix(10, 2))

    # Serialize 64-bit integer to string in hexadecimal, `'A'`.
    print(lexical.i64toa_radix(10, 16))

# PARSERS

# Parse 64-bit integer from string, `10`.
print(lexical.atoi64('10'))

# Parse 64-bit float from string, `10.5`.
print(lexical.atof64('10.5'))

if lexical.HAVE_RADIX:
    # Parse 64-bit integer from string in binary, `10`.
    print(lexical.atoi64_radix('1010', 2))

    # Parse 64-bit integer from string in hexadecimal, `10`.
    print(lexical.atoi64_radix('A', 16))

if lexical.HAVE_FORMAT:
    # Parse 64-bit integer from string in binary with digit separators, `10`.
    format = lexical.NumberFormat.ignore(b'_')
    print(lexical.atoi64_format('1_0', format))

    # Parse 64-bit float from string in binary with digit separators, `10.51`.
    print(lexical.atof64_format('1_0.5_1', format))

# PARTIAL PARSERS

# While parsing, it's frequently required to identify the substring to
# parse, and then parse that substring to a native type. This incurs
# significant overhead, so lexical includes partial parsers, which
# parsing until invalid data occurs, returning the value and the number 
# of parsed digits, the form `(value, processed)`.

# Parse 64-bit integer from string, `(10, 2)`.
print(lexical.atoi64_partial('10a'))

# Parse 64-bit float from string, `(10.5, 4)`.
print(lexical.atof64_partial('10.5a'))

if lexical.HAVE_RADIX:
    # Parse 64-bit integer from string in binary, `(10, 4)`.
    print(lexical.atoi64_partial_radix('10102', 2))

    # Parse 64-bit integer from string in hexadecimal, `(10, 1)`.
    print(lexical.atoi64_partial_radix('AG', 16))

if lexical.HAVE_FORMAT:
    # Parse 64-bit integer from string in binary with digit separators, (`10`, 3).
    print(lexical.atoi64_partial_format('1_0a', format))

    # Parse 64-bit float from string in binary with digit separators, (`10.51`, 7).
    print(lexical.atof64_partial_format('1_0.5_1a', format))
```

# CMake

For both C and C++, we will assume you're using CMake as a build system. In order to build lexical-capi as a part of the build script, use `external_poject_add`:

```cmake
include(ExternalProject)

ExternalProject_Add(
    lexical
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build 
    COMMAND cargo build --release
    BINARY_DIR "${CMAKE_BINARY_DIR}/lexical"
    INSTALL_COMMAND ""
    LOG_BUILD ON
)
```

Next, whether building the library or importing it, find the library. If lexical-capi is installed globally, use:

```cmake
find_library(lexical NAMES lexical_capi ${CMAKE_SHARED_LIBRARY_PREFIX}lexical_capi)
```

Otherwise, set the imported location from a fixed location.

```cmake
set(LIBLEXICAL "${CMAKE_SHARED_LIBRARY_PREFIX}lexical_capi${CMAKE_SHARED_LIBRARY_SUFFIX}")
add_library(lexical SHARED IMPORTED)
set_target_properties(lexical PROPERTIES IMPORTED_LOCATION "path/to/dir")
```

Next, determine the features the binary supports (`HAVE_RADIX` and `HAVE_ROUNDING`):

```cmake
include(CheckFunctionExists)
list(APPEND CMAKE_REQUIRED_LIBRARIES lexical)
check_function_exists(lexical_atou8_format HAVE_FORMAT)
check_function_exists(lexical_get_exponent_backup_char HAVE_RADIX)
check_function_exists(lexical_get_float_rounding HAVE_ROUNDING)
```

Finally, add the include directories, library path, and compile definitions to any target linked with lexical:

```cmake
target_link_libraries(target lexical)
target_include_directories(target PRIVATE "path/to/liblexical-capi/include")

if(HAVE_FORMAT)
    target_compile_definitions(target PRIVATE HAVE_FORMAT=1)
endif()

if(HAVE_RADIX)
    target_compile_definitions(target PRIVATE HAVE_RADIX=1)
endif()

if(HAVE_ROUNDING)
    target_compile_definitions(target PRIVATE HAVE_ROUNDING=1)
endif()
```

# C++

**Notes** The C++ API of lexical requires C++11, and has better performance using C++17 (for `string_view`).

To integrate lexical into your C++ project, first see [CMake](#cmake) for building lexical, feature detection, and linking. Due to C++'s strong generics, C++ provides a high-level API using templates. Lexical does not use C++ exceptions, due to their significant overhead, opting instead for Rust-style error-handling (using a tagged union). Simple usage of the can be summarized by the following example, for more advanced usage, please see the API [definitions](include/lexical.hpp) and the [test suite](tests/test_cc.cc).

```cpp
#include <iostream>
#include "lexical.hpp"

namespace lx = lexical;

int main() {
    // SERIALIZERS

    // Serialize 64-bit integer to string, `"10"`.
    std::cout << lx::to_string(uint64_t(10)) << std::endl;

    // Serialize 64-bit float to string, `"10.5"`.
    std::cout << lx::to_string(double(10.5)) << std::endl;

    #ifdef HAVE_RADIX
        // Have radix support in the library, can use non-decimal radixes.

        // Serialize 64-bit integer to string in binary, `"1010"`.
        std::cout << lx::to_string_radix(uint64_t(10), 2) << std::endl;

        // Serialize 64-bit integer to string in hexadecimal, `"A"`.
        std::cout << lx::to_string_radix(uint64_t(10), 16) << std::endl;
    #endif

    // PARSERS

    // Parse 64-bit integer from string, a result with value 10.
    auto result = lx::parse<uint64_t>("10");
    if (result.is_ok()) {
        // Successfully have a value, can extract.
        std::cout << result.ok() << std::endl;  // 10
    } else {
        // Have an error, can report the error.
        std::cerr 
            << "Error code: " 
            << static_cast<int32_t>(result.err().code) 
            << std::endl;
    }

    // Parse 64-bit float from string, a result with value 10.5.
    lx::parse<double>("10.5");

    #ifdef HAVE_RADIX
        // Parse 64-bit integer from string in binary, a result with value 10.
        lx::parse_radix<uint64_t>("1010", 2);

        // Parse 64-bit integer from string in hexadecimal, a result with value 10.
        lx::parse_radix<uint64_t>("A", 16);
    #endif

    #ifdef HAVE_FORMAT
        // Parse 64-bit integer from string in binary with digit separators, 
        // a result with value 10.
        auto format = lx::number_format_ignore('_').unwrap();
        lx::parse_format<uint64_t>("1_0", format);
    #endif

    // PARTIAL PARSERS

    // While parsing, it's frequently required to identify the substring to
    // parse, and then parse that substring to a native type. This incurs
    // significant overhead, so lexical includes partial parsers, which
    // parsing until invalid data occurs, returning the value and the number 
    // of parsed digits, the form `(value, processed)`.

    // Parse 64-bit integer from string, `std::tuple(10, 2)`.
    lx::parse_partial<uint64_t>("10");

    // Parse 64-bit float from string, `std::tuple(10.5, 4)`.
    lx::parse_partial<double>("10.5a");


    #ifdef HAVE_RADIX
        // Parse 64-bit integer from string in binary, a result with value std::tuple(10, 4).
        lx::parse_partial_radix<uint64_t>("10102", 2);

        // Parse 64-bit integer from string in hexadecimal, a result with value std::tuple(10, 1).
        lx::parse_partial_radix<uint64_t>("AG", 16);
    #endif

    #ifdef HAVE_FORMAT
        // Parse 64-bit integer from string in binary with digit separators, 
        // a result with value `std::tuple(10, 3)`.
        auto format = lx::number_format_ignore('_').unwrap();
        lx::parse_partial_format<uint64_t>("1_0a", format);
    #endif

    return 0;
}
```

# C

To integrate lexical into your C++ project, first see [CMake](#cmake) for building lexical, feature detection, and linking.. C provides simple type declarations to simplify working with lexical, but provides a relatively low-level API. Please see [lexical.hpp](include/lexical.hpp) for how to use the C declarations.
