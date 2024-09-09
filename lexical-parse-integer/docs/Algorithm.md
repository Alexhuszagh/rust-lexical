# Algorithm Approach

**Parsing Multiple Digits**

Normally, to parse 8 digits, we need at **least** 7 multiplies. However, for numeric strings with a radix <= 10, all digits are adjacent in the range `[0x30, 0x39]`, meaning we can validate all digits are valid using bitmasks or addition/subtraction operations, and can normalize the digits by subtracting `0x30`. For a detailed explanation of the algorithm, see ["Fast numeric string to int"](https://johnnylee-sde.github.io/Fast-numeric-string-to-int/) by Johnny Lee.

Once these digits are validated and normalized, we can scale the numbers from the range `0 <= N <= 9` to the range `0 <= Nn <= 99` with a single multiply. Likewise, we can go to `0 <= Nnnn <= 9999` and `0 <= Nnnnnnnn <= 99999999` in 2 and 3 multiplies, respectively.

This means we can parse 8 digits in 3 (rather than 7) multiplies and 4 digits in 2 (rather than 3) multiplies, considerably more efficient than a naive solution. Since multiply instructions are the primary bottleneck in parsing integers, this leads to dramatic performance gains.

**Minimizing Branching**

Integer parsing is relatively simple and fast, and therefore too many branches leads to a dramatic loss in performance. In most real-world datasets, integers are not uniformly distributed, and tend to be biased towards smaller values (such as indexes, or counts). Therefore, any optimizations for large integers must minimally affect small integers.

Therefore, only 1 optimization for parsing multiple digits was used for each type (4 for 32-bit integers, 8 for 64-bit and 128-bit integers), to avoid slowing down parsing simple integers. Likewise, all format-dependent or radix-dependent branching is done at compile-time, to avoid adding any performance penalties at run-time.

Finally, for 32-bit and 64-bit signed integers, we use no multiple-digit optimizations, since they provide **no** benefit for 32-bit integers in any cases, and only ~23% benefit for large 64-bit integers. However, for simple integers, due to the increased branching, they induce a performance penalty of ~50%.

In addition, rather than have separate branches for positive and negative numbers, both are parsed as unsigned integers, and then converted to the signed variant, after overflowing checking.
