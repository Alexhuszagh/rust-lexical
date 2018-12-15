//! Precalculated small powers for 16-bit limbs.

// DECIMAL

/// Small powers (u16) for base5 operations.
pub(super) const POW5: [u16; 7] = [1, 5, 25, 125, 625, 3125, 15625];

/// Small powers (u16) for base10 operations.
pub(super) const POW10: [u16; 5] = [1, 10, 100, 1000, 10000];

cfg_if! {
if #[cfg(feature = "radix")] {
// PRIME (EXCEPT 2)

/// Small powers (u16) for base3 operations.
pub(super) const POW3: [u16; 11] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049];

/// Small powers (u16) for base7 operations.
pub(super) const POW7: [u16; 6] = [1, 7, 49, 343, 2401, 16807];

/// Small powers (u16) for base11 operations.
pub(super) const POW11: [u16; 5] = [1, 11, 121, 1331, 14641];

/// Small powers (u16) for base13 operations.
pub(super) const POW13: [u16; 5] = [1, 13, 169, 2197, 28561];

/// Small powers (u16) for base17 operations.
pub(super) const POW17: [u16; 4] = [1, 17, 289, 4913];

/// Small powers (u16) for base19 operations.
pub(super) const POW19: [u16; 4] = [1, 19, 361, 6859];

/// Small powers (u16) for base23 operations.
pub(super) const POW23: [u16; 4] = [1, 23, 529, 12167];

/// Small powers (u16) for base29 operations.
pub(super) const POW29: [u16; 4] = [1, 29, 841, 24389];

/// Small powers (u16) for base31 operations.
pub(super) const POW31: [u16; 4] = [1, 31, 961, 29791];

// NON-PRIME (AND 2)

/// Small powers (u16) for base2 operations.
pub(super) const POW2: [u16; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768];

/// Small powers (u16) for base4 operations.
pub(super) const POW4: [u16; 8] = [1, 4, 16, 64, 256, 1024, 4096, 16384];

/// Small powers (u16) for base6 operations.
pub(super) const POW6: [u16; 7] = [1, 6, 36, 216, 1296, 7776, 46656];

/// Small powers (u16) for base8 operations.
pub(super) const POW8: [u16; 6] = [1, 8, 64, 512, 4096, 32768];

/// Small powers (u16) for base9 operations.
pub(super) const POW9: [u16; 6] = [1, 9, 81, 729, 6561, 59049];

/// Small powers (u16) for base12 operations.
pub(super) const POW12: [u16; 5] = [1, 12, 144, 1728, 20736];

/// Small powers (u16) for base14 operations.
pub(super) const POW14: [u16; 5] = [1, 14, 196, 2744, 38416];

/// Small powers (u16) for base15 operations.
pub(super) const POW15: [u16; 5] = [1, 15, 225, 3375, 50625];

/// Small powers (u16) for base16 operations.
pub(super) const POW16: [u16; 4] = [1, 16, 256, 4096];

/// Small powers (u16) for base18 operations.
pub(super) const POW18: [u16; 4] = [1, 18, 324, 5832];

/// Small powers (u16) for base20 operations.
pub(super) const POW20: [u16; 4] = [1, 20, 400, 8000];

/// Small powers (u16) for base21 operations.
pub(super) const POW21: [u16; 4] = [1, 21, 441, 9261];

/// Small powers (u16) for base22 operations.
pub(super) const POW22: [u16; 4] = [1, 22, 484, 10648];

/// Small powers (u16) for base24 operations.
pub(super) const POW24: [u16; 4] = [1, 24, 576, 13824];

/// Small powers (u16) for base25 operations.
pub(super) const POW25: [u16; 4] = [1, 25, 625, 15625];

/// Small powers (u16) for base26 operations.
pub(super) const POW26: [u16; 4] = [1, 26, 676, 17576];

/// Small powers (u16) for base27 operations.
pub(super) const POW27: [u16; 4] = [1, 27, 729, 19683];

/// Small powers (u16) for base28 operations.
pub(super) const POW28: [u16; 4] = [1, 28, 784, 21952];

/// Small powers (u16) for base30 operations.
pub(super) const POW30: [u16; 4] = [1, 30, 900, 27000];

/// Small powers (u16) for base32 operations.
pub(super) const POW32: [u16; 4] = [1, 32, 1024, 32768];

/// Small powers (u16) for base33 operations.
pub(super) const POW33: [u16; 4] = [1, 33, 1089, 35937];

/// Small powers (u16) for base34 operations.
pub(super) const POW34: [u16; 4] = [1, 34, 1156, 39304];

/// Small powers (u16) for base35 operations.
pub(super) const POW35: [u16; 4] = [1, 35, 1225, 42875];

/// Small powers (u16) for base36 operations.
pub(super) const POW36: [u16; 4] = [1, 36, 1296, 46656];

}}  // cfg_if
