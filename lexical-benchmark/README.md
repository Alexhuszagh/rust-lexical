lexical-benchmark
=================

Benchmarks comparing lexical to other string-to-float implementations. The data for all these benchmarks will be embedded in the binaries.

# Running the Benchmark

The benchmark requires the following:

1. A C++ compiler, for example, [Clang](https://clang.llvm.org/get_started.html), [GCC](https://gcc.gnu.org/install/), or [MSVC](https://docs.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=vs-2017).
2. An installation of [golang](https://golang.org/doc/install#install).
3. An installation of [Python3](https://www.python.org/downloads/).
4. An installation of [Rust](https://doc.rust-lang.org/1.0.0/book/installing-rust.html).
5. An installation of Google [Benchmark](https://github.com/google/benchmark).
5. An installation of [CMake](https://cmake.org/download/).

To run and then plot the benchmark, run "python3 run.py", and to plot them, run "python3 plot.py". It should build, run, and then plot all the required benchmarks, and normalize them to the same timescale.
