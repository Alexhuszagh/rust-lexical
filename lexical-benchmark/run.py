#!/usr/bin/env python3

import contextlib
import json
import os
import re
import sys
from subprocess import Popen, PIPE, check_call

HOME = os.path.dirname(os.path.realpath(__file__))

# TEST NAMES

TEST_NAMES = [
    "denormal10",
    "denormal20",
    "denormal30",
    "denormal40",
    "denormal50",
    "denormal100",
    "denormal200",
    "denormal400",
    "denormal800",
    "denormal1600",
    "denormal3200",
    "denormal6400",
    "large10",
    "large20",
    "large30",
    "large40",
    "large50",
    "large100",
    "large200",
    "large400",
    "large800",
    "large1600",
    "large3200",
    "large6400",
    "digits2",
    "digits8",
    "digits16",
    "digits32",
    "digits64",
]

# HELPERS

@contextlib.contextmanager
def change_directory(path):
    """Change directory and return to the original directory afterwards."""

    cwd = os.getcwd()
    try:
        os.chdir(path)
        yield
    finally:
        os.chdir(cwd)

def camel_to_snake(name):
    """Convert a camelcase name to snakecase"""

    string = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", string).lower()

# PROCESSING

def process_cpp_benchmark(entry):
    """Process the entry from JSON data"""

    name = entry["name"]
    iterations = entry["iterations"]
    speed = entry["cpu_time"]
    unit = entry["time_unit"]
    if unit == "ns":
        pass
    elif unit == "us":
        speed *= 1e3
    elif unit == "ms":
        speed *= 1e6
    else:
        raise ValueError("Unknown unit: " + unit)

    return name, iterations, speed


def process_golang_benchmark(line):
    """Process the result of an individual Golang benchmark."""

    line = line.split("\t")
    name = camel_to_snake(re.match(r"Benchmark(\w+)-\d+\s+", line[0]).group(1))
    iterations = int(line[1].strip())
    speed = float(re.match(r"\s+(\d+(?:\.\d+)?) ns/op\n", line[2]).group(1))

    return name, iterations, speed


def process_python_benchmark(line):
    """Process the result of an individual Python benchmark."""

    pattern = r"(\d+) loops, best of \d+: (\d+(?:\.\d+)?) (\wsec) per loop"
    match = re.match(pattern, line)
    iterations = int(match.group(1))
    speed = float(match.group(2))
    unit = match.group(3)
    if unit == "nsec":
        pass
    elif unit == "usec":
        speed *= 1e3
    elif unit == "msec":
        speed *= 1e6
    else:
        raise ValueError("Unknown unit: " + unit)

    return iterations, speed


def process_rust_benchmark(line):
    """Process the result of an individual Golang benchmark."""

    pattern = r"(\w+)\s+time:\s+\[\d*\.\d*\s+\w+\s+(\d*\.\d*)\s+(\w+)\s+\d*\.\d*\s+\w+\]"
    match = re.match(pattern, line)
    name = match.group(1)
    speed = float(match.group(2))
    unit = match.group(3)
    if unit == "ns":
        pass
    elif unit == "us":
        speed *= 1e3
    elif unit == "ms":
        speed *= 1e6
    else:
        raise ValueError("Unknown unit: " + unit)

    return name, 0, speed


# RUNNERS


def run_lexical(feature=""):
    """Build and run the benchmarks for lexical."""

    data = {}
    with change_directory(os.path.join(HOME, "lexical")):
        args = ["cargo", "bench"]
        if feature:
            args.append("--features={}".format(feature))
        proc = Popen(args, bufsize=1<<20 , stdout=PIPE, stderr=PIPE)
        for line in iter(proc.stdout.readline, b''):
            if b'time:' in line:
                name, *d = process_rust_benchmark(line.decode('utf-8'))
                data[name] = d
        proc.stdout.close()
        proc.wait()

    return data


def run_libcore():
    """Build and run the benchmarks for libcore."""

    data = {}
    with change_directory(os.path.join(HOME, "libcore")):
        args = ["cargo", "bench"]
        proc = Popen(args, bufsize=1<<20 , stdout=PIPE, stderr=PIPE)
        for line in iter(proc.stdout.readline, b''):
            if b'time:' in line:
                name, *d = process_rust_benchmark(line.decode('utf-8'))
                data[name] = d
        proc.stdout.close()
        proc.wait()

    for name in TEST_NAMES:
        data.setdefault(name, (0, float('nan')))

    return data


def run_golang():
    """Build and run the benchmarks for Golang."""

    data = {}
    with change_directory(os.path.join(HOME, "go")):
        args = ["go", "test", "-bench=."]
        proc = Popen(args, bufsize=1<<20 , stdout=PIPE, stderr=PIPE)
        for line in iter(proc.stdout.readline, b''):
            if line.startswith(b"Benchmark"):
                name, *d = process_golang_benchmark(line.decode('utf-8'))
                data[name] = d
        proc.stdout.close()
        proc.wait()

    return data


