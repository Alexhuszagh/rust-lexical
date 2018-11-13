//! Cached exponents for basen values.
//!
//! Exact versions of base**n as an extended-precision float, with both
//! large and small powers. Use the large powers to minimize the amount
//! of compounded error.
//!
//! These values were calculated using Python, using the arbitrary-precision
//! integer to calculate exact extended-representation of each value.
//! These values are all normalized.
//!
//! This files takes ~ 35KB of storage, assuming each array pads
//! each FloatType to 16 bytes (for alignment).
//!
//! This file is mostly automatically generated, do not change values
//! manually, unless you know what you are doing. The script to generate
//! the values is as follows:
//!
//! ```text
//! import math
//! from collections import deque
//!
//! STEP_STR = "const BASE{0}_STEP: i32 = {1};"
//! SMALL_STR = "const BASE{0}_SMALL_POWERS: [FloatType; BASE{0}_STEP as usize] = ["
//! LARGE_STR = "const BASE{0}_LARGE_POWERS: [FloatType; {1}] = ["
//! FP_STR_1 = "    FloatType {{ frac: {}, exp: {} }},"
//! FP_STR_2 = "// {}^{}"
//! BIAS_STR = "const BASE{0}_BIAS: i32 = -BASE{0}_LARGE_POWERS[0].exp;"
//!
//! def calculate_bitshift(base, exponent):
//!     '''
//!     Calculate the bitshift required for a given base. The exponent
//!     is the absolute value of the max exponent (log distance from 1.)
//!     '''
//!
//!     return 63 + math.ceil(math.log2(base**exponent))
//!
//!
//! def next_fp(fp, base, step = 1):
//!     '''Generate the next extended-floating point value.'''
//!
//!     return (fp[0] * (base**step), fp[1])
//!
//!
//! def prev_fp(fp, base, step = 1):
//!     '''Generate the previous extended-floating point value.'''
//!
//!     return (fp[0] // (base**step), fp[1])
//!
//!
//! def normalize_fp(fp):
//!     '''Normalize a extended-float so the MSB is the 64th bit'''
//!
//!     while fp[0] >> 64 != 0:
//!         fp = (fp[0] >> 1, fp[1] + 1)
//!     return fp
//!
//!
//! def print_fp(fp, base, exponent):
//!     '''Print extended-float to console.'''
//!
//!     fp = normalize_fp(fp)
//!     str1 = FP_STR_1.format(fp[0], fp[1])
//!     str2 = FP_STR_2.format(base, exponent)
//!     print(str1.ljust(64, " ") + str2)
//!
//!
//! def generate_small(base, count):
//!     '''Generate the small powers for a given base'''
//!
//!     print(SMALL_STR.format(base))
//!     bitshift = calculate_bitshift(base, count)
//!     fp = (1 << bitshift, -bitshift)
//!     for exp in range(count):
//!         print_fp(fp, base, exp)
//!         fp = next_fp(fp, base)
//!     print("];")
//!
//!
//! def generate_large(base, step):
//!     '''Generate the large powers for a given base.'''
//!
//!     # Get our starting parameters
//!     min_exp = math.floor(math.log(5e-324, base))
//!     max_exp = math.ceil(math.log(1.7976931348623157e+308, base))
//!     bitshift = calculate_bitshift(base, abs(min_exp - step))
//!     fps = deque()
//!
//!     # Add negative exponents
//!     # We need to go below the minimum exponent, since we need
//!     # all resulting exponents to be positive.
//!     fp = (1 << bitshift, -bitshift)
//!     for exp in range(-step, min_exp-step, -step):
//!         fp = prev_fp(fp, base, step)
//!         fps.appendleft((fp, exp))
//!
//!     # Add positive exponents
//!     fp = (1 << bitshift, -bitshift)
//!     fps.append((fp, 0))
//!     for exp in range(step, max_exp, step):
//!         fp = next_fp(fp, base, step)
//!         fps.append((fp, exp))
//!
//!     # Print the values
//!     print(LARGE_STR.format(base, len(fps)))
//!     for fp, exp in fps:
//!         print_fp(fp, base, exp)
//!     print("];")
//!
//!
//! def generate(base):
//!     '''Generate all powers and variables.'''
//!
//!     step = math.floor(math.log(1e10, base))
//!     print(STEP_STR.format(base, step))
//!     generate_small(base, step)
//!     generate_large(base, step)
//!     print(BIAS_STR.format(base))
//! ```

use float::FloatType;

// LOW-LEVEL
// ---------

// BASE3

const BASE3_STEP: i32 = 20;
const BASE3_SMALL_POWERS: [FloatType; BASE3_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 3^0
    FloatType { frac: 13835058055282163712, exp: -62 },         // 3^1
    FloatType { frac: 10376293541461622784, exp: -60 },         // 3^2
    FloatType { frac: 15564440312192434176, exp: -59 },         // 3^3
    FloatType { frac: 11673330234144325632, exp: -57 },         // 3^4
    FloatType { frac: 17509995351216488448, exp: -56 },         // 3^5
    FloatType { frac: 13132496513412366336, exp: -54 },         // 3^6
    FloatType { frac: 9849372385059274752, exp: -52 },          // 3^7
    FloatType { frac: 14774058577588912128, exp: -51 },         // 3^8
    FloatType { frac: 11080543933191684096, exp: -49 },         // 3^9
    FloatType { frac: 16620815899787526144, exp: -48 },         // 3^10
    FloatType { frac: 12465611924840644608, exp: -46 },         // 3^11
    FloatType { frac: 9349208943630483456, exp: -44 },          // 3^12
    FloatType { frac: 14023813415445725184, exp: -43 },         // 3^13
    FloatType { frac: 10517860061584293888, exp: -41 },         // 3^14
    FloatType { frac: 15776790092376440832, exp: -40 },         // 3^15
    FloatType { frac: 11832592569282330624, exp: -38 },         // 3^16
    FloatType { frac: 17748888853923495936, exp: -37 },         // 3^17
    FloatType { frac: 13311666640442621952, exp: -35 },         // 3^18
    FloatType { frac: 9983749980331966464, exp: -33 },          // 3^19
];
const BASE3_LARGE_POWERS: [FloatType; 67] = [
    FloatType { frac: 10783800460320302292, exp: -1141 },       // 3^-680
    FloatType { frac: 17509230984627012859, exp: -1110 },       // 3^-660
    FloatType { frac: 14214523479040558273, exp: -1078 },       // 3^-640
    FloatType { frac: 11539780240125690827, exp: -1046 },       // 3^-620
    FloatType { frac: 9368342750761260524, exp: -1014 },        // 3^-600
    FloatType { frac: 15211008194170796346, exp: -983 },        // 3^-580
    FloatType { frac: 12348756681875770872, exp: -951 },        // 3^-560
    FloatType { frac: 10025094302862174179, exp: -919 },        // 3^-540
    FloatType { frac: 16277349755993950451, exp: -888 },        // 3^-520
    FloatType { frac: 13214445025385558299, exp: -856 },        // 3^-500
    FloatType { frac: 10727886292707736997, exp: -824 },        // 3^-480
    FloatType { frac: 17418445358572088840, exp: -793 },        // 3^-460
    FloatType { frac: 14140820960965941427, exp: -761 },        // 3^-440
    FloatType { frac: 11479946305982273645, exp: -729 },        // 3^-420
    FloatType { frac: 9319767752666157840, exp: -697 },         // 3^-400
    FloatType { frac: 15132138887857638912, exp: -666 },        // 3^-380
    FloatType { frac: 12284728192712064755, exp: -634 },        // 3^-360
    FloatType { frac: 9973114038089604413, exp: -602 },         // 3^-340
    FloatType { frac: 16192951452641260116, exp: -571 },        // 3^-320
    FloatType { frac: 13145927929137795237, exp: -539 },        // 3^-300
    FloatType { frac: 10672262040895386089, exp: -507 },        // 3^-280
    FloatType { frac: 17328130457353990660, exp: -476 },        // 3^-260
    FloatType { frac: 14067500591556283265, exp: -444 },        // 3^-240
    FloatType { frac: 11420422611687500217, exp: -412 },        // 3^-220
    FloatType { frac: 9271444616666914905, exp: -380 },         // 3^-200
    FloatType { frac: 15053678520084183432, exp: -349 },        // 3^-180
    FloatType { frac: 12221031692227883264, exp: -317 },        // 3^-160
    FloatType { frac: 9921403291771844100, exp: -285 },         // 3^-140
    FloatType { frac: 16108990755761097026, exp: -254 },        // 3^-120
    FloatType { frac: 13077766095064811873, exp: -222 },        // 3^-100
    FloatType { frac: 10616926201665464118, exp: -190 },        // 3^-80
    FloatType { frac: 17238283840257358043, exp: -159 },        // 3^-60
    FloatType { frac: 13994560389365007134, exp: -127 },        // 3^-40
    FloatType { frac: 11361207548643088241, exp: -95 },         // 3^-20
    FloatType { frac: 9223372036854775808, exp: -63 },          // 3^0
    FloatType { frac: 14975624970497949696, exp: -32 },         // 3^20
    FloatType { frac: 12157665459056928801, exp: 0 },           // 3^40
    FloatType { frac: 9869960666451650558, exp: 32 },           // 3^60
    FloatType { frac: 16025465396357318008, exp: 63 },          // 3^80
    FloatType { frac: 13009957681126887596, exp: 95 },          // 3^100
    FloatType { frac: 10561877279594392463, exp: 127 },         // 3^120
    FloatType { frac: 17148903079221976570, exp: 158 },         // 3^140
    FloatType { frac: 13921998383219366688, exp: 190 },         // 3^160
    FloatType { frac: 11302299516591361707, exp: 222 },         // 3^180
    FloatType { frac: 18351097428184282358, exp: 253 },         // 3^200
    FloatType { frac: 14897976129740516999, exp: 285 },         // 3^220
    FloatType { frac: 12094627780758213915, exp: 317 },         // 3^240
    FloatType { frac: 9818784771917617934, exp: 349 },          // 3^260
    FloatType { frac: 15942373117198559022, exp: 380 },         // 3^280
    FloatType { frac: 12942500854835305460, exp: 412 },         // 3^300
    FloatType { frac: 10507113787012386253, exp: 444 },         // 3^320
    FloatType { frac: 17059985758777160561, exp: 475 },         // 3^340
    FloatType { frac: 13849812612167175924, exp: 507 },         // 3^360
    FloatType { frac: 11243696923572004730, exp: 539 },         // 3^380
    FloatType { frac: 18255946711954919292, exp: 570 },         // 3^400
    FloatType { frac: 14820729899390519784, exp: 602 },         // 3^420
    FloatType { frac: 12031916953769783440, exp: 634 },         // 3^440
    FloatType { frac: 9767874225166607426, exp: 666 },          // 3^460
    FloatType { frac: 15859711672757234610, exp: 697 },         // 3^480
    FloatType { frac: 12875393793202830082, exp: 729 },         // 3^500
    FloatType { frac: 10452634243963250834, exp: 761 },         // 3^520
    FloatType { frac: 16971529475976476179, exp: 792 },         // 3^540
    FloatType { frac: 13778001125423815423, exp: 824 },         // 3^560
    FloatType { frac: 11185398185879039609, exp: 856 },         // 3^580
    FloatType { frac: 18161289353620602647, exp: 887 },         // 3^600
    FloatType { frac: 14743884191906938838, exp: 919 },         // 3^620
    FloatType { frac: 11969531283362676572, exp: 951 },         // 3^640
];
const BASE3_BIAS: i32 = -BASE3_LARGE_POWERS[0].exp;

// BASE5

const BASE5_STEP: i32 = 14;
const BASE5_SMALL_POWERS: [FloatType; BASE5_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 5^0
    FloatType { frac: 11529215046068469760, exp: -61 },         // 5^1
    FloatType { frac: 14411518807585587200, exp: -59 },         // 5^2
    FloatType { frac: 18014398509481984000, exp: -57 },         // 5^3
    FloatType { frac: 11258999068426240000, exp: -54 },         // 5^4
    FloatType { frac: 14073748835532800000, exp: -52 },         // 5^5
    FloatType { frac: 17592186044416000000, exp: -50 },         // 5^6
    FloatType { frac: 10995116277760000000, exp: -47 },         // 5^7
    FloatType { frac: 13743895347200000000, exp: -45 },         // 5^8
    FloatType { frac: 17179869184000000000, exp: -43 },         // 5^9
    FloatType { frac: 10737418240000000000, exp: -40 },         // 5^10
    FloatType { frac: 13421772800000000000, exp: -38 },         // 5^11
    FloatType { frac: 16777216000000000000, exp: -36 },         // 5^12
    FloatType { frac: 10485760000000000000, exp: -33 },         // 5^13
];
const BASE5_LARGE_POWERS: [FloatType; 66] = [
    FloatType { frac: 15643822052986917253, exp: -1169 },       // 5^-476
    FloatType { frac: 11115604119273511155, exp: -1136 },       // 5^-462
    FloatType { frac: 15796223521069679172, exp: -1104 },       // 5^-448
    FloatType { frac: 11223891875338892399, exp: -1071 },       // 5^-434
    FloatType { frac: 15950109677957715915, exp: -1039 },       // 5^-420
    FloatType { frac: 11333234566249726012, exp: -1006 },       // 5^-406
    FloatType { frac: 16105494987428025427, exp: -974 },        // 5^-392
    FloatType { frac: 11443642469137689536, exp: -941 },        // 5^-378
    FloatType { frac: 16262394054163123565, exp: -909 },        // 5^-364
    FloatType { frac: 11555125961253852697, exp: -876 },        // 5^-350
    FloatType { frac: 16420821625123739831, exp: -844 },        // 5^-336
    FloatType { frac: 11667695520944036383, exp: -811 },        // 5^-322
    FloatType { frac: 16580792590934885855, exp: -779 },        // 5^-308
    FloatType { frac: 11781361728633673532, exp: -746 },        // 5^-294
    FloatType { frac: 16742321987285426889, exp: -714 },        // 5^-280
    FloatType { frac: 11896135267822264502, exp: -681 },        // 5^-266
    FloatType { frac: 16905424996341287883, exp: -649 },        // 5^-252
    FloatType { frac: 12012026926087520367, exp: -616 },        // 5^-238
    FloatType { frac: 17070116948172426941, exp: -584 },        // 5^-224
    FloatType { frac: 12129047596099288555, exp: -551 },        // 5^-210
    FloatType { frac: 17236413322193710308, exp: -519 },        // 5^-196
    FloatType { frac: 12247208276643356092, exp: -486 },        // 5^-182
    FloatType { frac: 17404329748619824289, exp: -454 },        // 5^-168
    FloatType { frac: 12366520073655226703, exp: -421 },        // 5^-154
    FloatType { frac: 17573882009934360870, exp: -389 },        // 5^-140
    FloatType { frac: 12486994201263968925, exp: -356 },        // 5^-126
    FloatType { frac: 17745086042373215101, exp: -324 },        // 5^-112
    FloatType { frac: 12608641982846233347, exp: -291 },        // 5^-98
    FloatType { frac: 17917957937422433684, exp: -259 },        // 5^-84
    FloatType { frac: 12731474852090538039, exp: -226 },        // 5^-70
    FloatType { frac: 18092513943330655534, exp: -194 },        // 5^-56
    FloatType { frac: 12855504354071922204, exp: -161 },        // 5^-42
    FloatType { frac: 18268770466636286477, exp: -129 },        // 5^-28
    FloatType { frac: 12980742146337069071, exp: -96 },         // 5^-14
    FloatType { frac: 9223372036854775808, exp: -63 },          // 5^0
    FloatType { frac: 13107200000000000000, exp: -31 },         // 5^14
    FloatType { frac: 9313225746154785156, exp: 2 },            // 5^28
    FloatType { frac: 13234889800848442797, exp: 34 },          // 5^42
    FloatType { frac: 9403954806578300063, exp: 67 },           // 5^56
    FloatType { frac: 13363823550460978230, exp: 99 },          // 5^70
    FloatType { frac: 9495567745759798747, exp: 132 },          // 5^84
    FloatType { frac: 13494013367335069727, exp: 164 },         // 5^98
    FloatType { frac: 9588073174409622174, exp: 197 },          // 5^112
    FloatType { frac: 13625471488026082303, exp: 229 },         // 5^126
    FloatType { frac: 9681479787123295682, exp: 262 },          // 5^140
    FloatType { frac: 13758210268297397763, exp: 294 },         // 5^154
    FloatType { frac: 9775796363198734982, exp: 327 },          // 5^168
    FloatType { frac: 13892242184281734271, exp: 359 },         // 5^182
    FloatType { frac: 9871031767461413346, exp: 392 },          // 5^196
    FloatType { frac: 14027579833653779454, exp: 424 },         // 5^210
    FloatType { frac: 9967194951097567535, exp: 457 },          // 5^224
    FloatType { frac: 14164235936814247246, exp: 489 },         // 5^238
    FloatType { frac: 10064294952495520794, exp: 522 },         // 5^252
    FloatType { frac: 14302223338085469768, exp: 554 },         // 5^266
    FloatType { frac: 10162340898095201970, exp: 587 },         // 5^280
    FloatType { frac: 14441555006918636608, exp: 619 },         // 5^294
    FloatType { frac: 10261342003245940623, exp: 652 },         // 5^308
    FloatType { frac: 14582244039112794984, exp: 684 },         // 5^322
    FloatType { frac: 10361307573072618726, exp: 717 },         // 5^336
    FloatType { frac: 14724303658045725350, exp: 749 },         // 5^350
    FloatType { frac: 10462247003350260393, exp: 782 },         // 5^364
    FloatType { frac: 14867747215916808149, exp: 814 },         // 5^378
    FloatType { frac: 10564169781387141817, exp: 847 },         // 5^392
    FloatType { frac: 15012588195001998509, exp: 879 },         // 5^406
    FloatType { frac: 10667085486916504429, exp: 912 },         // 5^420
    FloatType { frac: 15158840208921026870, exp: 944 },         // 5^434
];
const BASE5_BIAS: i32 = -BASE5_LARGE_POWERS[0].exp;

// BASE6

const BASE6_STEP: i32 = 12;
const BASE6_SMALL_POWERS: [FloatType; BASE6_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 6^0
    FloatType { frac: 13835058055282163712, exp: -61 },         // 6^1
    FloatType { frac: 10376293541461622784, exp: -58 },         // 6^2
    FloatType { frac: 15564440312192434176, exp: -56 },         // 6^3
    FloatType { frac: 11673330234144325632, exp: -53 },         // 6^4
    FloatType { frac: 17509995351216488448, exp: -51 },         // 6^5
    FloatType { frac: 13132496513412366336, exp: -48 },         // 6^6
    FloatType { frac: 9849372385059274752, exp: -45 },          // 6^7
    FloatType { frac: 14774058577588912128, exp: -43 },         // 6^8
    FloatType { frac: 11080543933191684096, exp: -40 },         // 6^9
    FloatType { frac: 16620815899787526144, exp: -38 },         // 6^10
    FloatType { frac: 12465611924840644608, exp: -35 },         // 6^11
];
const BASE6_LARGE_POWERS: [FloatType; 69] = [
    FloatType { frac: 11479946305982273645, exp: -1149 },       // 6^-420
    FloatType { frac: 11636570252986002899, exp: -1118 },       // 6^-408
    FloatType { frac: 11795331061968106016, exp: -1087 },       // 6^-396
    FloatType { frac: 11956257886702331980, exp: -1056 },       // 6^-384
    FloatType { frac: 12119380278715084095, exp: -1025 },       // 6^-372
    FloatType { frac: 12284728192712064755, exp: -994 },        // 6^-360
    FloatType { frac: 12452331992078957377, exp: -963 },        // 6^-348
    FloatType { frac: 12622222454457155586, exp: -932 },        // 6^-336
    FloatType { frac: 12794430777395563548, exp: -901 },        // 6^-324
    FloatType { frac: 12968988584079505325, exp: -870 },        // 6^-312
    FloatType { frac: 13145927929137795237, exp: -839 },        // 6^-300
    FloatType { frac: 13325281304529035642, exp: -808 },        // 6^-288
    FloatType { frac: 13507081645508223020, exp: -777 },        // 6^-276
    FloatType { frac: 13691362336674758052, exp: -746 },        // 6^-264
    FloatType { frac: 13878157218102970303, exp: -715 },        // 6^-252
    FloatType { frac: 14067500591556283265, exp: -684 },        // 6^-240
    FloatType { frac: 14259427226786160917, exp: -653 },        // 6^-228
    FloatType { frac: 14453972367916992462, exp: -622 },        // 6^-216
    FloatType { frac: 14651171739918087751, exp: -591 },        // 6^-204
    FloatType { frac: 14851061555163971849, exp: -560 },        // 6^-192
    FloatType { frac: 15053678520084183432, exp: -529 },        // 6^-180
    FloatType { frac: 15259059841903798156, exp: -498 },        // 6^-168
    FloatType { frac: 15467243235475914756, exp: -467 },        // 6^-156
    FloatType { frac: 15678266930207358578, exp: -436 },        // 6^-144
    FloatType { frac: 15892169677078874302, exp: -405 },        // 6^-132
    FloatType { frac: 16108990755761097026, exp: -374 },        // 6^-120
    FloatType { frac: 16328769981827608423, exp: -343 },        // 6^-108
    FloatType { frac: 16551547714066402526, exp: -312 },        // 6^-96
    FloatType { frac: 16777364861891103792, exp: -281 },        // 6^-84
    FloatType { frac: 17006262892853298360, exp: -250 },        // 6^-72
    FloatType { frac: 17238283840257358043, exp: -219 },        // 6^-60
    FloatType { frac: 17473470310879155380, exp: -188 },        // 6^-48
    FloatType { frac: 17711865492790087155, exp: -157 },        // 6^-36
    FloatType { frac: 17953513163287843146, exp: -126 },        // 6^-24
    FloatType { frac: 18198457696935376453, exp: -95 },         // 6^-12
    FloatType { frac: 9223372036854775808, exp: -63 },          // 6^0
    FloatType { frac: 9349208943630483456, exp: -32 },          // 6^12
    FloatType { frac: 9476762676643233792, exp: -1 },           // 6^24
    FloatType { frac: 9606056659007943744, exp: 30 },           // 6^36
    FloatType { frac: 9737114633407288801, exp: 61 },           // 6^48
    FloatType { frac: 9869960666451650558, exp: 92 },           // 6^60
    FloatType { frac: 10004619153098548172, exp: 123 },         // 6^72
    FloatType { frac: 10141114821132365302, exp: 154 },         // 6^84
    FloatType { frac: 10279472735705195138, exp: 185 },         // 6^96
    FloatType { frac: 10419718303939637392, exp: 216 },         // 6^108
    FloatType { frac: 10561877279594392463, exp: 247 },         // 6^120
    FloatType { frac: 10705975767793509530, exp: 278 },         // 6^132
    FloatType { frac: 10852040229820157048, exp: 309 },         // 6^144
    FloatType { frac: 11000097487975795902, exp: 340 },         // 6^156
    FloatType { frac: 11150174730505647564, exp: 371 },         // 6^168
    FloatType { frac: 11302299516591361707, exp: 402 },         // 6^180
    FloatType { frac: 11456499781411800112, exp: 433 },         // 6^192
    FloatType { frac: 11612803841272866179, exp: 464 },         // 6^204
    FloatType { frac: 11771240398807322073, exp: 495 },         // 6^216
    FloatType { frac: 11931838548245548344, exp: 526 },         // 6^228
    FloatType { frac: 12094627780758213915, exp: 557 },         // 6^240
    FloatType { frac: 12259637989871837542, exp: 588 },         // 6^252
    FloatType { frac: 12426899476958235198, exp: 619 },         // 6^264
    FloatType { frac: 12596442956798861450, exp: 650 },         // 6^276
    FloatType { frac: 12768299563225066619, exp: 681 },         // 6^288
    FloatType { frac: 12942500854835305460, exp: 712 },         // 6^300
    FloatType { frac: 13119078820790347231, exp: 743 },         // 6^312
    FloatType { frac: 13298065886687551351, exp: 774 },         // 6^324
    FloatType { frac: 13479494920515287357, exp: 805 },         // 6^336
    FloatType { frac: 13663399238688592583, exp: 836 },         // 6^348
    FloatType { frac: 13849812612167175924, exp: 867 },         // 6^360
    FloatType { frac: 14038769272656891137, exp: 898 },         // 6^372
    FloatType { frac: 14230303918895818486, exp: 929 },         // 6^384
    FloatType { frac: 14424451723026109070, exp: 960 },         // 6^396
];
const BASE6_BIAS: i32 = -BASE6_LARGE_POWERS[0].exp;

// BASE7

