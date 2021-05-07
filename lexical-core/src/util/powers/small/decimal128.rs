//! Precalculated small powers for decimal 128-bit powers.

// DECIMAL

pub(super) const POW5: [u128; 56] = [
    1,
    5,
    25,
    125,
    625,
    3125,
    15625,
    78125,
    390625,
    1953125,
    9765625,
    48828125,
    244140625,
    1220703125,
    6103515625,
    30517578125,
    152587890625,
    762939453125,
    3814697265625,
    19073486328125,
    95367431640625,
    476837158203125,
    2384185791015625,
    11920928955078125,
    59604644775390625,
    298023223876953125,
    1490116119384765625,
    7450580596923828125,
    37252902984619140625,
    186264514923095703125,
    931322574615478515625,
    4656612873077392578125,
    23283064365386962890625,
    116415321826934814453125,
    582076609134674072265625,
    2910383045673370361328125,
    14551915228366851806640625,
    72759576141834259033203125,
    363797880709171295166015625,
    1818989403545856475830078125,
    9094947017729282379150390625,
    45474735088646411895751953125,
    227373675443232059478759765625,
    1136868377216160297393798828125,
    5684341886080801486968994140625,
    28421709430404007434844970703125,
    142108547152020037174224853515625,
    710542735760100185871124267578125,
    3552713678800500929355621337890625,
    17763568394002504646778106689453125,
    88817841970012523233890533447265625,
    444089209850062616169452667236328125,
    2220446049250313080847263336181640625,
    11102230246251565404236316680908203125,
    55511151231257827021181583404541015625,
    277555756156289135105907917022705078125,
];
pub(super) const POW10: [u128; 39] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
    100000000000000000000,
    1000000000000000000000,
    10000000000000000000000,
    100000000000000000000000,
    1000000000000000000000000,
    10000000000000000000000000,
    100000000000000000000000000,
    1000000000000000000000000000,
    10000000000000000000000000000,
    100000000000000000000000000000,
    1000000000000000000000000000000,
    10000000000000000000000000000000,
    100000000000000000000000000000000,
    1000000000000000000000000000000000,
    10000000000000000000000000000000000,
    100000000000000000000000000000000000,
    1000000000000000000000000000000000000,
    10000000000000000000000000000000000000,
    100000000000000000000000000000000000000,
];