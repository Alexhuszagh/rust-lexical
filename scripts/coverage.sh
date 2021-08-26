#!/bin/bash
# Check code coverage from the unittests.
#
# Note that this is not meant to be fully comprehensive: tarpaulin
# has a few notable issues:
#   1. Does not cover `const fn`.
#   2. Does not cover inlined code.
#   3. Doesn't cover feature gates well.
#   4. It marks missing code coverage for code without branches.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

cargo +nightly tarpaulin
