#!/bin/bash

set -ex

# Detect our build command if we are on travis or not (so we can test locally).
if [ -z $CI ]; then
    # Not on CI, use cargo
    CARGO=cargo
else
    # On CI, use cross
    CARGO=cross
    CARGO_TARGET="--target $TARGET"
fi

# Force default tests to disable default feature on NO_STD.
if [ ! -z $NO_STD ]; then
    DEFAULT_FEATURES="--no-default-features"
    DOCTESTS="--tests"
fi

# Disable doctests on nostd or if not supported.
if [ ! -z $DISABLE_DOCTESTS ]; then
    DOCTESTS="--tests"
fi

# Force the elaborate tests to use the following features.
if [ ! -z $NO_FEATURES ]; then
    LEXICAL_FEATURES=()
    CORE_FEATURES=()
else
    LEXICAL_FEATURES=(
        "rounding"
        "rounding,radix"
        "rounding,unchecked_index"
        "trim_floats"
        "trim_floats,radix"
        "trim_floats,unchecked_index"
        "grisu3"
        "ryu"
        "correct"
        "correct,radix"
        "correct,unchecked_index"
    )
    CORE_FEATURES=(
        "${LEXICAL_FEATURES[@]}"
        "table"
        "table,radix"
        "table,unchecked_index"
    )
fi

# Create the full string for the tests from the features.
if [ -z $NO_STD ]; then
    # Have std, need to add `std` to features.
    LEXICAL_FEATURES=("${LEXICAL_FEATURES[@]/#/--features=std,}")
    CORE_FEATURES=("${CORE_FEATURES[@]/#/--features=std,}")
else
    # Nostd, just add `--features=` to the features.
    LEXICAL_FEATURES=("${LEXICAL_FEATURES[@]/#/--features=}")
    CORE_FEATURES=("${CORE_FEATURES[@]/#/--features=}")
fi

# Build target.
build() {
    $CARGO build $CARGO_TARGET $DEFAULT_FEATURES
    $CARGO build $CARGO_TARGET $DEFAULT_FEATURES --release
}

# Test target.
test() {
    # Process arguments.
    features=("$@")

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    # Default tests.
    $CARGO test $CARGO_TARGET $DEFAULT_FEATURES $DOCTESTS
    $CARGO test $CARGO_TARGET $DEFAULT_FEATURES $DOCTESTS --release

    # Iterate over special features.
    for i in "${features[@]}"; do
        $CARGO test $CARGO_TARGET --no-default-features $i $DOCTESTS
    done

    # Use special tests if we have std.
    if [ -z $NO_STD ]; then
        $CARGO test $CARGO_TARGET --features=correct,rounding,radix special_rounding -- --ignored --test-threads=1
    fi
}

# Dry-run bench target
bench() {
    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ ! -z $DISABLE_BENCHES ]; then
        return
    fi

    $CARGO bench $CARGO_TARGET $DEFAULT_FEATURES --verbose --no-run
}

# Run ffi tests.
ffi_tests() {
    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ -z $ENABLE_FFI_TESTS ]; then
        return
    fi

    python3 runtests.py
}

# Run derive tests.
derive_tests() {
    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ -z $ENABLE_DERIVE_TESTS ]; then
        return
    fi

    # Don't use cross, since we have a relative path.
    cargo test
}

main() {
    # Build and test lexical (only on std).
    if [ -z $NO_STD ]; then
        build
        test "${LEXICAL_FEATURES[@]}"
        bench
    fi

    # Build and test lexical-core.
    cd lexical-core
    build
    test "${CORE_FEATURES[@]}"

    # Build and test lexical-capi
    cd ../lexical-capi
    ffi_tests

    # Build and test lexical-derive
    cd ../lexical-derive
    derive_tests
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
