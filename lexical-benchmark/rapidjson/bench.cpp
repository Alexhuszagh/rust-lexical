// RapidJson's internal strtod implementation isn't a single implementation,
// it pre-parses individual components so it isn't a fair comparison.
// I'm using the generalized JSON parser with a single element, however.

#include "../include/data.hpp"
#include "../include/rapidjson/document.h"
#include "../include/rapidjson/stringbuffer.h"
#include <benchmark/benchmark.h>
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <iostream>

using namespace rapidjson;

inline double strtod(const std::string& value)
{
    double d;
    Document doc;
    doc.Parse<kParseFullPrecisionFlag>(value.data());
    if (!doc.HasParseError() && doc.IsNumber()) {
        d = doc.GetDouble();
    } else {
        throw std::invalid_argument("unable to parse float");
    }
    return d;
}

#include "../include/benchmark.hpp"

