// Note: This benchmark isn't currently supported on:
//  GCC <= 8.2
//  Clang <= 6.0.1

#include "../include/data.hpp"
#include <charconv>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

inline double strtod(const std::string& value)
{
    const char* first = value.data();
    const char* last = first + value.size();
    double result;
    auto [p, ec] = std::from_chars(first, last, result, std::chars_format::general);
    if (ec == std::errc()) {
        throw std::invalid_argument("unable to parse float");
    }
    return result;
}

#include "../include/benchmark.hpp"
