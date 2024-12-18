# Extras

This contains our unittests that depend on external, dev dependencies. This avoids having any packaging conflicts for versioning due to external, development packages. Lexical is a rather core crate, and the external dependencies create moving goalposts for versioning, as well as drag in more packages than required. By having a single, core workspace for each test, we can minimize build times and also external dependency logic.

This also includes logic for benchmarks and more.
