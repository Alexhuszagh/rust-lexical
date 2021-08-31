#!/bin/bash
# Generate all the symlinks for each child directory.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

WORKSPACES=(
    "lexical"
    "lexical-core"
    "lexical-parse-float"
    "lexical-parse-integer"
    "lexical-write-float"
    "lexical-write-integer"
    "lexical-util"
)
WORKSPACE_FILES=(
    "CODE_OF_CONDUCT.md"
    "LICENSE-APACHE"
    "LICENSE-MIT"
    "README.md"
)
for workspace in "${WORKSPACES[@]}"; do
    for file in "${WORKSPACE_FILES[@]}"; do
        unlink "$workspace/$file"
        ln -s ../"$file" "$workspace/$file"
    done
done

PROJECTS=(
    "lexical-asm"
    "lexical-benchmark"
    "lexical-size"
)
PROJECT_FILES=(
    "clippy.toml"
    "rustfmt.toml"
)

for project in "${PROJECTS[@]}"; do
    for file in "${PROJECT_FILES[@]}"; do
        unlink "$project/$file"
        ln -s ../"$file" "$project/$file"
    done
done