const BASE7_STEP: i32 = 11;
const BASE7_SMALL_POWERS: [FloatType; BASE7_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 7^0
    FloatType { frac: 16140901064495857664, exp: -61 },         // 7^1
    FloatType { frac: 14123288431433875456, exp: -58 },         // 7^2
    FloatType { frac: 12357877377504641024, exp: -55 },         // 7^3
    FloatType { frac: 10813142705316560896, exp: -52 },         // 7^4
    FloatType { frac: 9461499867151990784, exp: -49 },          // 7^5
    FloatType { frac: 16557624767515983872, exp: -47 },         // 7^6
    FloatType { frac: 14487921671576485888, exp: -44 },         // 7^7
    FloatType { frac: 12676931462629425152, exp: -41 },         // 7^8
    FloatType { frac: 11092315029800747008, exp: -38 },         // 7^9
    FloatType { frac: 9705775651075653632, exp: -35 },          // 7^10
];
const BASE7_LARGE_POWERS: [FloatType; 69] = [
    FloatType { frac: 10365007820408367996, exp: -1144 },       // 7^-385
    FloatType { frac: 9543731415037814164, exp: -1113 },        // 7^-374
    FloatType { frac: 17575058485347314089, exp: -1083 },       // 7^-363
    FloatType { frac: 16182490230010039076, exp: -1052 },       // 7^-352
    FloatType { frac: 14900262793588950961, exp: -1021 },       // 7^-341
    FloatType { frac: 13719633267955538670, exp: -990 },        // 7^-330
    FloatType { frac: 12632551493533408059, exp: -959 },        // 7^-319
    FloatType { frac: 11631605169031861852, exp: -928 },        // 7^-308
    FloatType { frac: 10709969310436274791, exp: -897 },        // 7^-297
    FloatType { frac: 9861359714639799269, exp: -866 },         // 7^-286
    FloatType { frac: 18159980220813419398, exp: -836 },        // 7^-275
    FloatType { frac: 16721065408999761282, exp: -805 },        // 7^-264
    FloatType { frac: 15396163707909854531, exp: -774 },        // 7^-253
    FloatType { frac: 14176241233598532153, exp: -743 },        // 7^-242
    FloatType { frac: 13052979906282242272, exp: -712 },        // 7^-231
    FloatType { frac: 12018720733250263776, exp: -681 },        // 7^-220
    FloatType { frac: 11066411585781870352, exp: -650 },        // 7^-209
    FloatType { frac: 10189559113984709052, exp: -619 },        // 7^-198
    FloatType { frac: 9382184471684205580, exp: -588 },         // 7^-187
    FloatType { frac: 17277565098945522629, exp: -558 },        // 7^-176
    FloatType { frac: 15908568875896010079, exp: -527 },        // 7^-165
    FloatType { frac: 14648045730389016129, exp: -496 },        // 7^-154
    FloatType { frac: 13487400745686688174, exp: -465 },        // 7^-143
    FloatType { frac: 12418720027433908743, exp: -434 },        // 7^-132
    FloatType { frac: 11434716742520575143, exp: -403 },        // 7^-121
    FloatType { frac: 10528681433580712628, exp: -372 },        // 7^-110
    FloatType { frac: 9694436270346269630, exp: -341 },         // 7^-99
    FloatType { frac: 17852585851834022264, exp: -311 },        // 7^-88
    FloatType { frac: 16438027581449061548, exp: -280 },        // 7^-77
    FloatType { frac: 15135552519453149331, exp: -249 },        // 7^-66
    FloatType { frac: 13936279698645574929, exp: -218 },        // 7^-55
    FloatType { frac: 12832031839555071753, exp: -187 },        // 7^-44
    FloatType { frac: 11815279593402393441, exp: -156 },        // 7^-33
    FloatType { frac: 10879090202998704701, exp: -125 },        // 7^-22
    FloatType { frac: 10017080231522506848, exp: -94 },         // 7^-11
    FloatType { frac: 9223372036854775808, exp: -63 },          // 7^0
    FloatType { frac: 16985107389382393856, exp: -33 },         // 7^11
    FloatType { frac: 15639284194331952196, exp: -2 },          // 7^22
    FloatType { frac: 14400097950748064600, exp: 29 },          // 7^33
    FloatType { frac: 13259099228230139701, exp: 60 },          // 7^44
    FloatType { frac: 12208508091080056405, exp: 91 },          // 7^55
    FloatType { frac: 11241161050565762112, exp: 122 },         // 7^66
    FloatType { frac: 10350462220447909415, exp: 153 },         // 7^77
    FloatType { frac: 9530338342721952463, exp: 184 },          // 7^88
    FloatType { frac: 17550394753834620135, exp: 214 },         // 7^99
    FloatType { frac: 16159780741186857313, exp: 245 },         // 7^110
    FloatType { frac: 14879352702091044991, exp: 276 },         // 7^121
    FloatType { frac: 13700379997665963732, exp: 307 },         // 7^132
    FloatType { frac: 12614823765422770599, exp: 338 },         // 7^143
    FloatType { frac: 11615282106028126090, exp: 369 },         // 7^154
    FloatType { frac: 10694939613220642893, exp: 400 },         // 7^165
    FloatType { frac: 9847520902748803399, exp: 431 },          // 7^176
    FloatType { frac: 18134495646931893353, exp: 461 },         // 7^187
    FloatType { frac: 16697600117649658875, exp: 492 },         // 7^198
    FloatType { frac: 15374557700263623520, exp: 523 },         // 7^209
    FloatType { frac: 14156347188413069088, exp: 554 },         // 7^220
    FloatType { frac: 13034662175384360011, exp: 585 },         // 7^231
    FloatType { frac: 12001854416615353596, exp: 616 },         // 7^242
    FloatType { frac: 11050881679899153397, exp: 647 },         // 7^253
    FloatType { frac: 10175259727702178785, exp: 678 },         // 7^264
    FloatType { frac: 9369018104186475301, exp: 709 },          // 7^275
    FloatType { frac: 17253318850937371954, exp: 739 },         // 7^286
    FloatType { frac: 15886243791070066478, exp: 770 },         // 7^297
    FloatType { frac: 14627489584451796037, exp: 801 },         // 7^308
    FloatType { frac: 13468473375910191470, exp: 832 },         // 7^319
    FloatType { frac: 12401292376951646786, exp: 863 },         // 7^330
    FloatType { frac: 11418669980349265042, exp: 894 },         // 7^341
    FloatType { frac: 10513906144367477972, exp: 925 },         // 7^352
    FloatType { frac: 9680831708316613461, exp: 956 },          // 7^363
];
const BASE7_BIAS: i32 = -BASE7_LARGE_POWERS[0].exp;

// BASE9

const BASE9_STEP: i32 = 10;
const BASE9_SMALL_POWERS: [FloatType; BASE9_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 9^0
    FloatType { frac: 10376293541461622784, exp: -60 },         // 9^1
    FloatType { frac: 11673330234144325632, exp: -57 },         // 9^2
    FloatType { frac: 13132496513412366336, exp: -54 },         // 9^3
    FloatType { frac: 14774058577588912128, exp: -51 },         // 9^4
    FloatType { frac: 16620815899787526144, exp: -48 },         // 9^5
    FloatType { frac: 9349208943630483456, exp: -44 },          // 9^6
    FloatType { frac: 10517860061584293888, exp: -41 },         // 9^7
    FloatType { frac: 11832592569282330624, exp: -38 },         // 9^8
    FloatType { frac: 13311666640442621952, exp: -35 },         // 9^9
];
const BASE9_LARGE_POWERS: [FloatType; 67] = [
    FloatType { frac: 10783800460320302292, exp: -1141 },       // 9^-340
    FloatType { frac: 17509230984627012859, exp: -1110 },       // 9^-330
    FloatType { frac: 14214523479040558273, exp: -1078 },       // 9^-320
    FloatType { frac: 11539780240125690827, exp: -1046 },       // 9^-310
    FloatType { frac: 9368342750761260524, exp: -1014 },        // 9^-300
    FloatType { frac: 15211008194170796346, exp: -983 },        // 9^-290
    FloatType { frac: 12348756681875770872, exp: -951 },        // 9^-280
    FloatType { frac: 10025094302862174179, exp: -919 },        // 9^-270
    FloatType { frac: 16277349755993950451, exp: -888 },        // 9^-260
    FloatType { frac: 13214445025385558299, exp: -856 },        // 9^-250
    FloatType { frac: 10727886292707736997, exp: -824 },        // 9^-240
    FloatType { frac: 17418445358572088840, exp: -793 },        // 9^-230
    FloatType { frac: 14140820960965941427, exp: -761 },        // 9^-220
    FloatType { frac: 11479946305982273645, exp: -729 },        // 9^-210
    FloatType { frac: 9319767752666157840, exp: -697 },         // 9^-200
    FloatType { frac: 15132138887857638912, exp: -666 },        // 9^-190
    FloatType { frac: 12284728192712064755, exp: -634 },        // 9^-180
    FloatType { frac: 9973114038089604413, exp: -602 },         // 9^-170
    FloatType { frac: 16192951452641260116, exp: -571 },        // 9^-160
    FloatType { frac: 13145927929137795237, exp: -539 },        // 9^-150
    FloatType { frac: 10672262040895386089, exp: -507 },        // 9^-140
    FloatType { frac: 17328130457353990660, exp: -476 },        // 9^-130
    FloatType { frac: 14067500591556283265, exp: -444 },        // 9^-120
    FloatType { frac: 11420422611687500217, exp: -412 },        // 9^-110
    FloatType { frac: 9271444616666914905, exp: -380 },         // 9^-100
    FloatType { frac: 15053678520084183432, exp: -349 },        // 9^-90
    FloatType { frac: 12221031692227883264, exp: -317 },        // 9^-80
    FloatType { frac: 9921403291771844100, exp: -285 },         // 9^-70
    FloatType { frac: 16108990755761097026, exp: -254 },        // 9^-60
    FloatType { frac: 13077766095064811873, exp: -222 },        // 9^-50
    FloatType { frac: 10616926201665464118, exp: -190 },        // 9^-40
    FloatType { frac: 17238283840257358043, exp: -159 },        // 9^-30
    FloatType { frac: 13994560389365007134, exp: -127 },        // 9^-20
    FloatType { frac: 11361207548643088241, exp: -95 },         // 9^-10
    FloatType { frac: 9223372036854775808, exp: -63 },          // 9^0
    FloatType { frac: 14975624970497949696, exp: -32 },         // 9^10
    FloatType { frac: 12157665459056928801, exp: 0 },           // 9^20
    FloatType { frac: 9869960666451650558, exp: 32 },           // 9^30
    FloatType { frac: 16025465396357318008, exp: 63 },          // 9^40
    FloatType { frac: 13009957681126887596, exp: 95 },          // 9^50
    FloatType { frac: 10561877279594392463, exp: 127 },         // 9^60
    FloatType { frac: 17148903079221976570, exp: 158 },         // 9^70
    FloatType { frac: 13921998383219366688, exp: 190 },         // 9^80
    FloatType { frac: 11302299516591361707, exp: 222 },         // 9^90
    FloatType { frac: 18351097428184282358, exp: 253 },         // 9^100
    FloatType { frac: 14897976129740516999, exp: 285 },         // 9^110
    FloatType { frac: 12094627780758213915, exp: 317 },         // 9^120
    FloatType { frac: 9818784771917617934, exp: 349 },          // 9^130
    FloatType { frac: 15942373117198559022, exp: 380 },         // 9^140
    FloatType { frac: 12942500854835305460, exp: 412 },         // 9^150
    FloatType { frac: 10507113787012386253, exp: 444 },         // 9^160
    FloatType { frac: 17059985758777160561, exp: 475 },         // 9^170
    FloatType { frac: 13849812612167175924, exp: 507 },         // 9^180
    FloatType { frac: 11243696923572004730, exp: 539 },         // 9^190
    FloatType { frac: 18255946711954919292, exp: 570 },         // 9^200
    FloatType { frac: 14820729899390519784, exp: 602 },         // 9^210
    FloatType { frac: 12031916953769783440, exp: 634 },         // 9^220
    FloatType { frac: 9767874225166607426, exp: 666 },          // 9^230
    FloatType { frac: 15859711672757234610, exp: 697 },         // 9^240
    FloatType { frac: 12875393793202830082, exp: 729 },         // 9^250
    FloatType { frac: 10452634243963250834, exp: 761 },         // 9^260
    FloatType { frac: 16971529475976476179, exp: 792 },         // 9^270
    FloatType { frac: 13778001125423815423, exp: 824 },         // 9^280
    FloatType { frac: 11185398185879039609, exp: 856 },         // 9^290
    FloatType { frac: 18161289353620602647, exp: 887 },         // 9^300
    FloatType { frac: 14743884191906938838, exp: 919 },         // 9^310
    FloatType { frac: 11969531283362676572, exp: 951 },         // 9^320
];
const BASE9_BIAS: i32 = -BASE9_LARGE_POWERS[0].exp;

// BASE10

const BASE10_STEP: i32 = 10;
const BASE10_SMALL_POWERS: [FloatType; BASE10_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 10^0
    FloatType { frac: 11529215046068469760, exp: -60 },         // 10^1
    FloatType { frac: 14411518807585587200, exp: -57 },         // 10^2
    FloatType { frac: 18014398509481984000, exp: -54 },         // 10^3
    FloatType { frac: 11258999068426240000, exp: -50 },         // 10^4
    FloatType { frac: 14073748835532800000, exp: -47 },         // 10^5
    FloatType { frac: 17592186044416000000, exp: -44 },         // 10^6
    FloatType { frac: 10995116277760000000, exp: -40 },         // 10^7
    FloatType { frac: 13743895347200000000, exp: -37 },         // 10^8
    FloatType { frac: 17179869184000000000, exp: -34 },         // 10^9
];
const BASE10_LARGE_POWERS: [FloatType; 64] = [
    FloatType { frac: 15660115838168849784, exp: -1160 },       // 10^-330
    FloatType { frac: 18230774251475056848, exp: -1127 },       // 10^-320
    FloatType { frac: 10611707258198326947, exp: -1093 },       // 10^-310
    FloatType { frac: 12353653155963782858, exp: -1060 },       // 10^-300
    FloatType { frac: 14381545078898527261, exp: -1027 },       // 10^-290
    FloatType { frac: 16742321987285426889, exp: -994 },        // 10^-280
    FloatType { frac: 9745314011399999080, exp: -960 },         // 10^-270
    FloatType { frac: 11345038669416679861, exp: -927 },        // 10^-260
    FloatType { frac: 13207363278391631158, exp: -894 },        // 10^-250
    FloatType { frac: 15375394465392026070, exp: -861 },        // 10^-240
    FloatType { frac: 17899314949046850752, exp: -828 },        // 10^-230
    FloatType { frac: 10418772551374772303, exp: -794 },        // 10^-220
    FloatType { frac: 12129047596099288555, exp: -761 },        // 10^-210
    FloatType { frac: 14120069793541087484, exp: -728 },        // 10^-200
    FloatType { frac: 16437924692338667210, exp: -695 },        // 10^-190
    FloatType { frac: 9568131466127621947, exp: -661 },         // 10^-180
    FloatType { frac: 11138771039116687545, exp: -628 },        // 10^-170
    FloatType { frac: 12967236152753102995, exp: -595 },        // 10^-160
    FloatType { frac: 15095849699286165408, exp: -562 },        // 10^-150
    FloatType { frac: 17573882009934360870, exp: -529 },        // 10^-140
    FloatType { frac: 10229345649675443343, exp: -495 },        // 10^-130
    FloatType { frac: 11908525658859223294, exp: -462 },        // 10^-120
    FloatType { frac: 13863348470604074297, exp: -429 },        // 10^-110
    FloatType { frac: 16139061738043178685, exp: -396 },        // 10^-100
    FloatType { frac: 9394170331095332911, exp: -362 },         // 10^-90
    FloatType { frac: 10936253623915059621, exp: -329 },        // 10^-80
    FloatType { frac: 12731474852090538039, exp: -296 },        // 10^-70
    FloatType { frac: 14821387422376473014, exp: -263 },        // 10^-60
    FloatType { frac: 17254365866976409468, exp: -230 },        // 10^-50
    FloatType { frac: 10043362776618689222, exp: -196 },        // 10^-40
    FloatType { frac: 11692013098647223345, exp: -163 },        // 10^-30
    FloatType { frac: 13611294676837538538, exp: -130 },        // 10^-20
    FloatType { frac: 15845632502852867518, exp: -97 },         // 10^-10
    FloatType { frac: 9223372036854775808, exp: -63 },          // 10^0
    FloatType { frac: 10737418240000000000, exp: -30 },         // 10^10
    FloatType { frac: 12500000000000000000, exp: 3 },           // 10^20
    FloatType { frac: 14551915228366851806, exp: 36 },          // 10^30
    FloatType { frac: 16940658945086006781, exp: 69 },          // 10^40
    FloatType { frac: 9860761315262647567, exp: 103 },          // 10^50
    FloatType { frac: 11479437019748901445, exp: 136 },         // 10^60
    FloatType { frac: 13363823550460978230, exp: 169 },         // 10^70
    FloatType { frac: 15557538194652854267, exp: 202 },         // 10^80
    FloatType { frac: 18111358157653424735, exp: 235 },         // 10^90
    FloatType { frac: 10542197943230523224, exp: 269 },         // 10^100
    FloatType { frac: 12272733663244316382, exp: 302 },         // 10^110
    FloatType { frac: 14287342391028437277, exp: 335 },         // 10^120
    FloatType { frac: 16632655625031838749, exp: 368 },         // 10^130
    FloatType { frac: 9681479787123295682, exp: 402 },          // 10^140
    FloatType { frac: 11270725851789228247, exp: 435 },         // 10^150
    FloatType { frac: 13120851772591970218, exp: 468 },         // 10^160
    FloatType { frac: 15274681817498023410, exp: 501 },         // 10^170
    FloatType { frac: 17782069995880619867, exp: 534 },         // 10^180
    FloatType { frac: 10350527006597618960, exp: 568 },         // 10^190
    FloatType { frac: 12049599325514420588, exp: 601 },         // 10^200
    FloatType { frac: 14027579833653779454, exp: 634 },         // 10^210
    FloatType { frac: 16330252207878254650, exp: 667 },         // 10^220
    FloatType { frac: 9505457831475799117, exp: 701 },          // 10^230
    FloatType { frac: 11065809325636130661, exp: 734 },         // 10^240
    FloatType { frac: 12882297539194266616, exp: 767 },         // 10^250
    FloatType { frac: 14996968138956309548, exp: 800 },         // 10^260
    FloatType { frac: 17458768723248864463, exp: 833 },         // 10^270
    FloatType { frac: 10162340898095201970, exp: 867 },         // 10^280
    FloatType { frac: 11830521861667747109, exp: 900 },         // 10^290
    FloatType { frac: 13772540099066387756, exp: 933 },         // 10^300
];
const BASE10_BIAS: i32 = -BASE10_LARGE_POWERS[0].exp;

// BASE11

const BASE11_STEP: i32 = 9;
const BASE11_SMALL_POWERS: [FloatType; BASE11_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 11^0
    FloatType { frac: 12682136550675316736, exp: -60 },         // 11^1
    FloatType { frac: 17437937757178560512, exp: -57 },         // 11^2
    FloatType { frac: 11988582208060260352, exp: -53 },         // 11^3
    FloatType { frac: 16484300536082857984, exp: -50 },         // 11^4
    FloatType { frac: 11332956618556964864, exp: -46 },         // 11^5
    FloatType { frac: 15582815350515826688, exp: -43 },         // 11^6
    FloatType { frac: 10713185553479630848, exp: -39 },         // 11^7
    FloatType { frac: 14730630136034492416, exp: -36 },         // 11^8
];
const BASE11_LARGE_POWERS: [FloatType; 68] = [
    FloatType { frac: 11191522591630754840, exp: -1153 },       // 11^-315
    FloatType { frac: 12288347284174558846, exp: -1122 },       // 11^-306
    FloatType { frac: 13492666233761944748, exp: -1091 },       // 11^-297
    FloatType { frac: 14815014410453217040, exp: -1060 },       // 11^-288
    FloatType { frac: 16266959263598494876, exp: -1029 },       // 11^-279
    FloatType { frac: 17861201909926315464, exp: -998 },        // 11^-270
    FloatType { frac: 9805844119283264859, exp: -966 },         // 11^-261
    FloatType { frac: 10766865452458105492, exp: -935 },        // 11^-252
    FloatType { frac: 11822071640254585128, exp: -904 },        // 11^-243
    FloatType { frac: 12980693265318349774, exp: -873 },        // 11^-234
    FloatType { frac: 14252865553152120313, exp: -842 },        // 11^-225
    FloatType { frac: 15649717031600177225, exp: -811 },        // 11^-216
    FloatType { frac: 17183466879401827195, exp: -780 },        // 11^-207
    FloatType { frac: 9433765907692842627, exp: -748 },         // 11^-198
    FloatType { frac: 10358321731667433590, exp: -717 },        // 11^-189
    FloatType { frac: 11373488609595385666, exp: -686 },        // 11^-180
    FloatType { frac: 12488146873940825498, exp: -655 },        // 11^-171
    FloatType { frac: 13712047173770907127, exp: -624 },        // 11^-162
    FloatType { frac: 15055895770097238115, exp: -593 },        // 11^-153
    FloatType { frac: 16531448190583591098, exp: -562 },        // 11^-144
    FloatType { frac: 18151612062879235750, exp: -531 },        // 11^-135
    FloatType { frac: 9965280013064351107, exp: -499 },         // 11^-126
    FloatType { frac: 10941926854184612877, exp: -468 },        // 11^-117
    FloatType { frac: 12014289927163860584, exp: -437 },        // 11^-108
    FloatType { frac: 13191749897208336388, exp: -406 },        // 11^-99
    FloatType { frac: 14484606781216284322, exp: -375 },        // 11^-90
    FloatType { frac: 15904170048801172427, exp: -344 },        // 11^-81
    FloatType { frac: 17462857553661839031, exp: -313 },        // 11^-72
    FloatType { frac: 9587152080358667750, exp: -281 },         // 11^-63
    FloatType { frac: 10526740509619734750, exp: -250 },        // 11^-54
    FloatType { frac: 11558413262671798860, exp: -219 },        // 11^-45
    FloatType { frac: 12691195059726361470, exp: -188 },        // 11^-36
    FloatType { frac: 13934995088312952370, exp: -157 },        // 11^-27
    FloatType { frac: 15300693685460773821, exp: -126 },        // 11^-18
    FloatType { frac: 16800237515163846269, exp: -95 },         // 11^-9
    FloatType { frac: 9223372036854775808, exp: -63 },          // 11^0
    FloatType { frac: 10127308218523713536, exp: -32 },         // 11^9
    FloatType { frac: 11119834626984462962, exp: -1 },          // 11^18
    FloatType { frac: 12209633543621683835, exp: 30 },          // 11^27
    FloatType { frac: 13406238156435497652, exp: 61 },          // 11^36
    FloatType { frac: 14720115953107913248, exp: 92 },          // 11^45
    FloatType { frac: 16162760287003157808, exp: 123 },         // 11^54
    FloatType { frac: 17746790917089950882, exp: 154 },         // 11^63
    FloatType { frac: 9743032200637278641, exp: 186 },          // 11^72
    FloatType { frac: 10697897654413860244, exp: 217 },         // 11^81
    FloatType { frac: 11746344656115154606, exp: 248 },         // 11^90
    FloatType { frac: 12897544661339799796, exp: 279 },         // 11^99
    FloatType { frac: 14161567973799797658, exp: 310 },         // 11^108
    FloatType { frac: 15549471836891389165, exp: 341 },         // 11^117
    FloatType { frac: 17073397158676562691, exp: 372 },         // 11^126
    FloatType { frac: 9373337358196117359, exp: 404 },          // 11^135
    FloatType { frac: 10291970884763903381, exp: 435 },         // 11^144
    FloatType { frac: 11300635050781198339, exp: 466 },         // 11^153
    FloatType { frac: 12408153305213523269, exp: 497 },         // 11^162
    FloatType { frac: 13624213838764580644, exp: 528 },         // 11^171
    FloatType { frac: 14959454332853289890, exp: 559 },         // 11^180
    FloatType { frac: 16425555014410689631, exp: 590 },         // 11^189
    FloatType { frac: 18035340830508227153, exp: 621 },         // 11^198
    FloatType { frac: 9901446818303059920, exp: 653 },          // 11^207
    FloatType { frac: 10871837689903097542, exp: 684 },         // 11^216
    FloatType { frac: 11937331677337075986, exp: 715 },         // 11^225
    FloatType { frac: 13107249357401447067, exp: 746 },         // 11^234
    FloatType { frac: 14391824769622635037, exp: 777 },         // 11^243
    FloatType { frac: 15802294940132787091, exp: 808 },         // 11^252
    FloatType { frac: 17350998179329134782, exp: 839 },         // 11^261
    FloatType { frac: 9525741006595626773, exp: 871 },          // 11^270
    FloatType { frac: 10459310846201225147, exp: 902 },         // 11^279
    FloatType { frac: 11484375157976259923, exp: 933 },         // 11^288
];
const BASE11_BIAS: i32 = -BASE11_LARGE_POWERS[0].exp;

// BASE12