def run_python():
    """Build and run the benchmarks for Python3."""

    data = {}
    halfway_benches = [
        ("DENORMAL_DATA", "denormal10", "0"),
        ("DENORMAL_DATA", "denormal20", "1"),
        ("DENORMAL_DATA", "denormal30", "2"),
        ("DENORMAL_DATA", "denormal40", "3"),
        ("DENORMAL_DATA", "denormal50", "4"),
        ("DENORMAL_DATA", "denormal100", "5"),
        ("DENORMAL_DATA", "denormal200", "6"),
        ("DENORMAL_DATA", "denormal400", "7"),
        ("DENORMAL_DATA", "denormal800", "8"),
        ("DENORMAL_DATA", "denormal1600", "9"),
        ("DENORMAL_DATA", "denormal3200", "10"),
        ("DENORMAL_DATA", "denormal6400", "11"),
        ("LARGE_DATA", "large10", "0"),
        ("LARGE_DATA", "large20", "1"),
        ("LARGE_DATA", "large30", "2"),
        ("LARGE_DATA", "large40", "3"),
        ("LARGE_DATA", "large50", "4"),
        ("LARGE_DATA", "large100", "5"),
        ("LARGE_DATA", "large200", "6"),
        ("LARGE_DATA", "large400", "7"),
        ("LARGE_DATA", "large800", "8"),
        ("LARGE_DATA", "large1600", "9"),
        ("LARGE_DATA", "large3200", "10"),
        ("LARGE_DATA", "large6400", "11"),
    ]
    digits_benches = [
        ("DIGITS2_DATA", "digits2"),
        ("DIGITS8_DATA", "digits8"),
        ("DIGITS16_DATA", "digits16"),
        ("DIGITS32_DATA", "digits32"),
        ("DIGITS64_DATA", "digits64"),
    ]
    with change_directory(os.path.join(HOME, "python")):
        for lst, name, idx in halfway_benches:
            args = [
                sys.executable,
                "-m", "timeit",
                "-s", "from bench import {0}".format(lst),
                "float({0}[{1}])".format(lst, idx)
            ]
            proc = Popen(args, bufsize=1<<20 , stdout=PIPE, stderr=PIPE)
            proc.wait()
            line = proc.stdout.read()
            proc.stdout.close()

            d = process_python_benchmark(line.decode('utf-8'))
            data[name] = d

        for lst, name in digits_benches:
            args = [
                sys.executable,
                "-m", "timeit",
                "-s", "from bench import {0}".format(lst),
                "for i in {0}:\tfloat(i)".format(lst)
            ]
            proc = Popen(args, bufsize=1<<20 , stdout=PIPE, stderr=PIPE)
            proc.wait()
            line = proc.stdout.read()
            proc.stdout.close()

            d = process_python_benchmark(line.decode('utf-8'))
            data[name] = d

    return data


def run_cpp(directory):
    """Build and run C++ benchmarks in a custom directory"""

    with change_directory(os.path.join(HOME, directory)):
        # Make a build directory, if not present.
        if not os.path.exists("build"):
            os.makedirs("build")

        # Build strtod
        with change_directory("build"):
            check_call(["cmake", "..", "-DCMAKE_BUILD_TYPE=Release"])
            check_call(["cmake", "--build", ".", "--config", "Release"])

        # Run the benchmarks
        args = ["build/bench", "--benchmark_format=json"]
        proc = Popen(args, bufsize=1<<20 , stdout=PIPE, stderr=PIPE)
        proc.wait()
        benchmarks = json.loads(proc.stdout.read())["benchmarks"]
        proc.stdout.close()

        data = {}
        for entry in benchmarks:
            k, *d = process_cpp_benchmark(entry)
            data[k] = d
        return data


def run_strtod():
    """Build and run the strtod benchmarks."""

    return run_cpp("strtod")


def run_from_chars():
    """Build and run the from_chars benchmarks."""

    # C++17 feature, currently not supported on most platforms.
    return run_cpp("from_chars")


def run_rapidjson():
    """Build and run the rapidjson benchmarks."""

    return run_cpp("rapidjson")


def run_double_conversion():
    """Build and run the double_conversion benchmarks."""

    return run_cpp("double_conversion")


def main():
    """Run the core benchmarks."""

    # Run the benchmarks.
    golang_data = run_golang()
    python_data = run_python()
    strtod_data = run_strtod()
    lexical_data = run_lexical()
    libcore_data = run_libcore()
    rapidjson_data = run_rapidjson()
    double_conversion_data = run_double_conversion()
    # Not supported on most C++17 compilers as of now.
    #from_chars_data = run_from_chars()

    # Make the results directories
    if not os.path.exists("results"):
        os.makedirs("results")

    # Dump to disk
    with open(os.path.join(HOME, "results","golang.json"), 'w') as f:
        json.dump(golang_data, f)
    with open(os.path.join(HOME, "results","python.json"), 'w') as f:
        json.dump(python_data, f)
    with open(os.path.join(HOME, "results","strtod.json"), 'w') as f:
        json.dump(strtod_data, f)
    with open(os.path.join(HOME, "results","lexical.json"), 'w') as f:
        json.dump(lexical_data, f)
    with open(os.path.join(HOME, "results","libcore.json"), 'w') as f:
        json.dump(libcore_data, f)
    with open(os.path.join(HOME, "results","rapidjson.json"), 'w') as f:
        json.dump(rapidjson_data, f)
    with open(os.path.join(HOME, "results","double_conversion.json"), 'w') as f:
        json.dump(double_conversion_data, f)
    #with open(os.path.join(HOME, "results","from_chars.json"), 'w') as f:
    #    json.dump(from_chars_data, f)

if __name__== '__main__':
    main()
