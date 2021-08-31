#!/bin/bash
# Run fuzzers to check for memory issues.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

cargo +nightly fuzz run "$@"