const BASE12_STEP: i32 = 9;
const BASE12_SMALL_POWERS: [FloatType; BASE12_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 12^0
    FloatType { frac: 13835058055282163712, exp: -60 },         // 12^1
    FloatType { frac: 10376293541461622784, exp: -56 },         // 12^2
    FloatType { frac: 15564440312192434176, exp: -53 },         // 12^3
    FloatType { frac: 11673330234144325632, exp: -49 },         // 12^4
    FloatType { frac: 17509995351216488448, exp: -46 },         // 12^5
    FloatType { frac: 13132496513412366336, exp: -42 },         // 12^6
    FloatType { frac: 9849372385059274752, exp: -38 },          // 12^7
    FloatType { frac: 14774058577588912128, exp: -35 },         // 12^8
];
const BASE12_LARGE_POWERS: [FloatType; 66] = [
    FloatType { frac: 9232805349408163458, exp: -1160 },        // 12^-306
    FloatType { frac: 11091876690210014731, exp: -1128 },       // 12^-297
    FloatType { frac: 13325281304529035642, exp: -1096 },       // 12^-288
    FloatType { frac: 16008393061343079134, exp: -1064 },       // 12^-279
    FloatType { frac: 9615881366772943927, exp: -1031 },        // 12^-270
    FloatType { frac: 11552086971569327107, exp: -999 },        // 12^-261
    FloatType { frac: 13878157218102970303, exp: -967 },        // 12^-252
    FloatType { frac: 16672593293696335722, exp: -935 },        // 12^-243
    FloatType { frac: 10014851495355986817, exp: -902 },        // 12^-234
    FloatType { frac: 12031391722600823274, exp: -870 },        // 12^-225
    FloatType { frac: 14453972367916992462, exp: -838 },        // 12^-216
    FloatType { frac: 17364351691754770668, exp: -806 },        // 12^-207
    FloatType { frac: 10430375193750279268, exp: -773 },        // 12^-198
    FloatType { frac: 12530583187169601247, exp: -741 },        // 12^-189
    FloatType { frac: 15053678520084183432, exp: -709 },        // 12^-180
    FloatType { frac: 18084811664478575592, exp: -677 },        // 12^-171
    FloatType { frac: 10863139281980340679, exp: -644 },        // 12^-162
    FloatType { frac: 13050486479932803075, exp: -612 },        // 12^-153
    FloatType { frac: 15678266930207358578, exp: -580 },        // 12^-144
    FloatType { frac: 9417582030861555141, exp: -547 },         // 12^-135
    FloatType { frac: 11313859076748534537, exp: -515 },        // 12^-126
    FloatType { frac: 13591960950173425616, exp: -483 },        // 12^-117
    FloatType { frac: 16328769981827608423, exp: -451 },        // 12^-108
    FloatType { frac: 9808324571298608904, exp: -418 },         // 12^-99
    FloatType { frac: 11783279573783601017, exp: -386 },        // 12^-90
    FloatType { frac: 14155901602220618825, exp: -354 },        // 12^-81
    FloatType { frac: 17006262892853298360, exp: -322 },        // 12^-72
    FloatType { frac: 10215279312745101062, exp: -289 },        // 12^-63
    FloatType { frac: 12272176679245716810, exp: -257 },        // 12^-54
    FloatType { frac: 14743240574804287352, exp: -225 },        // 12^-45
    FloatType { frac: 17711865492790087155, exp: -193 },        // 12^-36
    FloatType { frac: 10639118911577981124, exp: -160 },        // 12^-27
    FloatType { frac: 12781358492223474271, exp: -128 },        // 12^-18
    FloatType { frac: 15354948681789223882, exp: -96 },         // 12^-9
    FloatType { frac: 9223372036854775808, exp: -63 },          // 12^0
    FloatType { frac: 11080543933191684096, exp: -31 },         // 12^9
    FloatType { frac: 13311666640442621952, exp: 1 },           // 12^18
    FloatType { frac: 15992037016835457024, exp: 33 },          // 12^27
    FloatType { frac: 9606056659007943744, exp: 66 },           // 12^36
    FloatType { frac: 11540284009964194135, exp: 98 },          // 12^45
    FloatType { frac: 13863977671394362375, exp: 130 },         // 12^54
    FloatType { frac: 16655558624637160317, exp: 162 },         // 12^63
    FloatType { frac: 10004619153098548172, exp: 195 },         // 12^72
    FloatType { frac: 12019099047267988506, exp: 227 },         // 12^81
    FloatType { frac: 14439204501182606065, exp: 259 },         // 12^90
    FloatType { frac: 17346610241502516795, exp: 291 },         // 12^99
    FloatType { frac: 10419718303939637392, exp: 324 },         // 12^108
    FloatType { frac: 12517780479519279956, exp: 356 },         // 12^117
    FloatType { frac: 15038297923484984581, exp: 388 },         // 12^126
    FloatType { frac: 18066334108151547333, exp: 420 },         // 12^135
    FloatType { frac: 10852040229820157048, exp: 453 },         // 12^144
    FloatType { frac: 13037152578341684032, exp: 485 },         // 12^153
    FloatType { frac: 15662248181121787524, exp: 517 },         // 12^162
    FloatType { frac: 9407959928864140132, exp: 550 },          // 12^171
    FloatType { frac: 11302299516591361707, exp: 582 },         // 12^180
    FloatType { frac: 13578073815006577911, exp: 614 },         // 12^189
    FloatType { frac: 16312086602830473207, exp: 646 },         // 12^198
    FloatType { frac: 9798303241073980839, exp: 679 },          // 12^207
    FloatType { frac: 11771240398807322073, exp: 711 },         // 12^216
    FloatType { frac: 14141438279402131370, exp: 743 },         // 12^225
    FloatType { frac: 16988887307951181138, exp: 775 },         // 12^234
    FloatType { frac: 10204842190014742991, exp: 808 },         // 12^243
    FloatType { frac: 12259637989871837542, exp: 840 },         // 12^252
    FloatType { frac: 14728177157876426901, exp: 872 },         // 12^261
    FloatType { frac: 17693768981840924725, exp: 904 },         // 12^270
    FloatType { frac: 10628248744799039348, exp: 937 },         // 12^279
];
const BASE12_BIAS: i32 = -BASE12_LARGE_POWERS[0].exp;

// BASE13

const BASE13_STEP: i32 = 8;
const BASE13_SMALL_POWERS: [FloatType; BASE13_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 13^0
    FloatType { frac: 14987979559889010688, exp: -60 },         // 13^1
    FloatType { frac: 12177733392409821184, exp: -56 },         // 13^2
    FloatType { frac: 9894408381332979712, exp: -52 },          // 13^3
    FloatType { frac: 16078413619666092032, exp: -49 },         // 13^4
    FloatType { frac: 13063711065978699776, exp: -45 },         // 13^5
    FloatType { frac: 10614265241107693568, exp: -41 },         // 13^6
    FloatType { frac: 17248181016800002048, exp: -38 },         // 13^7
];
const BASE13_LARGE_POWERS: [FloatType; 72] = [
    FloatType { frac: 14673465379822171777, exp: -1159 },       // 13^-296
    FloatType { frac: 11147555423761605318, exp: -1129 },       // 13^-288
    FloatType { frac: 16937783776246970219, exp: -1100 },       // 13^-280
    FloatType { frac: 12867777209673117558, exp: -1070 },       // 13^-272
    FloatType { frac: 9775758889423702247, exp: -1040 },        // 13^-264
    FloatType { frac: 14853452979012869128, exp: -1011 },       // 13^-256
    FloatType { frac: 11284293521111612769, exp: -981 },        // 13^-248
    FloatType { frac: 17145545948207386966, exp: -952 },        // 13^-240
    FloatType { frac: 13025615884242430727, exp: -922 },        // 13^-232
    FloatType { frac: 9895670261906581517, exp: -892 },         // 13^-224
    FloatType { frac: 15035648341334079534, exp: -863 },        // 13^-216
    FloatType { frac: 11422708874734959378, exp: -833 },        // 13^-208
    FloatType { frac: 17355856571645749915, exp: -804 },        // 13^-200
    FloatType { frac: 13185390638896427802, exp: -774 },        // 13^-192
    FloatType { frac: 10017052490761162429, exp: -744 },        // 13^-184
    FloatType { frac: 15220078547640608376, exp: -715 },        // 13^-176
    FloatType { frac: 11562822058185475244, exp: -685 },        // 13^-168
    FloatType { frac: 17568746906366835671, exp: -656 },        // 13^-160
    FloatType { frac: 13347125221972482607, exp: -626 },        // 13^-152
    FloatType { frac: 10139923617799671626, exp: -596 },        // 13^-144
    FloatType { frac: 15406771010966328102, exp: -567 },        // 13^-136
    FloatType { frac: 11704653897376229735, exp: -537 },        // 13^-128
    FloatType { frac: 17784248595614306423, exp: -508 },        // 13^-120
    FloatType { frac: 13510843673109724761, exp: -478 },        // 13^-112
    FloatType { frac: 10264301906138736839, exp: -448 },        // 13^-104
    FloatType { frac: 15595753480598751694, exp: -419 },        // 13^-96
    FloatType { frac: 11848225473675019323, exp: -389 },        // 13^-88
    FloatType { frac: 18002393670774046392, exp: -360 },        // 13^-80
    FloatType { frac: 13676570326822204041, exp: -330 },        // 13^-72
    FloatType { frac: 10390205842913949994, exp: -300 },        // 13^-64
    FloatType { frac: 15787054046203585657, exp: -271 },        // 13^-56
    FloatType { frac: 11993558127037825287, exp: -241 },        // 13^-48
    FloatType { frac: 18223214556135190308, exp: -212 },        // 13^-40
    FloatType { frac: 13844329816115883890, exp: -182 },        // 13^-32
    FloatType { frac: 10517654142027727687, exp: -152 },        // 13^-24
    FloatType { frac: 15980701141999875583, exp: -123 },        // 13^-16
    FloatType { frac: 12140673459180707010, exp: -93 },         // 13^-8
    FloatType { frac: 9223372036854775808, exp: -63 },          // 13^0
    FloatType { frac: 14014147076150001664, exp: -34 },         // 13^8
    FloatType { frac: 10646665746930877456, exp: -4 },          // 13^16
    FloatType { frac: 16176723550986364864, exp: 25 },          // 13^24
    FloatType { frac: 12289593336790602348, exp: 55 },          // 13^32
    FloatType { frac: 9336507724055083356, exp: 85 },           // 13^40
    FloatType { frac: 14186047347943339851, exp: 114 },         // 13^48
    FloatType { frac: 10777259833438283283, exp: 144 },         // 13^56
    FloatType { frac: 16375150409219694755, exp: 173 },         // 13^64
    FloatType { frac: 12440339894775512302, exp: 203 },         // 13^72
    FloatType { frac: 9451031155744840189, exp: 233 },          // 13^80
    FloatType { frac: 14360056182125959135, exp: 262 },         // 13^88
    FloatType { frac: 10909455812579128852, exp: 292 },         // 13^96
    FloatType { frac: 16576011210145081669, exp: 321 },         // 13^104
    FloatType { frac: 12592935539554553092, exp: 351 },         // 13^112
    FloatType { frac: 9566959354269653198, exp: 381 },          // 13^120
    FloatType { frac: 14536199442736950948, exp: 410 },         // 13^128
    FloatType { frac: 11043273333482082198, exp: 440 },         // 13^136
    FloatType { frac: 16779335808980115413, exp: 469 },         // 13^144
    FloatType { frac: 12747402952388364654, exp: 499 },         // 13^152
    FloatType { frac: 9684309550774553205, exp: 529 },          // 13^160
    FloatType { frac: 14714503311068774005, exp: 558 },         // 13^168
    FloatType { frac: 11178732286295870598, exp: 588 },         // 13^176
    FloatType { frac: 16985154427152329948, exp: 617 },         // 13^184
    FloatType { frac: 12903765092750370582, exp: 647 },         // 13^192
    FloatType { frac: 9803099187765169579, exp: 677 },          // 13^200
    FloatType { frac: 14894994289558746218, exp: 706 },         // 13^208
    FloatType { frac: 11315852805145679810, exp: 736 },         // 13^216
    FloatType { frac: 17193497656791206265, exp: 765 },         // 13^224
    FloatType { frac: 13062045201739390598, exp: 795 },         // 13^232
    FloatType { frac: 9923345921700320715, exp: 825 },          // 13^240
    FloatType { frac: 15077699205728270417, exp: 854 },         // 13^248
    FloatType { frac: 11454655271125817073, exp: 884 },         // 13^256
    FloatType { frac: 17404396465275275042, exp: 913 },         // 13^264
    FloatType { frac: 13222266805534112801, exp: 943 },         // 13^272
];
const BASE13_BIAS: i32 = -BASE13_LARGE_POWERS[0].exp;

// BASE14

const BASE14_STEP: i32 = 8;
const BASE14_SMALL_POWERS: [FloatType; BASE14_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 14^0
    FloatType { frac: 16140901064495857664, exp: -60 },         // 14^1
    FloatType { frac: 14123288431433875456, exp: -56 },         // 14^2
    FloatType { frac: 12357877377504641024, exp: -52 },         // 14^3
    FloatType { frac: 10813142705316560896, exp: -48 },         // 14^4
    FloatType { frac: 9461499867151990784, exp: -44 },          // 14^5
    FloatType { frac: 16557624767515983872, exp: -41 },         // 14^6
    FloatType { frac: 14487921671576485888, exp: -37 },         // 14^7
];
const BASE14_LARGE_POWERS: [FloatType; 70] = [
    FloatType { frac: 12880143300754023535, exp: -1160 },       // 14^-288
    FloatType { frac: 17702928299982570560, exp: -1130 },       // 14^-280
    FloatType { frac: 12165767999490239948, exp: -1099 },       // 14^-272
    FloatType { frac: 16721065408999761282, exp: -1069 },       // 14^-264
    FloatType { frac: 11491014312609104256, exp: -1038 },       // 14^-256
    FloatType { frac: 15793659877858943182, exp: -1008 },       // 14^-248
    FloatType { frac: 10853684694473876180, exp: -977 },        // 14^-240
    FloatType { frac: 14917691321465419740, exp: -947 },        // 14^-232
    FloatType { frac: 10251703482589146278, exp: -916 },        // 14^-224
    FloatType { frac: 14090306875260685218, exp: -886 },        // 14^-216
    FloatType { frac: 9683110137559136558, exp: -855 },         // 14^-208
    FloatType { frac: 13308811903980028150, exp: -825 },        // 14^-200
    FloatType { frac: 18292105715960495534, exp: -795 },        // 14^-192
    FloatType { frac: 12570661225733134820, exp: -764 },        // 14^-184
    FloatType { frac: 17277565098945522629, exp: -734 },        // 14^-176
    FloatType { frac: 11873450822826176619, exp: -703 },        // 14^-168
    FloatType { frac: 16319294256419936609, exp: -673 },        // 14^-160
    FloatType { frac: 11214910012329090474, exp: -642 },        // 14^-152
    FloatType { frac: 15414172280784786485, exp: -612 },        // 14^-144
    FloatType { frac: 10592894050889065017, exp: -581 },        // 14^-136
    FloatType { frac: 14559251360287507272, exp: -551 },        // 14^-128
    FloatType { frac: 10005377149705503250, exp: -520 },        // 14^-120
    FloatType { frac: 13751747178554400168, exp: -490 },        // 14^-112
    FloatType { frac: 9450445876917551117, exp: -459 },         // 14^-104
    FloatType { frac: 12989029846596759700, exp: -429 },        // 14^-96
    FloatType { frac: 17852585851834022264, exp: -399 },        // 14^-88
    FloatType { frac: 12268615337757900164, exp: -368 },        // 14^-80
    FloatType { frac: 16862422458582420498, exp: -338 },        // 14^-72
    FloatType { frac: 11588157397706317457, exp: -307 },        // 14^-64
    FloatType { frac: 15927176798452085633, exp: -277 },        // 14^-56
    FloatType { frac: 10945439903127358164, exp: -246 },        // 14^-48
    FloatType { frac: 15043802952525257461, exp: -216 },        // 14^-40
    FloatType { frac: 10338369644227094261, exp: -185 },        // 14^-32
    FloatType { frac: 14209423938610553080, exp: -155 },        // 14^-24
    FloatType { frac: 9764969507542378307, exp: -124 },         // 14^-16
    FloatType { frac: 13421322341453983785, exp: -94 },         // 14^-8
    FloatType { frac: 9223372036854775808, exp: -63 },          // 14^0
    FloatType { frac: 12676931462629425152, exp: -33 },         // 14^8
    FloatType { frac: 17423626702474969088, exp: -3 },          // 14^16
    FloatType { frac: 11973826961285400900, exp: 28 },          // 14^24
    FloatType { frac: 16457254800854930971, exp: 58 },          // 14^32
    FloatType { frac: 11309718958523667683, exp: 89 },          // 14^40
    FloatType { frac: 15544481077627229210, exp: 119 },         // 14^48
    FloatType { frac: 10682444579695049354, exp: 150 },         // 14^56
    FloatType { frac: 14682332800738954595, exp: 180 },         // 14^64
    FloatType { frac: 10089960910324183248, exp: 211 },         // 14^72
    FloatType { frac: 13868002115678253630, exp: 241 },         // 14^80
    FloatType { frac: 9530338342721952463, exp: 272 },          // 14^88
    FloatType { frac: 13098836900821174211, exp: 302 },         // 14^96
    FloatType { frac: 18003508583233548621, exp: 332 },         // 14^104
    FloatType { frac: 12372332129971187630, exp: 363 },         // 14^112
    FloatType { frac: 17004974516675479989, exp: 393 },         // 14^120
    FloatType { frac: 11686121713960805382, exp: 424 },         // 14^128
    FloatType { frac: 16061822448435536582, exp: 454 },         // 14^136
    FloatType { frac: 11037970794744924274, exp: 485 },         // 14^144
    FloatType { frac: 15170980709914287138, exp: 515 },         // 14^152
    FloatType { frac: 10425768466889213611, exp: 546 },         // 14^160
    FloatType { frac: 14329547997401095751, exp: 576 },         // 14^168
    FloatType { frac: 9847520902748803399, exp: 607 },          // 14^176
    FloatType { frac: 13534783923074532648, exp: 637 },         // 14^184
    FloatType { frac: 9301344858947275744, exp: 668 },          // 14^192
    FloatType { frac: 12784100090075520076, exp: 698 },         // 14^200
    FloatType { frac: 17570923086015569737, exp: 728 },         // 14^208
    FloatType { frac: 12075051662586407952, exp: 759 },         // 14^216
    FloatType { frac: 16596381640322157656, exp: 789 },         // 14^224
    FloatType { frac: 11405329403461315009, exp: 820 },         // 14^232
    FloatType { frac: 15675891482926176126, exp: 850 },         // 14^240
    FloatType { frac: 10772752153475797540, exp: 881 },         // 14^248
    FloatType { frac: 14806454750802381310, exp: 911 },         // 14^256
    FloatType { frac: 10175259727702178785, exp: 942 },         // 14^264
];
const BASE14_BIAS: i32 = -BASE14_LARGE_POWERS[0].exp;

// BASE15

const BASE15_STEP: i32 = 8;
const BASE15_SMALL_POWERS: [FloatType; BASE15_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 15^0
    FloatType { frac: 17293822569102704640, exp: -60 },         // 15^1
    FloatType { frac: 16212958658533785600, exp: -56 },         // 15^2
    FloatType { frac: 15199648742375424000, exp: -52 },         // 15^3
    FloatType { frac: 14249670695976960000, exp: -48 },         // 15^4
    FloatType { frac: 13359066277478400000, exp: -44 },         // 15^5
    FloatType { frac: 12524124635136000000, exp: -40 },         // 15^6
    FloatType { frac: 11741366845440000000, exp: -36 },         // 15^7
];
const BASE15_LARGE_POWERS: [FloatType; 68] = [
    FloatType { frac: 9686178043528474499, exp: -1157 },        // 15^-280
    FloatType { frac: 11559862131178364723, exp: -1126 },       // 15^-272
    FloatType { frac: 13795989697002596758, exp: -1095 },       // 15^-264
    FloatType { frac: 16464671426007778306, exp: -1064 },       // 15^-256
    FloatType { frac: 9824790070164184132, exp: -1032 },        // 15^-248
    FloatType { frac: 11725287122380398084, exp: -1001 },       // 15^-240
    FloatType { frac: 13993414324420480958, exp: -970 },        // 15^-232
    FloatType { frac: 16700285665596816319, exp: -939 },        // 15^-224
    FloatType { frac: 9965385675239368708, exp: -907 },         // 15^-216
    FloatType { frac: 11893079393347852255, exp: -876 },        // 15^-208
    FloatType { frac: 14193664155710441018, exp: -845 },        // 15^-200
    FloatType { frac: 16939271613521887687, exp: -814 },        // 15^-192
    FloatType { frac: 10107993244338750184, exp: -782 },        // 15^-184
    FloatType { frac: 12063272820543086702, exp: -751 },        // 15^-176
    FloatType { frac: 14396779620362065880, exp: -720 },        // 15^-168
    FloatType { frac: 17181677519910502131, exp: -689 },        // 15^-160
    FloatType { frac: 10252641569253028545, exp: -657 },        // 15^-152
    FloatType { frac: 12235901765210495847, exp: -626 },        // 15^-144
    FloatType { frac: 14602801726422706134, exp: -595 },        // 15^-136
    FloatType { frac: 17427552325363535592, exp: -564 },        // 15^-128
    FloatType { frac: 10399359853791807565, exp: -532 },        // 15^-120
    FloatType { frac: 12411001080313881072, exp: -501 },        // 15^-112
    FloatType { frac: 14811772068776803956, exp: -470 },        // 15^-104
    FloatType { frac: 17676945670836105047, exp: -439 },        // 15^-96
    FloatType { frac: 10548177719679705225, exp: -407 },        // 15^-88
    FloatType { frac: 12588606117573098524, exp: -376 },        // 15^-80
    FloatType { frac: 15023732837543702665, exp: -345 },        // 15^-72
    FloatType { frac: 17929907907659841510, exp: -314 },        // 15^-64
    FloatType { frac: 10699125212536839185, exp: -282 },        // 15^-56
    FloatType { frac: 12768752734601403407, exp: -251 },        // 15^-48
    FloatType { frac: 15238726826595631383, exp: -220 },        // 15^-40
    FloatType { frac: 18186490107708584674, exp: -189 },        // 15^-32
    FloatType { frac: 10852232807944894743, exp: -157 },        // 15^-24
    FloatType { frac: 12951477302144931748, exp: -126 },        // 15^-16
    FloatType { frac: 15456797442197584532, exp: -95 },         // 15^-8
    FloatType { frac: 9223372036854775808, exp: -63 },          // 15^0
    FloatType { frac: 11007531417600000000, exp: -32 },         // 15^8
    FloatType { frac: 13136816711425781250, exp: -1 },          // 15^16
    FloatType { frac: 15677988711770840524, exp: 30 },          // 15^24
    FloatType { frac: 9355361174851030653, exp: 62 },           // 15^32
    FloatType { frac: 11165052395553650442, exp: 93 },          // 15^40
    FloatType { frac: 13324808381590173768, exp: 124 },         // 15^48
    FloatType { frac: 15902345292781888946, exp: 155 },         // 15^56
    FloatType { frac: 9489239115822963265, exp: 187 },          // 15^64
    FloatType { frac: 11324827544542942993, exp: 218 },         // 15^72
    FloatType { frac: 13515490267263203164, exp: 249 },         // 15^80
    FloatType { frac: 16129912481758560891, exp: 280 },         // 15^88
    FloatType { frac: 9625032889090827484, exp: 312 },          // 15^96
    FloatType { frac: 11486889122411397534, exp: 343 },         // 15^104
    FloatType { frac: 13708900866211693796, exp: 374 },         // 15^112
    FloatType { frac: 16360736223435182728, exp: 405 },         // 15^120
    FloatType { frac: 9762769910772315950, exp: 437 },          // 15^128
    FloatType { frac: 11651269848621662268, exp: 468 },         // 15^136
    FloatType { frac: 13905079227116716745, exp: 499 },         // 15^144
    FloatType { frac: 16594863120028599690, exp: 530 },         // 15^152
    FloatType { frac: 9902477989317744010, exp: 562 },          // 15^160
    FloatType { frac: 11818002910861417777, exp: 593 },         // 15^168
    FloatType { frac: 14104064957457333009, exp: 624 },         // 15^176
    FloatType { frac: 16832340440646942057, exp: 655 },         // 15^184
    FloatType { frac: 10044185331124443731, exp: 687 },         // 15^192
    FloatType { frac: 11987121971743813505, exp: 718 },         // 15^200
    FloatType { frac: 14305898231507155361, exp: 749 },         // 15^208
    FloatType { frac: 17073216130833033517, exp: 780 },         // 15^216
    FloatType { frac: 10187920546231501512, exp: 812 },         // 15^224
    FloatType { frac: 12158661175603789420, exp: 843 },         // 15^232
    FloatType { frac: 14510619798445343328, exp: 874 },         // 15^240
    FloatType { frac: 17317538822244368489, exp: 905 },         // 15^248
    FloatType { frac: 10333712654095989060, exp: 937 },         // 15^256
];
const BASE15_BIAS: i32 = -BASE15_LARGE_POWERS[0].exp;

// BASE17

