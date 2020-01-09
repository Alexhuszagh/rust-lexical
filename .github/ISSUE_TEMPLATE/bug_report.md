---
name: Bug report
about: Create a report to help us improve
title: "[BUG]"
labels: bug
assignees: Alexhuszagh

---

## Description

Please include a clear and concise description of the bug. If the bug includes a security vulnerability, please privately report the issue to the [maintainer](mailto:ahuszagh@gmail.com).

## Prerequisites

Here are a few things you should provide to help me understand the issue:

- Rust version : `rustc -V`
- lexical version :
- lexical compilation features used:

## Test case

Please provide a short, complete (with crate import, etc) test case for
the issue, showing clearly the expected and obtained results.

Example test case:

```
#[macro_use]
extern crate lexical_core;

fn main() {
  let value: f64 = lexical_core::parse("1.2").unwrap();
  assert_eq!(value, 1.2);
}
```

## Additional Context
Add any other context about the problem here.
