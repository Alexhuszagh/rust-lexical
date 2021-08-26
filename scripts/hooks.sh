#!/bin/bash
# Add hooks to git.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

# Install formatting hook.
echo 'echo "Running rustfmt and clippy checks."' > .git/hooks/pre-commit
echo "scripts/check.sh" >> .git/hooks/pre-commit
echo 'echo "Running linter checks."' > .git/hooks/pre-commit
echo 'RUSTFLAGS="--deny warnings" cargo +nightly build --features=lint' >> .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
