#pragma once

#include <benchmark/benchmark.h>

static void denormal10(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[0]));
    }
}

static void denormal20(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[1]));
    }
}

static void denormal30(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[2]));
    }
}

static void denormal40(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[3]));
    }
}

static void denormal50(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[4]));
    }
}

static void denormal100(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[5]));
    }
}

static void denormal200(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[6]));
    }
}

static void denormal400(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[7]));
    }
}

static void denormal800(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[8]));
    }
}

static void denormal1600(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[9]));
    }
}

static void denormal3200(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[10]));
    }
}

static void denormal6400(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(DENORMAL_DATA[11]));
    }
}

static void large10(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[0]));
    }
}

static void large20(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[1]));
    }
}

static void large30(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[2]));
    }
}

static void large40(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[3]));
    }
}

static void large50(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[4]));
    }
}

static void large100(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[5]));
    }
}

static void large200(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[6]));
    }
}

static void large400(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[7]));
    }
}

static void large800(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[8]));
    }
}

static void large1600(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[9]));
    }
}

static void large3200(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[10]));
    }
}

static void large6400(benchmark::State& state)
{
    for (auto _ : state) {
        benchmark::DoNotOptimize(strtod(LARGE_DATA[11]));
    }
}

static void digits2(benchmark::State& state)
{
    for (auto _ : state) {
        for (const auto& value: DIGITS2_DATA) {
            benchmark::DoNotOptimize(strtod(value));
        }
    }
}

static void digits8(benchmark::State& state)
{
    for (auto _ : state) {
        for (const auto& value: DIGITS8_DATA) {
            benchmark::DoNotOptimize(strtod(value));
        }
    }
}

static void digits16(benchmark::State& state)
{
    for (auto _ : state) {
        for (const auto& value: DIGITS16_DATA) {
            benchmark::DoNotOptimize(strtod(value));
        }
    }
}

static void digits32(benchmark::State& state)
{
    for (auto _ : state) {
        for (const auto& value: DIGITS32_DATA) {
            benchmark::DoNotOptimize(strtod(value));
        }
    }
}

static void digits64(benchmark::State& state)
{
    for (auto _ : state) {
        for (const auto& value: DIGITS64_DATA) {
            benchmark::DoNotOptimize(strtod(value));
        }
    }
}

BENCHMARK(denormal10);
BENCHMARK(denormal20);
BENCHMARK(denormal30);
BENCHMARK(denormal40);
BENCHMARK(denormal50);
BENCHMARK(denormal100);
BENCHMARK(denormal200);
BENCHMARK(denormal400);
BENCHMARK(denormal800);
BENCHMARK(denormal1600);
BENCHMARK(denormal3200);
BENCHMARK(denormal6400);
BENCHMARK(large10);
BENCHMARK(large20);
BENCHMARK(large30);
BENCHMARK(large40);
BENCHMARK(large50);
BENCHMARK(large100);
BENCHMARK(large200);
BENCHMARK(large400);
BENCHMARK(large800);
BENCHMARK(large1600);
BENCHMARK(large3200);
BENCHMARK(large6400);
BENCHMARK(digits2);
BENCHMARK(digits8);
BENCHMARK(digits16);
BENCHMARK(digits32);
BENCHMARK(digits64);
BENCHMARK_MAIN();
