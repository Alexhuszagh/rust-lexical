//! Precalculated small powers for binary 64-bit limbs.

#![cfg(feature = "power_of_two")]

// BINARY

pub(super) const POW2: [u64; 64] = [
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
    8192,
    16384,
    32768,
    65536,
    131072,
    262144,
    524288,
    1048576,
    2097152,
    4194304,
    8388608,
    16777216,
    33554432,
    67108864,
    134217728,
    268435456,
    536870912,
    1073741824,
    2147483648,
    4294967296,
    8589934592,
    17179869184,
    34359738368,
    68719476736,
    137438953472,
    274877906944,
    549755813888,
    1099511627776,
    2199023255552,
    4398046511104,
    8796093022208,
    17592186044416,
    35184372088832,
    70368744177664,
    140737488355328,
    281474976710656,
    562949953421312,
    1125899906842624,
    2251799813685248,
    4503599627370496,
    9007199254740992,
    18014398509481984,
    36028797018963968,
    72057594037927936,
    144115188075855872,
    288230376151711744,
    576460752303423488,
    1152921504606846976,
    2305843009213693952,
    4611686018427387904,
    9223372036854775808,
];
pub(super) const POW4: [u64; 32] = [
    1,
    4,
    16,
    64,
    256,
    1024,
    4096,
    16384,
    65536,
    262144,
    1048576,
    4194304,
    16777216,
    67108864,
    268435456,
    1073741824,
    4294967296,
    17179869184,
    68719476736,
    274877906944,
    1099511627776,
    4398046511104,
    17592186044416,
    70368744177664,
    281474976710656,
    1125899906842624,
    4503599627370496,
    18014398509481984,
    72057594037927936,
    288230376151711744,
    1152921504606846976,
    4611686018427387904,
];
pub(super) const POW8: [u64; 22] = [
    1,
    8,
    64,
    512,
    4096,
    32768,
    262144,
    2097152,
    16777216,
    134217728,
    1073741824,
    8589934592,
    68719476736,
    549755813888,
    4398046511104,
    35184372088832,
    281474976710656,
    2251799813685248,
    18014398509481984,
    144115188075855872,
    1152921504606846976,
    9223372036854775808,
];
pub(super) const POW16: [u64; 16] = [
    1,
    16,
    256,
    4096,
    65536,
    1048576,
    16777216,
    268435456,
    4294967296,
    68719476736,
    1099511627776,
    17592186044416,
    281474976710656,
    4503599627370496,
    72057594037927936,
    1152921504606846976,
];
pub(super) const POW32: [u64; 13] = [
    1,
    32,
    1024,
    32768,
    1048576,
    33554432,
    1073741824,
    34359738368,
    1099511627776,
    35184372088832,
    1125899906842624,
    36028797018963968,
    1152921504606846976,
];