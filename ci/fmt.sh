#!/bin/bash

set -ex

# Leave early if not on nightly.
version=$(rustc -V)
if [[ "$version" != *"nightly"* ]]; then
    # Error, not on nightly
    >&2 echo "Error: rustfmt must be run on nightly."
    exit 1
fi

# Check if we're not running from the project root.
config=.git/config
if [ ! -f "$config" ]; then
    >&2 echo "Error: script must be run from project root."
    exit 1
fi

# Format all subprojects.

cargo fmt

cd lexical-core
cargo fmt

cd ../lexical-capi
cargo fmt

cd ../lexical-derive
cargo fmt

cd ../lexical-benchmark/lexical
cargo fmt

cd ../minimal_lexical
cargo fmt
