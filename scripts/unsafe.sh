#!/bin/bash
# Print diagnostics on the amount of unsafe code used.

set -e

count() {
    echo -e "\e[0;94mCounting metrics for: \e[0;92m$1\e[0m"
    cd ../"$1"
    cargo +nightly count --separator , --unsafe-statistics
}

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/
count "lexical-util"
count "lexical-parse-integer"
count "lexical-parse-float"
count "lexical-write-integer"
count "lexical-write-float"
count "lexical-core"
count "lexical"
