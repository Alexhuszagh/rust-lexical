#!/bin/bash

export RUSTFLAGS="--emit asm -C llvm-args=-x86-asm-syntax=intel"
cargo +nightly build --release "$@"
