#!/bin/bash

set -ex

# Detect our build command. If we enabled cross, default to
# that. Otherwise, only use cross if we are on CI and did
# not explicitly disable it.
if [ ! -z $ENABLE_CROSS ]; then
    # Specifically enabled cross.
    CARGO=cross
    CARGO_TARGET="--target $TARGET"
elif [ -z $CI ] || [ ! -z $DISABLE_CROSS ]; then
    # Explicitly disabled cross, use cargo.
    CARGO=cargo
else
    # On CI, did not disable cross, use cross.
    CARGO=cross
    CARGO_TARGET="--target $TARGET"
fi

# Detect our Python command if we are on travis or not (so we can test locally).
if [ -z $CI ]; then
    # Not on CI, use latest Python3.
    PYTHON3=python3
else
    # On CI, use python3.6.
    PYTHON3=python3.6
fi

# Force default tests to disable default feature on NO_STD.
if [ ! -z $NO_STD ]; then
    DEFAULT_FEATURES="--no-default-features"
    DOCTESTS="--tests"
fi

# Have std, need to add `std` to features.
if [ -z $NO_STD ]; then
    REQUIRED_FEATURES="std,$REQUIRED_FEATURES"
fi

# Add property tests to all tests if enabled.
if [ -z $DISABLE_PROPERTY_TESTS ] && [ -z $DISABLE_TESTS ]; then
    REQUIRED_FEATURES="property_tests,$REQUIRED_FEATURES"
fi

# Add libm to all features if enabled.
if [ ! -z $ENABLE_LIBM ]; then
    REQUIRED_FEATURES="libm,$REQUIRED_FEATURES"
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
        "format"
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

## Create the full string for the tests from the features.
#LEXICAL_FEATURES=("${LEXICAL_FEATURES[@]/#/--features=}")
#CORE_FEATURES=("${CORE_FEATURES[@]/#/--features=}")

# Build target.
build() {
    features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    $CARGO build $CARGO_TARGET $features
    $CARGO build $CARGO_TARGET $features --release
}

# Test target.
test() {
    # Process arguments.
    special_features=("$@")

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    # Default tests.
    features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    $CARGO test $CARGO_TARGET $features $DOCTESTS
    $CARGO test $CARGO_TARGET $features $DOCTESTS --release

    # Iterate over special features.
    for i in "${special_features[@]}"; do
        test_features="--no-default-features --features=$REQUIRED_FEATURES,$i"
        $CARGO test $CARGO_TARGET $test_features $DOCTESTS
    done

    # Use special tests if we have std.
    if [ -z $NO_STD ]; then
        features="--features=$REQUIRED_FEATURES,correct,rounding,radix special_rounding"
        $CARGO test $CARGO_TARGET $features -- --ignored --test-threads=1
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

    features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    $CARGO bench $CARGO_TARGET $features --verbose --no-run
}

# Run ffi tests.
ffi_tests() {
    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ -z $ENABLE_FFI_TESTS ]; then
        return
    fi

    $PYTHON3 runtests.py
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