const BASE17_STEP: i32 = 8;
const BASE17_SMALL_POWERS: [FloatType; BASE17_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 17^0
    FloatType { frac: 9799832789158199296, exp: -59 },          // 17^1
    FloatType { frac: 10412322338480586752, exp: -55 },         // 17^2
    FloatType { frac: 11063092484635623424, exp: -51 },         // 17^3
    FloatType { frac: 11754535764925349888, exp: -47 },         // 17^4
    FloatType { frac: 12489194250233184256, exp: -43 },         // 17^5
    FloatType { frac: 13269768890872758272, exp: -39 },         // 17^6
    FloatType { frac: 14099129446552305664, exp: -35 },         // 17^7
];
const BASE17_LARGE_POWERS: [FloatType; 65] = [
    FloatType { frac: 17328852299072967575, exp: -1143 },       // 17^-264
    FloatType { frac: 14072501842077846052, exp: -1110 },       // 17^-256
    FloatType { frac: 11428068326595325663, exp: -1077 },       // 17^-248
    FloatType { frac: 9280563409615280245, exp: -1044 },        // 17^-240
    FloatType { frac: 15073213554289220394, exp: -1012 },       // 17^-232
    FloatType { frac: 12240731344920942400, exp: -979 },        // 17^-224
    FloatType { frac: 9940514895438007254, exp: -946 },         // 17^-216
    FloatType { frac: 16145087021687770276, exp: -914 },        // 17^-208
    FloatType { frac: 13111183760586542995, exp: -881 },        // 17^-200
    FloatType { frac: 10647396286743453217, exp: -848 },        // 17^-192
    FloatType { frac: 17293182638130712658, exp: -816 },        // 17^-184
    FloatType { frac: 14043535043777936273, exp: -783 },        // 17^-176
    FloatType { frac: 11404544822822581011, exp: -750 },        // 17^-168
    FloatType { frac: 9261460323937079649, exp: -717 },         // 17^-160
    FloatType { frac: 15042186893809203473, exp: -685 },        // 17^-152
    FloatType { frac: 12215535057871861844, exp: -652 },        // 17^-144
    FloatType { frac: 9920053367473418578, exp: -619 },         // 17^-136
    FloatType { frac: 16111854019870470980, exp: -587 },        // 17^-128
    FloatType { frac: 13084195736727816960, exp: -554 },        // 17^-120
    FloatType { frac: 10625479716106730764, exp: -521 },        // 17^-112
    FloatType { frac: 17257586399518441101, exp: -489 },        // 17^-104
    FloatType { frac: 14014627870654357169, exp: -456 },        // 17^-96
    FloatType { frac: 11381069739763987898, exp: -423 },        // 17^-88
    FloatType { frac: 9242396559996829853, exp: -390 },         // 17^-80
    FloatType { frac: 15011224098520048145, exp: -358 },        // 17^-72
    FloatType { frac: 12190390634789334486, exp: -325 },        // 17^-64
    FloatType { frac: 9899633957460570790, exp: -292 },         // 17^-56
    FloatType { frac: 16078689424770850259, exp: -260 },        // 17^-48
    FloatType { frac: 13057263264941664926, exp: -227 },        // 17^-40
    FloatType { frac: 10603608258477502216, exp: -194 },        // 17^-32
    FloatType { frac: 17222063432103834911, exp: -162 },        // 17^-24
    FloatType { frac: 13985780199974813110, exp: -129 },        // 17^-16
    FloatType { frac: 11357642977750484199, exp: -96 },         // 17^-8
    FloatType { frac: 9223372036854775808, exp: -63 },          // 17^0
    FloatType { frac: 14980325036961824768, exp: -31 },         // 17^8
    FloatType { frac: 12165297968916717120, exp: 2 },           // 17^16
    FloatType { frac: 9879256578703990224, exp: 35 },           // 17^24
    FloatType { frac: 16045593095580712414, exp: 67 },          // 17^32
    FloatType { frac: 13030386230879856604, exp: 100 },         // 17^40
    FloatType { frac: 10581781820995279550, exp: 133 },         // 17^48
    FloatType { frac: 17186613585065666435, exp: 165 },         // 17^56
    FloatType { frac: 13956991909259640275, exp: 198 },         // 17^64
    FloatType { frac: 11334264437318166304, exp: 231 },         // 17^72
    FloatType { frac: 18408773347475537258, exp: 263 },         // 17^80
    FloatType { frac: 14949489577945200446, exp: 296 },         // 17^88
    FloatType { frac: 12140256953717114113, exp: 329 },         // 17^96
    FloatType { frac: 9858921144686656932, exp: 362 },          // 17^104
    FloatType { frac: 16012564891781700940, exp: 394 },         // 17^112
    FloatType { frac: 13003564520429535778, exp: 427 },         // 17^120
    FloatType { frac: 10560000310990718510, exp: 460 },         // 17^128
    FloatType { frac: 17151236707893158013, exp: 492 },         // 17^136
    FloatType { frac: 13928262876281286641, exp: 525 },         // 17^144
    FloatType { frac: 11310934019207866827, exp: 558 },         // 17^152
    FloatType { frac: 18370880780077845311, exp: 590 },         // 17^160
    FloatType { frac: 14918717590550882042, exp: 623 },         // 17^168
    FloatType { frac: 12115267482872925081, exp: 656 },         // 17^176
    FloatType { frac: 9838627569069637357, exp: 689 },          // 17^184
    FloatType { frac: 15979604673144701925, exp: 721 },         // 17^192
    FloatType { frac: 12976798019712735820, exp: 754 },         // 17^200
    FloatType { frac: 10538263635985225157, exp: 787 },         // 17^208
    FloatType { frac: 17115932650385342947, exp: 819 },         // 17^216
    FloatType { frac: 13899592979063793037, exp: 852 },         // 17^224
    FloatType { frac: 11287651624364733171, exp: 885 },         // 17^232
    FloatType { frac: 18333066210634546428, exp: 917 },         // 17^240
    FloatType { frac: 14888008944129060322, exp: 950 },         // 17^248
];
const BASE17_BIAS: i32 = -BASE17_LARGE_POWERS[0].exp;

// BASE18

const BASE18_STEP: i32 = 7;
const BASE18_SMALL_POWERS: [FloatType; BASE18_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 18^0
    FloatType { frac: 10376293541461622784, exp: -59 },         // 18^1
    FloatType { frac: 11673330234144325632, exp: -55 },         // 18^2
    FloatType { frac: 13132496513412366336, exp: -51 },         // 18^3
    FloatType { frac: 14774058577588912128, exp: -47 },         // 18^4
    FloatType { frac: 16620815899787526144, exp: -43 },         // 18^5
    FloatType { frac: 9349208943630483456, exp: -38 },          // 18^6
];
const BASE18_LARGE_POWERS: [FloatType; 73] = [
    FloatType { frac: 18312018475493194258, exp: -1144 },       // 18^-259
    FloatType { frac: 10441042983020688038, exp: -1114 },       // 18^-252
    FloatType { frac: 11906429509033078491, exp: -1085 },       // 18^-245
    FloatType { frac: 13577481089208229636, exp: -1056 },       // 18^-238
    FloatType { frac: 15483062540952967857, exp: -1027 },       // 18^-231
    FloatType { frac: 17656089820489710741, exp: -998 },        // 18^-224
    FloatType { frac: 10067049297406417285, exp: -968 },        // 18^-217
    FloatType { frac: 11479946305982273645, exp: -939 },        // 18^-210
    FloatType { frac: 13091141534609253262, exp: -910 },        // 18^-203
    FloatType { frac: 14928465875303384176, exp: -881 },        // 18^-196
    FloatType { frac: 17023656248839843776, exp: -852 },        // 18^-189
    FloatType { frac: 9706451905352742522, exp: -822 },         // 18^-182
    FloatType { frac: 11068739548514628780, exp: -793 },        // 18^-175
    FloatType { frac: 12622222454457155586, exp: -764 },        // 18^-168
    FloatType { frac: 14393734624570008992, exp: -735 },        // 18^-161
    FloatType { frac: 16413876176725623927, exp: -706 },        // 18^-154
    FloatType { frac: 9358770957364699929, exp: -676 },         // 18^-147
    FloatType { frac: 10672262040895386089, exp: -647 },        // 18^-140
    FloatType { frac: 12170099854822007158, exp: -618 },        // 18^-133
    FloatType { frac: 13878157218102970303, exp: -589 },        // 18^-126
    FloatType { frac: 15825938165500818674, exp: -560 },        // 18^-119
    FloatType { frac: 18047087583901234911, exp: -531 },        // 18^-112
    FloatType { frac: 10289986187706530766, exp: -501 },        // 18^-105
    FloatType { frac: 11734172092969064177, exp: -472 },        // 18^-98
    FloatType { frac: 13381047573408163051, exp: -443 },        // 18^-91
    FloatType { frac: 15259059841903798156, exp: -414 },        // 18^-84
    FloatType { frac: 17400648639910404101, exp: -385 },        // 18^-77
    FloatType { frac: 9921403291771844100, exp: -355 },         // 18^-70
    FloatType { frac: 11313859076748534537, exp: -326 },        // 18^-63
    FloatType { frac: 12901744183172431346, exp: -297 },        // 18^-56
    FloatType { frac: 14712486856947913357, exp: -268 },        // 18^-49
    FloatType { frac: 16777364861891103792, exp: -239 },        // 18^-42
    FloatType { frac: 9566022877229980327, exp: -209 },         // 18^-35
    FloatType { frac: 10908601492662859386, exp: -180 },        // 18^-28
    FloatType { frac: 12439609234991117453, exp: -151 },        // 18^-21
    FloatType { frac: 14185491882103974832, exp: -122 },        // 18^-14
    FloatType { frac: 16176406841720334625, exp: -93 },         // 18^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 18^0
    FloatType { frac: 10517860061584293888, exp: -34 },         // 18^7
    FloatType { frac: 11994027762626592768, exp: -5 },          // 18^14
    FloatType { frac: 13677373641439044901, exp: 24 },          // 18^21
    FloatType { frac: 15596974880318657672, exp: 53 },          // 18^28
    FloatType { frac: 17785989605508530085, exp: 82 },          // 18^35
    FloatType { frac: 10141114821132365302, exp: 112 },         // 18^42
    FloatType { frac: 11564406827668344530, exp: 141 },         // 18^49
    FloatType { frac: 13187455978423603575, exp: 170 },         // 18^56
    FloatType { frac: 15038297923484984581, exp: 199 },         // 18^63
    FloatType { frac: 17148903079221976570, exp: 228 },         // 18^70
    FloatType { frac: 9777864433756263024, exp: 258 },          // 18^77
    FloatType { frac: 11150174730505647564, exp: 287 },         // 18^84
    FloatType { frac: 12715086956165281921, exp: 316 },         // 18^91
    FloatType { frac: 14499632535849309517, exp: 345 },         // 18^98
    FloatType { frac: 16534636719312342666, exp: 374 },         // 18^105
    FloatType { frac: 9427625519601420913, exp: 404 },          // 18^112
    FloatType { frac: 10750780249562856814, exp: 433 },         // 18^119
    FloatType { frac: 12259637989871837542, exp: 462 },         // 18^126
    FloatType { frac: 13980261911578014597, exp: 491 },         // 18^133
    FloatType { frac: 15942373117198559022, exp: 520 },         // 18^140
    FloatType { frac: 18179864026545065558, exp: 549 },         // 18^147
    FloatType { frac: 10365691907784965713, exp: 579 },         // 18^154
    FloatType { frac: 11820503010388934534, exp: 608 },         // 18^161
    FloatType { frac: 13479494920515287357, exp: 637 },         // 18^168
    FloatType { frac: 15371324143524666656, exp: 666 },         // 18^175
    FloatType { frac: 17528669087274082029, exp: 695 },         // 18^182
    FloatType { frac: 9994397265397337538, exp: 725 },          // 18^189
    FloatType { frac: 11397097657699641734, exp: 754 },         // 18^196
    FloatType { frac: 12996665188491343910, exp: 783 },         // 18^203
    FloatType { frac: 14820729899390519784, exp: 812 },         // 18^210
    FloatType { frac: 16900799671687597041, exp: 841 },         // 18^217
    FloatType { frac: 9636402237998480121, exp: 871 },          // 18^224
    FloatType { frac: 10988858503312433354, exp: 900 },         // 18^231
    FloatType { frac: 12531130210573617469, exp: 929 },         // 18^238
    FloatType { frac: 14289857705148955482, exp: 958 },         // 18^245
];
const BASE18_BIAS: i32 = -BASE18_LARGE_POWERS[0].exp;

// BASE19

const BASE19_STEP: i32 = 7;
const BASE19_SMALL_POWERS: [FloatType; BASE19_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 19^0
    FloatType { frac: 10952754293765046272, exp: -59 },         // 19^1
    FloatType { frac: 13006395723845992448, exp: -55 },         // 19^2
    FloatType { frac: 15445094922067116032, exp: -51 },         // 19^3
    FloatType { frac: 18341050219954700288, exp: -47 },         // 19^4
    FloatType { frac: 10889998568098103296, exp: -42 },         // 19^5
    FloatType { frac: 12931873299616497664, exp: -38 },         // 19^6
];
const BASE19_LARGE_POWERS: [FloatType; 72] = [
    FloatType { frac: 15912276110980153383, exp: -1164 },       // 19^-259
    FloatType { frac: 13246698229359450470, exp: -1134 },       // 19^-252
    FloatType { frac: 11027650146079950824, exp: -1104 },       // 19^-245
    FloatType { frac: 18360660994723606251, exp: -1075 },       // 19^-238
    FloatType { frac: 15284936849533635885, exp: -1045 },       // 19^-231
    FloatType { frac: 12724449003299523561, exp: -1015 },       // 19^-224
    FloatType { frac: 10592886580523254223, exp: -985 },        // 19^-217
    FloatType { frac: 17636794501472422448, exp: -956 },        // 19^-210
    FloatType { frac: 14682330350779734844, exp: -926 },        // 19^-203
    FloatType { frac: 12222789379976654044, exp: -896 },        // 19^-196
    FloatType { frac: 10175263507767080823, exp: -866 },        // 19^-189
    FloatType { frac: 16941466343535111364, exp: -837 },        // 19^-182
    FloatType { frac: 14103481529006456400, exp: -807 },        // 19^-175
    FloatType { frac: 11740907617180962231, exp: -777 },        // 19^-168
    FloatType { frac: 9774105166278679843, exp: -747 },         // 19^-161
    FloatType { frac: 16273551401031031665, exp: -718 },        // 19^-154
    FloatType { frac: 13547453741119703900, exp: -688 },        // 19^-147
    FloatType { frac: 11278023975525727060, exp: -658 },        // 19^-140
    FloatType { frac: 9388762436329270793, exp: -628 },         // 19^-133
    FloatType { frac: 15631968911773566269, exp: -599 },        // 19^-126
    FloatType { frac: 13013347271048440836, exp: -569 },        // 19^-119
    FloatType { frac: 10833389456740556437, exp: -539 },        // 19^-112
    FloatType { frac: 18037223579289291900, exp: -510 },        // 19^-105
    FloatType { frac: 15015680722474235794, exp: -480 },        // 19^-98
    FloatType { frac: 12500297873901968386, exp: -450 },        // 19^-91
    FloatType { frac: 10406284591707172986, exp: -420 },        // 19^-84
    FloatType { frac: 17326108560931302042, exp: -391 },        // 19^-77
    FloatType { frac: 14423689608892845377, exp: -361 },        // 19^-70
    FloatType { frac: 12007475377523598784, exp: -331 },        // 19^-63
    FloatType { frac: 9996018276276719532, exp: -301 },         // 19^-56
    FloatType { frac: 16643029152771930644, exp: -272 },        // 19^-49
    FloatType { frac: 13855037662215477149, exp: -242 },        // 19^-42
    FloatType { frac: 11534082339177879647, exp: -212 },        // 19^-35
    FloatType { frac: 9601926652984804576, exp: -182 },         // 19^-28
    FloatType { frac: 15986880054797934009, exp: -153 },        // 19^-21
    FloatType { frac: 13308804739049304804, exp: -123 },        // 19^-14
    FloatType { frac: 11079352755197736707, exp: -93 },         // 19^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 19^0
    FloatType { frac: 15356599543294590976, exp: -34 },         // 19^7
    FloatType { frac: 12784106972526145936, exp: -4 },          // 19^14
    FloatType { frac: 10642550821503597582, exp: 26 },          // 19^21
    FloatType { frac: 17719483767102098773, exp: 55 },          // 19^28
    FloatType { frac: 14751167752856224795, exp: 85 },          // 19^35
    FloatType { frac: 12280095342105548712, exp: 115 },         // 19^42
    FloatType { frac: 10222969742988875833, exp: 145 },         // 19^49
    FloatType { frac: 17020895596425699999, exp: 174 },         // 19^56
    FloatType { frac: 14169605026128220038, exp: 204 },         // 19^63
    FloatType { frac: 11795954299763191941, exp: 234 },         // 19^70
    FloatType { frac: 9819930589845265884, exp: 264 },          // 19^77
    FloatType { frac: 16349849166729084322, exp: 293 },         // 19^84
    FloatType { frac: 13610970328610229813, exp: 323 },         // 19^91
    FloatType { frac: 11330900450341615431, exp: 353 },         // 19^98
    FloatType { frac: 9432781198977253334, exp: 383 },          // 19^105
    FloatType { frac: 15705258648723927251, exp: 412 },         // 19^112
    FloatType { frac: 13074359725955544955, exp: 442 },         // 19^119
    FloatType { frac: 10884181283927938347, exp: 472 },         // 19^126
    FloatType { frac: 18121790237456409263, exp: 501 },         // 19^133
    FloatType { frac: 15086081021789818522, exp: 531 },         // 19^140
    FloatType { frac: 12558904921302722743, exp: 561 },         // 19^147
    FloatType { frac: 10455073958207408827, exp: 591 },         // 19^154
    FloatType { frac: 17407341190420966318, exp: 620 },         // 19^161
    FloatType { frac: 14491314386248513408, exp: 650 },         // 19^168
    FloatType { frac: 12063771850272711708, exp: 680 },         // 19^175
    FloatType { frac: 10042884128822494706, exp: 710 },         // 19^182
    FloatType { frac: 16721059197198717605, exp: 739 },         // 19^189
    FloatType { frac: 13919996342176535757, exp: 769 },         // 19^196
    FloatType { frac: 11588159331358018389, exp: 799 },         // 19^203
    FloatType { frac: 9646944825844903597, exp: 829 },          // 19^210
    FloatType { frac: 16061833775630288054, exp: 858 },         // 19^217
    FloatType { frac: 13371202432132867541, exp: 888 },         // 19^224
    FloatType { frac: 11131297769520092558, exp: 918 },         // 19^231
    FloatType { frac: 9266615374542536521, exp: 948 },          // 19^238
];
const BASE19_BIAS: i32 = -BASE19_LARGE_POWERS[0].exp;

// BASE20

const BASE20_STEP: i32 = 7;
const BASE20_SMALL_POWERS: [FloatType; BASE20_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 20^0
    FloatType { frac: 11529215046068469760, exp: -59 },         // 20^1
    FloatType { frac: 14411518807585587200, exp: -55 },         // 20^2
    FloatType { frac: 18014398509481984000, exp: -51 },         // 20^3
    FloatType { frac: 11258999068426240000, exp: -46 },         // 20^4
    FloatType { frac: 14073748835532800000, exp: -42 },         // 20^5
    FloatType { frac: 17592186044416000000, exp: -38 },         // 20^6
];
const BASE20_LARGE_POWERS: [FloatType; 70] = [
    FloatType { frac: 16905424996341287883, exp: -1153 },       // 20^-252
    FloatType { frac: 10076418516839318205, exp: -1122 },       // 20^-245
    FloatType { frac: 12012026926087520367, exp: -1092 },       // 20^-238
    FloatType { frac: 14319451959237480602, exp: -1062 },       // 20^-231
    FloatType { frac: 17070116948172426941, exp: -1032 },       // 20^-224
    FloatType { frac: 10174582569701926077, exp: -1001 },       // 20^-217
    FloatType { frac: 12129047596099288555, exp: -971 },        // 20^-210
    FloatType { frac: 14458951468586073584, exp: -941 },        // 20^-203
    FloatType { frac: 17236413322193710308, exp: -911 },        // 20^-196
    FloatType { frac: 10273702932711667006, exp: -880 },        // 20^-189
    FloatType { frac: 12247208276643356092, exp: -850 },        // 20^-182
    FloatType { frac: 14599809976391024699, exp: -820 },        // 20^-175
    FloatType { frac: 17404329748619824289, exp: -790 },        // 20^-168
    FloatType { frac: 10373788922202482396, exp: -759 },        // 20^-161
    FloatType { frac: 12366520073655226703, exp: -729 },        // 20^-154
    FloatType { frac: 14742040721959145907, exp: -699 },        // 20^-147
    FloatType { frac: 17573882009934360870, exp: -669 },        // 20^-140
    FloatType { frac: 10474849945267653984, exp: -638 },        // 20^-133
    FloatType { frac: 12486994201263968925, exp: -608 },        // 20^-126
    FloatType { frac: 14885657073574029118, exp: -578 },        // 20^-119
    FloatType { frac: 17745086042373215101, exp: -548 },        // 20^-112
    FloatType { frac: 10576895500643977583, exp: -517 },        // 20^-105
    FloatType { frac: 12608641982846233347, exp: -487 },        // 20^-98
    FloatType { frac: 15030672529752532658, exp: -457 },        // 20^-91
    FloatType { frac: 17917957937422433684, exp: -427 },        // 20^-84
    FloatType { frac: 10679935179604550411, exp: -396 },        // 20^-77
    FloatType { frac: 12731474852090538039, exp: -366 },        // 20^-70
    FloatType { frac: 15177100720513508366, exp: -336 },        // 20^-63
    FloatType { frac: 18092513943330655534, exp: -306 },        // 20^-56
    FloatType { frac: 10783978666860255917, exp: -275 },        // 20^-49
    FloatType { frac: 12855504354071922204, exp: -245 },        // 20^-42
    FloatType { frac: 15324955408658888583, exp: -215 },        // 20^-35
    FloatType { frac: 18268770466636286477, exp: -185 },        // 20^-28
    FloatType { frac: 10889035741470030830, exp: -154 },        // 20^-21
    FloatType { frac: 12980742146337069071, exp: -124 },        // 20^-14
    FloatType { frac: 15474250491067253436, exp: -94 },         // 20^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 20^0
    FloatType { frac: 10995116277760000000, exp: -33 },         // 20^7
    FloatType { frac: 13107200000000000000, exp: -3 },          // 20^14
    FloatType { frac: 15625000000000000000, exp: 27 },          // 20^21
    FloatType { frac: 9313225746154785156, exp: 58 },           // 20^28
    FloatType { frac: 11102230246251565404, exp: 88 },          // 20^35
    FloatType { frac: 13234889800848442797, exp: 118 },         // 20^42
    FloatType { frac: 15777218104420236108, exp: 148 },         // 20^49
    FloatType { frac: 9403954806578300063, exp: 179 },          // 20^56
    FloatType { frac: 11210387714598536567, exp: 209 },         // 20^63
    FloatType { frac: 13363823550460978230, exp: 239 },         // 20^70
    FloatType { frac: 15930919111324522770, exp: 269 },         // 20^77
    FloatType { frac: 9495567745759798747, exp: 300 },          // 20^84
    FloatType { frac: 11319598848533390459, exp: 330 },         // 20^91
    FloatType { frac: 13494013367335069727, exp: 360 },         // 20^98
    FloatType { frac: 16086117467087590369, exp: 390 },         // 20^105
    FloatType { frac: 9588073174409622174, exp: 421 },          // 20^112
    FloatType { frac: 11429873912822749822, exp: 451 },         // 20^119
    FloatType { frac: 13625471488026082303, exp: 481 },         // 20^126
    FloatType { frac: 16242827758820155028, exp: 511 },         // 20^133
    FloatType { frac: 9681479787123295682, exp: 542 },          // 20^140
    FloatType { frac: 11541223272232169725, exp: 572 },         // 20^147
    FloatType { frac: 13758210268297397763, exp: 602 },         // 20^154
    FloatType { frac: 16401064715739962772, exp: 632 },         // 20^161
    FloatType { frac: 9775796363198734982, exp: 663 },          // 20^168
    FloatType { frac: 11653657392500323036, exp: 693 },         // 20^175
    FloatType { frac: 13892242184281734271, exp: 723 },         // 20^182
    FloatType { frac: 16560843210556190337, exp: 753 },         // 20^189
    FloatType { frac: 9871031767461413346, exp: 784 },          // 20^196
    FloatType { frac: 11767186841322676356, exp: 814 },         // 20^203
    FloatType { frac: 14027579833653779454, exp: 844 },         // 20^210
    FloatType { frac: 16722178260867332761, exp: 874 },         // 20^217
    FloatType { frac: 9967194951097567535, exp: 905 },          // 20^224
    FloatType { frac: 11881822289344748896, exp: 935 },         // 20^231
];
const BASE20_BIAS: i32 = -BASE20_LARGE_POWERS[0].exp;

// BASE21

