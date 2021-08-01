#!/bin/bash
#
# Comprehensive unittests using both Valgrind and Miri to
# ensure the code doesn't have any obvious memory issues
# in the conversion routines.

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

cargo +nightly test
cargo +nightly valgrind test
cargo +nightly miri test --tests

# Test various feature combinations.
FEATURES=(
    "compact"
    "format"
    "power-of-two"
    "radix"
    "compact,format"
    "compact,radix"
    "format,power-of-two"
    "format,radix"
)
for features in "${FEATURES[@]}"; do
    cargo +nightly test --features="$features"
done

# This is very slow, but uses Valgrind to test all features.
if [ "$LEXICAL_VALGRIND_TEST_ALL" != "" ]; then
    for features in "${FEATURES[@]}"; do
        cargo +nightly valgrind test --features="$features"
    done
fi

# This is very slow, but uses Miri to test all features.
if [ "$LEXICAL_MIRI_TEST_ALL" != "" ]; then
    for features in "${FEATURES[@]}"; do
        cargo +nightly miri test --features="$features" --tests
    done
fi
