//! Limb sizes for arbitrary-precision arithmetic.

//  Type for a single limb of the big integer.
//
//  A limb is analogous to a digit in base10, except, it stores 32-bit
//  or 64-bit numbers instead.
//
//  This should be all-known 64-bit platforms supported by Rust.
//      https://forge.rust-lang.org/platform-support.html
//
//  Platforms where native 128-bit multiplication is explicitly supported:
//      - x86_64 (Supported via `MUL`).
//      - mips64 (Supported via `DMULTU`, which `HI` and `LO` can be read-from).
//
//  Platforms where native 64-bit multiplication is supported and
//  you can extract hi-lo for 64-bit multiplications.
//      aarch64 (Requires `UMULH` and `MUL` to capture high and low bits).
//      powerpc64 (Requires `MULHDU` and `MULLD` to capture high and low bits).
//
//  Platforms where native 128-bit multiplication is not supported,
//  requiring software emulation.
//      sparc64 (`UMUL` only supported double-word arguments).

cfg_if! {
if #[cfg(limb_width_64)] {
    pub type Limb = u64;
    pub type Wide = u128;
    pub type SignedWide = i128;
} else {
    pub type Limb = u32;
    pub type Wide = u64;
    pub type SignedWide = i64;
}} // cfg_if
