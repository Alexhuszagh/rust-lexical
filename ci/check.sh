#!/bin/bash
# Check our code passes formatting and lint checks.

set -ex

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

scripts/check.sh
RUSTFLAGS="--deny warnings" cargo +nightly build --features=lint
