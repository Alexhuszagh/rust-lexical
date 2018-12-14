//! Precalculated small powers.

// TODO(ahuszagh) These need to use limb.

// PRIME (EXCEPT 2)

/// Small powers (u32) for base3 operations.
const POW3: [u32; 21] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147,  531441, 1594323, 4782969, 14348907, 43046721, 129140163, 387420489, 1162261467, 3486784401];

/// Small powers (u32) for base5 operations.
const POW5: [u32; 14] = [1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625, 1220703125];

/// Small powers (u32) for base7 operations.
const POW7: [u32; 12] = [1, 7, 49, 343, 2401, 16807, 117649, 823543, 5764801, 40353607, 282475249, 1977326743];

/// Small powers (u32) for base11 operations.
const POW11: [u32; 10] = [1, 11, 121, 1331, 14641, 161051, 1771561, 19487171, 214358881, 2357947691];

/// Small powers (u32) for base13 operations.
const POW13: [u32; 9] = [1, 13, 169, 2197, 28561, 371293, 4826809, 62748517, 815730721];

/// Small powers (u32) for base17 operations.
const POW17: [u32; 8] = [1, 17, 289, 4913, 83521, 1419857, 24137569, 410338673];

/// Small powers (u32) for base19 operations.
const POW19: [u32; 8] = [1, 19, 361, 6859, 130321, 2476099, 47045881, 893871739];

/// Small powers (u32) for base23 operations.
const POW23: [u32; 8] = [1, 23, 529, 12167, 279841, 6436343, 148035889, 3404825447];

/// Small powers (u32) for base29 operations.
const POW29: [u32; 7] = [1, 29, 841, 24389, 707281, 20511149, 594823321];

/// Small powers (u32) for base31 operations.
const POW31: [u32; 7] = [1, 31, 961, 29791, 923521, 28629151, 887503681];

// NON-PRIME (AND 2)

/// Small powers (u32) for base2 operations.
const POW2: [u32; 32] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648];

/// Small powers (u32) for base4 operations.
const POW4: [u32; 16] = [1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216, 67108864, 268435456, 1073741824];

/// Small powers (u32) for base6 operations.
const POW6: [u32; 13] = [1, 6, 36, 216, 1296, 7776, 46656, 279936, 1679616, 10077696, 60466176, 362797056, 2176782336];

/// Small powers (u32) for base8 operations.
const POW8: [u32; 11] = [1, 8, 64, 512, 4096, 32768, 262144, 2097152, 16777216, 134217728, 1073741824];

/// Small powers (u32) for base9 operations.
const POW9: [u32; 11] = [1, 9, 81, 729, 6561, 59049, 531441, 4782969, 43046721, 387420489, 3486784401];

/// Small powers (u32) for base10 operations.
const POW10: [u32; 10] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];

/// Small powers (u32) for base12 operations.
const POW12: [u32; 9] = [1, 12, 144, 1728, 20736, 248832, 2985984, 35831808, 429981696];

/// Small powers (u32) for base14 operations.
const POW14: [u32; 9] = [1, 14, 196, 2744, 38416, 537824, 7529536, 105413504, 1475789056];

/// Small powers (u32) for base15 operations.
const POW15: [u32; 9] = [1, 15, 225, 3375, 50625, 759375, 11390625, 170859375, 2562890625];

/// Small powers (u32) for base16 operations.
const POW16: [u32; 8] = [1, 16, 256, 4096, 65536, 1048576, 16777216, 268435456];

/// Small powers (u32) for base18 operations.
const POW18: [u32; 8] = [1, 18, 324, 5832, 104976, 1889568, 34012224, 612220032];

/// Small powers (u32) for base20 operations.
const POW20: [u32; 8] = [1, 20, 400, 8000, 160000, 3200000, 64000000, 1280000000];

/// Small powers (u32) for base21 operations.
const POW21: [u32; 8] = [1, 21, 441, 9261, 194481, 4084101, 85766121, 1801088541];

/// Small powers (u32) for base22 operations.
const POW22: [u32; 8] = [1, 22, 484, 10648, 234256, 5153632, 113379904, 2494357888];

/// Small powers (u32) for base24 operations.
const POW24: [u32; 7] = [1, 24, 576, 13824, 331776, 7962624, 191102976];

/// Small powers (u32) for base25 operations.
const POW25: [u32; 7] = [1, 25, 625, 15625, 390625, 9765625, 244140625];

/// Small powers (u32) for base26 operations.
const POW26: [u32; 7] = [1, 26, 676, 17576, 456976, 11881376, 308915776];

/// Small powers (u32) for base27 operations.
const POW27: [u32; 7] = [1, 27, 729, 19683, 531441, 14348907, 387420489];

