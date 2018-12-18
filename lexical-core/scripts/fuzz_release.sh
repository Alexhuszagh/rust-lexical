#!/bin/bash

RUSTFLAGS="-Clink-arg=-fuse-ld=gold" rustup run nightly cargo fuzz run atof64 --release
