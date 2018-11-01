lexical
=======

Fast lexical conversion routines.

# Getting Started

# Benchmarks

**ftoa**

![ftoa benchmark](assets/ftoa.svg | width=200)

# Details

// Note use of a Grisu2 algorithm, rather than Grisu3, which creates the 
// non-shortest version rather than aborting, but sacrifices correctness in
// extremely rare edge cases (~ 0.5%) for speed.

https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf

# License

# Contributing
