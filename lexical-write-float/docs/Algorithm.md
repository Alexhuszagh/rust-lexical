# Algorithm Approach

Fast float-to-string algorithms use fixed-size integer arithmetic to quickly and accurately print floating-point numbers. Since the development of the [Grisu](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf) algorithm, numerous enhancements such as Ryũ and Schubfach have been developed, which ensure correctness and writing the minimum-length output.

## Dragonbox Algorithm

Dragonbox is a derivative of the [Schubfach](https://drive.google.com/file/d/1KLtG_LaIbK9ETXI290zqCxvBW94dj058/view) algorithm, or a pigeon-hole algorithm, which uses a non-iterative algorithm giving upper and lower boundaries on the floats representation to determine the shortest and correct representation. This differs significantly from Ryũ and Grisu, which use an iterative search within these boundaries.

A detailed description of the algorithm can be found [here](https://raw.githubusercontent.com/jk-jeon/dragonbox/master/other_files/Dragonbox.pdf), however, we make a few notable modifications from the reference implementation:

1. The original algorithm only trims zeros from the representation for 32-bit floats, and uses lookup tables to calculate the number of trailing zeros when iteratively generating digits.

This is undesirable for a few reasons, since this means to digit-printing algorithm still requires the same number of division/remainder operations, the primary bottleneck, but then adds an additional lookup step to determine if we can avoid writing the digits (cheap), which causes significant additional branching.

Meanwhile, the algorithm to trim zeros is cheap, non-iterative, and uses bitmasks to determine if we can remove trailing zeros and multiplication by the modular inverse to truncate the representation, significantly cheaper than iterative division/remainder steps.

This also allows us to use a simple, highly-optimized algorithm rather than heavily nested code, which is ~300 lines of code shorter, contains a single while statement rather than deep nesting and conditional branches, while out-performing the original algorithm.

2. Don't use Granlund-Montgomery fast division.

Compilers optimize division/remainder by a constant efficiently: the result is 2 multiplication instructions (on x86) and with similar efficiency for common architectures (32-bit ARM/THUMB, 64-bit ARM, PowerPC64, MIPS, etc.). In short, the difference in performance is <3%, and has no impact on the overall algorithm efficiency.

3. When truncating the representation, we still use round-nearest, tie-even for the rounding mode. 

In order to determine rounding, Dragonbox calculates boundaries (called endpoints) based on the interval for `w`, or the float we are trying to print. We therefore define that `w-` is the largest, positive float smaller than `w`, and `w+` is the smallest, positive float larger than `w`. For round-nearest rounding algorithms, the interval is bounded by the arithmetic mean (`I = [m−w,m+w]` where `m-w := (w- + w) / 2` and `m+w := (w + w+) / 2`). For round-to-zero, the interval is bounded by the next, larger float (`I = [w,w+)`), which can paradoxically cause unexpected round-up when creating truncated representations. Likewise, the round-to-∞ algorithm is bounded by the previous, smaller float (`I = (w-,w]`), which can paradoxically cause unexpected round-down when creating truncated representations.

The solution is therefore to use round-nearest, tie-even and the truncate the resulting representation.

## Grisu Algorithm

The Grisu algorithm defines a fast, iterative algorithm for printing floating point numbers using fixed-width integer arithmetic. The exact algorithm is described in depth in "Printing Floating-Point Numbers Quickly and Accurately with Integers", by Florian Loitsch, available online [here](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).

Although this algorithm was state-of-the-art in 2010, since then, faster algorithms have been described, most notably, Ryũ and Schubfach. However, Grisu still has one desirable property, in that it uses a very small amount of static storage while having satisfactory overall performance, making it an ideal algorithm when binary size is more important than pure performance.

Our modifications therefore aim to minimize the binary size of the algorithm, so we use a fast algorithm to calculate the binary exponent from the binary exponent, rather than explicitly storing the binary exponents, reducing the static storage by 2x.

Similarly, pre-computed powers-of-ten have been removed, reducing the total, overall storage required from ~1.55KB to ~700B.

## Power-of-2 Algorithms

Algorithms for writing strings that are powers-of-2 are quite easy, since the binary float can always be exactly represented. Similarly, truncating the number of significant digits is easy, since we can only round-up if we are **exactly** halfway or above, since we can never have rounding error. This makes formatting strings in radixes of powers-of-two, including hexadecimal floats, trivial to implement.

## Generic Radix Algorithms

In rare cases, we may want to write non-decimal, non-power-of-two floats. This is non-trivial to do efficiently, since we need to have pre-calculated powers for every radix, ensure we can correctly produce the shortest representation in each case, and guarantee there are minimal errors when doing a full round-trip. This would be a lot of theoretical work, validation, and additional complexity for an esoteric use-case with little real-world application.

A much simpler approach is to use the slow float-writing algorithms using native floats, iteratively producing digits until the remaining value is less than Δ, where `Δ := (w+ - w) / 2`. Although slow, this is a trivial algorithm to implement, which produces the nearest representation in nearly all-cases.

A naive implementation of the algorithm is as follows:

```rust
let mut integer = w.floor();
let mut fraction = w - integer;
let mut buffer = ...;
let mut iter = buffer.iter_mut();
let w_plus = F::from_bits(w.to_bits() + 1)
let delta = (w_plus - w) / 2.0;

while integer > 0 {
    let rem = integer % radix;
    integer = (integer - rem) / radix;
    *iter.next().unwrap() = digit_to_char(rem as u32);
}

*iter.next().unwrap() = b'.';
while fraction > delta {
    fraction *= radix;
    delta *= radix;
    *iter.next().unwrap() = digit_to_char(fraction as u32);
}
```
