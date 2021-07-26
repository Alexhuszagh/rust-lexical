Digit Separators
================

Supporting performant parsers using digit separators in a no-allocator context is difficult to support correctly with adequate performance. One of the major issues is that the syntax of numbers that accept digit separators varies between implementations.

For example, in Java literals, only internal (including consecutive) digit separators are allowed. Therefore, the following floats are considered valid or invalid:

```java
double x = 1.0_3_4_5;
double x = 1.0__3;
double x = 1.0__3e4_5;
double x = 1_.0;        // invalid
double x = 1._0;        // invalid
```

However, in Julia, internal (including consecutive) digit separators are only allowed in the significant digits of a number, and not the exponent. Therefore, the following floats are considered valid or invalid:

```java
double x = 1.0_3_4_5;
double x = 1.0__3;
double x = 1.0__3e4_5;  // invalid
double x = 1_.0;        // invalid
double x = 1._0;        // invalid
```

This means any parser must be context-aware, and also understand control characters: a digit separator followed by a decimal point is a trailing digit separator, while one followed by a digit is an internal one.

# Defining Grammar

Due to the context-aware nature, it's important to define the grammar on how digit separators work:

1. Leading digit separators come before any other input, or after control characters. Any digit separators after a leading digit separator are considered leading, even if consecutive digit separators are not allowed.

Examples therefore include:

```ocaml
_1
__1
_1.0
__1.0
1._0
1.__0
1.0e_5
1.0e__5
```

2. Trailing digit separators come after any other input, or before control characters. Any digit separators before another trailing digit separator are considered trailing, even if consecutive digit separators are not allowed.

Examples therefore include:

```ocaml
1_
1__
1_.0
1__.0
1.0_
1.0__
1.0e5_
1.0e5__
```

3. Internal digit separators therefore are any digit separators that cannot be classified as leading or trailing. Likewise, any digit separators that are adjacent to another internal digit separator are considered internal, even if consecutive digit separators are not allowed.

Examples therefore include:

```ocaml
1_2
1__2
1_2.0
1__2.0
1.0_2
1.0__2
1.0e5_4
1.0e5__4
```

**Practical Definition**

This opens up a lot of possibilities: what is a valid control character? In practice, it's much easier to define control characters as every character that's not a valid digit, and therefore to handle parsing we just need to check against valid digits and the digit separator.

// TODO(ahuszagh) 
//  Document the actual iterator design.
//  Document the algorithm.