/// Small powers (u32) for base28 operations.
const POW28: [u32; 7] = [1, 28, 784, 21952, 614656, 17210368, 481890304];

/// Small powers (u32) for base30 operations.
const POW30: [u32; 7] = [1, 30, 900, 27000, 810000, 24300000, 729000000];

/// Small powers (u32) for base32 operations.
const POW32: [u32; 7] = [1, 32, 1024, 32768, 1048576, 33554432, 1073741824];

/// Small powers (u32) for base33 operations.
const POW33: [u32; 7] = [1, 33, 1089, 35937, 1185921, 39135393, 1291467969];

/// Small powers (u32) for base34 operations.
const POW34: [u32; 7] = [1, 34, 1156, 39304, 1336336, 45435424, 1544804416];

/// Small powers (u32) for base35 operations.
const POW35: [u32; 7] = [1, 35, 1225, 42875, 1500625, 52521875, 1838265625];

/// Small powers (u32) for base36 operations.
const POW36: [u32; 7] = [1, 36, 1296, 46656, 1679616, 60466176, 2176782336];

// ASSERTIONS

// Ensure our small powers are valid.
const_assert!(u32_base2; POW2[1] / POW2[0] == 2);
const_assert!(u32_base3; POW3[1] / POW3[0] == 3);
const_assert!(u32_base4; POW4[1] / POW4[0] == 4);
const_assert!(u32_base5; POW5[1] / POW5[0] == 5);
const_assert!(u32_base6; POW6[1] / POW6[0] == 6);
const_assert!(u32_base7; POW7[1] / POW7[0] == 7);
const_assert!(u32_base8; POW8[1] / POW8[0] == 8);
const_assert!(u32_base9; POW9[1] / POW9[0] == 9);
const_assert!(u32_base10; POW10[1] / POW10[0] == 10);
const_assert!(u32_base11; POW11[1] / POW11[0] == 11);
const_assert!(u32_base12; POW12[1] / POW12[0] == 12);
const_assert!(u32_base13; POW13[1] / POW13[0] == 13);
const_assert!(u32_base14; POW14[1] / POW14[0] == 14);
const_assert!(u32_base15; POW15[1] / POW15[0] == 15);
const_assert!(u32_base16; POW16[1] / POW16[0] == 16);
const_assert!(u32_base17; POW17[1] / POW17[0] == 17);
const_assert!(u32_base18; POW18[1] / POW18[0] == 18);
const_assert!(u32_base19; POW19[1] / POW19[0] == 19);
const_assert!(u32_base20; POW20[1] / POW20[0] == 20);
const_assert!(u32_base21; POW21[1] / POW21[0] == 21);
const_assert!(u32_base22; POW22[1] / POW22[0] == 22);
const_assert!(u32_base23; POW23[1] / POW23[0] == 23);
const_assert!(u32_base24; POW24[1] / POW24[0] == 24);
const_assert!(u32_base25; POW25[1] / POW25[0] == 25);
const_assert!(u32_base26; POW26[1] / POW26[0] == 26);
const_assert!(u32_base27; POW27[1] / POW27[0] == 27);
const_assert!(u32_base28; POW28[1] / POW28[0] == 28);
const_assert!(u32_base29; POW29[1] / POW29[0] == 29);
const_assert!(u32_base30; POW30[1] / POW30[0] == 30);
const_assert!(u32_base31; POW31[1] / POW31[0] == 31);
const_assert!(u32_base32; POW32[1] / POW32[0] == 32);
const_assert!(u32_base33; POW33[1] / POW33[0] == 33);
const_assert!(u32_base34; POW34[1] / POW34[0] == 34);
const_assert!(u32_base35; POW35[1] / POW35[0] == 35);
const_assert!(u32_base36; POW36[1] / POW36[0] == 36);

// HELPER

/// Get the correct small power from the base.
pub(in atof::algorithm) fn get_small_powers(base: u32) -> &'static [u32] {
    match base {
        2  => &POW2,
        3  => &POW3,
        4  => &POW4,
        5  => &POW5,
        6  => &POW6,
        7  => &POW7,
        8  => &POW8,
        9  => &POW9,
        10  => &POW10,
        11  => &POW11,
        12  => &POW12,
        13  => &POW13,
        14  => &POW14,
        15  => &POW15,
        16  => &POW16,
        17  => &POW17,
        18  => &POW18,
        19  => &POW19,
        20  => &POW20,
        21  => &POW21,
        22  => &POW22,
        23  => &POW23,
        24  => &POW24,
        25  => &POW25,
        26  => &POW26,
        27  => &POW27,
        28  => &POW28,
        29  => &POW29,
        30  => &POW30,
        31  => &POW31,
        32  => &POW32,
        33  => &POW33,
        34  => &POW34,
        35  => &POW35,
        36  => &POW36,
        _  => unreachable!(),
    }
}
