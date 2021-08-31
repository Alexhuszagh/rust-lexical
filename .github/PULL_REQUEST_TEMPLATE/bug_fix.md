---
name: Bug fix
about: Fix a bug in lexical.
title: "[BUG]"
labels: bug
assignees: Alexhuszagh

---

**NOTE:** 
- If you have made non-trivial changes to the code, please make sure to run unittests with the relevant features enabled prior to committing.
- If you have made any changes to parsing algorithms, please run at least `test-parse-golang` or `test-parse-unittests` with `feature = 
comprehensive_float_test"` enabled prior to committing, to ensure there are no regressions.
- Please run `scripts/fmt.sh` or `cargo fmt` on nightly prior to committing.

## Optional Debugging Information

If applicable to the issue, here are a few things you should provide to help me understand the issue:

- Rust version: `rustc -V`
- lexical version:
- lexical compilation features used:

## Description
Please include a clear and concise description of the changes made.

## Additional Context
Add any other context or screenshots about the bug fix here.
