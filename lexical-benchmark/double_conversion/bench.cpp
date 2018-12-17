#include "../include/data.hpp"
#include "../include/double-conversion/src/double-conversion.h"
#include <cmath>
#include <limits>

using namespace double_conversion;

const double QUIET_NAN = std::numeric_limits<double>::quiet_NaN();
const StringToDoubleConverter converter(0, 0.0, QUIET_NAN, NULL, NULL);

inline double strtod(const std::string& value)
{
    int count;
    auto d = converter.StringToDouble(value.data(), (int)value.size(), &count);
    if (std::isnan(d)) {
        throw std::invalid_argument("unable to parse float");
    }
    return d;
}

#include "../include/benchmark.hpp"
