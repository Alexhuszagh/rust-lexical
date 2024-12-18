#!/bin/bash
# Generate all the symlinks for each child directory.

set -e

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
cd "${home}"

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
    "asm"
    "benchmark"
    "size"
)
PROJECT_FILES=(
    "clippy.toml"
    "rustfmt.toml"
)

for project in "${PROJECTS[@]}"; do
    for file in "${PROJECT_FILES[@]}"; do
        project_file="extras/${project}/${file}"
        unlink "${project_file}"
        ln -s ../"${file}" "${project_file}"
    done
done
