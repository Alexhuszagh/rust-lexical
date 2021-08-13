lexical-asm
===========

Utilities to carefully monitor the assembly generation of lexical's numeric conversion routines. See [scripts/asm.sh](/scripts/asm.sh) for use.

This is especially useful to ensure that the NumberFormat APIs are resolved at compile-time, and therefore any branching or numerical constants are generated. For example, we can see that the following code **is** generated at compile-time:

```rust
/// Parse 8 bytes read from bytes into 8 digits.
/// Credit for this goes to @aqrit, which further optimizes the
/// optimization described by Johnny Lee above.
#[inline]
pub fn parse_8digits<const FORMAT: u128>(mut v: u64) -> u64 {
    let radix = NumberFormat::<{ FORMAT }>::MANTISSA_RADIX as u64;
    debug_assert!(radix <= 10);

    // Create our masks. Assume the optimizer will do this at compile time.
    // It seems like an optimizing compiler **will** do this, so we
    // should be safe.
    let radix2 = radix * radix;
    let radix4 = radix2 * radix2;
    let radix6 = radix2 * radix4;
    let mask = 0x0000_00FF_0000_00FFu64;
    let mul1 = radix2 + (radix6 << 32);
    let mul2 = 1 + (radix4 << 32);

    // Normalize our digits to the base.
    v -= 0x3030_3030_3030_3030;
    // Scale digits in 0 <= Nn <= 99.
    v = (v * radix) + (v >> 8);
    let v1 = (v & mask).wrapping_mul(mul1);
    let v2 = ((v >> 16) & mask).wrapping_mul(mul2);

    ((v1.wrapping_add(v2) >> 32) as u32) as u64
    ...
}
```

Produces the following assembly:

```asm
    movabs  r11, 5063812098665367110    ;;  0x4646464646464646
    movabs  r14, -3472328296227680304   ;; -0x3030303030303030
    movabs  r13, -9187201950435737472   ;;  0x8080808080808080
    movabs  r15, 4294967296000100       ;;  radix2 + (radix6 << 32)
    movabs  r12, 42949672960001         ;;  1 + (radix4 << 32)
.LBB0_56:
    movabs  r14, 1095216660735          ;;  0x0000_00FF_0000_00FF
    lea r8, [rax + 9]
    lea rdx, [r11 + 4*r11]
    shr r11, 8
    lea rdx, [r11 + 2*rdx]
    mov rcx, rdx
    and rcx, r14
    movabs  r11, 4294967296000100       ;;  radix2 + (radix6 << 32)
    imul    rcx, r11
    shr rdx, 16
    and rdx, r14
    movabs  r14, 42949672960001         ;;  1 + (radix4 << 32)
    imul    rdx, r14
    add rdx, rcx
    shr rdx, 32
    imul    rsi, rsi, 100000000
    add rsi, rdx
    mov rcx, r12
    sub rcx, r8
    cmp rcx, 8
    jb  .LBB0_34
    mov rcx, qword ptr [r10 + r8]
    add rbp, rcx
    add r13, rcx
    or  rbp, r13
    movabs  rcx, -9187201950435737472   ;; 0x8080808080808080
```

That is to say, all the constants are appropriately generated at **compile-time**, avoiding any additional overhead.

Likewise, let's check the format checks for parsing floats:

```rust
match byte.integer_iter().peek() {
    Some(&b'+') if !format.no_positive_mantissa_sign() => (false, 1),
    Some(&b'+') if format.no_positive_mantissa_sign() => {
        return Err(Error::InvalidPositiveSign(byte.cursor()));
    },
    Some(&b'-') => (true, 1),
    Some(_) if format.required_mantissa_sign() => {
        return Err(Error::MissingSign(byte.cursor()));
    },
    _ => (false, 0),
}
```

And this compiles down to:

```asm
    mov bl, byte ptr [r10 + r8]
    mov ecx, 1
    cmp bl, 43
    je  .LBB0_59
    mov bpl, 1
    cmp bl, 45
    je  .LBB0_19
.LBB0_19:
    mov qword ptr [rsp + 8], rbp
    add rcx, r8
    xor r11d, r11d
    cmp rcx, r12
    jae .LBB0_23
    .p2align    4, 0x90
.LBB0_59:
    xor ebp, ebp
    jmp .LBB0_19
```

Likewise, for `case_insensitive_starts_with` or `starts_with`, when using the case-insensitive one, we see:

```asm
    lea rdx, [rip + .Lanon.c5e496e177efefe6ca3af6b5e2dec4d8.14]
    .p2align    4, 0x90
.LBB31_9:
    cmp rdi, 3
    je  .LBB31_46
    cmp rsi, rdi
    je  .LBB31_12
    movzx   eax, byte ptr [rbx + rdi]
    xor al, byte ptr [rdi + rdx]
    add rdi, 1
    test    al, -33
    je  .LBB31_9

.Lanon.c5e496e177efefe6ca3af6b5e2dec4d8.14:
    .ascii  "NaN"
```

Which is the code for `case_insensitive_starts_with`, but we don't see `starts_with` anywhere.

In short, the branching seems to be determined at compile time.
