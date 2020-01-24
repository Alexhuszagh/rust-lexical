#!/bin/bash

if [ -z $LEXICAL_TARGET ]; then
    LEXICAL_TARGET=atof64
fi

rustup run nightly cargo fuzz run "$LEXICAL_TARGET" \
    --features="$LEXICAL_FEATURES" \
    --release
