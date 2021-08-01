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
# TODO(ahuszagh) Add Miri tests.

# This is very slow, but uses Valgrind to test all features.
if [ "$VALGRIND_TEST_ALL" != "" ]; then
fi
