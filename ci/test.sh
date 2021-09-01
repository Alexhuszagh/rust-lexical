#!/bin/bash
# Run main test suite.

set -ex

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

# Print our cargo version, for debugging.
cargo --version

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
        cargo check --tests $check_features
    done

    # Check each of our sub-crates compiles.
    cd lexical-parse-float
    cargo check --tests $check_features

    cd ../lexical-parse-integer
    cargo check --tests $check_features

    cd ../lexical-write-float
    cargo check --tests $check_features

    cd ../lexical-write-integer
    cargo check --tests $check_features

    cd ..
}

# Build target.
build() {
    build_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    cargo build $build_features
    cargo build $build_features --release
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
    cargo test $test_features $DOCTESTS
    cargo test $test_features $DOCTESTS --release
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

    cd lexical-benchmark
    bench_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    cargo test $bench_features --bench '*'
    cd ..
}

main() {
    check
    build
    test
    bench

    if [ ! -z $NIGHTLY ]; then
        scripts/check.sh
        RUSTFLAGS="--deny warnings" cargo +nightly build --features=lint
    fi
}

main
