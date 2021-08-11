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

**Overflow Checking**

Rather than do checked multiplication and additions in each loop, which increases the amount of branching, we can check after if numeric overflow has occurred by checking the number of parsed digits, and the resulting value.

Given the following unoptimized code:

```rust,ignore
pub fn parse(bytes: &[u8]) -> Result<u64, ()> {
    let mut value: u64 = 0;
    let mut iter = bytes.iter();
    while let Some(&c) = iter.next() {
        let digit = match (c as char).to_digit(10) {
            Some(v) => v,
            None => return Err(()),
        };
        value = match value.checked_mul(10) {
            Some(v) => v,
            None => return Err(()),
        };
        value = match value.checked_add(digit as u64) {
            Some(v) => v,
            None => return Err(()),
        };
    }
    Ok(value)
}
```

This translates to the following assembly:

```asm
example::parse:
        xor     r11d, r11d
        mov     r10d, 10
        xor     eax, eax
        mov     r8d, 1
.LBB0_1:
        mov     r9, rax
        cmp     rsi, r11
        je      .LBB0_2
        movzx   ecx, byte ptr [rdi + r11]
        add     ecx, -48
        cmp     ecx, 10
        jae     .LBB0_6
        mov     rax, r9
        mul     r10
        jo      .LBB0_6
        mov     ecx, ecx
        add     r11, 1
        add     rax, rcx
        jae     .LBB0_1
.LBB0_6:
        mov     rax, r8
        mov     rdx, r9
        ret
.LBB0_2:
        xor     r8d, r8d
        mov     rax, r8
        mov     rdx, r9
        ret
```

We optimize it to the following code:

```rust,ignore
pub fn parse(bytes: &[u8]) -> Result<u64, ()> {
    let mut value: u64 = 0;
    let mut iter = bytes.iter();
    while let Some(&c) = iter.next() {
        let digit = match (c as char).to_digit(10) {
            Some(v) => v,
            None => return Err(()),
        };
        value = value.wrapping_mul(10);
        value = value.wrapping_mul(digit as u64);
    }
    Ok(value)
}
```

Which produces the following assembly:

```asm
example::parse:
        xor     eax, eax
        xor     ecx, ecx
.LBB0_1:
        cmp     rsi, rcx
        je      .LBB0_4
        movzx   edx, byte ptr [rdi + rcx]
        add     edx, -48
        add     rcx, 1
        cmp     edx, 10
        jb      .LBB0_1
        mov     eax, 1
.LBB0_4:
        xor     edx, edx
        ret
```

This is much more efficient, however, there is one major limitation: we cannot know if numerical overflow has occurred, and must do it after the fact. We have numerical overflow on two cases: we parsed more digits than we theoretically could, or we parsed the same number as the maximum, but the number wrapped. Since the number wrapping will always produce a smaller value than the minimum value for that number of digits, this is a simple comparison.

For unsigned integers, this is quite easy: we merely need to know the maximum number of digits that can be parsed without guaranteeing numerical overflow, and the number of digits that were parsed.

```rust,ignore
// For example, max could be 20 for u64.
let count = ...;    // Actual number of digits parsed.
let max = ...;      // Maximum number of digits that could be parsed.
// Calculate the minimum value from processing `max` digits.
let min_value = 10u64.pow(max as u32 - 1);
// If we've processed more than the max digits, or if the value wrapped,
// we have overflow.
let is_overflow = count > max || (count == max && value < min_value);
```

For signed integers, it's slightly more complicated, but still quite easy:

```rust,ignore
// For example, max could be 18 for i64.
let count = ...;        // Actual number of digits parsed.
let max = ...;          // Maximum number of digits that could be parsed.
let is_negative = ...;  // If the value is less than 0.
// Calculate the minimum value from processing `max` digits.
let min_value = 10u64.pow(max as u32 - 1);
let max_value = i64::MAX as u64 + 1;
let is_overflow = count > max
    || (count == max && (
        value < min_value 
        || value > max_value 
        || (!is_negative && value == max_value)
    ));
```

All of the powers and constant generation is resolved at compile-time, producing efficient routines. For example, for `u64`, the following rust code:

```rust,ignore
pub fn is_overflow(value: u64, count: usize) -> bool {
    let max: usize = 20;
    let min_value = 10u64.pow(max as u32 - 1);
    count > max || (count == max && value < min_value)
}
```

... produces the following assembly:

```asm
example::is_overflow:
        cmp     rsi, 20
        seta    cl
        sete    dl
        movabs  rax, -8446744073709551616
        cmp     rdi, rax
        setb    al
        and     al, dl
        or      al, cl
        ret
```

Not bad at all.

**Compact**

For our compact implementation, prioritizing code size at the cost of performance, we use a naive algorithm that parses 1 digit at a time, without any additional optimizations. This algorithm is trivial to verify, and is effectively analogous to the following code:

```rust,ignore
let mut value = 0;
while let Some(&c) = iter.next() {
    let digit = match (c as char).to_digit(radix) {
        Some(v) => v,
        None => return Err(...),
    };
    value = match value.checked_mul(radix) {
        Some(v) => v,
        None => return Err(...),
    };
    value = match value.checked_add(digit) {
        Some(v) => v,
        None => return Err(...),
    };
}
```
