#!/bin/bash
# Build human-readable ASM using Intel syntax.

export RUSTFLAGS="--emit asm -C llvm-args=-x86-asm-syntax=intel"
cargo +nightly build --release "$@"
