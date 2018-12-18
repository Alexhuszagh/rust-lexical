// RapidJson's internal strtod implementation isn't a single implementation,
// it pre-parses individual components so it isn't a fair comparison.
// I'm using the generalized JSON parser with a single element, however.

#include "../include/data.hpp"
#include "../include/rapidjson/reader.h"
#include "../include/rapidjson/stringbuffer.h"
#include <benchmark/benchmark.h>
#include <algorithm>
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <iostream>

using namespace rapidjson;

struct MyHandler : public BaseReaderHandler<UTF8<>, MyHandler> {
    double d_;
    bool Null() { throw std::invalid_argument("unable to parse float"); }
    bool Bool(bool b) { throw std::invalid_argument("unable to parse float"); }
    bool Int(int i) { throw std::invalid_argument("unable to parse float"); }
    bool Uint(unsigned u) { throw std::invalid_argument("unable to parse float"); }
    bool Int64(int64_t i) { throw std::invalid_argument("unable to parse float"); }
    bool Uint64(uint64_t u) { throw std::invalid_argument("unable to parse float"); }
    bool Double(double d) { d_ = d; return true; }
    bool String(const char* str, SizeType length, bool copy) {  throw std::invalid_argument("unable to parse float"); }
    bool StartObject() { throw std::invalid_argument("unable to parse float");}
    bool Key(const char* str, SizeType length, bool copy) { throw std::invalid_argument("unable to parse float"); }
    bool EndObject(SizeType memberCount) { throw std::invalid_argument("unable to parse float"); }
    bool StartArray() { throw std::invalid_argument("unable to parse float"); }
    bool EndArray(SizeType elementCount) { throw std::invalid_argument("unable to parse float"); }
};


inline double strtod(const std::string& value)
{
    MyHandler handler;
    Reader reader;
    StringStream ss(value.data());
    reader.ParseNumber<kParseFullPrecisionFlag>(ss, handler);

    return handler.d_;
}

#include "../include/benchmark.hpp"

