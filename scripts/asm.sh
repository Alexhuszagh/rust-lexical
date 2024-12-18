#!/bin/bash
# Build human-readable ASM using Intel syntax.

set -e

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
cd "${home}"/extras/asm

export RUSTFLAGS="--emit asm -C llvm-args=-x86-asm-syntax=intel"
cargo +nightly build --release "$@"