const BASE21_STEP: i32 = 7;
const BASE21_SMALL_POWERS: [FloatType; BASE21_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 21^0
    FloatType { frac: 12105675798371893248, exp: -59 },         // 21^1
    FloatType { frac: 15888699485363109888, exp: -55 },         // 21^2
    FloatType { frac: 10426959037269540864, exp: -50 },         // 21^3
    FloatType { frac: 13685383736416272384, exp: -46 },         // 21^4
    FloatType { frac: 17962066154046357504, exp: -42 },         // 21^5
    FloatType { frac: 11787605913592922112, exp: -37 },         // 21^6
];
const BASE21_LARGE_POWERS: [FloatType; 69] = [
    FloatType { frac: 17000740844691866712, exp: -1140 },       // 21^-245
    FloatType { frac: 14258473889848767691, exp: -1109 },       // 21^-238
    FloatType { frac: 11958542249702993646, exp: -1078 },       // 21^-231
    FloatType { frac: 10029596003240171126, exp: -1047 },       // 21^-224
    FloatType { frac: 16823588341749525709, exp: -1017 },       // 21^-217
    FloatType { frac: 14109896580142091329, exp: -986 },        // 21^-210
    FloatType { frac: 11833930874797054029, exp: -955 },        // 21^-203
    FloatType { frac: 9925084790952075138, exp: -924 },         // 21^-196
    FloatType { frac: 16648281817731599335, exp: -894 },        // 21^-189
    FloatType { frac: 13962867487806377083, exp: -863 },        // 21^-182
    FloatType { frac: 11710617985478380225, exp: -832 },        // 21^-175
    FloatType { frac: 9821662614901370847, exp: -801 },         // 21^-168
    FloatType { frac: 16474802037018309233, exp: -771 },        // 21^-161
    FloatType { frac: 13817370479981011975, exp: -740 },        // 21^-154
    FloatType { frac: 11588590051161810088, exp: -709 },        // 21^-147
    FloatType { frac: 9719318127024052612, exp: -678 },         // 21^-140
    FloatType { frac: 16303129964430447286, exp: -648 },        // 21^-133
    FloatType { frac: 13673389591914329770, exp: -617 },        // 21^-126
    FloatType { frac: 11467833682254685835, exp: -586 },        // 21^-119
    FloatType { frac: 9618040097506134632, exp: -555 },         // 21^-112
    FloatType { frac: 16133246763140728476, exp: -525 },        // 21^-105
    FloatType { frac: 13530909025211868449, exp: -494 },        // 21^-98
    FloatType { frac: 11348335628687672485, exp: -463 },        // 21^-91
    FloatType { frac: 9517817413551452467, exp: -432 },         // 21^-84
    FloatType { frac: 15965133792606908039, exp: -402 },        // 21^-77
    FloatType { frac: 13389913146102881332, exp: -371 },        // 21^-70
    FloatType { frac: 11230082778460885572, exp: -340 },        // 21^-63
    FloatType { frac: 9418639078162304415, exp: -309 },         // 21^-56
    FloatType { frac: 15798772606526436117, exp: -279 },        // 21^-49
    FloatType { frac: 13250386483724911652, exp: -248 },        // 21^-42
    FloatType { frac: 11113062156205168633, exp: -217 },        // 21^-35
    FloatType { frac: 9320494208932798947, exp: -186 },         // 21^-28
    FloatType { frac: 15634144950812425486, exp: -156 },        // 21^-21
    FloatType { frac: 13112313728426242332, exp: -125 },        // 21^-14
    FloatType { frac: 10997260921758362571, exp: -94 },         // 21^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 21^0
    FloatType { frac: 15471232761590710272, exp: -33 },         // 21^7
    FloatType { frac: 12975679730086034724, exp: -2 },          // 21^14
    FloatType { frac: 10882666368756410705, exp: 29 },          // 21^21
    FloatType { frac: 18254523810272339491, exp: 59 },          // 21^28
    FloatType { frac: 15310018163217775871, exp: 90 },          // 21^35
    FloatType { frac: 12840469496451971963, exp: 121 },         // 21^42
    FloatType { frac: 10769265923239144897, exp: 152 },         // 21^49
    FloatType { frac: 18064306536063374465, exp: 182 },         // 21^56
    FloatType { frac: 15150483466319342608, exp: 213 },         // 21^63
    FloatType { frac: 12706668191495224563, exp: 244 },         // 21^70
    FloatType { frac: 10657047142270599779, exp: 275 },         // 21^77
    FloatType { frac: 17876071379371335714, exp: 305 },         // 21^84
    FloatType { frac: 14992611165849387896, exp: 336 },         // 21^91
    FloatType { frac: 12574261133782557711, exp: 367 },         // 21^98
    FloatType { frac: 10545997712573703694, exp: 398 },         // 21^105
    FloatType { frac: 17689797685974006860, exp: 428 },         // 21^112
    FloatType { frac: 14836383939169393936, exp: 459 },         // 21^119
    FloatType { frac: 12443233794865401683, exp: 490 },         // 21^126
    FloatType { frac: 10436105449179196548, exp: 521 },         // 21^133
    FloatType { frac: 17505465016871978304, exp: 551 },         // 21^140
    FloatType { frac: 14681784644147610193, exp: 582 },         // 21^147
    FloatType { frac: 12313571797685708585, exp: 613 },         // 21^154
    FloatType { frac: 10327358294088626305, exp: 644 },         // 21^161
    FloatType { frac: 17323053146045965028, exp: 674 },         // 21^168
    FloatType { frac: 14528796317278122096, exp: 705 },         // 21^175
    FloatType { frac: 12185260914998420522, exp: 736 },         // 21^182
    FloatType { frac: 10219744314951277448, exp: 767 },         // 21^189
    FloatType { frac: 17142542058237493769, exp: 797 },         // 21^196
    FloatType { frac: 14377402171819519570, exp: 828 },         // 21^203
    FloatType { frac: 12058287067810376090, exp: 859 },         // 21^210
    FloatType { frac: 10113251703754886210, exp: 890 },         // 21^217
    FloatType { frac: 16963911946752716066, exp: 920 },         // 21^224
    FloatType { frac: 14227585595952961160, exp: 951 },         // 21^231
];
const BASE21_BIAS: i32 = -BASE21_LARGE_POWERS[0].exp;

// BASE22

const BASE22_STEP: i32 = 7;
const BASE22_SMALL_POWERS: [FloatType; BASE22_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 22^0
    FloatType { frac: 12682136550675316736, exp: -59 },         // 22^1
    FloatType { frac: 17437937757178560512, exp: -55 },         // 22^2
    FloatType { frac: 11988582208060260352, exp: -50 },         // 22^3
    FloatType { frac: 16484300536082857984, exp: -46 },         // 22^4
    FloatType { frac: 11332956618556964864, exp: -41 },         // 22^5
    FloatType { frac: 15582815350515826688, exp: -37 },         // 22^6
];
const BASE22_LARGE_POWERS: [FloatType; 68] = [
    FloatType { frac: 12505993140104023937, exp: -1156 },       // 22^-245
    FloatType { frac: 14526035001637582317, exp: -1125 },       // 22^-238
    FloatType { frac: 16872365953260472216, exp: -1094 },       // 22^-231
    FloatType { frac: 9798845067792082715, exp: -1062 },        // 22^-224
    FloatType { frac: 11381612386618310709, exp: -1031 },       // 22^-217
    FloatType { frac: 13220037629231758864, exp: -1000 },       // 22^-210
    FloatType { frac: 15355416173176400877, exp: -969 },        // 22^-203
    FloatType { frac: 17835713669231780592, exp: -938 },        // 22^-196
    FloatType { frac: 10358321731667433590, exp: -906 },        // 22^-189
    FloatType { frac: 12031459025026523680, exp: -875 },        // 22^-182
    FloatType { frac: 13974851334106036811, exp: -844 },        // 22^-175
    FloatType { frac: 16232151844936756579, exp: -813 },        // 22^-168
    FloatType { frac: 9427032431967498649, exp: -781 },         // 22^-161
    FloatType { frac: 10949742378252536811, exp: -750 },        // 22^-154
    FloatType { frac: 12718409426865212084, exp: -719 },        // 22^-147
    FloatType { frac: 14772762021382712235, exp: -688 },        // 22^-140
    FloatType { frac: 17158945778190527545, exp: -657 },        // 22^-133
    FloatType { frac: 9965280013064351107, exp: -625 },         // 22^-126
    FloatType { frac: 11574930887071326019, exp: -594 },        // 22^-119
    FloatType { frac: 13444582075449265201, exp: -563 },        // 22^-112
    FloatType { frac: 15616230364311619568, exp: -532 },        // 22^-105
    FloatType { frac: 18138656108661462534, exp: -501 },        // 22^-98
    FloatType { frac: 10534259477248206780, exp: -469 },        // 22^-91
    FloatType { frac: 12235815274209166465, exp: -438 },        // 22^-84
    FloatType { frac: 14212216411407346527, exp: -407 },        // 22^-77
    FloatType { frac: 16507857531195957209, exp: -376 },        // 22^-70
    FloatType { frac: 9587152080358667750, exp: -344 },         // 22^-63
    FloatType { frac: 11135725497779554116, exp: -313 },        // 22^-56
    FloatType { frac: 12934433638113158426, exp: -282 },        // 22^-49
    FloatType { frac: 15023679738882972932, exp: -251 },        // 22^-42
    FloatType { frac: 17450393207123747022, exp: -220 },        // 22^-35
    FloatType { frac: 10134541882409419905, exp: -188 },        // 22^-28
    FloatType { frac: 11771532933066741091, exp: -157 },        // 22^-21
    FloatType { frac: 13672940444874950532, exp: -126 },        // 22^-14
    FloatType { frac: 15881474526053323426, exp: -95 },         // 22^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 22^0
    FloatType { frac: 10713185553479630848, exp: -32 },         // 22^7
    FloatType { frac: 12443642546855641088, exp: -1 },          // 22^14
    FloatType { frac: 14453613172379218947, exp: 30 },          // 22^21
    FloatType { frac: 16788246122479815273, exp: 61 },          // 22^28
    FloatType { frac: 9749991386498543747, exp: 93 },           // 22^35
    FloatType { frac: 11324867570234788254, exp: 124 },         // 22^42
    FloatType { frac: 13154127055020322136, exp: 155 },         // 22^49
    FloatType { frac: 15278859333807672616, exp: 186 },         // 22^56
    FloatType { frac: 17746790917089950882, exp: 217 },         // 22^63
    FloatType { frac: 10306678691583236909, exp: 249 },         // 22^70
    FloatType { frac: 11971474296148943805, exp: 280 },         // 22^77
    FloatType { frac: 13905177517602390611, exp: 311 },         // 22^84
    FloatType { frac: 16151223902158337584, exp: 342 },         // 22^91
    FloatType { frac: 9380032480974399852, exp: 374 },          // 22^98
    FloatType { frac: 10895150717634104284, exp: 405 },         // 22^105
    FloatType { frac: 12654999798852712250, exp: 436 },         // 22^112
    FloatType { frac: 14699110095811391320, exp: 467 },         // 22^119
    FloatType { frac: 17073397158676562691, exp: 498 },         // 22^126
    FloatType { frac: 9915596544207462992, exp: 530 },          // 22^133
    FloatType { frac: 11517222250937216925, exp: 561 },         // 22^140
    FloatType { frac: 13377551999629643946, exp: 592 },         // 22^147
    FloatType { frac: 15538373194824147716, exp: 623 },         // 22^154
    FloatType { frac: 18048222989401488392, exp: 654 },         // 22^161
    FloatType { frac: 10481739271897017716, exp: 686 },         // 22^168
    FloatType { frac: 12174811695150892652, exp: 717 },         // 22^175
    FloatType { frac: 14141359174025375600, exp: 748 },         // 22^182
    FloatType { frac: 16425555014410689631, exp: 779 },         // 22^189
    FloatType { frac: 9539353827706830891, exp: 811 },          // 22^196
    FloatType { frac: 11080206589104387250, exp: 842 },         // 22^203
    FloatType { frac: 12869946987462278079, exp: 873 },         // 22^210
    FloatType { frac: 14948776823616759120, exp: 904 },         // 22^217
    FloatType { frac: 17363391530672110525, exp: 935 },         // 22^224
];
const BASE22_BIAS: i32 = -BASE22_LARGE_POWERS[0].exp;

// BASE23

const BASE23_STEP: i32 = 7;
const BASE23_SMALL_POWERS: [FloatType; BASE23_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 23^0
    FloatType { frac: 13258597302978740224, exp: -59 },         // 23^1
    FloatType { frac: 9529616811515969536, exp: -54 },          // 23^2
    FloatType { frac: 13698824166554206208, exp: -50 },         // 23^3
    FloatType { frac: 9846029869710835712, exp: -45 },          // 23^4
    FloatType { frac: 14153667937709326336, exp: -41 },         // 23^5
    FloatType { frac: 10172948830228578304, exp: -36 },         // 23^6
];
const BASE23_LARGE_POWERS: [FloatType; 67] = [
    FloatType { frac: 12105147475110827234, exp: -1140 },       // 23^-238
    FloatType { frac: 9596327823341159083, exp: -1108 },        // 23^-231
    FloatType { frac: 15214933627595239789, exp: -1077 },       // 23^-224
    FloatType { frac: 12061603644316153100, exp: -1045 },       // 23^-217
    FloatType { frac: 9561808551614073801, exp: -1013 },        // 23^-210
    FloatType { frac: 15160203481036150549, exp: -982 },        // 23^-203
    FloatType { frac: 12018216446491393101, exp: -950 },        // 23^-196
    FloatType { frac: 9527413450313687580, exp: -918 },         // 23^-189
    FloatType { frac: 15105670206117496642, exp: -887 },        // 23^-182
    FloatType { frac: 11974985318206853149, exp: -855 },        // 23^-175
    FloatType { frac: 9493142072782406120, exp: -823 },         // 23^-168
    FloatType { frac: 15051333094665716613, exp: -792 },        // 23^-161
    FloatType { frac: 11931909698059570948, exp: -760 },        // 23^-154
    FloatType { frac: 9458993973969322090, exp: -728 },         // 23^-147
    FloatType { frac: 14997191441054643808, exp: -697 },        // 23^-140
    FloatType { frac: 11888989026666025574, exp: -665 },        // 23^-133
    FloatType { frac: 9424968710424435661, exp: -633 },         // 23^-126
    FloatType { frac: 14943244542196343052, exp: -602 },        // 23^-119
    FloatType { frac: 11846222746654873270, exp: -570 },        // 23^-112
    FloatType { frac: 9391065840292895827, exp: -538 },         // 23^-105
    FloatType { frac: 14889491697531980297, exp: -507 },        // 23^-98
    FloatType { frac: 11803610302659709381, exp: -475 },        // 23^-91
    FloatType { frac: 9357284923309262442, exp: -443 },         // 23^-84
    FloatType { frac: 14835932209022725101, exp: -412 },        // 23^-77
    FloatType { frac: 11761151141311856318, exp: -380 },        // 23^-70
    FloatType { frac: 9323625520791788901, exp: -348 },         // 23^-63
    FloatType { frac: 14782565381140685845, exp: -317 },        // 23^-56
    FloatType { frac: 11718844711233177467, exp: -285 },        // 23^-49
    FloatType { frac: 9290087195636725377, exp: -253 },         // 23^-42
    FloatType { frac: 14729390520859877547, exp: -222 },        // 23^-35
    FloatType { frac: 11676690463028916948, exp: -190 },        // 23^-28
    FloatType { frac: 9256669512312642559, exp: -158 },         // 23^-21
    FloatType { frac: 14676406937647222172, exp: -127 },        // 23^-14
    FloatType { frac: 11634687849280565129, exp: -95 },         // 23^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 23^0
    FloatType { frac: 14623613943453581312, exp: -32 },         // 23^7
    FloatType { frac: 11592836324538749809, exp: 0 },           // 23^14
    FloatType { frac: 18380388673718779295, exp: 31 },          // 23^21
    FloatType { frac: 14571010852704821123, exp: 63 },          // 23^28
    FloatType { frac: 11551135345316152959, exp: 95 },          // 23^35
    FloatType { frac: 18314271962956325083, exp: 126 },         // 23^42
    FloatType { frac: 14518596982292909406, exp: 158 },         // 23^49
    FloatType { frac: 11509584370080452960, exp: 190 },         // 23^56
    FloatType { frac: 18248393082825183718, exp: 221 },         // 23^63
    FloatType { frac: 14466371651567044709, exp: 253 },         // 23^70
    FloatType { frac: 11468182859247292218, exp: 285 },         // 23^77
    FloatType { frac: 18182751177816837937, exp: 316 },         // 23^84
    FloatType { frac: 14414334182324817337, exp: 348 },         // 23^91
    FloatType { frac: 11426930275173270071, exp: 380 },         // 23^98
    FloatType { frac: 18117345395500148774, exp: 411 },         // 23^105
    FloatType { frac: 14362483898803402166, exp: 443 },         // 23^112
    FloatType { frac: 11385826082148960918, exp: 475 },         // 23^119
    FloatType { frac: 18052174886510285819, exp: 506 },         // 23^126
    FloatType { frac: 14310820127670783127, exp: 538 },         // 23^133
    FloatType { frac: 11344869746391957446, exp: 570 },         // 23^140
    FloatType { frac: 17987238804537697299, exp: 601 },         // 23^147
    FloatType { frac: 14259342198017009262, exp: 633 },         // 23^154
    FloatType { frac: 11304060736039938888, exp: 665 },         // 23^161
    FloatType { frac: 17922536306317119829, exp: 696 },         // 23^168
    FloatType { frac: 14208049441345482237, exp: 728 },         // 23^175
    FloatType { frac: 11263398521143764220, exp: 760 },         // 23^182
    FloatType { frac: 17858066551616627705, exp: 791 },         // 23^189
    FloatType { frac: 14156941191564275184, exp: 823 },         // 23^196
    FloatType { frac: 11222882573660590193, exp: 855 },         // 23^203
    FloatType { frac: 17793828703226721580, exp: 886 },         // 23^210
    FloatType { frac: 14106016784977482782, exp: 918 },         // 23^217
    FloatType { frac: 11182512367447014130, exp: 950 },         // 23^224
];
const BASE23_BIAS: i32 = -BASE23_LARGE_POWERS[0].exp;

// BASE24

const BASE24_STEP: i32 = 7;
const BASE24_SMALL_POWERS: [FloatType; BASE24_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 24^0
    FloatType { frac: 13835058055282163712, exp: -59 },         // 24^1
    FloatType { frac: 10376293541461622784, exp: -54 },         // 24^2
    FloatType { frac: 15564440312192434176, exp: -50 },         // 24^3
    FloatType { frac: 11673330234144325632, exp: -45 },         // 24^4
    FloatType { frac: 17509995351216488448, exp: -41 },         // 24^5
    FloatType { frac: 13132496513412366336, exp: -36 },         // 24^6
];
const BASE24_LARGE_POWERS: [FloatType; 66] = [
    FloatType { frac: 15825938165500818674, exp: -1155 },       // 24^-238
    FloatType { frac: 16900061898413227754, exp: -1123 },       // 24^-231
    FloatType { frac: 18047087583901234911, exp: -1091 },       // 24^-224
    FloatType { frac: 9635981578611328308, exp: -1058 },        // 24^-217
    FloatType { frac: 10289986187706530766, exp: -1026 },       // 24^-210
    FloatType { frac: 10988378804938565813, exp: -994 },        // 24^-203
    FloatType { frac: 11734172092969064177, exp: -962 },        // 24^-196
    FloatType { frac: 12530583187169601247, exp: -930 },        // 24^-189
    FloatType { frac: 13381047573408163051, exp: -898 },        // 24^-182
    FloatType { frac: 14289233907736158492, exp: -866 },        // 24^-175
    FloatType { frac: 15259059841903798156, exp: -834 },        // 24^-168
    FloatType { frac: 16294708922970511019, exp: -802 },        // 24^-161
    FloatType { frac: 17400648639910404101, exp: -770 },        // 24^-154
    FloatType { frac: 9290824847530286564, exp: -737 },         // 24^-147
    FloatType { frac: 9921403291771844100, exp: -705 },         // 24^-140
    FloatType { frac: 10594779784719249534, exp: -673 },        // 24^-133
    FloatType { frac: 11313859076748534537, exp: -641 },        // 24^-126
    FloatType { frac: 12081743066820822770, exp: -609 },        // 24^-119
    FloatType { frac: 12901744183172431346, exp: -577 },        // 24^-112
    FloatType { frac: 13777399672167044607, exp: -545 },        // 24^-105
    FloatType { frac: 14712486856947913357, exp: -513 },        // 24^-98
    FloatType { frac: 15711039431711468023, exp: -481 },        // 24^-91
    FloatType { frac: 16777364861891103792, exp: -449 },        // 24^-84
    FloatType { frac: 17916062965310470700, exp: -417 },        // 24^-77
    FloatType { frac: 9566022877229980327, exp: -384 },         // 24^-70
    FloatType { frac: 10215279312745101062, exp: -352 },        // 24^-63
    FloatType { frac: 10908601492662859386, exp: -320 },        // 24^-56
    FloatType { frac: 11648980207252770253, exp: -288 },        // 24^-49
    FloatType { frac: 12439609234991117453, exp: -256 },        // 24^-42
    FloatType { frac: 13283899119592565366, exp: -224 },        // 24^-35
    FloatType { frac: 14185491882103974832, exp: -192 },        // 24^-28
    FloatType { frac: 15148276731524117655, exp: -160 },        // 24^-21
    FloatType { frac: 16176406841720334625, exp: -128 },        // 24^-14
    FloatType { frac: 17274317267012876867, exp: -96 },         // 24^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 24^0
    FloatType { frac: 9849372385059274752, exp: -31 },          // 24^7
    FloatType { frac: 10517860061584293888, exp: 1 },           // 24^14
    FloatType { frac: 11231718727873462272, exp: 33 },          // 24^21
    FloatType { frac: 11994027762626592768, exp: 65 },          // 24^28
    FloatType { frac: 12808075545343924992, exp: 97 },          // 24^35
    FloatType { frac: 13677373641439044901, exp: 129 },         // 24^42
    FloatType { frac: 14605671950110933202, exp: 161 },         // 24^49
    FloatType { frac: 15596974880318657672, exp: 193 },         // 24^56
    FloatType { frac: 16655558624637160317, exp: 225 },         // 24^63
    FloatType { frac: 17785989605508530085, exp: 257 },         // 24^70
    FloatType { frac: 9496572086730262523, exp: 290 },          // 24^77
    FloatType { frac: 10141114821132365302, exp: 322 },         // 24^84
    FloatType { frac: 10829403375886954548, exp: 354 },         // 24^91
    FloatType { frac: 11564406827668344530, exp: 386 },         // 24^98
    FloatType { frac: 12349295767632162835, exp: 418 },         // 24^105
    FloatType { frac: 13187455978423603575, exp: 450 },         // 24^112
    FloatType { frac: 14082503039459189950, exp: 482 },         // 24^119
    FloatType { frac: 15038297923484984581, exp: 514 },         // 24^126
    FloatType { frac: 16058963651690264296, exp: 546 },         // 24^133
    FloatType { frac: 17148903079221976570, exp: 578 },         // 24^140
    FloatType { frac: 18312817887821515019, exp: 610 },         // 24^147
    FloatType { frac: 9777864433756263024, exp: 643 },          // 24^154
    FloatType { frac: 10441498787414525016, exp: 675 },         // 24^161
    FloatType { frac: 11150174730505647564, exp: 707 },         // 24^168
    FloatType { frac: 11906949284968677354, exp: 739 },         // 24^175
    FloatType { frac: 12715086956165281921, exp: 771 },         // 24^182
    FloatType { frac: 13578073815006577911, exp: 803 },         // 24^189
    FloatType { frac: 14499632535849309517, exp: 835 },         // 24^196
    FloatType { frac: 15483738455030488239, exp: 867 },         // 24^203
    FloatType { frac: 16534636719312342666, exp: 899 },         // 24^210
    FloatType { frac: 17656860598210983110, exp: 931 },         // 24^217
];
const BASE24_BIAS: i32 = -BASE24_LARGE_POWERS[0].exp;

// BASE25

