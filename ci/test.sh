#!/bin/bash
# shellcheck disable=SC2086,SC2236
# Run main test suite.

set -ex

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
version="${CARGO_VERSION}"
cd "${home}"

export RUSTFLAGS="--deny warnings"

# Print our cargo version, for debugging.
cargo ${version} --version

# Ensure we have all our benchmark data files
git submodule update --init extras/benchmark/data

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

check_error() {
    local feature=$1
    if 2>/dev/null cargo ${version} check --no-default-features --features="${feature}" ; then
        >&2 echo "The feature ${feature} did not error..."
        exit 1
    fi
}

check_test() {
    pushd "${home}/lexical-${1}"

    cargo ${version} check --tests
    if [ -z $DISABLE_EXTRA_TESTS ]; then
        cd "${home}/extras/${1}"
        cargo ${version} check --tests
    fi

    popd
}

# Don't build the target, but ensure the syntax is correct.
check() {
    if [ ! -z $NO_FEATURES ]; then
        return
    fi

    # Need to test a few permutations just to ensure everything compiles.
    for features in "${FEATURES[@]}"; do
        check_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES,$features"
        cargo ${version} check --tests $check_features
    done

    # Check each of our sub-crates compiles.
    check_test "util"
    check_test "parse-float"
    check_test "parse-integer"
    check_test "write-float"
    check_test "write-integer"

    # ensure our partial features aren't allowed, as are unsupported features
    cd "${home}/lexical-core"
    partial=(parse write floats integers)
    for feature in "${partial[@]}"; do
        check_error "${feature}"
    done

    cd "${home}/lexical"
    for feature in "${partial[@]}"; do
        check_error "${feature}"
    done

    cd "${home}"
}

# Build target.
build() {
    build_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    cargo ${version} build $build_features
    cargo ${version} build $build_features --release
}

run_test() {
    cargo ${version} test "$@"

    if [ -z $DISABLE_EXTRA_TESTS ]; then
        pushd "${home}/extras"
        cargo ${version} test "$@"
        popd
    fi
}

run_test_high_level() {
    # this fixes an issue where the lexical and lexical-core tests weren't being run
    cd "${home}/lexical-core"
    cargo ${version} test "$@"
    cd "${home}"

    cd "${home}/lexical"
    cargo ${version} test "$@"
    cd "${home}"
}

# Test target.
tests() {
    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ ! -z $NO_STD ]; then
        return
    fi

    # Default tests.
    test_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    run_test $test_features $DOCTESTS
    run_test $test_features $DOCTESTS --release
    run_test --features=radix,format,compact $DOCTESTS --release
    # NOTE: This tests a regressions, related to #96.
    run_test --features=format $DOCTESTS --release

    # Ensure we test radix without the compact feature
    # See #169
    run_test --features=radix,format --release
    run_test_high_level $test_features,format
    run_test_high_level $test_features,radix
    run_test_high_level $test_features,format,radix
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

    cd "${home}/extras/benchmark"
    bench_features="$DEFAULT_FEATURES --features=$REQUIRED_FEATURES"
    cargo ${version} test $bench_features --bench '*'
    cd "${home}"
}

main() {
    check
    build
    tests
    bench

    if [ ! -z $NIGHTLY ]; then
        scripts/check.sh
        cargo +nightly build --features=lint
    fi
}

main
