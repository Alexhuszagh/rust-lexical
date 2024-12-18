#!/bin/bash
# Check our code passes formatting and lint checks.

set -ex

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
cd "${home}"

export RUSTFLAGS="--deny warnings"

scripts/check.sh
cargo +nightly build --features=lint
