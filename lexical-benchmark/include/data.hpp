#pragma once

#include "json.hpp"
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

using json = nlohmann::json;

std::string read_to_string(const std::string& path)
{
    std::ifstream t(path);
    std::stringstream buffer;
    buffer << t.rdbuf();
    return buffer.str();
}

std::vector<std::string> parse_json(const std::string& name)
{
    std::string path = "../data/" + name;
    auto bytes = read_to_string(path);
    auto data = json::parse(bytes);
    return data.get<std::vector<std::string>>();
}

auto DENORMAL_DATA = parse_json("denormal_halfway.json");
auto LARGE_DATA = parse_json("large_halfway.json");
auto DIGITS2_DATA = parse_json("digits2.json");
auto DIGITS8_DATA = parse_json("digits8.json");
auto DIGITS16_DATA = parse_json("digits16.json");
auto DIGITS32_DATA = parse_json("digits32.json");
auto DIGITS64_DATA = parse_json("digits64.json");
