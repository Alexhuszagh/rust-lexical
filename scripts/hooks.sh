#!/bin/bash
# Add hooks to git.

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

# Install formatting hook.
echo 'echo "Running cargo fmt."' > .git/hooks/pre-commit
echo "scripts/fmt.sh" >> .git/hooks/pre-commit
echo "cargo +nightly build --features=lint" >> .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
