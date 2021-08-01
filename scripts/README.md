scripts
=======

Bash-only scripts for comprehensive tests, fuzzing, formatting, and clippy. Since many of the linting and other features require nightly features, this ensures everything is run on nightly, or uses the appropriate Valgrind or Miri tests.

# Dependencies

The entire list of dependencies required is:

- Nightly compiler (`rustup toolchain install nightly`)
  - Clippy (`rustup component add clippy --toolchain nightly`)
  - Fmt (`rustup component add rustfmt --toolchain nightly`)
  - Miri (`rustup +nightly component add miri`)
  - Valgrind (`cargo +nightly install valgrind`)
- Python3.6+