const BASE25_STEP: i32 = 7;
const BASE25_SMALL_POWERS: [FloatType; BASE25_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 25^0
    FloatType { frac: 14411518807585587200, exp: -59 },         // 25^1
    FloatType { frac: 11258999068426240000, exp: -54 },         // 25^2
    FloatType { frac: 17592186044416000000, exp: -50 },         // 25^3
    FloatType { frac: 13743895347200000000, exp: -45 },         // 25^4
    FloatType { frac: 10737418240000000000, exp: -40 },         // 25^5
    FloatType { frac: 16777216000000000000, exp: -36 },         // 25^6
];
const BASE25_LARGE_POWERS: [FloatType; 66] = [
    FloatType { frac: 15643822052986917253, exp: -1169 },       // 25^-238
    FloatType { frac: 11115604119273511155, exp: -1136 },       // 25^-231
    FloatType { frac: 15796223521069679172, exp: -1104 },       // 25^-224
    FloatType { frac: 11223891875338892399, exp: -1071 },       // 25^-217
    FloatType { frac: 15950109677957715915, exp: -1039 },       // 25^-210
    FloatType { frac: 11333234566249726012, exp: -1006 },       // 25^-203
    FloatType { frac: 16105494987428025427, exp: -974 },        // 25^-196
    FloatType { frac: 11443642469137689536, exp: -941 },        // 25^-189
    FloatType { frac: 16262394054163123565, exp: -909 },        // 25^-182
    FloatType { frac: 11555125961253852697, exp: -876 },        // 25^-175
    FloatType { frac: 16420821625123739831, exp: -844 },        // 25^-168
    FloatType { frac: 11667695520944036383, exp: -811 },        // 25^-161
    FloatType { frac: 16580792590934885855, exp: -779 },        // 25^-154
    FloatType { frac: 11781361728633673532, exp: -746 },        // 25^-147
    FloatType { frac: 16742321987285426889, exp: -714 },        // 25^-140
    FloatType { frac: 11896135267822264502, exp: -681 },        // 25^-133
    FloatType { frac: 16905424996341287883, exp: -649 },        // 25^-126
    FloatType { frac: 12012026926087520367, exp: -616 },        // 25^-119
    FloatType { frac: 17070116948172426941, exp: -584 },        // 25^-112
    FloatType { frac: 12129047596099288555, exp: -551 },        // 25^-105
    FloatType { frac: 17236413322193710308, exp: -519 },        // 25^-98
    FloatType { frac: 12247208276643356092, exp: -486 },        // 25^-91
    FloatType { frac: 17404329748619824289, exp: -454 },        // 25^-84
    FloatType { frac: 12366520073655226703, exp: -421 },        // 25^-77
    FloatType { frac: 17573882009934360870, exp: -389 },        // 25^-70
    FloatType { frac: 12486994201263968925, exp: -356 },        // 25^-63
    FloatType { frac: 17745086042373215101, exp: -324 },        // 25^-56
    FloatType { frac: 12608641982846233347, exp: -291 },        // 25^-49
    FloatType { frac: 17917957937422433684, exp: -259 },        // 25^-42
    FloatType { frac: 12731474852090538039, exp: -226 },        // 25^-35
    FloatType { frac: 18092513943330655534, exp: -194 },        // 25^-28
    FloatType { frac: 12855504354071922204, exp: -161 },        // 25^-21
    FloatType { frac: 18268770466636286477, exp: -129 },        // 25^-14
    FloatType { frac: 12980742146337069071, exp: -96 },         // 25^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 25^0
    FloatType { frac: 13107200000000000000, exp: -31 },         // 25^7
    FloatType { frac: 9313225746154785156, exp: 2 },            // 25^14
    FloatType { frac: 13234889800848442797, exp: 34 },          // 25^21
    FloatType { frac: 9403954806578300063, exp: 67 },           // 25^28
    FloatType { frac: 13363823550460978230, exp: 99 },          // 25^35
    FloatType { frac: 9495567745759798747, exp: 132 },          // 25^42
    FloatType { frac: 13494013367335069727, exp: 164 },         // 25^49
    FloatType { frac: 9588073174409622174, exp: 197 },          // 25^56
    FloatType { frac: 13625471488026082303, exp: 229 },         // 25^63
    FloatType { frac: 9681479787123295682, exp: 262 },          // 25^70
    FloatType { frac: 13758210268297397763, exp: 294 },         // 25^77
    FloatType { frac: 9775796363198734982, exp: 327 },          // 25^84
    FloatType { frac: 13892242184281734271, exp: 359 },         // 25^91
    FloatType { frac: 9871031767461413346, exp: 392 },          // 25^98
    FloatType { frac: 14027579833653779454, exp: 424 },         // 25^105
    FloatType { frac: 9967194951097567535, exp: 457 },          // 25^112
    FloatType { frac: 14164235936814247246, exp: 489 },         // 25^119
    FloatType { frac: 10064294952495520794, exp: 522 },         // 25^126
    FloatType { frac: 14302223338085469768, exp: 554 },         // 25^133
    FloatType { frac: 10162340898095201970, exp: 587 },         // 25^140
    FloatType { frac: 14441555006918636608, exp: 619 },         // 25^147
    FloatType { frac: 10261342003245940623, exp: 652 },         // 25^154
    FloatType { frac: 14582244039112794984, exp: 684 },         // 25^161
    FloatType { frac: 10361307573072618726, exp: 717 },         // 25^168
    FloatType { frac: 14724303658045725350, exp: 749 },         // 25^175
    FloatType { frac: 10462247003350260393, exp: 782 },         // 25^182
    FloatType { frac: 14867747215916808149, exp: 814 },         // 25^189
    FloatType { frac: 10564169781387141817, exp: 847 },         // 25^196
    FloatType { frac: 15012588195001998509, exp: 879 },         // 25^203
    FloatType { frac: 10667085486916504429, exp: 912 },         // 25^210
    FloatType { frac: 15158840208921026870, exp: 944 },         // 25^217
];
const BASE25_BIAS: i32 = -BASE25_LARGE_POWERS[0].exp;

// BASE26

const BASE26_STEP: i32 = 7;
const BASE26_SMALL_POWERS: [FloatType; BASE26_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 26^0
    FloatType { frac: 14987979559889010688, exp: -59 },         // 26^1
    FloatType { frac: 12177733392409821184, exp: -54 },         // 26^2
    FloatType { frac: 9894408381332979712, exp: -49 },          // 26^3
    FloatType { frac: 16078413619666092032, exp: -45 },         // 26^4
    FloatType { frac: 13063711065978699776, exp: -40 },         // 26^5
    FloatType { frac: 10614265241107693568, exp: -35 },         // 26^6
];
const BASE26_LARGE_POWERS: [FloatType; 65] = [
    FloatType { frac: 10583312905946974966, exp: -1149 },       // 26^-231
    FloatType { frac: 9895670261906581517, exp: -1116 },        // 26^-224
    FloatType { frac: 9252706671590202790, exp: -1083 },        // 26^-217
    FloatType { frac: 17303038295456506514, exp: -1051 },       // 26^-210
    FloatType { frac: 16178786644847745028, exp: -1018 },       // 26^-203
    FloatType { frac: 15127582383507515360, exp: -985 },        // 26^-196
    FloatType { frac: 14144679313308326113, exp: -952 },        // 26^-189
    FloatType { frac: 13225639616708097270, exp: -919 },        // 26^-182
    FloatType { frac: 12366313819957994305, exp: -886 },        // 26^-175
    FloatType { frac: 11562822058185475244, exp: -853 },        // 26^-168
    FloatType { frac: 10811536557764206566, exp: -820 },        // 26^-161
    FloatType { frac: 10109065256878566708, exp: -787 },        // 26^-154
    FloatType { frac: 9452236490329416245, exp: -754 },         // 26^-147
    FloatType { frac: 17676169338865748372, exp: -722 },        // 26^-140
    FloatType { frac: 16527673784713390059, exp: -689 },        // 26^-133
    FloatType { frac: 15453800848879553322, exp: -656 },        // 26^-126
    FloatType { frac: 14449701983936623969, exp: -623 },        // 26^-119
    FloatType { frac: 13510843673109724761, exp: -590 },        // 26^-112
    FloatType { frac: 12632986961401522264, exp: -557 },        // 26^-105
    FloatType { frac: 11812168316666510159, exp: -524 },        // 26^-98
    FloatType { frac: 11044681734222321153, exp: -491 },        // 26^-91
    FloatType { frac: 10327062004200202236, exp: -458 },        // 26^-84
    FloatType { frac: 9656069066086567364, exp: -425 },         // 26^-77
    FloatType { frac: 18057346759632441273, exp: -393 },        // 26^-70
    FloatType { frac: 16884084494735168740, exp: -360 },        // 26^-63
    FloatType { frac: 15787054046203585657, exp: -327 },        // 26^-56
    FloatType { frac: 14761302310200400353, exp: -294 },        // 26^-49
    FloatType { frac: 13802198007013635263, exp: -261 },        // 26^-42
    FloatType { frac: 12905410770780760221, exp: -228 },        // 26^-35
    FloatType { frac: 12066891597841972649, exp: -195 },        // 26^-28
    FloatType { frac: 11282854565446737172, exp: -162 },        // 26^-21
    FloatType { frac: 10549759738273355365, exp: -129 },        // 26^-14
    FloatType { frac: 9864297185584324446, exp: -96 },          // 26^-7
    FloatType { frac: 9223372036854775808, exp: -63 },          // 26^0
    FloatType { frac: 17248181016800002048, exp: -31 },         // 26^7
    FloatType { frac: 16127493675824287744, exp: 2 },           // 26^14
    FloatType { frac: 15079622135830712445, exp: 35 },          // 26^21
    FloatType { frac: 14099835245963182583, exp: 68 },          // 26^28
    FloatType { frac: 13183709258266090507, exp: 101 },         // 26^35
    FloatType { frac: 12327107854416477244, exp: 134 },         // 26^42
    FloatType { frac: 11526163470203963629, exp: 167 },         // 26^49
    FloatType { frac: 10777259833438283283, exp: 200 },         // 26^56
    FloatType { frac: 10077015636442889080, exp: 233 },         // 26^63
    FloatType { frac: 9422269269415772631, exp: 266 },          // 26^70
    FloatType { frac: 17620129091456925542, exp: 298 },         // 26^77
    FloatType { frac: 16475274709425560342, exp: 331 },         // 26^84
    FloatType { frac: 15404806363345084091, exp: 364 },         // 26^91
    FloatType { frac: 14403890877545881062, exp: 397 },         // 26^98
    FloatType { frac: 13468009108242878856, exp: 430 },         // 26^105
    FloatType { frac: 12592935539554553092, exp: 463 },         // 26^112
    FloatType { frac: 11774719205254957782, exp: 496 },         // 26^119
    FloatType { frac: 11009665850120294209, exp: 529 },         // 26^126
    FloatType { frac: 10294321250328313309, exp: 562 },         // 26^133
    FloatType { frac: 9625455617601982106, exp: 595 },          // 26^140
    FloatType { frac: 18000098033363922638, exp: 627 },         // 26^147
    FloatType { frac: 16830555460575262708, exp: 660 },         // 26^154
    FloatType { frac: 15737003020008648959, exp: 693 },         // 26^161
    FloatType { frac: 14714503311068774005, exp: 726 },         // 26^168
    FloatType { frac: 13758439736979533044, exp: 759 },         // 26^175
    FloatType { frac: 12864495660801764695, exp: 792 },         // 26^182
    FloatType { frac: 12028634915772762381, exp: 825 },         // 26^189
    FloatType { frac: 11247083581971537298, exp: 858 },         // 26^196
    FloatType { frac: 10516312947031287874, exp: 891 },         // 26^203
    FloatType { frac: 9833023573966516058, exp: 924 },          // 26^210
    FloatType { frac: 18388260808361729691, exp: 956 },         // 26^217
];
const BASE26_BIAS: i32 = -BASE26_LARGE_POWERS[0].exp;

// BASE27

const BASE27_STEP: i32 = 6;
const BASE27_SMALL_POWERS: [FloatType; BASE27_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 27^0
    FloatType { frac: 15564440312192434176, exp: -59 },         // 27^1
    FloatType { frac: 13132496513412366336, exp: -54 },         // 27^2
    FloatType { frac: 11080543933191684096, exp: -49 },         // 27^3
    FloatType { frac: 9349208943630483456, exp: -44 },          // 27^4
    FloatType { frac: 15776790092376440832, exp: -40 },         // 27^5
];
const BASE27_LARGE_POWERS: [FloatType; 74] = [
    FloatType { frac: 17041067394086403622, exp: -1148 },       // 27^-228
    FloatType { frac: 12297292543386873229, exp: -1119 },       // 27^-222
    FloatType { frac: 17748114058878258402, exp: -1091 },       // 27^-216
    FloatType { frac: 12807516432401518638, exp: -1062 },       // 27^-210
    FloatType { frac: 9242248309993244326, exp: -1033 },        // 27^-204
    FloatType { frac: 13338909893173747895, exp: -1005 },       // 27^-198
    FloatType { frac: 9625716122873707062, exp: -976 },         // 27^-192
    FloatType { frac: 13892351267110242231, exp: -948 },        // 27^-186
    FloatType { frac: 10025094302862174179, exp: -919 },        // 27^-180
    FloatType { frac: 14468755338661289290, exp: -891 },        // 27^-174
    FloatType { frac: 10441042983020688038, exp: -862 },        // 27^-168
    FloatType { frac: 15069074847369989965, exp: -834 },        // 27^-162
    FloatType { frac: 10874249685827050127, exp: -805 },        // 27^-156
    FloatType { frac: 15694302062657520659, exp: -777 },        // 27^-150
    FloatType { frac: 11325430459582219446, exp: -748 },        // 27^-144
    FloatType { frac: 16345470423947416967, exp: -720 },        // 27^-138
    FloatType { frac: 11795331061968106016, exp: -691 },        // 27^-132
    FloatType { frac: 17023656248839843776, exp: -663 },        // 27^-126
    FloatType { frac: 12284728192712064755, exp: -634 },        // 27^-120
    FloatType { frac: 17729980512159296735, exp: -606 },        // 27^-114
    FloatType { frac: 12794430777395563548, exp: -577 },        // 27^-108
    FloatType { frac: 9232805349408163458, exp: -548 },         // 27^-102
    FloatType { frac: 13325281304529035642, exp: -520 },        // 27^-96
    FloatType { frac: 9615881366772943927, exp: -491 },         // 27^-90
    FloatType { frac: 13878157218102970303, exp: -463 },        // 27^-84
    FloatType { frac: 10014851495355986817, exp: -434 },        // 27^-78
    FloatType { frac: 14453972367916992462, exp: -406 },        // 27^-72
    FloatType { frac: 10430375193750279268, exp: -377 },        // 27^-66
    FloatType { frac: 15053678520084183432, exp: -349 },        // 27^-60
    FloatType { frac: 10863139281980340679, exp: -320 },        // 27^-54
    FloatType { frac: 15678266930207358578, exp: -292 },        // 27^-48
    FloatType { frac: 11313859076748534537, exp: -263 },        // 27^-42
    FloatType { frac: 16328769981827608423, exp: -235 },        // 27^-36
    FloatType { frac: 11783279573783601017, exp: -206 },        // 27^-30
    FloatType { frac: 17006262892853298360, exp: -178 },        // 27^-24
    FloatType { frac: 12272176679245716810, exp: -149 },        // 27^-18
    FloatType { frac: 17711865492790087155, exp: -121 },        // 27^-12
    FloatType { frac: 12781358492223474271, exp: -92 },         // 27^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 27^0
    FloatType { frac: 13311666640442621952, exp: -35 },         // 27^6
    FloatType { frac: 9606056659007943744, exp: -6 },           // 27^12
    FloatType { frac: 13863977671394362375, exp: 22 },          // 27^18
    FloatType { frac: 10004619153098548172, exp: 51 },          // 27^24
    FloatType { frac: 14439204501182606065, exp: 79 },          // 27^30
    FloatType { frac: 10419718303939637392, exp: 108 },         // 27^36
    FloatType { frac: 15038297923484984581, exp: 136 },         // 27^42
    FloatType { frac: 10852040229820157048, exp: 165 },         // 27^48
    FloatType { frac: 15662248181121787524, exp: 193 },         // 27^54
    FloatType { frac: 11302299516591361707, exp: 222 },         // 27^60
    FloatType { frac: 16312086602830473207, exp: 250 },         // 27^66
    FloatType { frac: 11771240398807322073, exp: 279 },         // 27^72
    FloatType { frac: 16988887307951181138, exp: 307 },         // 27^78
    FloatType { frac: 12259637989871837542, exp: 336 },         // 27^84
    FloatType { frac: 17693768981840924725, exp: 364 },         // 27^90
    FloatType { frac: 12768299563225066619, exp: 393 },         // 27^96
    FloatType { frac: 18427896724951050158, exp: 421 },         // 27^102
    FloatType { frac: 13298065886687551351, exp: 450 },         // 27^108
    FloatType { frac: 9596241989312152815, exp: 479 },          // 27^114
    FloatType { frac: 13849812612167175924, exp: 507 },         // 27^120
    FloatType { frac: 9994397265397337538, exp: 536 },          // 27^126
    FloatType { frac: 14424451723026109070, exp: 564 },         // 27^132
    FloatType { frac: 10409072302452601000, exp: 593 },         // 27^138
    FloatType { frac: 15022933041500086259, exp: 621 },         // 27^144
    FloatType { frac: 10840952517748290136, exp: 650 },         // 27^150
    FloatType { frac: 15646245798661648271, exp: 678 },         // 27^156
    FloatType { frac: 11290751767031273467, exp: 707 },         // 27^162
    FloatType { frac: 16295420269522331823, exp: 735 },         // 27^168
    FloatType { frac: 11759213524458657188, exp: 764 },         // 27^174
    FloatType { frac: 16971529475976476179, exp: 792 },         // 27^180
    FloatType { frac: 12247112111487835932, exp: 821 },         // 27^186
    FloatType { frac: 17675690960401445308, exp: 849 },         // 27^192
    FloatType { frac: 12755253976754113245, exp: 878 },         // 27^198
    FloatType { frac: 18409068632845853217, exp: 906 },         // 27^204
    FloatType { frac: 13284479029051404288, exp: 935 },         // 27^210
];
const BASE27_BIAS: i32 = -BASE27_LARGE_POWERS[0].exp;

// BASE28

const BASE28_STEP: i32 = 6;
const BASE28_SMALL_POWERS: [FloatType; BASE28_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 28^0
    FloatType { frac: 16140901064495857664, exp: -59 },         // 28^1
    FloatType { frac: 14123288431433875456, exp: -54 },         // 28^2
    FloatType { frac: 12357877377504641024, exp: -49 },         // 28^3
    FloatType { frac: 10813142705316560896, exp: -44 },         // 28^4
    FloatType { frac: 9461499867151990784, exp: -39 },          // 28^5
];
const BASE28_LARGE_POWERS: [FloatType; 74] = [
    FloatType { frac: 17488953546307848045, exp: -1160 },       // 28^-228
    FloatType { frac: 15697920957714630238, exp: -1131 },       // 28^-222
    FloatType { frac: 14090306875260685218, exp: -1102 },       // 28^-216
    FloatType { frac: 12647327526607851831, exp: -1073 },       // 28^-210
    FloatType { frac: 11352122773573968201, exp: -1044 },       // 28^-204
    FloatType { frac: 10189559113984709052, exp: -1015 },       // 28^-198
    FloatType { frac: 18292105715960495534, exp: -987 },        // 28^-192
    FloatType { frac: 16418822825447359765, exp: -958 },        // 28^-186
    FloatType { frac: 14737381642082644874, exp: -929 },        // 28^-180
    FloatType { frac: 13228135778880165762, exp: -900 },        // 28^-174
    FloatType { frac: 11873450822826176619, exp: -871 },        // 28^-168
    FloatType { frac: 10657498289906897377, exp: -842 },        // 28^-162
    FloatType { frac: 9566070681070377880, exp: -813 },         // 28^-156
    FloatType { frac: 17172830956378919788, exp: -785 },        // 28^-150
    FloatType { frac: 15414172280784786485, exp: -756 },        // 28^-144
    FloatType { frac: 13835616719528574716, exp: -727 },        // 28^-138
    FloatType { frac: 12418720027433908743, exp: -698 },        // 28^-132
    FloatType { frac: 11146926822720122755, exp: -669 },        // 28^-126
    FloatType { frac: 10005377149705503250, exp: -640 },        // 28^-120
    FloatType { frac: 17961465702601665525, exp: -612 },        // 28^-114
    FloatType { frac: 16122043445170466212, exp: -583 },        // 28^-108
    FloatType { frac: 14470995249030000148, exp: -554 },        // 28^-102
    FloatType { frac: 12989029846596759700, exp: -525 },        // 28^-96
    FloatType { frac: 11658831576707932907, exp: -496 },        // 28^-90
    FloatType { frac: 10464858064026730335, exp: -467 },        // 28^-84
    FloatType { frac: 9393158617970892313, exp: -438 },         // 28^-78
    FloatType { frac: 16862422458582420498, exp: -410 },        // 28^-72
    FloatType { frac: 15135552519453149331, exp: -381 },        // 28^-66
    FloatType { frac: 13585530230416439557, exp: -352 },        // 28^-60
    FloatType { frac: 12194244736314878063, exp: -323 },        // 28^-54
    FloatType { frac: 10945439903127358164, exp: -294 },        // 28^-48
    FloatType { frac: 9824524377159351811, exp: -265 },         // 28^-42
    FloatType { frac: 17636802191900948811, exp: -237 },        // 28^-36
    FloatType { frac: 15830628517722738088, exp: -208 },        // 28^-30
    FloatType { frac: 14209423938610553080, exp: -179 },        // 28^-24
    FloatType { frac: 12754245887402290033, exp: -150 },        // 28^-18
    FloatType { frac: 11448091693168579255, exp: -121 },        // 28^-12
    FloatType { frac: 10275699917675706335, exp: -92 },         // 28^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 28^0
    FloatType { frac: 16557624767515983872, exp: -35 },         // 28^6
    FloatType { frac: 14861968965709594624, exp: -6 },          // 28^12
    FloatType { frac: 13339964194082398208, exp: 23 },          // 28^18
    FloatType { frac: 11973826961285400900, exp: 52 },          // 28^24
    FloatType { frac: 10747594971986893695, exp: 81 },          // 28^30
    FloatType { frac: 9646940619348801089, exp: 110 },          // 28^36
    FloatType { frac: 17318007155239366140, exp: 138 },         // 28^42
    FloatType { frac: 15544481077627229210, exp: 167 },         // 28^48
    FloatType { frac: 13952580675520064463, exp: 196 },         // 28^54
    FloatType { frac: 12523705779222565186, exp: 225 },         // 28^60
    FloatType { frac: 11241161050565762112, exp: 254 },         // 28^66
    FloatType { frac: 10089960910324183248, exp: 283 },         // 28^72
    FloatType { frac: 18113308885783841476, exp: 311 },         // 28^78
    FloatType { frac: 16258336464718499495, exp: 340 },         // 28^84
    FloatType { frac: 14593330587292989709, exp: 369 },         // 28^90
    FloatType { frac: 13098836900821174211, exp: 398 },         // 28^96
    FloatType { frac: 11757393360479052160, exp: 427 },         // 28^102
    FloatType { frac: 10553326198326110898, exp: 456 },         // 28^108
    FloatType { frac: 9472566787009190529, exp: 485 },          // 28^114
    FloatType { frac: 17004974516675479989, exp: 513 },         // 28^120
    FloatType { frac: 15263505912112072336, exp: 542 },         // 28^126
    FloatType { frac: 13700379997665963732, exp: 571 },         // 28^132
    FloatType { frac: 12297332812083457696, exp: 600 },         // 28^138
    FloatType { frac: 11037970794744924274, exp: 629 },         // 28^144
    FloatType { frac: 9907579239127697723, exp: 658 },          // 28^150
    FloatType { frac: 17785900724855568076, exp: 686 },         // 28^156
    FloatType { frac: 15964457964924108341, exp: 715 },         // 28^162
    FloatType { frac: 14329547997401095751, exp: 744 },         // 28^168
    FloatType { frac: 12862068117875988113, exp: 773 },         // 28^174
    FloatType { frac: 11544871917724549298, exp: 802 },         // 28^180
    FloatType { frac: 10362568941103939059, exp: 831 },         // 28^186
    FloatType { frac: 9301344858947275744, exp: 860 },          // 28^192
    FloatType { frac: 16697600117649658875, exp: 888 },         // 28^198
    FloatType { frac: 14987609529429357277, exp: 917 },         // 28^204
    FloatType { frac: 13452737987730670580, exp: 946 },         // 28^210
];
const BASE28_BIAS: i32 = -BASE28_LARGE_POWERS[0].exp;

// BASE29

