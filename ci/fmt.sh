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
# We have a few files that seem to be ignored, enable these.
rustfmt --config-path rustfmt.toml src/atof/algorithm/cached/float160.rs
rustfmt --config-path rustfmt.toml src/atof/algorithm/cached/float160_decimal.rs
rustfmt --config-path rustfmt.toml src/atof/algorithm/cached/float160_radix.rs
rustfmt --config-path rustfmt.toml src/atof/algorithm/cached/float80_radix.rs
rustfmt --config-path rustfmt.toml src/atof/algorithm/powers/large32_radix.rs
rustfmt --config-path rustfmt.toml src/atof/algorithm/powers/large64_radix.rs
rustfmt --config-path rustfmt.toml src/atof/algorithm/powers/small32_radix.rs
rustfmt --config-path rustfmt.toml src/atof/algorithm/powers/small64_radix.rs

cd ../lexical-capi
cargo fmt

cd ../lexical-derive
cargo fmt

cd ../lexical-benchmark/lexical
cargo fmt

cd ../minimal_lexical
cargo fmt
