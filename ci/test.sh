#!/bin/bash
# Run main test suite.

set -ex

# Print our cargo version, for debugging.
cargo --version

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

# Force default tests to disable default feature on NO_STD.
if [ ! -z $NO_STD ]; then
    DEFAULT_FEATURES="--no-default-features"
    REQUIRED_FEATURES="parse-floats,parse-integers,write-floats,write-integers"
    DOCTESTS="--tests"
fi

# Test various feature combinations.
FEATURES=(
    "compact"
    "format"
    "power-of-two"
    "radix"
    "compact,format"
    "compact,radix"
    "format,power-of-two"
    "format,radix"
)

# Don't build the target, but ensure the syntax is correct.
check() {
    if [ ! -z $NO_FEATURES ]; then
        return
    fi

    # Need to test a few permutations just to ensure everything compiles.
    for features in "${FEATURES[@]}"; do
        check_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES,$features"
        $CARGO check $CARGO_TARGET --tests $check_features
    done
}

# Build target.
build() {
    build_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    $CARGO build $CARGO_TARGET $build_features
    $CARGO build $CARGO_TARGET $build_features --release

    # Check each of our sub-crates compiles.
    cd lexical-parse-float
    $CARGO build $CARGO_TARGET

    cd ../lexical-parse-integer
    $CARGO build $CARGO_TARGET

    cd ../lexical-write-float
    $CARGO build $CARGO_TARGET

    cd ../lexical-write-integer
    $CARGO build $CARGO_TARGET

    cd ..
}

# Test target.
test() {
    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ ! -z $NO_STD ]; then
        return
    fi

    # Default tests.
    test_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    $CARGO test $CARGO_TARGET $test_features $DOCTESTS
    $CARGO test $CARGO_TARGET $test_features $DOCTESTS --release

    if [ ! -z $NO_FEATURES ]; then
        return
    fi

    # Iterate over special features.
    for features in "${FEATURES[@]}"; do
        test_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES,$features"
        $CARGO test $CARGO_TARGET $test_features $DOCTESTS
    done
}

# Dry-run bench target
bench() {
    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ ! -z $DISABLE_BENCHES ]; then
        return
    fi
    if [ ! -z $NO_STD ]; then
        return
    fi
    if [ ! -z $NO_FEATURES ]; then
        # Benches are extremely slow, so disable them unless features are enabled.
        return
    fi

    bench_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    $CARGO test $CARGO_TARGET $bench_features --bench '*'
}

main() {
    check
    build
    test
    bench

    if [ ! -z $NIGHTLY ]; then
        rustup toolchain install nightly
        scripts/check.sh
        RUSTFLAGS="--deny warnings" cargo +nightly build --features=lint
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