const BASE29_STEP: i32 = 6;
const BASE29_SMALL_POWERS: [FloatType; BASE29_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 29^0
    FloatType { frac: 16717361816799281152, exp: -59 },         // 29^1
    FloatType { frac: 15150109146474348544, exp: -54 },         // 29^2
    FloatType { frac: 13729786413992378368, exp: -49 },         // 29^3
    FloatType { frac: 12442618937680592896, exp: -44 },         // 29^4
    FloatType { frac: 11276123412273037312, exp: -39 },         // 29^5
];
const BASE29_LARGE_POWERS: [FloatType; 73] = [
    FloatType { frac: 13301466184228767173, exp: -1142 },       // 29^-222
    FloatType { frac: 14737289938837575007, exp: -1113 },       // 29^-216
    FloatType { frac: 16328103363438049788, exp: -1084 },       // 29^-210
    FloatType { frac: 18090636782853846909, exp: -1055 },       // 29^-204
    FloatType { frac: 10021713236516230810, exp: -1025 },       // 29^-198
    FloatType { frac: 11103504802015131117, exp: -996 },        // 29^-192
    FloatType { frac: 12302070113036945059, exp: -967 },        // 29^-186
    FloatType { frac: 13630014285094069421, exp: -938 },        // 29^-180
    FloatType { frac: 15101303090037955289, exp: -909 },        // 29^-174
    FloatType { frac: 16731409831799452344, exp: -880 },        // 29^-168
    FloatType { frac: 9268739038298839376, exp: -850 },         // 29^-162
    FloatType { frac: 10269250974512215384, exp: -821 },        // 29^-156
    FloatType { frac: 11377762945074294339, exp: -792 },        // 29^-150
    FloatType { frac: 12605932989231929836, exp: -763 },        // 29^-144
    FloatType { frac: 13966677570638048918, exp: -734 },        // 29^-138
    FloatType { frac: 15474307417689145256, exp: -705 },        // 29^-132
    FloatType { frac: 17144678027117236568, exp: -676 },        // 29^-126
    FloatType { frac: 9497678206828984163, exp: -646 },         // 29^-120
    FloatType { frac: 10522902929736936910, exp: -617 },        // 29^-114
    FloatType { frac: 11658795303156142066, exp: -588 },        // 29^-108
    FloatType { frac: 12917301321555931503, exp: -559 },        // 29^-102
    FloatType { frac: 14311656487072982013, exp: -530 },        // 29^-96
    FloatType { frac: 15856525005124406387, exp: -501 },        // 29^-90
    FloatType { frac: 17568154005459773215, exp: -472 },        // 29^-84
    FloatType { frac: 9732272205284828726, exp: -442 },         // 29^-78
    FloatType { frac: 10782820124222926001, exp: -413 },        // 29^-72
    FloatType { frac: 11946769200332301461, exp: -384 },        // 29^-66
    FloatType { frac: 13236360495839591806, exp: -355 },        // 29^-60
    FloatType { frac: 14665156431661011058, exp: -326 },        // 29^-54
    FloatType { frac: 16248183421166822582, exp: -297 },        // 29^-48
    FloatType { frac: 18002089900515211938, exp: -268 },        // 29^-42
    FloatType { frac: 9972660708767378680, exp: -238 },         // 29^-36
    FloatType { frac: 11049157310268331325, exp: -209 },        // 29^-30
    FloatType { frac: 12241856093602695018, exp: -180 },        // 29^-24
    FloatType { frac: 13563300476969856592, exp: -151 },        // 29^-18
    FloatType { frac: 15027387875005778139, exp: -122 },        // 29^-12
    FloatType { frac: 16649515855621676607, exp: -93 },         // 29^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 29^0
    FloatType { frac: 10218986842372440064, exp: -34 },         // 29^6
    FloatType { frac: 11322073062575009312, exp: -5 },          // 29^12
    FloatType { frac: 12544231674979455490, exp: 24 },          // 29^18
    FloatType { frac: 13898315921993315819, exp: 53 },          // 29^24
    FloatType { frac: 15398566486364679514, exp: 82 },          // 29^30
    FloatType { frac: 17060761258115507449, exp: 111 },         // 29^36
    FloatType { frac: 9451190634016324153, exp: 141 },          // 29^42
    FloatType { frac: 10471397266405980105, exp: 170 },         // 29^48
    FloatType { frac: 11601729872662437745, exp: 199 },         // 29^54
    FloatType { frac: 12854075976092328003, exp: 228 },         // 29^60
    FloatType { frac: 14241606296013212103, exp: 257 },         // 29^66
    FloatType { frac: 15778913262056350490, exp: 286 },         // 29^72
    FloatType { frac: 17482164480364512070, exp: 315 },         // 29^78
    FloatType { frac: 9684636382831873707, exp: 345 },          // 29^84
    FloatType { frac: 10730042263704319491, exp: 374 },         // 29^90
    FloatType { frac: 11888294245613666403, exp: 403 },         // 29^96
    FloatType { frac: 13171573475377840424, exp: 432 },         // 29^102
    FloatType { frac: 14593375990949121808, exp: 461 },         // 29^108
    FloatType { frac: 16168654657039833397, exp: 490 },         // 29^114
    FloatType { frac: 17913976421956996862, exp: 519 },         // 29^120
    FloatType { frac: 9923848274745194421, exp: 549 },          // 29^126
    FloatType { frac: 10995075829111145761, exp: 578 },         // 29^132
    FloatType { frac: 12181936799210906401, exp: 607 },         // 29^138
    FloatType { frac: 13496913207916061440, exp: 636 },         // 29^144
    FloatType { frac: 14953834447230017102, exp: 665 },         // 29^150
    FloatType { frac: 16568022719743769659, exp: 694 },         // 29^156
    FloatType { frac: 18356454179736676324, exp: 723 },         // 29^162
    FloatType { frac: 10168968734308426004, exp: 753 },         // 29^168
    FloatType { frac: 11266655761164621625, exp: 782 },         // 29^174
    FloatType { frac: 12482832365519782645, exp: 811 },         // 29^180
    FloatType { frac: 13830288877980342143, exp: 840 },         // 29^186
    FloatType { frac: 15323196278493163895, exp: 869 },         // 29^192
    FloatType { frac: 16977255230225892037, exp: 898 },         // 29^198
    FloatType { frac: 9404930600437880197, exp: 928 },          // 29^204
    FloatType { frac: 10420143703980341466, exp: 957 },         // 29^210
];
const BASE29_BIAS: i32 = -BASE29_LARGE_POWERS[0].exp;

// BASE30

const BASE30_STEP: i32 = 6;
const BASE30_SMALL_POWERS: [FloatType; BASE30_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 30^0
    FloatType { frac: 17293822569102704640, exp: -59 },         // 30^1
    FloatType { frac: 16212958658533785600, exp: -54 },         // 30^2
    FloatType { frac: 15199648742375424000, exp: -49 },         // 30^3
    FloatType { frac: 14249670695976960000, exp: -44 },         // 30^4
    FloatType { frac: 13359066277478400000, exp: -39 },         // 30^5
];
const BASE30_LARGE_POWERS: [FloatType; 72] = [
    FloatType { frac: 14677985448278451843, exp: -1153 },       // 30^-222
    FloatType { frac: 9965385675239368708, exp: -1123 },        // 30^-216
    FloatType { frac: 13531681443098000788, exp: -1094 },       // 30^-210
    FloatType { frac: 18374241463874359754, exp: -1065 },       // 30^-204
    FloatType { frac: 12474900136854879801, exp: -1035 },       // 30^-198
    FloatType { frac: 16939271613521887687, exp: -1006 },       // 30^-192
    FloatType { frac: 11500650091336533543, exp: -976 },        // 30^-186
    FloatType { frac: 15616368347004676150, exp: -947 },        // 30^-180
    FloatType { frac: 10602485877430447296, exp: -917 },        // 30^-174
    FloatType { frac: 14396779620362065880, exp: -888 },        // 30^-168
    FloatType { frac: 9774465433549085656, exp: -858 },         // 30^-162
    FloatType { frac: 13272436896445757604, exp: -829 },        // 30^-156
    FloatType { frac: 18022221508452589239, exp: -800 },        // 30^-150
    FloatType { frac: 12235901765210495847, exp: -770 },        // 30^-144
    FloatType { frac: 16614743297618723424, exp: -741 },        // 30^-138
    FloatType { frac: 11280316732790367097, exp: -711 },        // 30^-132
    FloatType { frac: 15317184660964044954, exp: -682 },        // 30^-126
    FloatType { frac: 10399359853791807565, exp: -652 },        // 30^-120
    FloatType { frac: 14120961229157126909, exp: -623 },        // 30^-114
    FloatType { frac: 9587202906660312336, exp: -593 },         // 30^-108
    FloatType { frac: 13018159044823362852, exp: -564 },        // 30^-102
    FloatType { frac: 17676945670836105047, exp: -535 },        // 30^-96
    FloatType { frac: 12001482205502242389, exp: -505 },        // 30^-90
    FloatType { frac: 16296432405358431306, exp: -476 },        // 30^-84
    FloatType { frac: 11064204595523231124, exp: -446 },        // 30^-78
    FloatType { frac: 15023732837543702665, exp: -417 },        // 30^-72
    FloatType { frac: 10200125387468709836, exp: -387 },        // 30^-66
    FloatType { frac: 13850427060322257636, exp: -358 },        // 30^-60
    FloatType { frac: 9403528018831206315, exp: -328 },         // 30^-54
    FloatType { frac: 12768752734601403407, exp: -299 },        // 30^-48
    FloatType { frac: 17338284744926585040, exp: -270 },        // 30^-42
    FloatType { frac: 11771553735296689434, exp: -240 },        // 30^-36
    FloatType { frac: 15984219821228248249, exp: -211 },        // 30^-30
    FloatType { frac: 10852232807944894743, exp: -181 },        // 30^-24
    FloatType { frac: 14735903063773789011, exp: -152 },        // 30^-18
    FloatType { frac: 10004707922685045925, exp: -122 },        // 30^-12
    FloatType { frac: 13585075876931470780, exp: -93 },         // 30^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 30^0
    FloatType { frac: 12524124635136000000, exp: -34 },         // 30^6
    FloatType { frac: 17006112000000000000, exp: -5 },          // 30^12
    FloatType { frac: 11546030312776565551, exp: 25 },          // 30^18
    FloatType { frac: 15677988711770840524, exp: 54 },          // 30^24
    FloatType { frac: 10644322047830505987, exp: 84 },          // 30^30
    FloatType { frac: 14453587630518598230, exp: 113 },         // 30^36
    FloatType { frac: 9813034332029575584, exp: 143 },          // 30^42
    FloatType { frac: 13324808381590173768, exp: 172 },         // 30^48
    FloatType { frac: 18093335088676282534, exp: 201 },         // 30^54
    FloatType { frac: 12284183203843431517, exp: 231 },         // 30^60
    FloatType { frac: 16680303133282552614, exp: 260 },         // 30^66
    FloatType { frac: 11324827544542942993, exp: 290 },         // 30^72
    FloatType { frac: 15377624481863911156, exp: 319 },         // 30^78
    FloatType { frac: 10440394512637323916, exp: 349 },         // 30^84
    FloatType { frac: 14176680892170610158, exp: 378 },         // 30^90
    FloatType { frac: 9625032889090827484, exp: 408 },          // 30^96
    FloatType { frac: 13069527179276967861, exp: 437 },         // 30^102
    FloatType { frac: 17746696832949127203, exp: 466 },         // 30^108
    FloatType { frac: 12048838651943871501, exp: 496 },         // 30^114
    FloatType { frac: 16360736223435182728, exp: 525 },         // 30^120
    FloatType { frac: 11107862654034279481, exp: 555 },         // 30^126
    FloatType { frac: 15083014731837417449, exp: 584 },         // 30^132
    FloatType { frac: 10240373890390132852, exp: 614 },         // 30^138
    FloatType { frac: 13905079227116716745, exp: 643 },         // 30^144
    FloatType { frac: 9440633241616270046, exp: 673 },          // 30^150
    FloatType { frac: 12819136740897336720, exp: 702 },         // 30^156
    FloatType { frac: 17406699590597596894, exp: 731 },         // 30^162
    FloatType { frac: 11818002910861417777, exp: 761 },         // 30^168
    FloatType { frac: 16047291684929232224, exp: 790 },         // 30^174
    FloatType { frac: 10895054450550498712, exp: 820 },         // 30^180
    FloatType { frac: 14794049215412351417, exp: 849 },         // 30^186
    FloatType { frac: 10044185331124443731, exp: 879 },         // 30^192
    FloatType { frac: 13638680998961850032, exp: 908 },         // 30^198
    FloatType { frac: 9259766385185707988, exp: 938 },          // 30^204
];
const BASE30_BIAS: i32 = -BASE30_LARGE_POWERS[0].exp;

// BASE31

const BASE31_STEP: i32 = 6;
const BASE31_SMALL_POWERS: [FloatType; BASE31_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 31^0
    FloatType { frac: 17870283321406128128, exp: -59 },         // 31^1
    FloatType { frac: 17311836967612186624, exp: -54 },         // 31^2
    FloatType { frac: 16770842062374305792, exp: -49 },         // 31^3
    FloatType { frac: 16246753247925108736, exp: -44 },         // 31^4
    FloatType { frac: 15739042208927449088, exp: -39 },         // 31^5
];
const BASE31_LARGE_POWERS: [FloatType; 72] = [
    FloatType { frac: 10365468257053156090, exp: -1163 },       // 31^-222
    FloatType { frac: 17135201456813756829, exp: -1134 },       // 31^-216
    FloatType { frac: 14163138687236953263, exp: -1104 },       // 31^-210
    FloatType { frac: 11706573627364173290, exp: -1074 },       // 31^-204
    FloatType { frac: 9676094340331131706, exp: -1044 },        // 31^-198
    FloatType { frac: 15995594383677739926, exp: -1015 },       // 31^-192
    FloatType { frac: 13221193938792609146, exp: -985 },        // 31^-186
    FloatType { frac: 10928007110853986174, exp: -955 },        // 31^-180
    FloatType { frac: 18065136926019727780, exp: -926 },        // 31^-174
    FloatType { frac: 14931778907414090897, exp: -896 },        // 31^-168
    FloatType { frac: 12341894902482781431, exp: -866 },        // 31^-162
    FloatType { frac: 10201220546354171409, exp: -836 },        // 31^-156
    FloatType { frac: 16863682839241173595, exp: -807 },        // 31^-150
    FloatType { frac: 13938714372965575021, exp: -777 },        // 31^-144
    FloatType { frac: 11521075213714088050, exp: -747 },        // 31^-138
    FloatType { frac: 9522770215989197442, exp: -717 },         // 31^-132
    FloatType { frac: 15742133595063495983, exp: -688 },        // 31^-126
    FloatType { frac: 13011695363011784957, exp: -658 },        // 31^-120
    FloatType { frac: 10754845599386459585, exp: -628 },        // 31^-114
    FloatType { frac: 17778882864941161542, exp: -599 },        // 31^-108
    FloatType { frac: 14695174979700806287, exp: -569 },        // 31^-102
    FloatType { frac: 12146329402386737855, exp: -539 },        // 31^-96
    FloatType { frac: 10039575449430160254, exp: -509 },        // 31^-90
    FloatType { frac: 16596466614020050649, exp: -480 },        // 31^-84
    FloatType { frac: 13717846210614220387, exp: -450 },        // 31^-78
    FloatType { frac: 11338516145303865769, exp: -420 },        // 31^-72
    FloatType { frac: 9371875613960541536, exp: -390 },         // 31^-66
    FloatType { frac: 15492689060539222513, exp: -361 },        // 31^-60
    FloatType { frac: 12805516430937677459, exp: -331 },        // 31^-54
    FloatType { frac: 10584427946771654325, exp: -301 },        // 31^-48
    FloatType { frac: 17497164689077279120, exp: -272 },        // 31^-42
    FloatType { frac: 14462320197950402007, exp: -242 },        // 31^-36
    FloatType { frac: 11953862767183809017, exp: -212 },        // 31^-30
    FloatType { frac: 9880491726141866768, exp: -182 },         // 31^-24
    FloatType { frac: 16333484606893641287, exp: -153 },        // 31^-18
    FloatType { frac: 13500477850600094178, exp: -123 },        // 31^-12
    FloatType { frac: 11158849846261135900, exp: -93 },         // 31^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 31^0
    FloatType { frac: 15247197139898466304, exp: -34 },         // 31^6
    FloatType { frac: 12602604540616796176, exp: -4 },          // 31^12
    FloatType { frac: 10416710674748495795, exp: 26 },          // 31^18
    FloatType { frac: 17219910524322248562, exp: 55 },          // 31^24
    FloatType { frac: 14233155154461633068, exp: 85 },          // 31^30
    FloatType { frac: 11764445893307051548, exp: 115 },         // 31^36
    FloatType { frac: 9723928789827359386, exp: 145 },          // 31^42
    FloatType { frac: 16074669723871456105, exp: 174 },         // 31^48
    FloatType { frac: 13286553836236866995, exp: 204 },         // 31^54
    FloatType { frac: 10982030478739077827, exp: 234 },         // 31^60
    FloatType { frac: 18154443194596327488, exp: 263 },         // 31^66
    FloatType { frac: 15005595201356001156, exp: 293 },         // 31^72
    FloatType { frac: 12402907923608447627, exp: 323 },         // 31^78
    FloatType { frac: 10251650993997756459, exp: 353 },         // 31^84
    FloatType { frac: 16947049635463054782, exp: 382 },         // 31^90
    FloatType { frac: 14007621382887632827, exp: 412 },         // 31^96
    FloatType { frac: 11578030455268066882, exp: 442 },         // 31^102
    FloatType { frac: 9569846696947249770, exp: 472 },          // 31^108
    FloatType { frac: 15819955934111728582, exp: 501 },         // 31^114
    FloatType { frac: 13076019589586139267, exp: 531 },         // 31^120
    FloatType { frac: 10808012931221917147, exp: 561 },         // 31^126
    FloatType { frac: 17866774016535005152, exp: 590 },         // 31^132
    FloatType { frac: 14767821605568725557, exp: 620 },         // 31^138
    FloatType { frac: 12206375631777172937, exp: 650 },         // 31^144
    FloatType { frac: 10089206793225315915, exp: 680 },         // 31^150
    FloatType { frac: 16678512408132988469, exp: 709 },         // 31^156
    FloatType { frac: 13785661343319529298, exp: 739 },         // 31^162
    FloatType { frac: 11394568893327831301, exp: 769 },         // 31^168
    FloatType { frac: 9418206136893990095, exp: 799 },          // 31^174
    FloatType { frac: 15569278253075119325, exp: 828 },         // 31^180
    FloatType { frac: 12868821397533098208, exp: 858 },         // 31^186
    FloatType { frac: 10636752807015729117, exp: 888 },         // 31^192
    FloatType { frac: 17583663147154342787, exp: 917 },         // 31^198
    FloatType { frac: 14533815689909759814, exp: 947 },         // 31^204
];
const BASE31_BIAS: i32 = -BASE31_LARGE_POWERS[0].exp;

// BASE33

const BASE33_STEP: i32 = 6;
const BASE33_SMALL_POWERS: [FloatType; BASE33_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 33^0
    FloatType { frac: 9511602413006487552, exp: -58 },          // 33^1
    FloatType { frac: 9808839988412940288, exp: -53 },          // 33^2
    FloatType { frac: 10115366238050844672, exp: -48 },         // 33^3
    FloatType { frac: 10431471432989933568, exp: -43 },         // 33^4
    FloatType { frac: 10757454915270868992, exp: -38 },         // 33^5
];
const BASE33_LARGE_POWERS: [FloatType; 70] = [
    FloatType { frac: 12262357879342609130, exp: -1153 },       // 33^-216
    FloatType { frac: 14748836332546310936, exp: -1123 },       // 33^-210
    FloatType { frac: 17739506162243888511, exp: -1093 },       // 33^-204
    FloatType { frac: 10668301952265249182, exp: -1062 },       // 33^-198
    FloatType { frac: 12831548466319904021, exp: -1032 },       // 33^-192
    FloatType { frac: 15433443558330863109, exp: -1002 },       // 33^-186
    FloatType { frac: 9281466718275888268, exp: -971 },         // 33^-180
    FloatType { frac: 11163500111543439891, exp: -941 },        // 33^-174
    FloatType { frac: 13427159577595330562, exp: -911 },        // 33^-168
    FloatType { frac: 16149828684624228128, exp: -881 },        // 33^-162
    FloatType { frac: 9712291160146516482, exp: -850 },         // 33^-156
    FloatType { frac: 11681684235978010467, exp: -820 },        // 33^-150
    FloatType { frac: 14050417593436164695, exp: -790 },        // 33^-144
    FloatType { frac: 16899466768835551431, exp: -760 },        // 33^-138
    FloatType { frac: 10163113486548439647, exp: -729 },        // 33^-132
    FloatType { frac: 12223921281461810852, exp: -699 },        // 33^-126
    FloatType { frac: 14702605819874780450, exp: -669 },        // 33^-120
    FloatType { frac: 17683901337162836029, exp: -639 },        // 33^-114
    FloatType { frac: 10634861953510936381, exp: -608 },        // 33^-108
    FloatType { frac: 12791327729538214791, exp: -578 },        // 33^-102
    FloatType { frac: 15385067131072375518, exp: -548 },        // 33^-96
    FloatType { frac: 9252373781378705800, exp: -517 },         // 33^-90
    FloatType { frac: 11128507904583594948, exp: -487 },        // 33^-84
    FloatType { frac: 13385071886268464065, exp: -457 },        // 33^-78
    FloatType { frac: 16099206734335173177, exp: -427 },        // 33^-72
    FloatType { frac: 9681847795705762071, exp: -396 },         // 33^-66
    FloatType { frac: 11645067770860388376, exp: -366 },        // 33^-60
    FloatType { frac: 14006376287807173243, exp: -336 },        // 33^-54
    FloatType { frac: 16846495063476347823, exp: -306 },        // 33^-48
    FloatType { frac: 10131257010808365886, exp: -275 },        // 33^-42
    FloatType { frac: 12185605163840289543, exp: -245 },        // 33^-36
    FloatType { frac: 14656520217639143557, exp: -215 },        // 33^-30
    FloatType { frac: 17628470806481188820, exp: -185 },        // 33^-24
    FloatType { frac: 10601526773079323099, exp: -154 },        // 33^-18
    FloatType { frac: 12751233065433685927, exp: -124 },        // 33^-12
    FloatType { frac: 15336842340660548274, exp: -94 },         // 33^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 33^0
    FloatType { frac: 11093625381373083648, exp: -33 },         // 33^6
    FloatType { frac: 13343116119623879688, exp: -3 },          // 33^12
    FloatType { frac: 16048743459528137878, exp: 27 },          // 33^18
    FloatType { frac: 9651499856579507665, exp: 58 },           // 33^24
    FloatType { frac: 11608566080760702539, exp: 88 },          // 33^30
    FloatType { frac: 13962473030502269494, exp: 118 },         // 33^36
    FloatType { frac: 16793689398951866695, exp: 148 },         // 33^42
    FloatType { frac: 10099500389807484117, exp: 179 },         // 33^48
    FloatType { frac: 12147409148830342864, exp: 209 },         // 33^54
    FloatType { frac: 14610579071614836924, exp: 239 },         // 33^60
    FloatType { frac: 17573214023869781748, exp: 269 },         // 33^66
    FloatType { frac: 10568296082415350040, exp: 300 },         // 33^72
    FloatType { frac: 12711264078829073096, exp: 330 },         // 33^78
    FloatType { frac: 15288768711786753433, exp: 360 },         // 33^84
    FloatType { frac: 18388922397719682024, exp: 390 },         // 33^90
    FloatType { frac: 11058852198106072831, exp: 421 },         // 33^96
    FloatType { frac: 13301291864141109889, exp: 451 },         // 33^102
    FloatType { frac: 15998438362831755651, exp: 481 },         // 33^108
    FloatType { frac: 9621247043655259795, exp: 512 },          // 33^114
    FloatType { frac: 11572178805914439916, exp: 542 },         // 33^120
    FloatType { frac: 13918707388806312258, exp: 572 },         // 33^126
    FloatType { frac: 16741049254803901004, exp: 602 },         // 33^132
    FloatType { frac: 10067843310549183526, exp: 633 },         // 33^138
    FloatType { frac: 12109332859968012500, exp: 663 },         // 33^144
    FloatType { frac: 14564781929001072895, exp: 693 },         // 33^150
    FloatType { frac: 17518130444711929011, exp: 723 },         // 33^156
    FloatType { frac: 10535169553993820096, exp: 754 },         // 33^162
    FloatType { frac: 12671420375785822681, exp: 784 },         // 33^168
    FloatType { frac: 15240845770632227134, exp: 814 },         // 33^174
    FloatType { frac: 18331281964891256972, exp: 844 },         // 33^180
    FloatType { frac: 11024188012054395372, exp: 875 },         // 33^186
    FloatType { frac: 13259598707595875029, exp: 905 },         // 33^192
    FloatType { frac: 15948290948433680084, exp: 935 },         // 33^198
];
const BASE33_BIAS: i32 = -BASE33_LARGE_POWERS[0].exp;

// BASE34

