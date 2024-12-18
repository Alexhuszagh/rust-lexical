#!/bin/bash
# Run fuzzers to check for memory issues.

set -e

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
cd "${home}"

cargo +nightly fuzz run "$@"
