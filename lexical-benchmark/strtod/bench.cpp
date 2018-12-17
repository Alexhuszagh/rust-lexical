#include "../include/data.hpp"
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

inline double strtod(const std::string& value)
{
    return std::strtod(value.data(), nullptr);
}

#include "../include/benchmark.hpp"