const BASE34_STEP: i32 = 6;
const BASE34_SMALL_POWERS: [FloatType; BASE34_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 34^0
    FloatType { frac: 9799832789158199296, exp: -58 },          // 34^1
    FloatType { frac: 10412322338480586752, exp: -53 },         // 34^2
    FloatType { frac: 11063092484635623424, exp: -48 },         // 34^3
    FloatType { frac: 11754535764925349888, exp: -43 },         // 34^4
    FloatType { frac: 12489194250233184256, exp: -38 },         // 34^5
];
const BASE34_LARGE_POWERS: [FloatType; 70] = [
    FloatType { frac: 9940514895438007254, exp: -1162 },        // 34^-216
    FloatType { frac: 14301530372152488549, exp: -1132 },       // 34^-210
    FloatType { frac: 10287886147601198282, exp: -1101 },       // 34^-204
    FloatType { frac: 14801297292224652053, exp: -1071 },       // 34^-198
    FloatType { frac: 10647396286743453217, exp: -1040 },       // 34^-192
    FloatType { frac: 15318528565264575918, exp: -1010 },       // 34^-186
    FloatType { frac: 11019469506220361724, exp: -979 },        // 34^-180
    FloatType { frac: 15853834483014935870, exp: -949 },        // 34^-174
    FloatType { frac: 11404544822822581011, exp: -918 },        // 34^-168
    FloatType { frac: 16407846663860846991, exp: -888 },        // 34^-162
    FloatType { frac: 11803076594780713339, exp: -857 },        // 34^-156
    FloatType { frac: 16981218798089296108, exp: -827 },        // 34^-150
    FloatType { frac: 12215535057871861844, exp: -796 },        // 34^-144
    FloatType { frac: 17574627419191661979, exp: -766 },        // 34^-138
    FloatType { frac: 12642406880260427750, exp: -735 },        // 34^-132
    FloatType { frac: 18188772702119398880, exp: -705 },        // 34^-126
    FloatType { frac: 13084195736727816960, exp: -674 },        // 34^-120
    FloatType { frac: 9412189644717380884, exp: -643 },         // 34^-114
    FloatType { frac: 13541422902968601381, exp: -613 },        // 34^-108
    FloatType { frac: 9741098573165682574, exp: -582 },         // 34^-102
    FloatType { frac: 14014627870654357169, exp: -552 },        // 34^-96
    FloatType { frac: 10081501222766715924, exp: -521 },        // 34^-90
    FloatType { frac: 14504368983990906269, exp: -491 },        // 34^-84
    FloatType { frac: 10433799241558921201, exp: -460 },        // 34^-78
    FloatType { frac: 15011224098520048145, exp: -430 },        // 34^-72
    FloatType { frac: 10798408313169791102, exp: -399 },        // 34^-66
    FloatType { frac: 15535791262943115320, exp: -369 },        // 34^-60
    FloatType { frac: 11175758647289472494, exp: -338 },        // 34^-54
    FloatType { frac: 16078689424770850259, exp: -308 },        // 34^-48
    FloatType { frac: 11566295487283966163, exp: -277 },        // 34^-42
    FloatType { frac: 16640559160632214299, exp: -247 },        // 34^-36
    FloatType { frac: 11970479635546867736, exp: -216 },        // 34^-30
    FloatType { frac: 17222063432103834911, exp: -186 },        // 34^-24
    FloatType { frac: 12388787997209523031, exp: -155 },        // 34^-18
    FloatType { frac: 17823888367951909878, exp: -125 },        // 34^-12
    FloatType { frac: 12821714142851132552, exp: -94 },         // 34^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 34^0
    FloatType { frac: 13269768890872758272, exp: -33 },         // 34^6
    FloatType { frac: 9545682734772404224, exp: -2 },           // 34^12
    FloatType { frac: 13733480910222387686, exp: 28 },          // 34^18
    FloatType { frac: 9879256578703990224, exp: 59 },           // 34^24
    FloatType { frac: 14213397344182222761, exp: 89 },          // 34^30
    FloatType { frac: 10224487159240697338, exp: 120 },         // 34^36
    FloatType { frac: 14710084455954213119, exp: 150 },         // 34^42
    FloatType { frac: 10581781820995279550, exp: 181 },         // 34^48
    FloatType { frac: 15224128296805573036, exp: 211 },         // 34^54
    FloatType { frac: 10951562143236309252, exp: 242 },         // 34^60
    FloatType { frac: 15756135397562640779, exp: 272 },         // 34^66
    FloatType { frac: 11334264437318166304, exp: 303 },         // 34^72
    FloatType { frac: 16306733484268988021, exp: 333 },         // 34^78
    FloatType { frac: 11730340261493716029, exp: 364 },         // 34^84
    FloatType { frac: 16876572218852198941, exp: 394 },         // 34^90
    FloatType { frac: 12140256953717114113, exp: 425 },         // 34^96
    FloatType { frac: 17466323965673246884, exp: 455 },         // 34^102
    FloatType { frac: 12564498183065403345, exp: 486 },         // 34^108
    FloatType { frac: 18076684584862935827, exp: 516 },         // 34^114
    FloatType { frac: 13003564520429535778, exp: 547 },         // 34^120
    FloatType { frac: 9354187126690740272, exp: 578 },          // 34^126
    FloatType { frac: 13457974029148190318, exp: 608 },         // 34^132
    FloatType { frac: 9681069157385005207, exp: 639 },          // 34^138
    FloatType { frac: 13928262876281286641, exp: 669 },         // 34^144
    FloatType { frac: 10019374079298318020, exp: 700 },         // 34^150
    FloatType { frac: 14414985965244449544, exp: 730 },         // 34^156
    FloatType { frac: 10369501065317377529, exp: 761 },         // 34^162
    FloatType { frac: 14918717590550882042, exp: 791 },         // 34^168
    FloatType { frac: 10731863237423767546, exp: 822 },         // 34^174
    FloatType { frac: 15440052115433190547, exp: 852 },         // 34^180
    FloatType { frac: 11106888154145020298, exp: 883 },         // 34^186
    FloatType { frac: 15979604673144701925, exp: 913 },         // 34^192
    FloatType { frac: 11495018315039655259, exp: 944 },         // 34^198
];
const BASE34_BIAS: i32 = -BASE34_LARGE_POWERS[0].exp;

// BASE35

const BASE35_STEP: i32 = 6;
const BASE35_SMALL_POWERS: [FloatType; BASE35_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 35^0
    FloatType { frac: 10088063165309911040, exp: -58 },         // 35^1
    FloatType { frac: 11033819087057715200, exp: -53 },         // 35^2
    FloatType { frac: 12068239626469376000, exp: -48 },         // 35^3
    FloatType { frac: 13199637091450880000, exp: -43 },         // 35^4
    FloatType { frac: 14437103068774400000, exp: -38 },         // 35^5
];
const BASE35_LARGE_POWERS: [FloatType; 69] = [
    FloatType { frac: 16631665395337738380, exp: -1141 },       // 35^-210
    FloatType { frac: 14236857547774631404, exp: -1110 },       // 35^-204
    FloatType { frac: 12186880148060573338, exp: -1079 },       // 35^-198
    FloatType { frac: 10432080762542161338, exp: -1048 },       // 35^-192
    FloatType { frac: 17859912908640730010, exp: -1018 },       // 35^-186
    FloatType { frac: 15288248642090717076, exp: -987 },        // 35^-180
    FloatType { frac: 13086880531724678972, exp: -956 },        // 35^-174
    FloatType { frac: 11202489314578100722, exp: -925 },        // 35^-168
    FloatType { frac: 9589433214356533221, exp: -894 },         // 35^-162
    FloatType { frac: 16417284907013989533, exp: -864 },        // 35^-156
    FloatType { frac: 14053345890899718904, exp: -833 },        // 35^-150
    FloatType { frac: 12029792492965214691, exp: -802 },        // 35^-144
    FloatType { frac: 10297612294415481616, exp: -771 },        // 35^-138
    FloatType { frac: 17629700433836653197, exp: -741 },        // 35^-132
    FloatType { frac: 15091184660126225295, exp: -710 },        // 35^-126
    FloatType { frac: 12918191962520288360, exp: -679 },        // 35^-120
    FloatType { frac: 11058090357972464737, exp: -648 },        // 35^-114
    FloatType { frac: 9465826388078148767, exp: -617 },         // 35^-108
    FloatType { frac: 16205667761547463659, exp: -587 },        // 35^-102
    FloatType { frac: 13872199680760223069, exp: -556 },        // 35^-96
    FloatType { frac: 11874729681889289960, exp: -525 },        // 35^-90
    FloatType { frac: 10164877111271147984, exp: -494 },        // 35^-84
    FloatType { frac: 17402455374597619654, exp: -464 },        // 35^-78
    FloatType { frac: 14896660812999728329, exp: -433 },        // 35^-72
    FloatType { frac: 12751677771947325078, exp: -402 },        // 35^-66
    FloatType { frac: 10915552689343391453, exp: -371 },        // 35^-60
    FloatType { frac: 9343812841314943660, exp: -340 },         // 35^-54
    FloatType { frac: 15996778339727633381, exp: -310 },        // 35^-48
    FloatType { frac: 13693388426986467236, exp: -279 },        // 35^-42
    FloatType { frac: 11721665614797754707, exp: -248 },        // 35^-36
    FloatType { frac: 10033852871240677221, exp: -217 },        // 35^-30
    FloatType { frac: 17178139481236495112, exp: -187 },        // 35^-24
    FloatType { frac: 14704644358629538426, exp: -156 },        // 35^-18
    FloatType { frac: 12587309932484688516, exp: -125 },        // 35^-12
    FloatType { frac: 10774852316876721446, exp: -94 },         // 35^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 35^0
    FloatType { frac: 15790581481472000000, exp: -33 },         // 35^6
    FloatType { frac: 13516882032226562500, exp: -2 },          // 35^12
    FloatType { frac: 11570574528082381959, exp: 29 },          // 35^18
    FloatType { frac: 9904517520440015906, exp: 60 },           // 35^24
    FloatType { frac: 16956714997100751955, exp: 90 },          // 35^30
    FloatType { frac: 14515102976976096155, exp: 121 },         // 35^36
    FloatType { frac: 12425060777883196253, exp: 152 },         // 35^42
    FloatType { frac: 10635965558010358334, exp: 183 },         // 35^48
    FloatType { frac: 18208967404416189641, exp: 213 },         // 35^54
    FloatType { frac: 15587042479907979542, exp: 244 },         // 35^60
    FloatType { frac: 13342650787080448100, exp: 275 },         // 35^66
    FloatType { frac: 11421430990225254489, exp: 306 },         // 35^72
    FloatType { frac: 9776849289257450184, exp: 337 },          // 35^78
    FloatType { frac: 16738144652217302890, exp: 367 },         // 35^84
    FloatType { frac: 14328004764601889967, exp: 398 },         // 35^90
    FloatType { frac: 12264902998508825496, exp: 429 },         // 35^96
    FloatType { frac: 10498869035448040891, exp: 460 },         // 35^102
    FloatType { frac: 17974255652391389054, exp: 490 },         // 35^108
    FloatType { frac: 15386127075531072702, exp: 521 },         // 35^114
    FloatType { frac: 13170665365099231511, exp: 552 },         // 35^120
    FloatType { frac: 11274209897518154234, exp: 583 },         // 35^126
    FloatType { frac: 9650826688689364000, exp: 614 },          // 35^132
    FloatType { frac: 16522391657019252092, exp: 644 },         // 35^138
    FloatType { frac: 14143318229301497845, exp: 675 },         // 35^144
    FloatType { frac: 12106809636745979660, exp: 706 },         // 35^150
    FloatType { frac: 10363539673224497244, exp: 737 },         // 35^156
    FloatType { frac: 17742569311160898016, exp: 767 },         // 35^162
    FloatType { frac: 15187801450438335382, exp: 798 },         // 35^168
    FloatType { frac: 13000896817848986534, exp: 829 },         // 35^174
    FloatType { frac: 11128886469837128365, exp: 860 },         // 35^180
    FloatType { frac: 9526428506723229038, exp: 891 },          // 35^186
    FloatType { frac: 16309419696153507876, exp: 921 },         // 35^192
    FloatType { frac: 13961012284800847178, exp: 952 },         // 35^198
];
const BASE35_BIAS: i32 = -BASE35_LARGE_POWERS[0].exp;

// BASE36

const BASE36_STEP: i32 = 6;
const BASE36_SMALL_POWERS: [FloatType; BASE36_STEP as usize] = [
    FloatType { frac: 9223372036854775808, exp: -63 },          // 36^0
    FloatType { frac: 10376293541461622784, exp: -58 },         // 36^1
    FloatType { frac: 11673330234144325632, exp: -53 },         // 36^2
    FloatType { frac: 13132496513412366336, exp: -48 },         // 36^3
    FloatType { frac: 14774058577588912128, exp: -43 },         // 36^4
    FloatType { frac: 16620815899787526144, exp: -38 },         // 36^5
];
const BASE36_LARGE_POWERS: [FloatType; 69] = [
    FloatType { frac: 11479946305982273645, exp: -1149 },       // 36^-210
    FloatType { frac: 11636570252986002899, exp: -1118 },       // 36^-204
    FloatType { frac: 11795331061968106016, exp: -1087 },       // 36^-198
    FloatType { frac: 11956257886702331980, exp: -1056 },       // 36^-192
    FloatType { frac: 12119380278715084095, exp: -1025 },       // 36^-186
    FloatType { frac: 12284728192712064755, exp: -994 },        // 36^-180
    FloatType { frac: 12452331992078957377, exp: -963 },        // 36^-174
    FloatType { frac: 12622222454457155586, exp: -932 },        // 36^-168
    FloatType { frac: 12794430777395563548, exp: -901 },        // 36^-162
    FloatType { frac: 12968988584079505325, exp: -870 },        // 36^-156
    FloatType { frac: 13145927929137795237, exp: -839 },        // 36^-150
    FloatType { frac: 13325281304529035642, exp: -808 },        // 36^-144
    FloatType { frac: 13507081645508223020, exp: -777 },        // 36^-138
    FloatType { frac: 13691362336674758052, exp: -746 },        // 36^-132
    FloatType { frac: 13878157218102970303, exp: -715 },        // 36^-126
    FloatType { frac: 14067500591556283265, exp: -684 },        // 36^-120
    FloatType { frac: 14259427226786160917, exp: -653 },        // 36^-114
    FloatType { frac: 14453972367916992462, exp: -622 },        // 36^-108
    FloatType { frac: 14651171739918087751, exp: -591 },        // 36^-102
    FloatType { frac: 14851061555163971849, exp: -560 },        // 36^-96
    FloatType { frac: 15053678520084183432, exp: -529 },        // 36^-90
    FloatType { frac: 15259059841903798156, exp: -498 },        // 36^-84
    FloatType { frac: 15467243235475914756, exp: -467 },        // 36^-78
    FloatType { frac: 15678266930207358578, exp: -436 },        // 36^-72
    FloatType { frac: 15892169677078874302, exp: -405 },        // 36^-66
    FloatType { frac: 16108990755761097026, exp: -374 },        // 36^-60
    FloatType { frac: 16328769981827608423, exp: -343 },        // 36^-54
    FloatType { frac: 16551547714066402526, exp: -312 },        // 36^-48
    FloatType { frac: 16777364861891103792, exp: -281 },        // 36^-42
    FloatType { frac: 17006262892853298360, exp: -250 },        // 36^-36
    FloatType { frac: 17238283840257358043, exp: -219 },        // 36^-30
    FloatType { frac: 17473470310879155380, exp: -188 },        // 36^-24
    FloatType { frac: 17711865492790087155, exp: -157 },        // 36^-18
    FloatType { frac: 17953513163287843146, exp: -126 },        // 36^-12
    FloatType { frac: 18198457696935376453, exp: -95 },         // 36^-6
    FloatType { frac: 9223372036854775808, exp: -63 },          // 36^0
    FloatType { frac: 9349208943630483456, exp: -32 },          // 36^6
    FloatType { frac: 9476762676643233792, exp: -1 },           // 36^12
    FloatType { frac: 9606056659007943744, exp: 30 },           // 36^18
    FloatType { frac: 9737114633407288801, exp: 61 },           // 36^24
    FloatType { frac: 9869960666451650558, exp: 92 },           // 36^30
    FloatType { frac: 10004619153098548172, exp: 123 },         // 36^36
    FloatType { frac: 10141114821132365302, exp: 154 },         // 36^42
    FloatType { frac: 10279472735705195138, exp: 185 },         // 36^48
    FloatType { frac: 10419718303939637392, exp: 216 },         // 36^54
    FloatType { frac: 10561877279594392463, exp: 247 },         // 36^60
    FloatType { frac: 10705975767793509530, exp: 278 },         // 36^66
    FloatType { frac: 10852040229820157048, exp: 309 },         // 36^72
    FloatType { frac: 11000097487975795902, exp: 340 },         // 36^78
    FloatType { frac: 11150174730505647564, exp: 371 },         // 36^84
    FloatType { frac: 11302299516591361707, exp: 402 },         // 36^90
    FloatType { frac: 11456499781411800112, exp: 433 },         // 36^96
    FloatType { frac: 11612803841272866179, exp: 464 },         // 36^102
    FloatType { frac: 11771240398807322073, exp: 495 },         // 36^108
    FloatType { frac: 11931838548245548344, exp: 526 },         // 36^114
    FloatType { frac: 12094627780758213915, exp: 557 },         // 36^120
    FloatType { frac: 12259637989871837542, exp: 588 },         // 36^126
    FloatType { frac: 12426899476958235198, exp: 619 },         // 36^132
    FloatType { frac: 12596442956798861450, exp: 650 },         // 36^138
    FloatType { frac: 12768299563225066619, exp: 681 },         // 36^144
    FloatType { frac: 12942500854835305460, exp: 712 },         // 36^150
    FloatType { frac: 13119078820790347231, exp: 743 },         // 36^156
    FloatType { frac: 13298065886687551351, exp: 774 },         // 36^162
    FloatType { frac: 13479494920515287357, exp: 805 },         // 36^168
    FloatType { frac: 13663399238688592583, exp: 836 },         // 36^174
    FloatType { frac: 13849812612167175924, exp: 867 },         // 36^180
    FloatType { frac: 14038769272656891137, exp: 898 },         // 36^186
    FloatType { frac: 14230303918895818486, exp: 929 },         // 36^192
    FloatType { frac: 14424451723026109070, exp: 960 },         // 36^198
];
const BASE36_BIAS: i32 = -BASE36_LARGE_POWERS[0].exp;

// HIGH-LEVEL
// ----------

/// Precalculated powers of base N.
#[repr(C)]
#[doc(hidden)]
pub(crate) struct Powers {
    // Pre-calculate small powers.
    pub small: &'static [FloatType],
    // Pre-calculate large powers.
    pub large: &'static [FloatType],
    // Step between large powers and number of small powers.
    pub step: i32,
    // Exponent bias for the large powers.
    pub bias: i32,
}

pub(crate) const BASE3_POWERS: Powers = Powers {
    small: &BASE3_SMALL_POWERS,
    large: &BASE3_LARGE_POWERS,
    step: BASE3_STEP,
    bias: BASE3_BIAS,
};

pub(crate) const BASE5_POWERS: Powers = Powers {
    small: &BASE5_SMALL_POWERS,
    large: &BASE5_LARGE_POWERS,
    step: BASE5_STEP,
    bias: BASE5_BIAS,
};

pub(crate) const BASE6_POWERS: Powers = Powers {
    small: &BASE6_SMALL_POWERS,
    large: &BASE6_LARGE_POWERS,
    step: BASE6_STEP,
    bias: BASE6_BIAS,
};

pub(crate) const BASE7_POWERS: Powers = Powers {
    small: &BASE7_SMALL_POWERS,
    large: &BASE7_LARGE_POWERS,
    step: BASE7_STEP,
    bias: BASE7_BIAS,
};

pub(crate) const BASE9_POWERS: Powers = Powers {
    small: &BASE9_SMALL_POWERS,
    large: &BASE9_LARGE_POWERS,
    step: BASE9_STEP,
    bias: BASE9_BIAS,
};

pub(crate) const BASE10_POWERS: Powers = Powers {
    small: &BASE10_SMALL_POWERS,
    large: &BASE10_LARGE_POWERS,
    step: BASE10_STEP,
    bias: BASE10_BIAS,
};

pub(crate) const BASE11_POWERS: Powers = Powers {
    small: &BASE11_SMALL_POWERS,
    large: &BASE11_LARGE_POWERS,
    step: BASE11_STEP,
    bias: BASE11_BIAS,
};

pub(crate) const BASE12_POWERS: Powers = Powers {
    small: &BASE12_SMALL_POWERS,
    large: &BASE12_LARGE_POWERS,
    step: BASE12_STEP,
    bias: BASE12_BIAS,
};

pub(crate) const BASE13_POWERS: Powers = Powers {
    small: &BASE13_SMALL_POWERS,
    large: &BASE13_LARGE_POWERS,
    step: BASE13_STEP,
    bias: BASE13_BIAS,
};

pub(crate) const BASE14_POWERS: Powers = Powers {
    small: &BASE14_SMALL_POWERS,
    large: &BASE14_LARGE_POWERS,
    step: BASE14_STEP,
    bias: BASE14_BIAS,
};

pub(crate) const BASE15_POWERS: Powers = Powers {
    small: &BASE15_SMALL_POWERS,
    large: &BASE15_LARGE_POWERS,
    step: BASE15_STEP,
    bias: BASE15_BIAS,
};

pub(crate) const BASE17_POWERS: Powers = Powers {
    small: &BASE17_SMALL_POWERS,
    large: &BASE17_LARGE_POWERS,
    step: BASE17_STEP,
    bias: BASE17_BIAS,
};

pub(crate) const BASE18_POWERS: Powers = Powers {
    small: &BASE18_SMALL_POWERS,
    large: &BASE18_LARGE_POWERS,
    step: BASE18_STEP,
    bias: BASE18_BIAS,
};

pub(crate) const BASE19_POWERS: Powers = Powers {
    small: &BASE19_SMALL_POWERS,
    large: &BASE19_LARGE_POWERS,
    step: BASE19_STEP,
    bias: BASE19_BIAS,
};

pub(crate) const BASE20_POWERS: Powers = Powers {
    small: &BASE20_SMALL_POWERS,
    large: &BASE20_LARGE_POWERS,
    step: BASE20_STEP,
    bias: BASE20_BIAS,
};

pub(crate) const BASE21_POWERS: Powers = Powers {
    small: &BASE21_SMALL_POWERS,
    large: &BASE21_LARGE_POWERS,
    step: BASE21_STEP,
    bias: BASE21_BIAS,
};

pub(crate) const BASE22_POWERS: Powers = Powers {
    small: &BASE22_SMALL_POWERS,
    large: &BASE22_LARGE_POWERS,
    step: BASE22_STEP,
    bias: BASE22_BIAS,
};

pub(crate) const BASE23_POWERS: Powers = Powers {
    small: &BASE23_SMALL_POWERS,
    large: &BASE23_LARGE_POWERS,
    step: BASE23_STEP,
    bias: BASE23_BIAS,
};

pub(crate) const BASE24_POWERS: Powers = Powers {
    small: &BASE24_SMALL_POWERS,
    large: &BASE24_LARGE_POWERS,
    step: BASE24_STEP,
    bias: BASE24_BIAS,
};

pub(crate) const BASE25_POWERS: Powers = Powers {
    small: &BASE25_SMALL_POWERS,
    large: &BASE25_LARGE_POWERS,
    step: BASE25_STEP,
    bias: BASE25_BIAS,
};

pub(crate) const BASE26_POWERS: Powers = Powers {
    small: &BASE26_SMALL_POWERS,
    large: &BASE26_LARGE_POWERS,
    step: BASE26_STEP,
    bias: BASE26_BIAS,
};

pub(crate) const BASE27_POWERS: Powers = Powers {
    small: &BASE27_SMALL_POWERS,
    large: &BASE27_LARGE_POWERS,
    step: BASE27_STEP,
    bias: BASE27_BIAS,
};

pub(crate) const BASE28_POWERS: Powers = Powers {
    small: &BASE28_SMALL_POWERS,
    large: &BASE28_LARGE_POWERS,
    step: BASE28_STEP,
    bias: BASE28_BIAS,
};

pub(crate) const BASE29_POWERS: Powers = Powers {
    small: &BASE29_SMALL_POWERS,
    large: &BASE29_LARGE_POWERS,
    step: BASE29_STEP,
    bias: BASE29_BIAS,
};

pub(crate) const BASE30_POWERS: Powers = Powers {
    small: &BASE30_SMALL_POWERS,
    large: &BASE30_LARGE_POWERS,
    step: BASE30_STEP,
    bias: BASE30_BIAS,
};

pub(crate) const BASE31_POWERS: Powers = Powers {
    small: &BASE31_SMALL_POWERS,
    large: &BASE31_LARGE_POWERS,
    step: BASE31_STEP,
    bias: BASE31_BIAS,
};

pub(crate) const BASE33_POWERS: Powers = Powers {
    small: &BASE33_SMALL_POWERS,
    large: &BASE33_LARGE_POWERS,
    step: BASE33_STEP,
    bias: BASE33_BIAS,
};

pub(crate) const BASE34_POWERS: Powers = Powers {
    small: &BASE34_SMALL_POWERS,
    large: &BASE34_LARGE_POWERS,
    step: BASE34_STEP,
    bias: BASE34_BIAS,
};

pub(crate) const BASE35_POWERS: Powers = Powers {
    small: &BASE35_SMALL_POWERS,
    large: &BASE35_LARGE_POWERS,
    step: BASE35_STEP,
    bias: BASE35_BIAS,
};

pub(crate) const BASE36_POWERS: Powers = Powers {
    small: &BASE36_SMALL_POWERS,
    large: &BASE36_LARGE_POWERS,
    step: BASE36_STEP,
    bias: BASE36_BIAS,
};

/// Get powers from base.
pub(crate) fn get_powers(base: u64) -> &'static Powers {
    match base {
        3  => &BASE3_POWERS,
        5  => &BASE5_POWERS,
        6  => &BASE6_POWERS,
        7  => &BASE7_POWERS,
        9  => &BASE9_POWERS,
        10 => &BASE10_POWERS,
        11 => &BASE11_POWERS,
        12 => &BASE12_POWERS,
        13 => &BASE13_POWERS,
        14 => &BASE14_POWERS,
        15 => &BASE15_POWERS,
        17 => &BASE17_POWERS,
        18 => &BASE18_POWERS,
        19 => &BASE19_POWERS,
        20 => &BASE20_POWERS,
        21 => &BASE21_POWERS,
        22 => &BASE22_POWERS,
        23 => &BASE23_POWERS,
        24 => &BASE24_POWERS,
        25 => &BASE25_POWERS,
        26 => &BASE26_POWERS,
        27 => &BASE27_POWERS,
        28 => &BASE28_POWERS,
        29 => &BASE29_POWERS,
        30 => &BASE30_POWERS,
        31 => &BASE31_POWERS,
        33 => &BASE33_POWERS,
        34 => &BASE34_POWERS,
        35 => &BASE35_POWERS,
        36 => &BASE36_POWERS,
        // Powers of 2, and others, should already be handled by now.
        _  => unreachable!(),
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    const POW2: [u64; 5] = [2, 4, 8, 16, 32];
    const BASEN: [u64; 30] = [
        3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
        22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
    ];

    #[test]
    fn normalization_test() {
        // Ensure each valid is normalized.
        for base in BASEN.iter().cloned() {
            let powers = get_powers(base);
            for fp in powers.small {
                assert_eq!(fp.frac.leading_zeros(), 0);
            }
            for fp in powers.large {
                assert_eq!(fp.frac.leading_zeros(), 0);
            }
        }
    }

    #[test]
    #[should_panic]
    fn pow2_test() {
        for base in POW2.iter().cloned() {
            let _ = get_powers(base);
        }
    }
}
