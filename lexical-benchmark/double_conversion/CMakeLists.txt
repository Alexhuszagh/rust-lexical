cmake_minimum_required(VERSION 3.0.0)
set(CMAKE_EXECUTABLE_SUFFIX "")
project(double_conversion)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/../include/double-conversion" "${CMAKE_CURRENT_BINARY_DIR}/double-conversion")
find_package(Threads REQUIRED)
find_package(benchmark REQUIRED)
add_executable(bench bench.cpp)
target_link_libraries(bench ${CMAKE_THREAD_LIBS_INIT} benchmark double-conversion)
