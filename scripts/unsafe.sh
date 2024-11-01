#!/bin/bash
# Print diagnostics on the amount of unsafe code used.
# NOTE: This is currently unused since cargo-count is
# unmaintained and requires an old version of clap.

set -e

count() {
    echo -e "\e[0;94mCounting metrics for: \e[0;92m$1\e[0m"
    cd ../"$1"
    cargo +nightly count --separator , --unsafe-statistics
}

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
cd "${script_home}"/
count "lexical-util"
count "lexical-parse-integer"
count "lexical-parse-float"
count "lexical-write-integer"
count "lexical-write-float"
count "lexical-core"
count "lexical"
