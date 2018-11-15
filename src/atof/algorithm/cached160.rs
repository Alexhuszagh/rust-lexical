//! Cached exponents for basen values with 160-bit extended floats.
//!
//! Exact versions of base**n as an extended-precision float, with both
//! large and small powers. Use the large powers to minimize the amount
//! of compounded error.
//!
//! These values were calculated using Python, using the arbitrary-precision
//! integer to calculate exact extended-representation of each value.
//! These values are all normalized.
//!
//! This files takes ~ 70KB of storage, assuming each array pads
//! each ExtendedFloat160 to 32 bytes (for alignment).
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
//! SMALL_STR = "const BASE{0}_SMALL_POWERS: [ExtendedFloat160; BASE{0}_STEP as usize] = ["
//! SMALL_INT_STR = "const BASE{0}_SMALL_INT_POWERS: [u128; BASE{0}_STEP as usize] = {1};"
//! LARGE_STR = "const BASE{0}_LARGE_POWERS: [ExtendedFloat160; {1}] = ["
//! FP_STR_1 = "    ExtendedFloat160 {{ frac: {}, exp: {} }},"
//! FP_STR_2 = "// {}^{}"
//! BIAS_STR = "const BASE{0}_BIAS: i32 = {1};"
//! POWER_STR = """pub(crate) const BASE{0}_POWERS: Powers<u128> = Powers {{
//!     small: &BASE{0}_SMALL_POWERS,
//!     small_int: &BASE{0}_SMALL_INT_POWERS,
//!     large: &BASE{0}_LARGE_POWERS,
//!     step: BASE{0}_STEP,
//!     bias: BASE{0}_BIAS,
//! }};\n"""
//!
//! def calculate_bitshift(base, exponent):
//!     '''
//!     Calculate the bitshift required for a given base. The exponent
//!     is the absolute value of the max exponent (log distance from 1.)
//!     '''
//!
//!     return 127 + math.ceil(math.log2(base**exponent))
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
//!     '''Normalize a extended-float so the MSB is the 128th bit'''
//!
//!     while fp[0] >> 128 != 0:
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
//!     print(str1.ljust(106, " ") + str2)
//!
//!
//! def generate_small(base, count):
//!     '''Generate the small powers for a given base'''
//!
//!     bitshift = calculate_bitshift(base, count)
//!     fps = []
//!     fp = (1 << bitshift, -bitshift)
//!     for exp in range(count):
//!         fps.append((fp, exp))
//!         fp = next_fp(fp, base)
//!
//!     # Print the small powers as Float-type
//!     print(SMALL_STR.format(base))
//!     for fp, exp in fps:
//!         print_fp(fp, base, exp)
//!     print("];")
//!
//!     # Print the small powers as integers.
//!     ints = [base**i for _, i in fps]
//!     print(SMALL_INT_STR.format(base, ints))
//!
//!
//! def generate_large(base, step):
//!     '''Generate the large powers for a given base.'''
//!
//!     # Get our starting parameters
//!     min_exp = math.floor(math.log(5e-324, base) - math.log(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, base))
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
//!     # Return the smallest exp, AKA, the bias
//!     return -fps[0][1]
//!
//!
//! def generate_base(base):
//!     '''Generate all powers and variables.'''
//!
//!     step = math.floor(math.log(1e10, base))
//!     print(STEP_STR.format(base, step))
//!     generate_small(base, step)
//!     bias = generate_large(base, step)
//!     print(BIAS_STR.format(base, bias))
//!
//!
//! def generate():
//!     '''Generate all bases.'''
//!
//!     bases = [
//!         3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
//!         22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
//!     ]
//!
//!     for base in bases:
//!         print("// BASE{}\n".format(base))
//!         generate_base(base)
//!         print("")
//!
//!     print("// HIGH LEVEL\n// ----------\n")
//!
//!     for base in bases:
//!         print(POWER_STR.format(base))
//!
//!
//! if __name__ == '__main__':
//!     generate()
//! ```

use float::ExtendedFloat160;
use super::cached::Powers;

// LOW-LEVEL
// ---------

// BASE3

const BASE3_STEP: i32 = 20;
const BASE3_SMALL_POWERS: [ExtendedFloat160; BASE3_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 3^0
    ExtendedFloat160 { frac: 255211775190703847597530955573826158592, exp: -126 },                        // 3^1
    ExtendedFloat160 { frac: 191408831393027885698148216680369618944, exp: -124 },                        // 3^2
    ExtendedFloat160 { frac: 287113247089541828547222325020554428416, exp: -123 },                        // 3^3
    ExtendedFloat160 { frac: 215334935317156371410416743765415821312, exp: -121 },                        // 3^4
    ExtendedFloat160 { frac: 323002402975734557115625115648123731968, exp: -120 },                        // 3^5
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -118 },                        // 3^6
    ExtendedFloat160 { frac: 181688851673850688377539127552069599232, exp: -116 },                        // 3^7
    ExtendedFloat160 { frac: 272533277510776032566308691328104398848, exp: -115 },                        // 3^8
    ExtendedFloat160 { frac: 204399958133082024424731518496078299136, exp: -113 },                        // 3^9
    ExtendedFloat160 { frac: 306599937199623036637097277744117448704, exp: -112 },                        // 3^10
    ExtendedFloat160 { frac: 229949952899717277477822958308088086528, exp: -110 },                        // 3^11
    ExtendedFloat160 { frac: 172462464674787958108367218731066064896, exp: -108 },                        // 3^12
    ExtendedFloat160 { frac: 258693697012181937162550828096599097344, exp: -107 },                        // 3^13
    ExtendedFloat160 { frac: 194020272759136452871913121072449323008, exp: -105 },                        // 3^14
    ExtendedFloat160 { frac: 291030409138704679307869681608673984512, exp: -104 },                        // 3^15
    ExtendedFloat160 { frac: 218272806854028509480902261206505488384, exp: -102 },                        // 3^16
    ExtendedFloat160 { frac: 327409210281042764221353391809758232576, exp: -101 },                        // 3^17
    ExtendedFloat160 { frac: 245556907710782073166015043857318674432, exp: -99 },                         // 3^18
    ExtendedFloat160 { frac: 184167680783086554874511282892989005824, exp: -97 },                         // 3^19
];
const BASE3_SMALL_INT_POWERS: [u128; BASE3_STEP as usize] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 14348907, 43046721, 129140163, 387420489, 1162261467];
const BASE3_LARGE_POWERS: [ExtendedFloat160; 71] = [
    ExtendedFloat160 { frac: 228981627321413421057217801224022768458, exp: -1332 },                       // 3^-760
    ExtendedFloat160 { frac: 185894213211699326013762494029347155134, exp: -1300 },                       // 3^-740
    ExtendedFloat160 { frac: 301829093537629265639465570217176944359, exp: -1269 },                       // 3^-720
    ExtendedFloat160 { frac: 245033990385703656345786023933864839340, exp: -1237 },                       // 3^-700
    ExtendedFloat160 { frac: 198926007233479871031630637668169238011, exp: -1205 },                       // 3^-680
    ExtendedFloat160 { frac: 322988302900880006728964617948539328448, exp: -1174 },                       // 3^-660
    ExtendedFloat160 { frac: 262211676747596696167096696967233799204, exp: -1142 },                       // 3^-640
    ExtendedFloat160 { frac: 212871372756449173771443137071089544143, exp: -1110 },                       // 3^-620
    ExtendedFloat160 { frac: 172815421118085121562612771428651141606, exp: -1078 },                       // 3^-600
    ExtendedFloat160 { frac: 280593575260967566098415738074481154338, exp: -1047 },                       // 3^-580
    ExtendedFloat160 { frac: 227794354139073103116567345878808448350, exp: -1015 },                       // 3^-560
    ExtendedFloat160 { frac: 184930348919702200346046943747485274024, exp: -983 },                        // 3^-540
    ExtendedFloat160 { frac: 300264105147079021545114594266031000970, exp: -952 },                        // 3^-520
    ExtendedFloat160 { frac: 243763485459391712918376663011554847091, exp: -920 },                        // 3^-500
    ExtendedFloat160 { frac: 197894572893436379626501802082900685163, exp: -888 },                        // 3^-480
    ExtendedFloat160 { frac: 321313603691473325606249593990411338331, exp: -857 },                        // 3^-460
    ExtendedFloat160 { frac: 260852105259086286749566195634740776863, exp: -825 },                        // 3^-440
    ExtendedFloat160 { frac: 211767631486382365261996259087726574961, exp: -793 },                        // 3^-420
    ExtendedFloat160 { frac: 171919370559843833352674924374427532806, exp: -761 },                        // 3^-400
    ExtendedFloat160 { frac: 279138693352137745884317186629683060895, exp: -730 },                        // 3^-380
    ExtendedFloat160 { frac: 226613236986043931067161987739751269180, exp: -698 },                        // 3^-360
    ExtendedFloat160 { frac: 183971482278558945643179980616811190964, exp: -666 },                        // 3^-340
    ExtendedFloat160 { frac: 298707231244876640116631457791747347925, exp: -635 },                        // 3^-320
    ExtendedFloat160 { frac: 242499568120235502703106353919523432682, exp: -603 },                        // 3^-300
    ExtendedFloat160 { frac: 196868486555962367745019627988939060464, exp: -571 },                        // 3^-280
    ExtendedFloat160 { frac: 319647587822660709450189016904055940251, exp: -540 },                        // 3^-260
    ExtendedFloat160 { frac: 259499583169196479959998361450291137700, exp: -508 },                        // 3^-240
    ExtendedFloat160 { frac: 210669613131404954481085620515615417585, exp: -476 },                        // 3^-220
    ExtendedFloat160 { frac: 171027966037226738058674240289082148799, exp: -444 },                        // 3^-200
    ExtendedFloat160 { frac: 277691355027891684120101092281051616669, exp: -413 },                        // 3^-180
    ExtendedFloat160 { frac: 225438243943221318560556485110109738278, exp: -381 },                        // 3^-160
    ExtendedFloat160 { frac: 183017587375374702561553597022155160742, exp: -349 },                        // 3^-140
    ExtendedFloat160 { frac: 297158429757277967604640789526650060843, exp: -318 },                        // 3^-120
    ExtendedFloat160 { frac: 241242204211496523037749538228345943134, exp: -286 },                        // 3^-100
    ExtendedFloat160 { frac: 195847720491584060106836777189641681162, exp: -254 },                        // 3^-80
    ExtendedFloat160 { frac: 317990210271190550439415903835536554761, exp: -223 },                        // 3^-60
    ExtendedFloat160 { frac: 258154073926689380540223575440483383976, exp: -191 },                        // 3^-40
    ExtendedFloat160 { frac: 209577288018116110386327960504760073299, exp: -159 },                        // 3^-20
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 3^0
    ExtendedFloat160 { frac: 276251521174629832311766924339483508736, exp: -96 },                         // 3^20
    ExtendedFloat160 { frac: 224269343257001716702690972139746492416, exp: -64 },                         // 3^40
    ExtendedFloat160 { frac: 182068638431613361423174859113151594496, exp: -32 },                         // 3^60
    ExtendedFloat160 { frac: 295617658828691846632166420412766595202, exp: -1 },                          // 3^80
    ExtendedFloat160 { frac: 239991359753539474232337032335634004651, exp: 31 },                          // 3^100
    ExtendedFloat160 { frac: 194832247114605420104007752175098574688, exp: 63 },                          // 3^120
    ExtendedFloat160 { frac: 316341426247257477645159711999449660471, exp: 94 },                          // 3^140
    ExtendedFloat160 { frac: 256815541169845811576524073480007610450, exp: 126 },                         // 3^160
    ExtendedFloat160 { frac: 208490626626972031635281014538153149532, exp: 158 },                         // 3^180
    ExtendedFloat160 { frac: 338517997729425004575949331160209430911, exp: 189 },                         // 3^200
    ExtendedFloat160 { frac: 274819152881557244028610584245948464515, exp: 221 },                         // 3^220
    ExtendedFloat160 { frac: 223106503338424488684979682521025988628, exp: 253 },                         // 3^240
    ExtendedFloat160 { frac: 181124609802400910077427551154104473922, exp: 285 },                         // 3^260
    ExtendedFloat160 { frac: 294084876820548989626661915132664622178, exp: 316 },                         // 3^280
    ExtendedFloat160 { frac: 238747000942913976797497733353022683918, exp: 348 },                         // 3^300
    ExtendedFloat160 { frac: 193822038982362660063056049982127016523, exp: 380 },                         // 3^320
    ExtendedFloat160 { frac: 314701191193291934781116205950433765545, exp: 411 },                         // 3^340
    ExtendedFloat160 { frac: 255483948725482657093998355855298189652, exp: 443 },                         // 3^360
    ExtendedFloat160 { frac: 207409599591488195571905341445445255582, exp: 475 },                         // 3^380
    ExtendedFloat160 { frac: 336762776818711782198286065086981891498, exp: 506 },                         // 3^400
    ExtendedFloat160 { frac: 273394211439632029990640781047045990695, exp: 538 },                         // 3^420
    ExtendedFloat160 { frac: 221949692762318233808346663450192754968, exp: 570 },                         // 3^440
    ExtendedFloat160 { frac: 180185475975832393914650652957737664335, exp: 602 },                         // 3^460
    ExtendedFloat160 { frac: 292560042310176717160312096633717510967, exp: 633 },                         // 3^480
    ExtendedFloat160 { frac: 237509094151441049982785534773499431992, exp: 665 },                         // 3^500
    ExtendedFloat160 { frac: 192817068794482616882547252154136283242, exp: 697 },                         // 3^520
    ExtendedFloat160 { frac: 313069460782756034010893203297842312622, exp: 728 },                         // 3^540
    ExtendedFloat160 { frac: 254159260607975299744356396919078736707, exp: 760 },                         // 3^560
    ExtendedFloat160 { frac: 206334177697445743564032291193028958152, exp: 792 },                         // 3^580
    ExtendedFloat160 { frac: 335016656754825225194410391893304442626, exp: 823 },                         // 3^600
    ExtendedFloat160 { frac: 271976658340519265432186039827268288213, exp: 855 },                         // 3^620
    ExtendedFloat160 { frac: 220798880266451537830039115389735391778, exp: 887 },                         // 3^640
];
const BASE3_BIAS: i32 = 760;

// BASE5

const BASE5_STEP: i32 = 14;
const BASE5_SMALL_POWERS: [ExtendedFloat160; BASE5_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 5^0
    ExtendedFloat160 { frac: 212676479325586539664609129644855132160, exp: -125 },                        // 5^1
    ExtendedFloat160 { frac: 265845599156983174580761412056068915200, exp: -123 },                        // 5^2
    ExtendedFloat160 { frac: 332306998946228968225951765070086144000, exp: -121 },                        // 5^3
    ExtendedFloat160 { frac: 207691874341393105141219853168803840000, exp: -118 },                        // 5^4
    ExtendedFloat160 { frac: 259614842926741381426524816461004800000, exp: -116 },                        // 5^5
    ExtendedFloat160 { frac: 324518553658426726783156020576256000000, exp: -114 },                        // 5^6
    ExtendedFloat160 { frac: 202824096036516704239472512860160000000, exp: -111 },                        // 5^7
    ExtendedFloat160 { frac: 253530120045645880299340641075200000000, exp: -109 },                        // 5^8
    ExtendedFloat160 { frac: 316912650057057350374175801344000000000, exp: -107 },                        // 5^9
    ExtendedFloat160 { frac: 198070406285660843983859875840000000000, exp: -104 },                        // 5^10
    ExtendedFloat160 { frac: 247588007857076054979824844800000000000, exp: -102 },                        // 5^11
    ExtendedFloat160 { frac: 309485009821345068724781056000000000000, exp: -100 },                        // 5^12
    ExtendedFloat160 { frac: 193428131138340667952988160000000000000, exp: -97 },                         // 5^13
];
const BASE5_SMALL_INT_POWERS: [u128; BASE5_STEP as usize] = [1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625, 1220703125];
const BASE5_LARGE_POWERS: [ExtendedFloat160; 69] = [
    ExtendedFloat160 { frac: 201109222516499671628641054110261305647, exp: -1330 },                       // 5^-518
    ExtendedFloat160 { frac: 285793394306920833441610418092098634655, exp: -1298 },                       // 5^-504
    ExtendedFloat160 { frac: 203068420253004570555511362849258201390, exp: -1265 },                       // 5^-490
    ExtendedFloat160 { frac: 288577581746103207017755725657449092679, exp: -1233 },                       // 5^-476
    ExtendedFloat160 { frac: 205046704412910121830119952091883627559, exp: -1200 },                       // 5^-462
    ExtendedFloat160 { frac: 291388892624283530821742192659774598780, exp: -1168 },                       // 5^-448
    ExtendedFloat160 { frac: 207044260935364498850036477975162511299, exp: -1135 },                       // 5^-434
    ExtendedFloat160 { frac: 294227591176883860910658765384315687611, exp: -1103 },                       // 5^-420
    ExtendedFloat160 { frac: 209061277570927374050781655074839937648, exp: -1070 },                       // 5^-406
    ExtendedFloat160 { frac: 297093944213496817569054052050375869453, exp: -1038 },                       // 5^-392
    ExtendedFloat160 { frac: 211097943899216614887176072592734406508, exp: -1005 },                       // 5^-378
    ExtendedFloat160 { frac: 299988221142963048588365030287739055137, exp: -973 },                        // 5^-364
    ExtendedFloat160 { frac: 213154451346726893197828921904416471830, exp: -940 },                        // 5^-350
    ExtendedFloat160 { frac: 302910693998692996157485768413290076965, exp: -908 },                        // 5^-336
    ExtendedFloat160 { frac: 215230993204821882725842221200657943544, exp: -875 },                        // 5^-322
    ExtendedFloat160 { frac: 305861637464235347360161968596028634045, exp: -843 },                        // 5^-308
    ExtendedFloat160 { frac: 217327764647901735884376228537482684576, exp: -810 },                        // 5^-294
    ExtendedFloat160 { frac: 308841328899094571460716776609676066664, exp: -778 },                        // 5^-280
    ExtendedFloat160 { frac: 219444962751747547330237450047488370802, exp: -745 },                        // 5^-266
    ExtendedFloat160 { frac: 311850048364799970571308236412006025948, exp: -713 },                        // 5^-252
    ExtendedFloat160 { frac: 221582786512044528543660416923448526878, exp: -680 },                        // 5^-238
    ExtendedFloat160 { frac: 314888078651228693933689466069052580904, exp: -648 },                        // 5^-224
    ExtendedFloat160 { frac: 223741436863085634409521749481834675708, exp: -615 },                        // 5^-210
    ExtendedFloat160 { frac: 317955705303185189918510999237120523316, exp: -583 },                        // 5^-196
    ExtendedFloat160 { frac: 225921116696657399755928707376370229068, exp: -550 },                        // 5^-182
    ExtendedFloat160 { frac: 321053216647239593947814323906257853121, exp: -518 },                        // 5^-168
    ExtendedFloat160 { frac: 228122030881109760932058580285014566244, exp: -485 },                        // 5^-154
    ExtendedFloat160 { frac: 324180903818827574883781864350871964922, exp: -453 },                        // 5^-140
    ExtendedFloat160 { frac: 230344386280611654799899571593522271174, exp: -420 },                        // 5^-126
    ExtendedFloat160 { frac: 327339060789614187001318969682759915221, exp: -388 },                        // 5^-112
    ExtendedFloat160 { frac: 232588391774594204975783618524161450993, exp: -355 },                        // 5^-98
    ExtendedFloat160 { frac: 330527984395124299475957654016385519914, exp: -323 },                        // 5^-84
    ExtendedFloat160 { frac: 234854258277383322788948059678933702737, exp: -290 },                        // 5^-70
    ExtendedFloat160 { frac: 333747974362642200374222141588992517906, exp: -258 },                        // 5^-56
    ExtendedFloat160 { frac: 237142198758023568227473377297792835283, exp: -225 },                        // 5^-42
    ExtendedFloat160 { frac: 336999333339382997433337688587745383420, exp: -193 },                        // 5^-28
    ExtendedFloat160 { frac: 239452428260295134118491722992235809940, exp: -160 },                        // 5^-14
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 5^0
    ExtendedFloat160 { frac: 241785163922925834941235200000000000000, exp: -95 },                         // 5^14
    ExtendedFloat160 { frac: 171798691840000000000000000000000000000, exp: -62 },                         // 5^28
    ExtendedFloat160 { frac: 244140625000000000000000000000000000000, exp: -30 },                         // 5^42
    ExtendedFloat160 { frac: 173472347597680709441192448139190673828, exp: 3 },                           // 5^56
    ExtendedFloat160 { frac: 246519032881566189191165176650870696772, exp: 35 },                          // 5^70
    ExtendedFloat160 { frac: 175162308040602133865466197911239516410, exp: 68 },                          // 5^84
    ExtendedFloat160 { frac: 248920611114445668285762562151204969623, exp: 100 },                         // 5^98
    ExtendedFloat160 { frac: 176868732008334225927912486150152183216, exp: 133 },                         // 5^112
    ExtendedFloat160 { frac: 251345585423243599518503524095297312920, exp: 165 },                         // 5^126
    ExtendedFloat160 { frac: 178591779887855465971216179422709524914, exp: 198 },                         // 5^140
    ExtendedFloat160 { frac: 253794183731564922327402455583054354682, exp: 230 },                         // 5^154
    ExtendedFloat160 { frac: 180331613628627651967947866455016278082, exp: 263 },                         // 5^168
    ExtendedFloat160 { frac: 256266636183436918326986907537468991453, exp: 295 },                         // 5^182
    ExtendedFloat160 { frac: 182088396757817547443627082897044283139, exp: 328 },                         // 5^196
    ExtendedFloat160 { frac: 258763175164940474024358370140027266101, exp: 360 },                         // 5^210
    ExtendedFloat160 { frac: 183862294395666818064937594201088633455, exp: 393 },                         // 5^224
    ExtendedFloat160 { frac: 261284035326052074402891767876281837538, exp: 425 },                         // 5^238
    ExtendedFloat160 { frac: 185653473271011701515143789632334288014, exp: 458 },                         // 5^252
    ExtendedFloat160 { frac: 263829453602698580304979415177988198613, exp: 490 },                         // 5^266
    ExtendedFloat160 { frac: 187462101736953869352205554703508169192, exp: 523 },                         // 5^280
    ExtendedFloat160 { frac: 266399669239026862544798113253119949479, exp: 555 },                         // 5^294
    ExtendedFloat160 { frac: 189288349786683953755640255602884245064, exp: 588 },                         // 5^308
    ExtendedFloat160 { frac: 268994923809890385876486015494726082500, exp: 620 },                         // 5^322
    ExtendedFloat160 { frac: 191132389069459226417170338759437756337, exp: 653 },                         // 5^336
    ExtendedFloat160 { frac: 271615461243554856334256923502490730495, exp: 685 },                         // 5^350
    ExtendedFloat160 { frac: 192994392906736931318972184714148973580, exp: 718 },                         // 5^364
    ExtendedFloat160 { frac: 274261527844625066050770363850331497104, exp: 750 },                         // 5^378
    ExtendedFloat160 { frac: 194874536308464787773268059716493991903, exp: 783 },                         // 5^392
    ExtendedFloat160 { frac: 276933372317195090450451374005771742621, exp: 815 },                         // 5^406
    ExtendedFloat160 { frac: 196772995989530194869453349330805553038, exp: 848 },                         // 5^420
    ExtendedFloat160 { frac: 279631245788224013707368483964622716141, exp: 880 },                         // 5^434
];
const BASE5_BIAS: i32 = 518;

// BASE6

const BASE6_STEP: i32 = 12;
const BASE6_SMALL_POWERS: [ExtendedFloat160; BASE6_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 6^0
    ExtendedFloat160 { frac: 255211775190703847597530955573826158592, exp: -125 },                        // 6^1
    ExtendedFloat160 { frac: 191408831393027885698148216680369618944, exp: -122 },                        // 6^2
    ExtendedFloat160 { frac: 287113247089541828547222325020554428416, exp: -120 },                        // 6^3
    ExtendedFloat160 { frac: 215334935317156371410416743765415821312, exp: -117 },                        // 6^4
    ExtendedFloat160 { frac: 323002402975734557115625115648123731968, exp: -115 },                        // 6^5
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -112 },                        // 6^6
    ExtendedFloat160 { frac: 181688851673850688377539127552069599232, exp: -109 },                        // 6^7
    ExtendedFloat160 { frac: 272533277510776032566308691328104398848, exp: -107 },                        // 6^8
    ExtendedFloat160 { frac: 204399958133082024424731518496078299136, exp: -104 },                        // 6^9
    ExtendedFloat160 { frac: 306599937199623036637097277744117448704, exp: -102 },                        // 6^10
    ExtendedFloat160 { frac: 229949952899717277477822958308088086528, exp: -99 },                         // 6^11
];
const BASE6_SMALL_INT_POWERS: [u128; BASE6_STEP as usize] = [1, 6, 36, 216, 1296, 7776, 46656, 279936, 1679616, 10077696, 60466176, 362797056];
const BASE6_LARGE_POWERS: [ExtendedFloat160; 73] = [
    ExtendedFloat160 { frac: 200594500948068090486693848039128919647, exp: -1337 },                       // 6^-468
    ExtendedFloat160 { frac: 203331264836010463860204821197057175037, exp: -1306 },                       // 6^-456
    ExtendedFloat160 { frac: 206105367118290399407064648402758144682, exp: -1275 },                       // 6^-444
    ExtendedFloat160 { frac: 208917317212507950117664039252872831665, exp: -1244 },                       // 6^-432
    ExtendedFloat160 { frac: 211767631486382365261996259087726574961, exp: -1213 },                       // 6^-420
    ExtendedFloat160 { frac: 214656833352574406771088703014069554755, exp: -1182 },                       // 6^-408
    ExtendedFloat160 { frac: 217585453364802351586979201161384846208, exp: -1151 },                       // 6^-396
    ExtendedFloat160 { frac: 220554029315269330081435801781477974040, exp: -1120 },                       // 6^-384
    ExtendedFloat160 { frac: 223563106333419891448609016293621894840, exp: -1089 },                       // 6^-372
    ExtendedFloat160 { frac: 226613236986043931067161987739751269180, exp: -1058 },                       // 6^-360
    ExtendedFloat160 { frac: 229704981378746362247969882824709232796, exp: -1027 },                       // 6^-348
    ExtendedFloat160 { frac: 232838907258801165579649662968151663564, exp: -996 },                        // 6^-336
    ExtendedFloat160 { frac: 236015590119408703302029793810763336632, exp: -965 },                        // 6^-324
    ExtendedFloat160 { frac: 239235613305375443823879271798297650114, exp: -934 },                        // 6^-312
    ExtendedFloat160 { frac: 242499568120235502703106353919523432682, exp: -903 },                        // 6^-300
    ExtendedFloat160 { frac: 245808053934833671173174941698733239342, exp: -872 },                        // 6^-288
    ExtendedFloat160 { frac: 249161678297389871677290466673500998400, exp: -841 },                        // 6^-276
    ExtendedFloat160 { frac: 252561057045065251911260457800735557729, exp: -810 },                        // 6^-264
    ExtendedFloat160 { frac: 256006814417050404626793229969178591795, exp: -779 },                        // 6^-252
    ExtendedFloat160 { frac: 259499583169196479959998361450291137700, exp: -748 },                        // 6^-240
    ExtendedFloat160 { frac: 263040004690210240376322725691803307553, exp: -717 },                        // 6^-228
    ExtendedFloat160 { frac: 266628729119434395515123988465075762881, exp: -686 },                        // 6^-216
    ExtendedFloat160 { frac: 270266415466234845327287688358055741312, exp: -655 },                        // 6^-204
    ExtendedFloat160 { frac: 273953731731016754981191818978678705632, exp: -624 },                        // 6^-192
    ExtendedFloat160 { frac: 277691355027891684120101092281051616669, exp: -593 },                        // 6^-180
    ExtendedFloat160 { frac: 281479971709018296242657937208050445965, exp: -562 },                        // 6^-168
    ExtendedFloat160 { frac: 285320277490639481303204301467482637509, exp: -531 },                        // 6^-156
    ExtendedFloat160 { frac: 289212977580839036146652597763405686112, exp: -500 },                        // 6^-144
    ExtendedFloat160 { frac: 293158786809041363160730749526943361727, exp: -469 },                        // 6^-132
    ExtendedFloat160 { frac: 297158429757277967604640789526650060843, exp: -438 },                        // 6^-120
    ExtendedFloat160 { frac: 301212640893244858516269504216828222245, exp: -407 },                        // 6^-108
    ExtendedFloat160 { frac: 305322164705175286969651759320250334279, exp: -376 },                        // 6^-96
    ExtendedFloat160 { frac: 309487755838552588810803796052767101096, exp: -345 },                        // 6^-84
    ExtendedFloat160 { frac: 313710179234688236904530296665341569850, exp: -314 },                        // 6^-72
    ExtendedFloat160 { frac: 317990210271190550439415903835536554761, exp: -283 },                        // 6^-60
    ExtendedFloat160 { frac: 322328634904349856025836233807108654402, exp: -252 },                        // 6^-48
    ExtendedFloat160 { frac: 326726249813466247246220462666861782844, exp: -221 },                        // 6^-36
    ExtendedFloat160 { frac: 331183862547146446042592332649497399781, exp: -190 },                        // 6^-24
    ExtendedFloat160 { frac: 335702291671596630919115661345637412333, exp: -159 },                        // 6^-12
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 6^0
    ExtendedFloat160 { frac: 172462464674787958108367218731066064896, exp: -96 },                         // 6^12
    ExtendedFloat160 { frac: 174815415743320440759790006808579407872, exp: -65 },                         // 6^24
    ExtendedFloat160 { frac: 177200468746272961345336076752392290304, exp: -34 },                         // 6^36
    ExtendedFloat160 { frac: 179618061658836457920697688990341398528, exp: -3 },                          // 6^48
    ExtendedFloat160 { frac: 182068638431613361423174859113151594496, exp: 28 },                          // 6^60
    ExtendedFloat160 { frac: 184552649072141716781794491390137475072, exp: 59 },                          // 6^72
    ExtendedFloat160 { frac: 187070549727531559196917812917453861026, exp: 90 },                          // 6^84
    ExtendedFloat160 { frac: 189622802768228720381105803326920695033, exp: 121 },                         // 6^96
    ExtendedFloat160 { frac: 192209876872921446586714266254161951235, exp: 152 },                         // 6^108
    ExtendedFloat160 { frac: 194832247114605420104007752175098574688, exp: 183 },                         // 6^120
    ExtendedFloat160 { frac: 197490395047822988635051696441052554380, exp: 214 },                         // 6^132
    ExtendedFloat160 { frac: 200184808797092622572327630249651738267, exp: 245 },                         // 6^144
    ExtendedFloat160 { frac: 202915983146544838776512848181734408257, exp: 276 },                         // 6^156
    ExtendedFloat160 { frac: 205684419630781050995309380627725821797, exp: 307 },                         // 6^168
    ExtendedFloat160 { frac: 208490626626972031635281014538153149532, exp: 338 },                         // 6^180
    ExtendedFloat160 { frac: 211335119448212897232599978727666183358, exp: 369 },                         // 6^192
    ExtendedFloat160 { frac: 214218420438151760708217936124820030498, exp: 400 },                         // 6^204
    ExtendedFloat160 { frac: 217141059066909427380630585083218539864, exp: 431 },                         // 6^216
    ExtendedFloat160 { frac: 220103572028307748788051030668660629356, exp: 462 },                         // 6^228
    ExtendedFloat160 { frac: 223106503338424488684979682521025988628, exp: 493 },                         // 6^240
    ExtendedFloat160 { frac: 226150404435492799169987273137391228527, exp: 524 },                         // 6^252
    ExtendedFloat160 { frac: 229235834281163651816744244429413474808, exp: 555 },                         // 6^264
    ExtendedFloat160 { frac: 232363359463149818964276081092475750857, exp: 586 },                         // 6^276
    ExtendedFloat160 { frac: 235533554299270254021060647605641184828, exp: 617 },                         // 6^288
    ExtendedFloat160 { frac: 238747000942913976797497733353022683918, exp: 648 },                         // 6^300
    ExtendedFloat160 { frac: 242004289489942830549695955106475311593, exp: 679 },                         // 6^312
    ExtendedFloat160 { frac: 245306018087052741642305313258629505287, exp: 710 },                         // 6^324
    ExtendedFloat160 { frac: 248652793041613380567795520750960012282, exp: 741 },                         // 6^336
    ExtendedFloat160 { frac: 252045228933006394543323172270604972624, exp: 772 },                         // 6^348
    ExtendedFloat160 { frac: 255483948725482657093998355855298189652, exp: 803 },                         // 6^360
    ExtendedFloat160 { frac: 258969583882559258973487053363982248701, exp: 834 },                         // 6^372
    ExtendedFloat160 { frac: 262502774482977247520692697766891651596, exp: 865 },                         // 6^384
    ExtendedFloat160 { frac: 266084169338241408156670471179837543899, exp: 896 },                         // 6^396
];
const BASE6_BIAS: i32 = 468;

// BASE7

const BASE7_STEP: i32 = 11;
const BASE7_SMALL_POWERS: [ExtendedFloat160; BASE7_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 7^0
    ExtendedFloat160 { frac: 297747071055821155530452781502797185024, exp: -125 },                        // 7^1
    ExtendedFloat160 { frac: 260528687173843511089146183814947536896, exp: -122 },                        // 7^2
    ExtendedFloat160 { frac: 227962601277113072203002910838079094784, exp: -119 },                        // 7^3
    ExtendedFloat160 { frac: 199467276117473938177627546983319207936, exp: -116 },                        // 7^4
    ExtendedFloat160 { frac: 174533866602789695905424103610404306944, exp: -113 },                        // 7^5
    ExtendedFloat160 { frac: 305434266554881967834492181318207537152, exp: -111 },                        // 7^6
    ExtendedFloat160 { frac: 267254983235521721855180658653431595008, exp: -108 },                        // 7^7
    ExtendedFloat160 { frac: 233848110331081506623283076321752645632, exp: -105 },                        // 7^8
    ExtendedFloat160 { frac: 204617096539696318295372691781533564928, exp: -102 },                        // 7^9
    ExtendedFloat160 { frac: 179039959472234278508451105308841869312, exp: -99 },                         // 7^10
];
const BASE7_SMALL_INT_POWERS: [u128; BASE7_STEP as usize] = [1, 7, 49, 343, 2401, 16807, 117649, 823543, 5764801, 40353607, 282475249];
const BASE7_LARGE_POWERS: [ExtendedFloat160; 73] = [
    ExtendedFloat160 { frac: 266008220737434990362931884041265905224, exp: -1332 },                       // 7^-429
    ExtendedFloat160 { frac: 244930930771854420969529305678871455590, exp: -1301 },                       // 7^-418
    ExtendedFloat160 { frac: 225523710066019267908218449093892378358, exp: -1270 },                       // 7^-407
    ExtendedFloat160 { frac: 207654229874777697952558290622819921790, exp: -1239 },                       // 7^-396
    ExtendedFloat160 { frac: 191200646585071218824745087565327043379, exp: -1208 },                       // 7^-385
    ExtendedFloat160 { frac: 176050770921424471559828841966017113260, exp: -1177 },                       // 7^-374
    ExtendedFloat160 { frac: 324202605959679334675793064499391555083, exp: -1147 },                       // 7^-363
    ExtendedFloat160 { frac: 298514255748300407155832676761118149894, exp: -1116 },                       // 7^-352
    ExtendedFloat160 { frac: 274861334384351909098688755274300249465, exp: -1085 },                       // 7^-341
    ExtendedFloat160 { frac: 253082563679127241756740166823033329043, exp: -1054 },                       // 7^-330
    ExtendedFloat160 { frac: 233029444399168140266755890269514708731, exp: -1023 },                       // 7^-319
    ExtendedFloat160 { frac: 214565243719567885018434177936373702812, exp: -992 },                        // 7^-308
    ExtendedFloat160 { frac: 197564062906901525077828647577582055243, exp: -961 },                        // 7^-297
    ExtendedFloat160 { frac: 181909978874749832229630901118123515094, exp: -930 },                        // 7^-286
    ExtendedFloat160 { frac: 334992507516972618831705765612473924060, exp: -900 },                        // 7^-275
    ExtendedFloat160 { frac: 308449214239576126269380201889590576494, exp: -869 },                        // 7^-264
    ExtendedFloat160 { frac: 284009091636748185128341566413187692045, exp: -838 },                        // 7^-253
    ExtendedFloat160 { frac: 261505493963360706349361998265286260917, exp: -807 },                        // 7^-242
    ExtendedFloat160 { frac: 240784979730461811097321743219099069136, exp: -776 },                        // 7^-231
    ExtendedFloat160 { frac: 221706265459654420066135217688280855198, exp: -745 },                        // 7^-220
    ExtendedFloat160 { frac: 204139262337252438224351496710052887294, exp: -714 },                        // 7^-209
    ExtendedFloat160 { frac: 187964189199610581269006081054389519147, exp: -683 },                        // 7^-198
    ExtendedFloat160 { frac: 173070755801490399767170008783447571912, exp: -652 },                        // 7^-187
    ExtendedFloat160 { frac: 318714821597104302344272129046782646031, exp: -622 },                        // 7^-176
    ExtendedFloat160 { frac: 293461298632634947256039303556899737616, exp: -591 },                        // 7^-165
    ExtendedFloat160 { frac: 270208750768480083796084384984407980627, exp: -560 },                        // 7^-154
    ExtendedFloat160 { frac: 248798629775241702400557051000361115726, exp: -529 },                        // 7^-143
    ExtendedFloat160 { frac: 229084950069124576377435007616984162844, exp: -498 },                        // 7^-132
    ExtendedFloat160 { frac: 210933293304638808346189699277966975327, exp: -467 },                        // 7^-121
    ExtendedFloat160 { frac: 194219891838880796776419735762035423417, exp: -436 },                        // 7^-110
    ExtendedFloat160 { frac: 178830784817964977889863278397948540205, exp: -405 },                        // 7^-99
    ExtendedFloat160 { frac: 329322082262710237520775170056072850605, exp: -375 },                        // 7^-88
    ExtendedFloat160 { frac: 303228087871569629902816732340606701122, exp: -344 },                        // 7^-77
    ExtendedFloat160 { frac: 279201663740542055384000770694089408486, exp: -313 },                        // 7^-66
    ExtendedFloat160 { frac: 257078984940548995242906668328695771951, exp: -282 },                        // 7^-55
    ExtendedFloat160 { frac: 236709207289964795762477040448859922676, exp: -251 },                        // 7^-44
    ExtendedFloat160 { frac: 217953438818817001855782665313698094789, exp: -220 },                        // 7^-33
    ExtendedFloat160 { frac: 200683792729517998822275406364627986706, exp: -189 },                        // 7^-22
    ExtendedFloat160 { frac: 184782515396710906443711214287193178833, exp: -158 },                        // 7^-11
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 7^0
    ExtendedFloat160 { frac: 313319929076409987389789434290473271296, exp: -97 },                         // 7^11
    ExtendedFloat160 { frac: 288493873028852398739253829029106548736, exp: -66 },                         // 7^22
    ExtendedFloat160 { frac: 265634921533798919351224824788236107776, exp: -35 },                         // 7^33
    ExtendedFloat160 { frac: 244587210111081219100242972308429222416, exp: -4 },                          // 7^44
    ExtendedFloat160 { frac: 225207224277966141315155116349116687572, exp: 27 },                          // 7^55
    ExtendedFloat160 { frac: 207362820991138609531788808643065835705, exp: 58 },                          // 7^66
    ExtendedFloat160 { frac: 190932327625202079604864455739987836428, exp: 89 },                          // 7^77
    ExtendedFloat160 { frac: 175803712344053086257499345217280929659, exp: 120 },                         // 7^88
    ExtendedFloat160 { frac: 323747640416561983962207324433251030705, exp: 150 },                         // 7^99
    ExtendedFloat160 { frac: 298095339619934405668456872884344580325, exp: 181 },                         // 7^110
    ExtendedFloat160 { frac: 274475611277932187651307585787198545738, exp: 212 },                         // 7^121
    ExtendedFloat160 { frac: 252727403529513497084111370284014563793, exp: 243 },                         // 7^132
    ExtendedFloat160 { frac: 232702425535702904483240997815950541938, exp: 274 },                         // 7^143
    ExtendedFloat160 { frac: 214264136353838934720254978302625752306, exp: 305 },                         // 7^154
    ExtendedFloat160 { frac: 197286813928859418414994791325492154413, exp: 336 },                         // 7^165
    ExtendedFloat160 { frac: 181654697853512422889189735564216996803, exp: 367 },                         // 7^176
    ExtendedFloat160 { frac: 334522400104752565046325502667785120787, exp: 397 },                         // 7^187
    ExtendedFloat160 { frac: 308016356015425756696586706818979868623, exp: 428 },                         // 7^198
    ExtendedFloat160 { frac: 283610531143243549972387138852602501496, exp: 459 },                         // 7^209
    ExtendedFloat160 { frac: 261138513603233655506120063924465915820, exp: 490 },                         // 7^220
    ExtendedFloat160 { frac: 240447077236577495145565086308268824286, exp: 521 },                         // 7^231
    ExtendedFloat160 { frac: 221395136833224081873557773886454798418, exp: 552 },                         // 7^242
    ExtendedFloat160 { frac: 203852786137945162132478938839041683065, exp: 583 },                         // 7^253
    ExtendedFloat160 { frac: 187700412080445632409314398956810353518, exp: 614 },                         // 7^264
    ExtendedFloat160 { frac: 172827879189879361689316520165778889316, exp: 645 },                         // 7^275
    ExtendedFloat160 { frac: 318267557265350256871483486812089934265, exp: 675 },                         // 7^286
    ExtendedFloat160 { frac: 293049473506426909088112326597996634777, exp: 706 },                         // 7^297
    ExtendedFloat160 { frac: 269829556805234360375792834335085449819, exp: 737 },                         // 7^308
    ExtendedFloat160 { frac: 248449481428986202531286841797890645972, exp: 768 },                         // 7^319
    ExtendedFloat160 { frac: 228763466660872229208763580066607974130, exp: 799 },                         // 7^330
    ExtendedFloat160 { frac: 210637282789652967136592584117374228567, exp: 830 },                         // 7^341
    ExtendedFloat160 { frac: 193947335860149215724544902551342320202, exp: 861 },                         // 7^352
    ExtendedFloat160 { frac: 178579824943969003959847035500732297209, exp: 892 },                         // 7^363
];
const BASE7_BIAS: i32 = 429;

// BASE9

const BASE9_STEP: i32 = 10;
const BASE9_SMALL_POWERS: [ExtendedFloat160; BASE9_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 9^0
    ExtendedFloat160 { frac: 191408831393027885698148216680369618944, exp: -124 },                        // 9^1
    ExtendedFloat160 { frac: 215334935317156371410416743765415821312, exp: -121 },                        // 9^2
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -118 },                        // 9^3
    ExtendedFloat160 { frac: 272533277510776032566308691328104398848, exp: -115 },                        // 9^4
    ExtendedFloat160 { frac: 306599937199623036637097277744117448704, exp: -112 },                        // 9^5
    ExtendedFloat160 { frac: 172462464674787958108367218731066064896, exp: -108 },                        // 9^6
    ExtendedFloat160 { frac: 194020272759136452871913121072449323008, exp: -105 },                        // 9^7
    ExtendedFloat160 { frac: 218272806854028509480902261206505488384, exp: -102 },                        // 9^8
    ExtendedFloat160 { frac: 245556907710782073166015043857318674432, exp: -99 },                         // 9^9
];
const BASE9_SMALL_INT_POWERS: [u128; BASE9_STEP as usize] = [1, 9, 81, 729, 6561, 59049, 531441, 4782969, 43046721, 387420489];
const BASE9_LARGE_POWERS: [ExtendedFloat160; 71] = [
    ExtendedFloat160 { frac: 228981627321413421057217801224022768458, exp: -1332 },                       // 9^-380
    ExtendedFloat160 { frac: 185894213211699326013762494029347155134, exp: -1300 },                       // 9^-370
    ExtendedFloat160 { frac: 301829093537629265639465570217176944359, exp: -1269 },                       // 9^-360
    ExtendedFloat160 { frac: 245033990385703656345786023933864839340, exp: -1237 },                       // 9^-350
    ExtendedFloat160 { frac: 198926007233479871031630637668169238011, exp: -1205 },                       // 9^-340
    ExtendedFloat160 { frac: 322988302900880006728964617948539328448, exp: -1174 },                       // 9^-330
    ExtendedFloat160 { frac: 262211676747596696167096696967233799204, exp: -1142 },                       // 9^-320
    ExtendedFloat160 { frac: 212871372756449173771443137071089544143, exp: -1110 },                       // 9^-310
    ExtendedFloat160 { frac: 172815421118085121562612771428651141606, exp: -1078 },                       // 9^-300
    ExtendedFloat160 { frac: 280593575260967566098415738074481154338, exp: -1047 },                       // 9^-290
    ExtendedFloat160 { frac: 227794354139073103116567345878808448350, exp: -1015 },                       // 9^-280
    ExtendedFloat160 { frac: 184930348919702200346046943747485274024, exp: -983 },                        // 9^-270
    ExtendedFloat160 { frac: 300264105147079021545114594266031000970, exp: -952 },                        // 9^-260
    ExtendedFloat160 { frac: 243763485459391712918376663011554847091, exp: -920 },                        // 9^-250
    ExtendedFloat160 { frac: 197894572893436379626501802082900685163, exp: -888 },                        // 9^-240
    ExtendedFloat160 { frac: 321313603691473325606249593990411338331, exp: -857 },                        // 9^-230
    ExtendedFloat160 { frac: 260852105259086286749566195634740776863, exp: -825 },                        // 9^-220
    ExtendedFloat160 { frac: 211767631486382365261996259087726574961, exp: -793 },                        // 9^-210
    ExtendedFloat160 { frac: 171919370559843833352674924374427532806, exp: -761 },                        // 9^-200
    ExtendedFloat160 { frac: 279138693352137745884317186629683060895, exp: -730 },                        // 9^-190
    ExtendedFloat160 { frac: 226613236986043931067161987739751269180, exp: -698 },                        // 9^-180
    ExtendedFloat160 { frac: 183971482278558945643179980616811190964, exp: -666 },                        // 9^-170
    ExtendedFloat160 { frac: 298707231244876640116631457791747347925, exp: -635 },                        // 9^-160
    ExtendedFloat160 { frac: 242499568120235502703106353919523432682, exp: -603 },                        // 9^-150
    ExtendedFloat160 { frac: 196868486555962367745019627988939060464, exp: -571 },                        // 9^-140
    ExtendedFloat160 { frac: 319647587822660709450189016904055940251, exp: -540 },                        // 9^-130
    ExtendedFloat160 { frac: 259499583169196479959998361450291137700, exp: -508 },                        // 9^-120
    ExtendedFloat160 { frac: 210669613131404954481085620515615417585, exp: -476 },                        // 9^-110
    ExtendedFloat160 { frac: 171027966037226738058674240289082148799, exp: -444 },                        // 9^-100
    ExtendedFloat160 { frac: 277691355027891684120101092281051616669, exp: -413 },                        // 9^-90
    ExtendedFloat160 { frac: 225438243943221318560556485110109738278, exp: -381 },                        // 9^-80
    ExtendedFloat160 { frac: 183017587375374702561553597022155160742, exp: -349 },                        // 9^-70
    ExtendedFloat160 { frac: 297158429757277967604640789526650060843, exp: -318 },                        // 9^-60
    ExtendedFloat160 { frac: 241242204211496523037749538228345943134, exp: -286 },                        // 9^-50
    ExtendedFloat160 { frac: 195847720491584060106836777189641681162, exp: -254 },                        // 9^-40
    ExtendedFloat160 { frac: 317990210271190550439415903835536554761, exp: -223 },                        // 9^-30
    ExtendedFloat160 { frac: 258154073926689380540223575440483383976, exp: -191 },                        // 9^-20
    ExtendedFloat160 { frac: 209577288018116110386327960504760073299, exp: -159 },                        // 9^-10
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 9^0
    ExtendedFloat160 { frac: 276251521174629832311766924339483508736, exp: -96 },                         // 9^10
    ExtendedFloat160 { frac: 224269343257001716702690972139746492416, exp: -64 },                         // 9^20
    ExtendedFloat160 { frac: 182068638431613361423174859113151594496, exp: -32 },                         // 9^30
    ExtendedFloat160 { frac: 295617658828691846632166420412766595202, exp: -1 },                          // 9^40
    ExtendedFloat160 { frac: 239991359753539474232337032335634004651, exp: 31 },                          // 9^50
    ExtendedFloat160 { frac: 194832247114605420104007752175098574688, exp: 63 },                          // 9^60
    ExtendedFloat160 { frac: 316341426247257477645159711999449660471, exp: 94 },                          // 9^70
    ExtendedFloat160 { frac: 256815541169845811576524073480007610450, exp: 126 },                         // 9^80
    ExtendedFloat160 { frac: 208490626626972031635281014538153149532, exp: 158 },                         // 9^90
    ExtendedFloat160 { frac: 338517997729425004575949331160209430911, exp: 189 },                         // 9^100
    ExtendedFloat160 { frac: 274819152881557244028610584245948464515, exp: 221 },                         // 9^110
    ExtendedFloat160 { frac: 223106503338424488684979682521025988628, exp: 253 },                         // 9^120
    ExtendedFloat160 { frac: 181124609802400910077427551154104473922, exp: 285 },                         // 9^130
    ExtendedFloat160 { frac: 294084876820548989626661915132664622178, exp: 316 },                         // 9^140
    ExtendedFloat160 { frac: 238747000942913976797497733353022683918, exp: 348 },                         // 9^150
    ExtendedFloat160 { frac: 193822038982362660063056049982127016523, exp: 380 },                         // 9^160
    ExtendedFloat160 { frac: 314701191193291934781116205950433765545, exp: 411 },                         // 9^170
    ExtendedFloat160 { frac: 255483948725482657093998355855298189652, exp: 443 },                         // 9^180
    ExtendedFloat160 { frac: 207409599591488195571905341445445255582, exp: 475 },                         // 9^190
    ExtendedFloat160 { frac: 336762776818711782198286065086981891498, exp: 506 },                         // 9^200
    ExtendedFloat160 { frac: 273394211439632029990640781047045990695, exp: 538 },                         // 9^210
    ExtendedFloat160 { frac: 221949692762318233808346663450192754968, exp: 570 },                         // 9^220
    ExtendedFloat160 { frac: 180185475975832393914650652957737664335, exp: 602 },                         // 9^230
    ExtendedFloat160 { frac: 292560042310176717160312096633717510967, exp: 633 },                         // 9^240
    ExtendedFloat160 { frac: 237509094151441049982785534773499431992, exp: 665 },                         // 9^250
    ExtendedFloat160 { frac: 192817068794482616882547252154136283242, exp: 697 },                         // 9^260
    ExtendedFloat160 { frac: 313069460782756034010893203297842312622, exp: 728 },                         // 9^270
    ExtendedFloat160 { frac: 254159260607975299744356396919078736707, exp: 760 },                         // 9^280
    ExtendedFloat160 { frac: 206334177697445743564032291193028958152, exp: 792 },                         // 9^290
    ExtendedFloat160 { frac: 335016656754825225194410391893304442626, exp: 823 },                         // 9^300
    ExtendedFloat160 { frac: 271976658340519265432186039827268288213, exp: 855 },                         // 9^310
    ExtendedFloat160 { frac: 220798880266451537830039115389735391778, exp: 887 },                         // 9^320
];
const BASE9_BIAS: i32 = 380;

// BASE10

const BASE10_STEP: i32 = 10;
const BASE10_SMALL_POWERS: [ExtendedFloat160; BASE10_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 10^0
    ExtendedFloat160 { frac: 212676479325586539664609129644855132160, exp: -124 },                        // 10^1
    ExtendedFloat160 { frac: 265845599156983174580761412056068915200, exp: -121 },                        // 10^2
    ExtendedFloat160 { frac: 332306998946228968225951765070086144000, exp: -118 },                        // 10^3
    ExtendedFloat160 { frac: 207691874341393105141219853168803840000, exp: -114 },                        // 10^4
    ExtendedFloat160 { frac: 259614842926741381426524816461004800000, exp: -111 },                        // 10^5
    ExtendedFloat160 { frac: 324518553658426726783156020576256000000, exp: -108 },                        // 10^6
    ExtendedFloat160 { frac: 202824096036516704239472512860160000000, exp: -104 },                        // 10^7
    ExtendedFloat160 { frac: 253530120045645880299340641075200000000, exp: -101 },                        // 10^8
    ExtendedFloat160 { frac: 316912650057057350374175801344000000000, exp: -98 },                         // 10^9
];
const BASE10_SMALL_INT_POWERS: [u128; BASE10_STEP as usize] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
const BASE10_LARGE_POWERS: [ExtendedFloat160; 68] = [
    ExtendedFloat160 { frac: 314560448973203621636593449998996267479, exp: -1357 },                       // 10^-370
    ExtendedFloat160 { frac: 183098279506203032585672015556481356895, exp: -1323 },                       // 10^-360
    ExtendedFloat160 { frac: 213154451346726893197828921904416471830, exp: -1290 },                       // 10^-350
    ExtendedFloat160 { frac: 248144440523729302452212341484167231049, exp: -1257 },                       // 10^-340
    ExtendedFloat160 { frac: 288878149031346317441449898160257412877, exp: -1224 },                       // 10^-330
    ExtendedFloat160 { frac: 336298426882534191759128470626028036788, exp: -1191 },                       // 10^-320
    ExtendedFloat160 { frac: 195751447977110622310503659901458325789, exp: -1157 },                       // 10^-310
    ExtendedFloat160 { frac: 227884678143438210606695688214919443462, exp: -1124 },                       // 10^-300
    ExtendedFloat160 { frac: 265292681454958173686982700851419292695, exp: -1091 },                       // 10^-290
    ExtendedFloat160 { frac: 308841328899094571460716776609676066664, exp: -1058 },                       // 10^-280
    ExtendedFloat160 { frac: 179769313486231590772930519078902473361, exp: -1024 },                       // 10^-270
    ExtendedFloat160 { frac: 209279024841067836122739267394531603625, exp: -991 },                        // 10^-260
    ExtendedFloat160 { frac: 243632850284999977008834559696879707771, exp: -958 },                        // 10^-250
    ExtendedFloat160 { frac: 283625966735416996535885333662014114404, exp: -925 },                        // 10^-240
    ExtendedFloat160 { frac: 330184081959790778970212365572822879074, exp: -892 },                        // 10^-230
    ExtendedFloat160 { frac: 192192430817400325887261637005036975649, exp: -858 },                        // 10^-220
    ExtendedFloat160 { frac: 223741436863085634409521749481834675708, exp: -825 },                        // 10^-210
    ExtendedFloat160 { frac: 260469313784369307581244210575049132700, exp: -792 },                        // 10^-200
    ExtendedFloat160 { frac: 303226189902482213896285056340332530323, exp: -759 },                        // 10^-190
    ExtendedFloat160 { frac: 176500872419263593559319302637789241459, exp: -725 },                        // 10^-180
    ExtendedFloat160 { frac: 205474058654233340126601167300005025998, exp: -692 },                        // 10^-170
    ExtendedFloat160 { frac: 239203286653190548679094257880939433814, exp: -659 },                        // 10^-160
    ExtendedFloat160 { frac: 278469275977917188637766821636980671685, exp: -626 },                        // 10^-150
    ExtendedFloat160 { frac: 324180903818827574883781864350871964922, exp: -593 },                        // 10^-140
    ExtendedFloat160 { frac: 188698121241077067612077729049413444545, exp: -559 },                        // 10^-130
    ExtendedFloat160 { frac: 219673525124179510879420825570604582952, exp: -526 },                        // 10^-120
    ExtendedFloat160 { frac: 255733641241886083594780445064656183766, exp: -493 },                        // 10^-110
    ExtendedFloat160 { frac: 297713141471480582369003031710926657271, exp: -460 },                        // 10^-100
    ExtendedFloat160 { frac: 173291855882550928723650886508942731464, exp: -426 },                        // 10^-90
    ExtendedFloat160 { frac: 201738271725539733566868685312735302682, exp: -393 },                        // 10^-80
    ExtendedFloat160 { frac: 234854258277383322788948059678933702737, exp: -360 },                        // 10^-70
    ExtendedFloat160 { frac: 273406340597876490546562778389702670669, exp: -327 },                        // 10^-60
    ExtendedFloat160 { frac: 318286871302263450979444638813965337664, exp: -294 },                        // 10^-50
    ExtendedFloat160 { frac: 185267342779705912677713576013900652565, exp: -260 },                        // 10^-40
    ExtendedFloat160 { frac: 215679573337205118357336120696157045389, exp: -227 },                        // 10^-30
    ExtendedFloat160 { frac: 251084069415467230553431576928306656644, exp: -194 },                        // 10^-20
    ExtendedFloat160 { frac: 292300327466180583640736966543256603931, exp: -161 },                        // 10^-10
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 10^0
    ExtendedFloat160 { frac: 198070406285660843983859875840000000000, exp: -94 },                         // 10^10
    ExtendedFloat160 { frac: 230584300921369395200000000000000000000, exp: -61 },                         // 10^20
    ExtendedFloat160 { frac: 268435456000000000000000000000000000000, exp: -28 },                         // 10^30
    ExtendedFloat160 { frac: 312500000000000000000000000000000000000, exp: 5 },                           // 10^40
    ExtendedFloat160 { frac: 181898940354585647583007812500000000000, exp: 39 },                          // 10^50
    ExtendedFloat160 { frac: 211758236813575084767080625169910490512, exp: 72 },                          // 10^60
    ExtendedFloat160 { frac: 246519032881566189191165176650870696772, exp: 105 },                         // 10^70
    ExtendedFloat160 { frac: 286985925493722536125179818657774823686, exp: 138 },                         // 10^80
    ExtendedFloat160 { frac: 334095588761524455767567058393935234851, exp: 171 },                         // 10^90
    ExtendedFloat160 { frac: 194469227433160678348252001680628882518, exp: 205 },                         // 10^100
    ExtendedFloat160 { frac: 226391976970667809187727982272194794517, exp: 238 },                         // 10^110
    ExtendedFloat160 { frac: 263554948580763080608714351281750475192, exp: 271 },                         // 10^120
    ExtendedFloat160 { frac: 306818341581107909568485747186642227685, exp: 304 },                         // 10^130
    ExtendedFloat160 { frac: 178591779887855465971216179422709524914, exp: 338 },                         // 10^140
    ExtendedFloat160 { frac: 207908195312897984370608091613638127355, exp: 371 },                         // 10^150
    ExtendedFloat160 { frac: 242036994678082392051126914580396990473, exp: 404 },                         // 10^160
    ExtendedFloat160 { frac: 281768146294730706199918541335962934504, exp: 437 },                         // 10^170
    ExtendedFloat160 { frac: 328021294314799255458543241647960309061, exp: 470 },                         // 10^180
    ExtendedFloat160 { frac: 190933522718725292628248712075851106236, exp: 504 },                         // 10^190
    ExtendedFloat160 { frac: 222275874948507748344271341427056009691, exp: 537 },                         // 10^200
    ExtendedFloat160 { frac: 258763175164940474024358370140027266101, exp: 570 },                         // 10^210
    ExtendedFloat160 { frac: 301239983137860514717593754339063617053, exp: 603 },                         // 10^220
    ExtendedFloat160 { frac: 175344747920672243180215448571289666610, exp: 637 },                         // 10^230
    ExtendedFloat160 { frac: 204128152598478183127259193653345185577, exp: 670 },                         // 10^240
    ExtendedFloat160 { frac: 237636445786894977939384050729387888658, exp: 703 },                         // 10^250
    ExtendedFloat160 { frac: 276645233140903266541874095249674153349, exp: 736 },                         // 10^260
    ExtendedFloat160 { frac: 322057438479856665411351825168442625260, exp: 769 },                         // 10^270
    ExtendedFloat160 { frac: 187462101736953869352205554703508169192, exp: 803 },                         // 10^280
    ExtendedFloat160 { frac: 218234609040610805796698614376955862613, exp: 836 },                         // 10^290
    ExtendedFloat160 { frac: 254058522452380049271391022923583936195, exp: 869 },                         // 10^300
];
const BASE10_BIAS: i32 = 370;

// BASE11

const BASE11_STEP: i32 = 9;
const BASE11_SMALL_POWERS: [ExtendedFloat160; BASE11_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 11^0
    ExtendedFloat160 { frac: 233944127258145193631070042609340645376, exp: -124 },                        // 11^1
    ExtendedFloat160 { frac: 321673174979949641242721308587843387392, exp: -121 },                        // 11^2
    ExtendedFloat160 { frac: 221150307798715378354370899654142328832, exp: -117 },                        // 11^3
    ExtendedFloat160 { frac: 304081673223233645237259987024445702144, exp: -114 },                        // 11^4
    ExtendedFloat160 { frac: 209056150340973131100616241079306420224, exp: -110 },                        // 11^5
    ExtendedFloat160 { frac: 287452206718838055263347331484046327808, exp: -107 },                        // 11^6
    ExtendedFloat160 { frac: 197623392119201162993551290395281850368, exp: -103 },                        // 11^7
    ExtendedFloat160 { frac: 271732164163901599116133024293512544256, exp: -100 },                        // 11^8
];
const BASE11_SMALL_INT_POWERS: [u128; BASE11_STEP as usize] = [1, 11, 121, 1331, 14641, 161051, 1771561, 19487171, 214358881];
const BASE11_LARGE_POWERS: [ExtendedFloat160; 72] = [
    ExtendedFloat160 { frac: 284067592451440402783250740870803882848, exp: -1342 },                       // 11^-351
    ExtendedFloat160 { frac: 311907624690236024263722849036730520835, exp: -1311 },                       // 11^-342
    ExtendedFloat160 { frac: 171238059048456285069153007806767780751, exp: -1279 },                       // 11^-333
    ExtendedFloat160 { frac: 188020237695718721716314139814895141308, exp: -1248 },                       // 11^-324
    ExtendedFloat160 { frac: 206447153042951189194086232691622882083, exp: -1217 },                       // 11^-315
    ExtendedFloat160 { frac: 226679997440031906769564274339983654138, exp: -1186 },                       // 11^-306
    ExtendedFloat160 { frac: 248895760886189129917706060109685643694, exp: -1155 },                       // 11^-297
    ExtendedFloat160 { frac: 273288779277949488106534827711219187027, exp: -1124 },                       // 11^-288
    ExtendedFloat160 { frac: 300072434393060227248554937081761703289, exp: -1093 },                       // 11^-279
    ExtendedFloat160 { frac: 329481020481262984343182946079685486146, exp: -1062 },                       // 11^-270
    ExtendedFloat160 { frac: 180885896895108223589085363619768036079, exp: -1030 },                       // 11^-261
    ExtendedFloat160 { frac: 198613611477559667549559644399735817047, exp: -999 },                        // 11^-252
    ExtendedFloat160 { frac: 218078729968836026458633966547967004969, exp: -968 },                        // 11^-243
    ExtendedFloat160 { frac: 239451526564652757041971603611460905574, exp: -937 },                        // 11^-234
    ExtendedFloat160 { frac: 262918963175987885652063848834117643272, exp: -906 },                        // 11^-225
    ExtendedFloat160 { frac: 288686324907902004950333378158035100892, exp: -875 },                        // 11^-216
    ExtendedFloat160 { frac: 316979015823390018300576675081611005238, exp: -844 },                        // 11^-207
    ExtendedFloat160 { frac: 174022265350496153683469750680295263035, exp: -812 },                        // 11^-198
    ExtendedFloat160 { frac: 191077310017213090920037889613280023761, exp: -781 },                        // 11^-189
    ExtendedFloat160 { frac: 209803833606456768694276179860414245615, exp: -750 },                        // 11^-180
    ExtendedFloat160 { frac: 230365649338482385703357683130616542302, exp: -719 },                        // 11^-171
    ExtendedFloat160 { frac: 252942624941184287338239913752743474826, exp: -688 },                        // 11^-162
    ExtendedFloat160 { frac: 277732256071429933018369293137137136134, exp: -657 },                        // 11^-153
    ExtendedFloat160 { frac: 304951393939484349279174830456271141999, exp: -626 },                        // 11^-144
    ExtendedFloat160 { frac: 334838142249192351062534286575243350001, exp: -595 },                        // 11^-135
    ExtendedFloat160 { frac: 183826970023851061892407973678416422311, exp: -563 },                        // 11^-126
    ExtendedFloat160 { frac: 201842924352393404728314593418040675821, exp: -532 },                        // 11^-117
    ExtendedFloat160 { frac: 221624531513738305774870271734740961947, exp: -501 },                        // 11^-108
    ExtendedFloat160 { frac: 243344834238186465986122993315325591576, exp: -470 },                        // 11^-99
    ExtendedFloat160 { frac: 267193834301414676708834595107417746247, exp: -439 },                        // 11^-90
    ExtendedFloat160 { frac: 293380154594991977782504639045547494542, exp: -408 },                        // 11^-81
    ExtendedFloat160 { frac: 322132864088045607402818330486415493575, exp: -377 },                        // 11^-72
    ExtendedFloat160 { frac: 176851740822108453297561032060281913714, exp: -345 },                        // 11^-63
    ExtendedFloat160 { frac: 194184088111306107232610402385759498575, exp: -314 },                        // 11^-54
    ExtendedFloat160 { frac: 213215091354676888577329040601887973466, exp: -283 },                        // 11^-45
    ExtendedFloat160 { frac: 234111227256299197427917870802090967668, exp: -252 },                        // 11^-36
    ExtendedFloat160 { frac: 257055288062508663991611489278928709768, exp: -221 },                        // 11^-27
    ExtendedFloat160 { frac: 282247980565918687696808555402942563478, exp: -190 },                        // 11^-18
    ExtendedFloat160 { frac: 309909681819761564465444461107912469729, exp: -159 },                        // 11^-9
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 11^0
    ExtendedFloat160 { frac: 186815862862682349392341454201789874176, exp: -96 },                         // 11^9
    ExtendedFloat160 { frac: 205124743505955904636591107127579246592, exp: -65 },                         // 11^18
    ExtendedFloat160 { frac: 225227985212968648451224991661780107264, exp: -34 },                         // 11^27
    ExtendedFloat160 { frac: 247301444262965381085386823495099626888, exp: -3 },                          // 11^36
    ExtendedFloat160 { frac: 271538211722310826714720080747313199115, exp: 28 },                          // 11^45
    ExtendedFloat160 { frac: 298150302539063592923933328180322755271, exp: 59 },                          // 11^54
    ExtendedFloat160 { frac: 327370510177191550022527822209865447333, exp: 90 },                          // 11^63
    ExtendedFloat160 { frac: 179727221507067050840782578187764330990, exp: 122 },                         // 11^72
    ExtendedFloat160 { frac: 197341380157710189331417385894277071911, exp: 153 },                         // 11^81
    ExtendedFloat160 { frac: 216681813672942089280666997109850212851, exp: 184 },                         // 11^90
    ExtendedFloat160 { frac: 237917705546974217796728292841033536358, exp: 215 },                         // 11^99
    ExtendedFloat160 { frac: 261234820095126400184969690376374338338, exp: 246 },                         // 11^108
    ExtendedFloat160 { frac: 286837127456489808703442688252756740915, exp: 277 },                         // 11^117
    ExtendedFloat160 { frac: 314948587854906379895896832699151980516, exp: 308 },                         // 11^126
    ExtendedFloat160 { frac: 172907555363184572545974225978641470455, exp: 340 },                         // 11^135
    ExtendedFloat160 { frac: 189853352925309785288573204303090400331, exp: 371 },                         // 11^144
    ExtendedFloat160 { frac: 208459922652152508348644224820166488004, exp: 402 },                         // 11^153
    ExtendedFloat160 { frac: 228890028448627145607277691781146666326, exp: 433 },                         // 11^162
    ExtendedFloat160 { frac: 251322385889182188596856419989450117198, exp: 464 },                         // 11^171
    ExtendedFloat160 { frac: 275953225560490099471194727985491113834, exp: 495 },                         // 11^180
    ExtendedFloat160 { frac: 302998009619470597652934458975103615488, exp: 526 },                         // 11^189
    ExtendedFloat160 { frac: 332693316582509542060143699364121908613, exp: 557 },                         // 11^198
    ExtendedFloat160 { frac: 182649455416682266091504883729813020363, exp: 589 },                         // 11^207
    ExtendedFloat160 { frac: 200550007476552106538373760555841065584, exp: 620 },                         // 11^216
    ExtendedFloat160 { frac: 220204902374823007850602460837751774656, exp: 651 },                         // 11^225
    ExtendedFloat160 { frac: 241786074406278472344858080843135119733, exp: 682 },                         // 11^234
    ExtendedFloat160 { frac: 265482308278902675848717553283442644763, exp: 713 },                         // 11^243
    ExtendedFloat160 { frac: 291500890537904924029114199837605959154, exp: 744 },                         // 11^252
    ExtendedFloat160 { frac: 320069422837484936971208194402230692262, exp: 775 },                         // 11^261
    ExtendedFloat160 { frac: 175718906461109937023963056381563419957, exp: 807 },                         // 11^270
    ExtendedFloat160 { frac: 192940230367248485471498733602562118815, exp: 838 },                         // 11^279
    ExtendedFloat160 { frac: 211849329385655768374338506351963365331, exp: 869 },                         // 11^288
];
const BASE11_BIAS: i32 = 351;

// BASE12

const BASE12_STEP: i32 = 9;
const BASE12_SMALL_POWERS: [ExtendedFloat160; BASE12_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 12^0
    ExtendedFloat160 { frac: 255211775190703847597530955573826158592, exp: -124 },                        // 12^1
    ExtendedFloat160 { frac: 191408831393027885698148216680369618944, exp: -120 },                        // 12^2
    ExtendedFloat160 { frac: 287113247089541828547222325020554428416, exp: -117 },                        // 12^3
    ExtendedFloat160 { frac: 215334935317156371410416743765415821312, exp: -113 },                        // 12^4
    ExtendedFloat160 { frac: 323002402975734557115625115648123731968, exp: -110 },                        // 12^5
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -106 },                        // 12^6
    ExtendedFloat160 { frac: 181688851673850688377539127552069599232, exp: -102 },                        // 12^7
    ExtendedFloat160 { frac: 272533277510776032566308691328104398848, exp: -99 },                         // 12^8
];
const BASE12_SMALL_INT_POWERS: [u128; BASE12_STEP as usize] = [1, 12, 144, 1728, 20736, 248832, 2985984, 35831808, 429981696];
const BASE12_LARGE_POWERS: [ExtendedFloat160; 70] = [
    ExtendedFloat160 { frac: 327060412939660347810097743318775450603, exp: -1354 },                       // 12^-342
    ExtendedFloat160 { frac: 196457827999613483457829403129377966132, exp: -1321 },                       // 12^-333
    ExtendedFloat160 { frac: 236015590119408703302029793810763336632, exp: -1289 },                       // 12^-324
    ExtendedFloat160 { frac: 283538504658222748235708766575760177913, exp: -1257 },                       // 12^-315
    ExtendedFloat160 { frac: 170315197362908885300398426895467760677, exp: -1224 },                       // 12^-306
    ExtendedFloat160 { frac: 204609010601448705405745986119597896326, exp: -1192 },                       // 12^-297
    ExtendedFloat160 { frac: 245808053934833671173174941698733239342, exp: -1160 },                       // 12^-288
    ExtendedFloat160 { frac: 295302729833943551617529441983408590696, exp: -1128 },                       // 12^-279
    ExtendedFloat160 { frac: 177381702616012906692133545122052956869, exp: -1095 },                       // 12^-270
    ExtendedFloat160 { frac: 213098391881773806300126011269370626834, exp: -1063 },                       // 12^-261
    ExtendedFloat160 { frac: 256006814417050404626793229969178591795, exp: -1031 },                       // 12^-252
    ExtendedFloat160 { frac: 307555061533862494767405465422567274311, exp: -999 },                        // 12^-243
    ExtendedFloat160 { frac: 184741402471039290909022270993420155647, exp: -966 },                        // 12^-234
    ExtendedFloat160 { frac: 221940003957364890317522299802459040748, exp: -934 },                        // 12^-225
    ExtendedFloat160 { frac: 266628729119434395515123988465075762881, exp: -902 },                        // 12^-216
    ExtendedFloat160 { frac: 320315751663685742610118741757695693407, exp: -870 },                        // 12^-207
    ExtendedFloat160 { frac: 192406461791880080316008520325217417399, exp: -837 },                        // 12^-198
    ExtendedFloat160 { frac: 231148461148045387015380597263260157877, exp: -805 },                        // 12^-189
    ExtendedFloat160 { frac: 277691355027891684120101092281051616669, exp: -773 },                        // 12^-180
    ExtendedFloat160 { frac: 333605892395873536287594592246578306329, exp: -741 },                        // 12^-171
    ExtendedFloat160 { frac: 200389550171752283164939097875653100692, exp: -708 },                        // 12^-162
    ExtendedFloat160 { frac: 240738984132727062349578629363188475398, exp: -676 },                        // 12^-153
    ExtendedFloat160 { frac: 289212977580839036146652597763405686112, exp: -644 },                        // 12^-144
    ExtendedFloat160 { frac: 173723725516468955947099703423373843986, exp: -611 },                        // 12^-135
    ExtendedFloat160 { frac: 208703862874796048578293668364396201854, exp: -579 },                        // 12^-126
    ExtendedFloat160 { frac: 250727425107703285166415666163110988836, exp: -547 },                        // 12^-117
    ExtendedFloat160 { frac: 301212640893244858516269504216828222245, exp: -515 },                        // 12^-108
    ExtendedFloat160 { frac: 180931653158622392278312153671259457350, exp: -482 },                        // 12^-99
    ExtendedFloat160 { frac: 217363142646555453321168098187951653993, exp: -450 },                        // 12^-90
    ExtendedFloat160 { frac: 261130293988778746809115702919522241550, exp: -418 },                        // 12^-81
    ExtendedFloat160 { frac: 313710179234688236904530296665341569850, exp: -386 },                        // 12^-72
    ExtendedFloat160 { frac: 188438643123668474334468683754392032451, exp: -353 },                        // 12^-63
    ExtendedFloat160 { frac: 226381702429392491474935736226666160567, exp: -321 },                        // 12^-54
    ExtendedFloat160 { frac: 271964785700545191021799322274747927151, exp: -289 },                        // 12^-45
    ExtendedFloat160 { frac: 326726249813466247246220462666861782844, exp: -257 },                        // 12^-36
    ExtendedFloat160 { frac: 196257103731642338395610271199702162833, exp: -224 },                        // 12^-27
    ExtendedFloat160 { frac: 235774449020380624184618955567855082461, exp: -192 },                        // 12^-18
    ExtendedFloat160 { frac: 283248808597909657338003839260381566656, exp: -160 },                        // 12^-9
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 12^0
    ExtendedFloat160 { frac: 204399958133082024424731518496078299136, exp: -95 },                         // 12^9
    ExtendedFloat160 { frac: 245556907710782073166015043857318674432, exp: -63 },                         // 12^18
    ExtendedFloat160 { frac: 295001014066853243782145636489477750784, exp: -31 },                         // 12^27
    ExtendedFloat160 { frac: 177200468746272961345336076752392290304, exp: 2 },                           // 12^36
    ExtendedFloat160 { frac: 212880665669732098276382446210774990848, exp: 34 },                          // 12^45
    ExtendedFloat160 { frac: 255745247947835503562868389206950936576, exp: 66 },                          // 12^54
    ExtendedFloat160 { frac: 307240827353347547401607574753443315712, exp: 98 },                          // 12^63
    ExtendedFloat160 { frac: 184552649072141716781794491390137475072, exp: 131 },                         // 12^72
    ExtendedFloat160 { frac: 221713244121518884974124815309574946401, exp: 163 },                         // 12^81
    ExtendedFloat160 { frac: 266356310061270520809673995345359110719, exp: 195 },                         // 12^90
    ExtendedFloat160 { frac: 319988479671385965643116043114178672868, exp: 227 },                         // 12^99
    ExtendedFloat160 { frac: 192209876872921446586714266254161951235, exp: 260 },                         // 12^108
    ExtendedFloat160 { frac: 230912292876569386789935113689005718149, exp: 292 },                         // 12^117
    ExtendedFloat160 { frac: 277407633098725295421526662764935275289, exp: 324 },                         // 12^126
    ExtendedFloat160 { frac: 333265041643201293321649737744276185517, exp: 356 },                         // 12^135
    ExtendedFloat160 { frac: 200184808797092622572327630249651738267, exp: 389 },                         // 12^144
    ExtendedFloat160 { frac: 240493017062571660772163375622796335712, exp: 421 },                         // 12^153
    ExtendedFloat160 { frac: 288917483816076538023589582665008561757, exp: 453 },                         // 12^162
    ExtendedFloat160 { frac: 173546229063471511777292289904643662141, exp: 486 },                         // 12^171
    ExtendedFloat160 { frac: 208490626626972031635281014538153149532, exp: 518 },                         // 12^180
    ExtendedFloat160 { frac: 250471252679363433757155530343900661758, exp: 550 },                         // 12^189
    ExtendedFloat160 { frac: 300904886870600004067510516586852827477, exp: 582 },                         // 12^198
    ExtendedFloat160 { frac: 180746792244690548097558883605316900733, exp: 615 },                         // 12^207
    ExtendedFloat160 { frac: 217141059066909427380630585083218539864, exp: 647 },                         // 12^216
    ExtendedFloat160 { frac: 260863492774290665230282703014708894052, exp: 679 },                         // 12^225
    ExtendedFloat160 { frac: 313389656266867868879861721401276560157, exp: 711 },                         // 12^234
    ExtendedFloat160 { frac: 188246112191795662327951607127115677904, exp: 744 },                         // 12^243
    ExtendedFloat160 { frac: 226150404435492799169987273137391228527, exp: 776 },                         // 12^252
    ExtendedFloat160 { frac: 271686914703601365116141326731156710883, exp: 808 },                         // 12^261
    ExtendedFloat160 { frac: 326392428107359965184387801150473482685, exp: 840 },                         // 12^270
    ExtendedFloat160 { frac: 196056584547032659751107943421776414785, exp: 873 },                         // 12^279
];
const BASE12_BIAS: i32 = 342;

// BASE13

const BASE13_STEP: i32 = 8;
const BASE13_SMALL_POWERS: [ExtendedFloat160; BASE13_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 13^0
    ExtendedFloat160 { frac: 276479423123262501563991868538311671808, exp: -124 },                        // 13^1
    ExtendedFloat160 { frac: 224639531287650782520743393187378233344, exp: -120 },                        // 13^2
    ExtendedFloat160 { frac: 182519619171216260798104006964744814592, exp: -116 },                        // 13^3
    ExtendedFloat160 { frac: 296594381153226423796919011317710323712, exp: -113 },                        // 13^4
    ExtendedFloat160 { frac: 240982934686996469334996696695639638016, exp: -109 },                        // 13^5
    ExtendedFloat160 { frac: 195798634433184631334684816065207205888, exp: -105 },                        // 13^6
    ExtendedFloat160 { frac: 318172780953925025918862826105961709568, exp: -102 },                        // 13^7
];
const BASE13_SMALL_INT_POWERS: [u128; BASE13_STEP as usize] = [1, 13, 169, 2197, 28561, 371293, 4826809, 62748517];
const BASE13_LARGE_POWERS: [ExtendedFloat160; 76] = [
    ExtendedFloat160 { frac: 203144294263616275774101896291847629186, exp: -1341 },                       // 13^-328
    ExtendedFloat160 { frac: 308660867859974258960304731483787471388, exp: -1312 },                       // 13^-320
    ExtendedFloat160 { frac: 234492264952419818661296823055466035238, exp: -1282 },                       // 13^-312
    ExtendedFloat160 { frac: 178145751691013993109826848158654507171, exp: -1252 },                       // 13^-304
    ExtendedFloat160 { frac: 270677660536016922109887166011364690074, exp: -1223 },                       // 13^-296
    ExtendedFloat160 { frac: 205636101949623162244352604410701629545, exp: -1193 },                       // 13^-288
    ExtendedFloat160 { frac: 312446962496257587924386799659895562352, exp: -1164 },                       // 13^-280
    ExtendedFloat160 { frac: 237368592984352411701605597109952160960, exp: -1134 },                       // 13^-272
    ExtendedFloat160 { frac: 180330922359490147340520816471555077038, exp: -1104 },                       // 13^-264
    ExtendedFloat160 { frac: 273997845714729128559081401928980460360, exp: -1075 },                       // 13^-256
    ExtendedFloat160 { frac: 208158474636564731932432542641850208607, exp: -1045 },                       // 13^-248
    ExtendedFloat160 { frac: 316279498110609430339960234785638219567, exp: -1016 },                       // 13^-240
    ExtendedFloat160 { frac: 240280202619066059925234911435300433349, exp: -986 },                        // 13^-232
    ExtendedFloat160 { frac: 182542896759209079117924981191278218615, exp: -956 },                        // 13^-224
    ExtendedFloat160 { frac: 277358756934885281135534847026826478882, exp: -927 },                        // 13^-216
    ExtendedFloat160 { frac: 210711787240726612911538723377341283448, exp: -897 },                        // 13^-208
    ExtendedFloat160 { frac: 320159044357159213189440578296814412270, exp: -868 },                        // 13^-200
    ExtendedFloat160 { frac: 243227526627608078053980201967728671175, exp: -838 },                        // 13^-192
    ExtendedFloat160 { frac: 184782003669985976085437039910686134984, exp: -808 },                        // 13^-184
    ExtendedFloat160 { frac: 280760893750083272032555777889582843511, exp: -779 },                        // 13^-176
    ExtendedFloat160 { frac: 213296419277190995686832685182416132955, exp: -749 },                        // 13^-168
    ExtendedFloat160 { frac: 324086177877525444638594829459150002632, exp: -720 },                        // 13^-160
    ExtendedFloat160 { frac: 246211003089480177174216643591955743445, exp: -690 },                        // 13^-152
    ExtendedFloat160 { frac: 187048575904513609075482455349961378053, exp: -660 },                        // 13^-144
    ExtendedFloat160 { frac: 284204761841543230201034707437221523825, exp: -631 },                        // 13^-136
    ExtendedFloat160 { frac: 215912754916246372204794564138901211197, exp: -601 },                        // 13^-128
    ExtendedFloat160 { frac: 328061482386525623150638173263346085618, exp: -572 },                        // 13^-120
    ExtendedFloat160 { frac: 249231075457753005677502945704601914895, exp: -542 },                        // 13^-112
    ExtendedFloat160 { frac: 189342950357830398121128390896356453239, exp: -512 },                        // 13^-104
    ExtendedFloat160 { frac: 287690873093270135400938070638952191684, exp: -483 },                        // 13^-96
    ExtendedFloat160 { frac: 218561183040489207765436578950529361150, exp: -453 },                        // 13^-88
    ExtendedFloat160 { frac: 332085548758937481126223466223271077691, exp: -424 },                        // 13^-80
    ExtendedFloat160 { frac: 252288192624979397908801362020362227394, exp: -394 },                        // 13^-72
    ExtendedFloat160 { frac: 191665468057395263450492571375008762286, exp: -364 },                        // 13^-64
    ExtendedFloat160 { frac: 291219745668138391592456499213129019442, exp: -335 },                        // 13^-56
    ExtendedFloat160 { frac: 221242097302626033221832363181059870585, exp: -305 },                        // 13^-48
    ExtendedFloat160 { frac: 336158975117324458944453423021025623710, exp: -276 },                        // 13^-40
    ExtendedFloat160 { frac: 255382808989916127627430380985009698372, exp: -246 },                        // 13^-32
    ExtendedFloat160 { frac: 194016474213776704407345321083633835842, exp: -216 },                        // 13^-24
    ExtendedFloat160 { frac: 294791904084909668600573533146696407742, exp: -187 },                        // 13^-16
    ExtendedFloat160 { frac: 223955896183984548959338021657683010637, exp: -157 },                        // 13^-8
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 13^0
    ExtendedFloat160 { frac: 258515384525064083559076046211093889024, exp: -98 },                         // 13^8
    ExtendedFloat160 { frac: 196396318271963640537903969427202768896, exp: -68 },                         // 13^16
    ExtendedFloat160 { frac: 298407879296195459704416306334760173568, exp: -39 },                         // 13^24
    ExtendedFloat160 { frac: 226702983053742437531808354380178063872, exp: -9 },                          // 13^32
    ExtendedFloat160 { frac: 172228168527856562581024588413877561828, exp: 21 },                          // 13^40
    ExtendedFloat160 { frac: 261686384845036905964465455013477410965, exp: 50 },                          // 13^48
    ExtendedFloat160 { frac: 198805353963305641500281734389939339052, exp: 80 },                          // 13^56
    ExtendedFloat160 { frac: 302068208767375943221882620634577632307, exp: 109 },                         // 13^64
    ExtendedFloat160 { frac: 229483766228882688509711411881831930515, exp: 139 },                         // 13^72
    ExtendedFloat160 { frac: 174340752962680465097022527378326959423, exp: 169 },                         // 13^80
    ExtendedFloat160 { frac: 264896281275768246289732905498826672107, exp: 198 },                         // 13^88
    ExtendedFloat160 { frac: 201243939358090266003114541899396801267, exp: 228 },                         // 13^96
    ExtendedFloat160 { frac: 305773436556486878301580275426022079145, exp: 257 },                         // 13^104
    ExtendedFloat160 { frac: 232298659034884347081172590621418853088, exp: 287 },                         // 13^112
    ExtendedFloat160 { frac: 176479250771793883849064971202097683281, exp: 317 },                         // 13^120
    ExtendedFloat160 { frac: 268145550924567936613388404500729180677, exp: 346 },                         // 13^128
    ExtendedFloat160 { frac: 203712436918765324677453955129179600235, exp: 376 },                         // 13^136
    ExtendedFloat160 { frac: 309524113395086409019191949184662333574, exp: 405 },                         // 13^144
    ExtendedFloat160 { frac: 235148079867157707257686510259099274415, exp: 435 },                         // 13^152
    ExtendedFloat160 { frac: 178643979813719299400243488890704650366, exp: 465 },                         // 13^160
    ExtendedFloat160 { frac: 271434676751037481783657313824389119896, exp: 494 },                         // 13^168
    ExtendedFloat160 { frac: 206211213553813977645157972930830162165, exp: 524 },                         // 13^176
    ExtendedFloat160 { frac: 313320796770113796525378889251061848013, exp: 553 },                         // 13^184
    ExtendedFloat160 { frac: 238032452253233081187768481790039531230, exp: 583 },                         // 13^192
    ExtendedFloat160 { frac: 180835261845894060934288352757379708280, exp: 613 },                         // 13^200
    ExtendedFloat160 { frac: 274764147638855414866553566635921015415, exp: 642 },                         // 13^208
    ExtendedFloat160 { frac: 208740640672290674581982064710037218783, exp: 672 },                         // 13^216
    ExtendedFloat160 { frac: 317164051006752246428908041221997358677, exp: 701 },                         // 13^224
    ExtendedFloat160 { frac: 240952204915712388090624317255539471706, exp: 731 },                         // 13^232
    ExtendedFloat160 { frac: 183053422572495239382420468754129441919, exp: 761 },                         // 13^240
    ExtendedFloat160 { frac: 278134458468443185818361944895170594350, exp: 790 },                         // 13^248
    ExtendedFloat160 { frac: 211301094239026043298793364640604498188, exp: 820 },                         // 13^256
    ExtendedFloat160 { frac: 321054447352308147843744474962905446408, exp: 849 },                         // 13^264
    ExtendedFloat160 { frac: 243907771835992919704646129069158162971, exp: 879 },                         // 13^272
];
const BASE13_BIAS: i32 = 328;

// BASE14

const BASE14_STEP: i32 = 8;
const BASE14_SMALL_POWERS: [ExtendedFloat160; BASE14_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 14^0
    ExtendedFloat160 { frac: 297747071055821155530452781502797185024, exp: -124 },                        // 14^1
    ExtendedFloat160 { frac: 260528687173843511089146183814947536896, exp: -120 },                        // 14^2
    ExtendedFloat160 { frac: 227962601277113072203002910838079094784, exp: -116 },                        // 14^3
    ExtendedFloat160 { frac: 199467276117473938177627546983319207936, exp: -112 },                        // 14^4
    ExtendedFloat160 { frac: 174533866602789695905424103610404306944, exp: -108 },                        // 14^5
    ExtendedFloat160 { frac: 305434266554881967834492181318207537152, exp: -105 },                        // 14^6
    ExtendedFloat160 { frac: 267254983235521721855180658653431595008, exp: -101 },                        // 14^7
];
const BASE14_SMALL_INT_POWERS: [u128; BASE14_STEP as usize] = [1, 14, 196, 2744, 38416, 537824, 7529536, 105413504];
const BASE14_LARGE_POWERS: [ExtendedFloat160; 74] = [
    ExtendedFloat160 { frac: 266319365027620731733435303165159667122, exp: -1346 },                       // 14^-320
    ExtendedFloat160 { frac: 183019416550468566646294542446352912686, exp: -1315 },                       // 14^-312
    ExtendedFloat160 { frac: 251548413169278560512334209582633427955, exp: -1285 },                       // 14^-304
    ExtendedFloat160 { frac: 172868555043538834443100066630384298338, exp: -1254 },                       // 14^-296
    ExtendedFloat160 { frac: 237596707101714066585640360644079693184, exp: -1224 },                       // 14^-288
    ExtendedFloat160 { frac: 326561387705008590881101164026582541311, exp: -1194 },                       // 14^-280
    ExtendedFloat160 { frac: 224418808746721891131396635947419054595, exp: -1163 },                       // 14^-272
    ExtendedFloat160 { frac: 308449214239576126269380201889590576494, exp: -1133 },                       // 14^-264
    ExtendedFloat160 { frac: 211971800172033630882960469392933087940, exp: -1102 },                       // 14^-256
    ExtendedFloat160 { frac: 291341601754078780972414349774563326428, exp: -1072 },                       // 14^-248
    ExtendedFloat160 { frac: 200215143815698040798730279921859793515, exp: -1041 },                       // 14^-240
    ExtendedFloat160 { frac: 275182833977670641254081992250398936156, exp: -1011 },                       // 14^-232
    ExtendedFloat160 { frac: 189110550462878905579110756278883461660, exp: -980 },                        // 14^-224
    ExtendedFloat160 { frac: 259920284847963995399800125815215983072, exp: -950 },                        // 14^-216
    ExtendedFloat160 { frac: 178621854545095883446307559621296276382, exp: -919 },                        // 14^-208
    ExtendedFloat160 { frac: 245504247117858718392171207907774065824, exp: -889 },                        // 14^-200
    ExtendedFloat160 { frac: 337429792711562885676838629607664070711, exp: -859 },                        // 14^-192
    ExtendedFloat160 { frac: 231887770468403152813044191455947332680, exp: -828 },                        // 14^-184
    ExtendedFloat160 { frac: 318714821597104302344272129046782646031, exp: -798 },                        // 14^-176
    ExtendedFloat160 { frac: 219026508600450572879143037057044702127, exp: -767 },                        // 14^-168
    ExtendedFloat160 { frac: 301037844611736789461197056572315300910, exp: -737 },                        // 14^-160
    ExtendedFloat160 { frac: 206878574807117564156377107253687360168, exp: -706 },                        // 14^-152
    ExtendedFloat160 { frac: 284341291171704802743493772571841275116, exp: -676 },                        // 14^-144
    ExtendedFloat160 { frac: 195404405556671025581418948604574579552, exp: -645 },                        // 14^-136
    ExtendedFloat160 { frac: 268570783748031302676865943988466296381, exp: -615 },                        // 14^-128
    ExtendedFloat160 { frac: 184566631641558957302915986868221103411, exp: -584 },                        // 14^-120
    ExtendedFloat160 { frac: 253674960769150428442670675281025859157, exp: -554 },                        // 14^-112
    ExtendedFloat160 { frac: 174329956473941702727882427159635204541, exp: -523 },                        // 14^-104
    ExtendedFloat160 { frac: 239605309345945263344621501916001364416, exp: -493 },                        // 14^-96
    ExtendedFloat160 { frac: 329322082262710237520775170056072850605, exp: -463 },                        // 14^-88
    ExtendedFloat160 { frac: 226316007274407653805017736090948441653, exp: -432 },                        // 14^-80
    ExtendedFloat160 { frac: 311056791556242112413125050076207081650, exp: -402 },                        // 14^-72
    ExtendedFloat160 { frac: 213763773801352511153375590062662203372, exp: -371 },                        // 14^-64
    ExtendedFloat160 { frac: 293804554217770280277607620947080882230, exp: -341 },                        // 14^-56
    ExtendedFloat160 { frac: 201907728667158642949418150287074186439, exp: -310 },                        // 14^-48
    ExtendedFloat160 { frac: 277509182960549548157083678768414391770, exp: -280 },                        // 14^-40
    ExtendedFloat160 { frac: 190709258966464876623809832149485832940, exp: -249 },                        // 14^-32
    ExtendedFloat160 { frac: 262117606830390855604604612394616145902, exp: -219 },                        // 14^-24
    ExtendedFloat160 { frac: 180131893393211845729384454981934494079, exp: -188 },                        // 14^-16
    ExtendedFloat160 { frac: 247579698363561878555441197267606485702, exp: -158 },                        // 14^-8
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 14^0
    ExtendedFloat160 { frac: 233848110331081506623283076321752645632, exp: -97 },                         // 14^8
    ExtendedFloat160 { frac: 321409182616407632938244080939940446208, exp: -67 },                         // 14^16
    ExtendedFloat160 { frac: 220878121537715117784741212850409701376, exp: -36 },                         // 14^24
    ExtendedFloat160 { frac: 303582767467198764972828371186555551744, exp: -6 },                          // 14^32
    ExtendedFloat160 { frac: 208627491173467029036775346642092064768, exp: 25 },                          // 14^40
    ExtendedFloat160 { frac: 286745064197610355009611023687762970225, exp: 55 },                          // 14^48
    ExtendedFloat160 { frac: 197056321243220373650760726805477101625, exp: 86 },                          // 14^56
    ExtendedFloat160 { frac: 270841235580262673674173137819514560921, exp: 116 },                         // 14^64
    ExtendedFloat160 { frac: 186126926626483659918254253754028720893, exp: 147 },                         // 14^72
    ExtendedFloat160 { frac: 255819485841579348845580110620374327715, exp: 177 },                         // 14^80
    ExtendedFloat160 { frac: 175803712344053086257499345217280929659, exp: 208 },                         // 14^88
    ExtendedFloat160 { frac: 241630891972710985114650364591580944199, exp: 238 },                         // 14^96
    ExtendedFloat160 { frac: 332106115263742508816700348007180790592, exp: 268 },                         // 14^104
    ExtendedFloat160 { frac: 228229244396512279339912293302076319311, exp: 299 },                         // 14^112
    ExtendedFloat160 { frac: 313686412889065357315780098042512623701, exp: 329 },                         // 14^120
    ExtendedFloat160 { frac: 215570896471654994597359469530054904893, exp: 360 },                         // 14^128
    ExtendedFloat160 { frac: 296288328063653274657691113306601058431, exp: 390 },                         // 14^136
    ExtendedFloat160 { frac: 203614622343740041422835873088956724196, exp: 421 },                         // 14^144
    ExtendedFloat160 { frac: 279855198502973302491761604313633635665, exp: 451 },                         // 14^152
    ExtendedFloat160 { frac: 192321482680456518790460799730877816261, exp: 482 },                         // 14^160
    ExtendedFloat160 { frac: 264333504599995236391965677440014878764, exp: 512 },                         // 14^168
    ExtendedFloat160 { frac: 181654697853512422889189735564216996803, exp: 543 },                         // 14^176
    ExtendedFloat160 { frac: 249672695121914450880056351845343996855, exp: 573 },                         // 14^184
    ExtendedFloat160 { frac: 171579528154314464133715598246382584383, exp: 604 },                         // 14^192
    ExtendedFloat160 { frac: 235825022574310344970824197408281461914, exp: 634 },                         // 14^200
    ExtendedFloat160 { frac: 324126321306564057111299587260117144567, exp: 664 },                         // 14^208
    ExtendedFloat160 { frac: 222745387696552489161405202381218740358, exp: 695 },                         // 14^216
    ExtendedFloat160 { frac: 306149204668634768979587047598946613225, exp: 725 },                         // 14^224
    ExtendedFloat160 { frac: 210391192582005308252369450519735221250, exp: 756 },                         // 14^232
    ExtendedFloat160 { frac: 289169158312782474283830561810879736710, exp: 786 },                         // 14^240
    ExtendedFloat160 { frac: 198722201944671478335130298919429936059, exp: 817 },                         // 14^248
    ExtendedFloat160 { frac: 273130881427012463325938578210122255044, exp: 847 },                         // 14^256
    ExtendedFloat160 { frac: 187700412080445632409314398956810353518, exp: 878 },                         // 14^264
];
const BASE14_BIAS: i32 = 320;

// BASE15

const BASE15_STEP: i32 = 8;
const BASE15_SMALL_POWERS: [ExtendedFloat160; BASE15_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 15^0
    ExtendedFloat160 { frac: 319014718988379809496913694467282698240, exp: -124 },                        // 15^1
    ExtendedFloat160 { frac: 299076299051606071403356588563077529600, exp: -120 },                        // 15^2
    ExtendedFloat160 { frac: 280384030360880691940646801777885184000, exp: -116 },                        // 15^3
    ExtendedFloat160 { frac: 262860028463325648694356376666767360000, exp: -112 },                        // 15^4
    ExtendedFloat160 { frac: 246431276684367795650959103125094400000, exp: -108 },                        // 15^5
    ExtendedFloat160 { frac: 231029321891594808422774159179776000000, exp: -104 },                        // 15^6
    ExtendedFloat160 { frac: 216589989273370132896350774231040000000, exp: -100 },                        // 15^7
];
const BASE15_SMALL_INT_POWERS: [u128; BASE15_STEP as usize] = [1, 15, 225, 3375, 50625, 759375, 11390625, 170859375];
const BASE15_LARGE_POWERS: [ExtendedFloat160; 72] = [
    ExtendedFloat160 { frac: 176157581170134694175945662700929721613, exp: -1346 },                       // 15^-312
    ExtendedFloat160 { frac: 210233318295150407445977087805804313303, exp: -1315 },                       // 15^-304
    ExtendedFloat160 { frac: 250900630150587280377850342682701393565, exp: -1284 },                       // 15^-296
    ExtendedFloat160 { frac: 299434584015762656519487081521438784545, exp: -1253 },                       // 15^-288
    ExtendedFloat160 { frac: 178678447421354466262014715708770369510, exp: -1221 },                       // 15^-280
    ExtendedFloat160 { frac: 213241818261213966824485132704511196490, exp: -1190 },                       // 15^-272
    ExtendedFloat160 { frac: 254491091184140684406056053498886823977, exp: -1159 },                       // 15^-264
    ExtendedFloat160 { frac: 303719580053283976738045342609656736770, exp: -1128 },                       // 15^-256
    ExtendedFloat160 { frac: 181235388002241613446887230826356048960, exp: -1096 },                       // 15^-248
    ExtendedFloat160 { frac: 216293370737313530448778168911616746705, exp: -1065 },                       // 15^-240
    ExtendedFloat160 { frac: 258132932759965856034693127408739651425, exp: -1034 },                       // 15^-232
    ExtendedFloat160 { frac: 308065895631104646134849028190499821539, exp: -1003 },                       // 15^-224
    ExtendedFloat160 { frac: 183828919146951883093674448351327494412, exp: -971 },                        // 15^-216
    ExtendedFloat160 { frac: 219388591817396682923135459647357436704, exp: -940 },                        // 15^-208
    ExtendedFloat160 { frac: 261826890148575264294539329193196343516, exp: -909 },                        // 15^-200
    ExtendedFloat160 { frac: 312474408249691315911417866816293319027, exp: -878 },                        // 15^-192
    ExtendedFloat160 { frac: 186459564477102023710796186076704586060, exp: -846 },                        // 15^-184
    ExtendedFloat160 { frac: 222528106411894691985532295229724475420, exp: -815 },                        // 15^-176
    ExtendedFloat160 { frac: 265573709142416387133758214804806317848, exp: -784 },                        // 15^-168
    ExtendedFloat160 { frac: 316946007966797681726272939583274683428, exp: -753 },                        // 15^-160
    ExtendedFloat160 { frac: 189127855107486501747742000280219617523, exp: -721 },                        // 15^-152
    ExtendedFloat160 { frac: 225712548373888955751123878842649157480, exp: -690 },                        // 15^-144
    ExtendedFloat160 { frac: 269374146206443663446975509775551676690, exp: -659 },                        // 15^-136
    ExtendedFloat160 { frac: 321481597577162915672598741300570427622, exp: -628 },                        // 15^-128
    ExtendedFloat160 { frac: 191834329753307055373436045708607941890, exp: -596 },                        // 15^-120
    ExtendedFloat160 { frac: 228942560627082928532238637834630336646, exp: -565 },                        // 15^-112
    ExtendedFloat160 { frac: 273228968630845173556739192069109710925, exp: -534 },                        // 15^-104
    ExtendedFloat160 { frac: 326082092794781635112624979076905003855, exp: -503 },                        // 15^-96
    ExtendedFloat160 { frac: 194579534838936734374686465136944546456, exp: -471 },                        // 15^-88
    ExtendedFloat160 { frac: 232218795295605362158733969001967701715, exp: -440 },                        // 15^-80
    ExtendedFloat160 { frac: 277138954685954882938577351637270128950, exp: -409 },                        // 15^-72
    ExtendedFloat160 { frac: 330748422437782207809506145002547658085, exp: -378 },                        // 15^-64
    ExtendedFloat160 { frac: 197364024608240385117657735223630441823, exp: -346 },                        // 15^-56
    ExtendedFloat160 { frac: 235541913835671069456466721167496749113, exp: -315 },                        // 15^-48
    ExtendedFloat160 { frac: 281104893779381725235703590002221275596, exp: -284 },                        // 15^-40
    ExtendedFloat160 { frac: 335481528615950719408183659471355674975, exp: -253 },                        // 15^-32
    ExtendedFloat160 { frac: 200188361236473853754168248068850933442, exp: -221 },                        // 15^-24
    ExtendedFloat160 { frac: 238912587169125791566529710613345919103, exp: -190 },                        // 15^-16
    ExtendedFloat160 { frac: 285127586615387248178740525206042107172, exp: -159 },                        // 15^-8
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 15^0
    ExtendedFloat160 { frac: 203053114943784499590328850841600000000, exp: -96 },                         // 15^8
    ExtendedFloat160 { frac: 242331495818902131179520000000000000000, exp: -65 },                         // 15^16
    ExtendedFloat160 { frac: 289207845356544000000000000000000000000, exp: -34 },                         // 15^24
    ExtendedFloat160 { frac: 172575953309595678001642227172851562500, exp: -2 },                          // 15^32
    ExtendedFloat160 { frac: 205958864110335933831952325359648803271, exp: 29 },                          // 15^40
    ExtendedFloat160 { frac: 245799330046413899594233809090775284541, exp: 60 },                          // 15^48
    ExtendedFloat160 { frac: 293346493787707294402529081970820087350, exp: 91 },                          // 15^56
    ExtendedFloat160 { frac: 175045565423820113080636775231191731391, exp: 123 },                         // 15^64
    ExtendedFloat160 { frac: 208906195393080226844550976655564036201, exp: 154 },                         // 15^72
    ExtendedFloat160 { frac: 249316789990916616866725045444974174010, exp: 185 },                         // 15^80
    ExtendedFloat160 { frac: 297544367482333459204270734183314844936, exp: 216 },                         // 15^88
    ExtendedFloat160 { frac: 177550518406095745907734479894927853604, exp: 248 },                         // 15^96
    ExtendedFloat160 { frac: 211895703844201159681805788312089434384, exp: 279 },                         // 15^104
    ExtendedFloat160 { frac: 252884585810862301272632571838697691236, exp: 310 },                         // 15^112
    ExtendedFloat160 { frac: 301802313971178147521594347169174498450, exp: 341 },                         // 15^120
    ExtendedFloat160 { frac: 180091317994529147280371081412973024665, exp: 373 },                         // 15^128
    ExtendedFloat160 { frac: 214927993031252433012289596290523334542, exp: 404 },                         // 15^136
    ExtendedFloat160 { frac: 256503437827277086943219190518379691262, exp: 435 },                         // 15^144
    ExtendedFloat160 { frac: 306121192913408770879770801728425641044, exp: 466 },                         // 15^152
    ExtendedFloat160 { frac: 182668477164486370906358954938714362551, exp: 498 },                         // 15^160
    ExtendedFloat160 { frac: 218003675159015088778073023304915283497, exp: 529 },                         // 15^168
    ExtendedFloat160 { frac: 260174076669190616934963528529542450091, exp: 560 },                         // 15^176
    ExtendedFloat160 { frac: 310501876270165601037714752361776484586, exp: 591 },                         // 15^184
    ExtendedFloat160 { frac: 185282516232160242762660936045505469826, exp: 623 },                         // 15^192
    ExtendedFloat160 { frac: 221123371193098747019724833334214306438, exp: 654 },                         // 15^200
    ExtendedFloat160 { frac: 263897243421146573236258451562349376134, exp: 685 },                         // 15^208
    ExtendedFloat160 { frac: 314945248480606581688897076801472259251, exp: 716 },                         // 15^216
    ExtendedFloat160 { frac: 187933962959619728504626775329093970226, exp: 748 },                         // 15^224
    ExtendedFloat160 { frac: 224287710985311612370215929865332425589, exp: 779 },                         // 15^232
    ExtendedFloat160 { frac: 267673689772824125386430153566188172461, exp: 810 },                         // 15^240
    ExtendedFloat160 { frac: 319452206640471392649554807130291119305, exp: 841 },                         // 15^248
    ExtendedFloat160 { frac: 190623352661362587893902772482928415571, exp: 873 },                         // 15^256
];
const BASE15_BIAS: i32 = 312;

// BASE17

const BASE17_STEP: i32 = 8;
const BASE17_SMALL_POWERS: [ExtendedFloat160; BASE17_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 17^0
    ExtendedFloat160 { frac: 180775007426748558714917760198126862336, exp: -123 },                        // 17^1
    ExtendedFloat160 { frac: 192073445390920343634600120210509791232, exp: -119 },                        // 17^2
    ExtendedFloat160 { frac: 204078035727852865111762627723666653184, exp: -115 },                        // 17^3
    ExtendedFloat160 { frac: 216832912960843669181247791956395819008, exp: -111 },                        // 17^4
    ExtendedFloat160 { frac: 230384970020896398505075778953670557696, exp: -107 },                        // 17^5
    ExtendedFloat160 { frac: 244784030647202423411643015138274967552, exp: -103 },                        // 17^6
    ExtendedFloat160 { frac: 260083032562652574874870703584417153024, exp: -99 },                         // 17^7
];
const BASE17_SMALL_INT_POWERS: [u128; BASE17_STEP as usize] = [1, 17, 289, 4913, 83521, 1419857, 24137569, 410338673];
const BASE17_LARGE_POWERS: [ExtendedFloat160; 69] = [
    ExtendedFloat160 { frac: 183748361348386970108177566939545116168, exp: -1337 },                       // 17^-296
    ExtendedFloat160 { frac: 298438593500193022210049498250663150397, exp: -1305 },                       // 17^-288
    ExtendedFloat160 { frac: 242357519372662728375060373498171096966, exp: -1272 },                       // 17^-280
    ExtendedFloat160 { frac: 196814917627041423628769027913700117445, exp: -1239 },                       // 17^-272
    ExtendedFloat160 { frac: 319660903452112403161269325795427500295, exp: -1207 },                       // 17^-264
    ExtendedFloat160 { frac: 259591839957616255097162958982226744872, exp: -1174 },                       // 17^-256
    ExtendedFloat160 { frac: 210810651677570156300974624360660789801, exp: -1141 },                       // 17^-248
    ExtendedFloat160 { frac: 171196178077006380843936555487576333355, exp: -1108 },                       // 17^-240
    ExtendedFloat160 { frac: 278051712804343163076920967240438328697, exp: -1076 },                       // 17^-232
    ExtendedFloat160 { frac: 225801638394791143591435147984466155468, exp: -1043 },                       // 17^-224
    ExtendedFloat160 { frac: 183370134237042583472660416437030762483, exp: -1010 },                       // 17^-216
    ExtendedFloat160 { frac: 297824288336843871393537735572731133862, exp: -978 },                        // 17^-208
    ExtendedFloat160 { frac: 241858651334916724628852435647948831754, exp: -945 },                        // 17^-200
    ExtendedFloat160 { frac: 196409794352921881357331916371145369426, exp: -912 },                        // 17^-192
    ExtendedFloat160 { frac: 319002914345514633220184430371226516661, exp: -880 },                        // 17^-184
    ExtendedFloat160 { frac: 259057496842743054458415989246502258570, exp: -847 },                        // 17^-176
    ExtendedFloat160 { frac: 210376719623757394611528557965027181599, exp: -814 },                        // 17^-168
    ExtendedFloat160 { frac: 170843788344482468189688357828247614648, exp: -781 },                        // 17^-160
    ExtendedFloat160 { frac: 277479371939006412580670873073737346072, exp: -749 },                        // 17^-152
    ExtendedFloat160 { frac: 225336848935989032116058614991615739830, exp: -716 },                        // 17^-144
    ExtendedFloat160 { frac: 182992685667322765043701960137104341962, exp: -683 },                        // 17^-136
    ExtendedFloat160 { frac: 297211247657519026846228781764556419486, exp: -651 },                        // 17^-128
    ExtendedFloat160 { frac: 241360810165739638162969982622013142483, exp: -618 },                        // 17^-120
    ExtendedFloat160 { frac: 196005504983412884662857445298594989473, exp: -585 },                        // 17^-112
    ExtendedFloat160 { frac: 318346279641847361755138868550752397781, exp: -553 },                        // 17^-104
    ExtendedFloat160 { frac: 258524253618237975606418771469177669493, exp: -520 },                        // 17^-96
    ExtendedFloat160 { frac: 209943680774466452613153056292584472116, exp: -487 },                        // 17^-88
    ExtendedFloat160 { frac: 170492123969995067410761994182235561833, exp: -454 },                        // 17^-80
    ExtendedFloat160 { frac: 276908209178500704519431941303594138118, exp: -422 },                        // 17^-72
    ExtendedFloat160 { frac: 224873016198504574919954139464723650485, exp: -389 },                        // 17^-64
    ExtendedFloat160 { frac: 182616014036679619640392724861134514431, exp: -356 },                        // 17^-56
    ExtendedFloat160 { frac: 296599468859408121472112670217716316867, exp: -324 },                        // 17^-48
    ExtendedFloat160 { frac: 240863993751428088431121063565557553364, exp: -291 },                        // 17^-40
    ExtendedFloat160 { frac: 195602047802007523394399179976814437447, exp: -258 },                        // 17^-32
    ExtendedFloat160 { frac: 317690996553211397504541477525504659745, exp: -226 },                        // 17^-24
    ExtendedFloat160 { frac: 257992108020089771434235624403421929746, exp: -193 },                        // 17^-16
    ExtendedFloat160 { frac: 209511533291127649202286079418781645629, exp: -160 },                        // 17^-8
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 17^0
    ExtendedFloat160 { frac: 276338222097818360804550122558443225088, exp: -95 },                         // 17^8
    ExtendedFloat160 { frac: 224410138213025296601555690180338253824, exp: -62 },                         // 17^16
    ExtendedFloat160 { frac: 182240117745863932172015090234506084352, exp: -29 },                         // 17^24
    ExtendedFloat160 { frac: 295988949345058405730513287165118905920, exp: 3 },                           // 17^32
    ExtendedFloat160 { frac: 240368199982629535993820184805831872348, exp: 36 },                          // 17^40
    ExtendedFloat160 { frac: 195199421095732140407812372336079928061, exp: 69 },                          // 17^48
    ExtendedFloat160 { frac: 317037062297446153078380490386689107619, exp: 101 },                         // 17^56
    ExtendedFloat160 { frac: 257461057788947429232455063849242347039, exp: 134 },                         // 17^64
    ExtendedFloat160 { frac: 209080275338955809947349984471742343690, exp: 167 },                         // 17^72
    ExtendedFloat160 { frac: 339581930651806711314806824645491109283, exp: 199 },                         // 17^80
    ExtendedFloat160 { frac: 275769408276943332346622203534744539377, exp: 232 },                         // 17^88
    ExtendedFloat160 { frac: 223948213014292349032159641795245781843, exp: 265 },                         // 17^96
    ExtendedFloat160 { frac: 181864995198918377644202761532864575404, exp: 298 },                         // 17^104
    ExtendedFloat160 { frac: 295379686522363719531147616048257012833, exp: 330 },                         // 17^112
    ExtendedFloat160 { frac: 239873426754333326759263874720553852658, exp: 363 },                         // 17^120
    ExtendedFloat160 { frac: 194797623155139058727680168785666503632, exp: 396 },                         // 17^128
    ExtendedFloat160 { frac: 316384474098117832632118586933880356805, exp: 428 },                         // 17^136
    ExtendedFloat160 { frac: 256931100670110578075784008065054807112, exp: 461 },                         // 17^144
    ExtendedFloat160 { frac: 208649905086942477070838417411852870460, exp: 494 },                         // 17^152
    ExtendedFloat160 { frac: 338882936158725697632383782036927673666, exp: 526 },                         // 17^160
    ExtendedFloat160 { frac: 275201765300840924300371814765015192837, exp: 559 },                         // 17^168
    ExtendedFloat160 { frac: 223487238641092167380922055560365365575, exp: 592 },                         // 17^176
    ExtendedFloat160 { frac: 181490644803170745141604509031893001355, exp: 625 },                         // 17^184
    ExtendedFloat160 { frac: 294771677804553486829405243638525556429, exp: 657 },                         // 17^192
    ExtendedFloat160 { frac: 239379671965861754658715697694088165244, exp: 690 },                         // 17^200
    ExtendedFloat160 { frac: 194396652274299323679629299288271234884, exp: 723 },                         // 17^208
    ExtendedFloat160 { frac: 315733229184507643855046849844924656388, exp: 755 },                         // 17^216
    ExtendedFloat160 { frac: 256402234413519915955050497805856036474, exp: 788 },                         // 17^224
    ExtendedFloat160 { frac: 208220420707848135466354215936729966175, exp: 821 },                         // 17^232
    ExtendedFloat160 { frac: 338185380473947645648707735110624890484, exp: 853 },                         // 17^240
    ExtendedFloat160 { frac: 274635290759447542459052551448368297186, exp: 886 },                         // 17^248
];
const BASE17_BIAS: i32 = 296;

// BASE18

const BASE18_STEP: i32 = 7;
const BASE18_SMALL_POWERS: [ExtendedFloat160; BASE18_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 18^0
    ExtendedFloat160 { frac: 191408831393027885698148216680369618944, exp: -123 },                        // 18^1
    ExtendedFloat160 { frac: 215334935317156371410416743765415821312, exp: -119 },                        // 18^2
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -115 },                        // 18^3
    ExtendedFloat160 { frac: 272533277510776032566308691328104398848, exp: -111 },                        // 18^4
    ExtendedFloat160 { frac: 306599937199623036637097277744117448704, exp: -107 },                        // 18^5
    ExtendedFloat160 { frac: 172462464674787958108367218731066064896, exp: -102 },                        // 18^6
];
const BASE18_SMALL_INT_POWERS: [u128; BASE18_STEP as usize] = [1, 18, 324, 5832, 104976, 1889568, 34012224];
const BASE18_LARGE_POWERS: [ExtendedFloat160; 78] = [
    ExtendedFloat160 { frac: 175173187664825964142525661203029234593, exp: -1353 },                       // 18^-294
    ExtendedFloat160 { frac: 199758512075434917661860422906539806164, exp: -1324 },                       // 18^-287
    ExtendedFloat160 { frac: 227794354139073103116567345878808448350, exp: -1295 },                       // 18^-280
    ExtendedFloat160 { frac: 259764989429046712145887613713888779974, exp: -1266 },                       // 18^-273
    ExtendedFloat160 { frac: 296222660952677279411722167462707735076, exp: -1237 },                       // 18^-266
    ExtendedFloat160 { frac: 337797118290463899238253918549284876092, exp: -1208 },                       // 18^-259
    ExtendedFloat160 { frac: 192603247770383575639211190527648274245, exp: -1178 },                       // 18^-252
    ExtendedFloat160 { frac: 219634857984796466920734002291401705412, exp: -1149 },                       // 18^-245
    ExtendedFloat160 { frac: 250460318818255417964791343261171179660, exp: -1120 },                       // 18^-238
    ExtendedFloat160 { frac: 285612092170198511649999639102587856294, exp: -1091 },                       // 18^-231
    ExtendedFloat160 { frac: 325697370261002112643262654266086944683, exp: -1062 },                       // 18^-224
    ExtendedFloat160 { frac: 185704281966673733437923590446998072591, exp: -1032 },                       // 18^-217
    ExtendedFloat160 { frac: 211767631486382365261996259087726574961, exp: -1003 },                       // 18^-210
    ExtendedFloat160 { frac: 241488937521646207617474790890828249100, exp: -974 },                        // 18^-203
    ExtendedFloat160 { frac: 275381589414827976227270551469877695982, exp: -945 },                        // 18^-196
    ExtendedFloat160 { frac: 314031030021154964119856834958393443507, exp: -916 },                        // 18^-189
    ExtendedFloat160 { frac: 179052434161812488744424286609186187994, exp: -886 },                        // 18^-182
    ExtendedFloat160 { frac: 204182205669996766442639895844185984708, exp: -857 },                        // 18^-175
    ExtendedFloat160 { frac: 232838907258801165579649662968151663564, exp: -828 },                        // 18^-168
    ExtendedFloat160 { frac: 265517538884334791214783518037108753711, exp: -799 },                        // 18^-161
    ExtendedFloat160 { frac: 302782573089615796089597203369720463425, exp: -770 },                        // 18^-154
    ExtendedFloat160 { frac: 172638852694972345186098175788410725025, exp: -740 },                        // 18^-147
    ExtendedFloat160 { frac: 196868486555962367745019627988939060464, exp: -711 },                        // 18^-140
    ExtendedFloat160 { frac: 224498717373391335032231518045098273537, exp: -682 },                        // 18^-133
    ExtendedFloat160 { frac: 256006814417050404626793229969178591795, exp: -653 },                        // 18^-126
    ExtendedFloat160 { frac: 291937031065346039954998156631577529912, exp: -624 },                        // 18^-119
    ExtendedFloat160 { frac: 332910005936047335476283449703688561122, exp: -595 },                        // 18^-112
    ExtendedFloat160 { frac: 189816741726628588213403698819375225722, exp: -565 },                        // 18^-105
    ExtendedFloat160 { frac: 216457269515865090355509585365869594574, exp: -536 },                        // 18^-98
    ExtendedFloat160 { frac: 246836760024792608106756526472045881483, exp: -507 },                        // 18^-91
    ExtendedFloat160 { frac: 281479971709018296242657937208050445965, exp: -478 },                        // 18^-84
    ExtendedFloat160 { frac: 320985312176969416466104839150917967197, exp: -449 },                        // 18^-77
    ExtendedFloat160 { frac: 183017587375374702561553597022155160742, exp: -419 },                        // 18^-70
    ExtendedFloat160 { frac: 208703862874796048578293668364396201854, exp: -390 },                        // 18^-63
    ExtendedFloat160 { frac: 237995173051452727716558620615765508935, exp: -361 },                        // 18^-56
    ExtendedFloat160 { frac: 271397479737933588417468230506889186025, exp: -332 },                        // 18^-49
    ExtendedFloat160 { frac: 309487755838552588810803796052767101096, exp: -303 },                        // 18^-42
    ExtendedFloat160 { frac: 176461975819512133258798291874254633040, exp: -273 },                        // 18^-35
    ExtendedFloat160 { frac: 201228179937237770199942876645925476060, exp: -244 },                        // 18^-28
    ExtendedFloat160 { frac: 229470287934835004924643178169318563534, exp: -215 },                        // 18^-21
    ExtendedFloat160 { frac: 261676138308856451194147028266269550444, exp: -186 },                        // 18^-14
    ExtendedFloat160 { frac: 298402037041419227483658365640566588740, exp: -157 },                        // 18^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 18^0
    ExtendedFloat160 { frac: 194020272759136452871913121072449323008, exp: -98 },                         // 18^7
    ExtendedFloat160 { frac: 221250760550139932836609227367108313088, exp: -69 },                         // 18^14
    ExtendedFloat160 { frac: 252303011164126931290527343657214803968, exp: -40 },                         // 18^21
    ExtendedFloat160 { frac: 287713403941314941508226937857819803648, exp: -11 },                         // 18^28
    ExtendedFloat160 { frac: 328093598350474163167634651360244400128, exp: 18 },                          // 18^35
    ExtendedFloat160 { frac: 187070549727531559196917812917453861026, exp: 48 },                          // 18^42
    ExtendedFloat160 { frac: 213325653114257310428744028742785781912, exp: 77 },                          // 18^49
    ExtendedFloat160 { frac: 243265625417291205836310243227923719532, exp: 106 },                         // 18^56
    ExtendedFloat160 { frac: 277407633098725295421526662764935275289, exp: 135 },                         // 18^63
    ExtendedFloat160 { frac: 316341426247257477645159711999449660471, exp: 164 },                         // 18^70
    ExtendedFloat160 { frac: 180369762796928745579122531717097251784, exp: 194 },                         // 18^77
    ExtendedFloat160 { frac: 205684419630781050995309380627725821797, exp: 223 },                         // 18^84
    ExtendedFloat160 { frac: 234551954955343535589691141355422293223, exp: 252 },                         // 18^91
    ExtendedFloat160 { frac: 267471010551644448060009348077202513313, exp: 281 },                         // 18^98
    ExtendedFloat160 { frac: 305010211912915299914630616083972269987, exp: 310 },                         // 18^105
    ExtendedFloat160 { frac: 173908995182860443486855135343139262701, exp: 340 },                         // 18^112
    ExtendedFloat160 { frac: 198316891856377323275537495574245323224, exp: 369 },                         // 18^119
    ExtendedFloat160 { frac: 226150404435492799169987273137391228527, exp: 398 },                         // 18^126
    ExtendedFloat160 { frac: 257890313566309108293837274983090159159, exp: 427 },                         // 18^133
    ExtendedFloat160 { frac: 294084876820548989626661915132664622178, exp: 456 },                         // 18^140
    ExtendedFloat160 { frac: 335359298992515654651080492391625827617, exp: 485 },                         // 18^147
    ExtendedFloat160 { frac: 191213265769831372286179520084128641258, exp: 515 },                         // 18^154
    ExtendedFloat160 { frac: 218049793855157992570938056229892893589, exp: 544 },                         // 18^161
    ExtendedFloat160 { frac: 248652793041613380567795520750960012282, exp: 573 },                         // 18^168
    ExtendedFloat160 { frac: 283550882549632193861238568804430594202, exp: 602 },                         // 18^175
    ExtendedFloat160 { frac: 323346872605688987884591669129361771279, exp: 631 },                         // 18^182
    ExtendedFloat160 { frac: 184364088525767284952804747951506893851, exp: 661 },                         // 18^189
    ExtendedFloat160 { frac: 210239343674659878049714940191476577896, exp: 690 },                         // 18^196
    ExtendedFloat160 { frac: 239746156543789930881397013133212694240, exp: 719 },                         // 18^203
    ExtendedFloat160 { frac: 273394211439632029990640781047045990695, exp: 748 },                         // 18^210
    ExtendedFloat160 { frac: 311764726184655516350818907233192566650, exp: 777 },                         // 18^217
    ExtendedFloat160 { frac: 177760245875679923341865534533290364422, exp: 807 },                         // 18^224
    ExtendedFloat160 { frac: 202708660472811443153981984577454872377, exp: 836 },                         // 18^231
    ExtendedFloat160 { frac: 231158551948781603682221903513060749406, exp: 865 },                         // 18^238
    ExtendedFloat160 { frac: 263601347936609267755115798860540547258, exp: 894 },                         // 18^245
];
const BASE18_BIAS: i32 = 294;

// BASE19

const BASE19_STEP: i32 = 7;
const BASE19_SMALL_POWERS: [ExtendedFloat160; BASE19_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 19^0
    ExtendedFloat160 { frac: 202042655359307212681378673162612375552, exp: -123 },                        // 19^1
    ExtendedFloat160 { frac: 239925653239177315059137174380602195968, exp: -119 },                        // 19^2
    ExtendedFloat160 { frac: 284911713221523061632725394576965107712, exp: -115 },                        // 19^3
    ExtendedFloat160 { frac: 338332659450558635688861406060146065408, exp: -111 },                        // 19^4
    ExtendedFloat160 { frac: 200885016548769189940261459848211726336, exp: -106 },                        // 19^5
    ExtendedFloat160 { frac: 238550957151663413054060483569751425024, exp: -102 },                        // 19^6
];
const BASE19_SMALL_INT_POWERS: [u128; BASE19_STEP as usize] = [1, 19, 361, 6859, 130321, 2476099, 47045881];
const BASE19_LARGE_POWERS: [ExtendedFloat160; 76] = [
    ExtendedFloat160 { frac: 305577016199347453916386629066628369566, exp: -1347 },                       // 19^-287
    ExtendedFloat160 { frac: 254387649585066250802448835894591219323, exp: -1317 },                       // 19^-280
    ExtendedFloat160 { frac: 211773375714873706954526795446522416463, exp: -1287 },                       // 19^-273
    ExtendedFloat160 { frac: 176297720171655089222657736919708805072, exp: -1257 },                       // 19^-266
    ExtendedFloat160 { frac: 293529685049453215881543661471874663538, exp: -1228 },                       // 19^-259
    ExtendedFloat160 { frac: 244358452058655253685245151337508023797, exp: -1198 },                       // 19^-252
    ExtendedFloat160 { frac: 203424239979142604035433607238695466107, exp: -1168 },                       // 19^-245
    ExtendedFloat160 { frac: 338694414393807804581041366088196084612, exp: -1139 },                       // 19^-238
    ExtendedFloat160 { frac: 281957318246159342222500231430112814088, exp: -1109 },                       // 19^-231
    ExtendedFloat160 { frac: 234724654242834897063227749239967575385, exp: -1079 },                       // 19^-224
    ExtendedFloat160 { frac: 195404267752744776874588226122954622761, exp: -1049 },                       // 19^-217
    ExtendedFloat160 { frac: 325341434449269614607198466720898964379, exp: -1020 },                       // 19^-210
    ExtendedFloat160 { frac: 270841190386491955899443752473167688523, exp: -990 },                        // 19^-203
    ExtendedFloat160 { frac: 225470667559284387835624717321131334248, exp: -960 },                        // 19^-196
    ExtendedFloat160 { frac: 187700481810335462307725392489901825033, exp: -930 },                        // 19^-189
    ExtendedFloat160 { frac: 312514893872556142260088491662230499613, exp: -901 },                        // 19^-182
    ExtendedFloat160 { frac: 260163314313871975296322899252180920269, exp: -871 },                        // 19^-175
    ExtendedFloat160 { frac: 216581518007204247993872678149555585108, exp: -841 },                        // 19^-168
    ExtendedFloat160 { frac: 180300416551865148993646942230063074210, exp: -811 },                        // 19^-161
    ExtendedFloat160 { frac: 300194037865176954165930472692882676095, exp: -782 },                        // 19^-154
    ExtendedFloat160 { frac: 249906412012854191998973084780461202007, exp: -752 },                        // 19^-147
    ExtendedFloat160 { frac: 208042821933683442841807340743863291908, exp: -722 },                        // 19^-140
    ExtendedFloat160 { frac: 173192097831823827445985218112992713522, exp: -692 },                        // 19^-133
    ExtendedFloat160 { frac: 288358929883670982310136287441393538062, exp: -663 },                        // 19^-126
    ExtendedFloat160 { frac: 240053886651337192078493312543483732688, exp: -633 },                        // 19^-119
    ExtendedFloat160 { frac: 199840762759316398351061941853830886312, exp: -603 },                        // 19^-112
    ExtendedFloat160 { frac: 332728047167428932050765437002481330484, exp: -574 },                        // 19^-105
    ExtendedFloat160 { frac: 276990419380016367557580152018464109110, exp: -544 },                        // 19^-98
    ExtendedFloat160 { frac: 230589795725005243268941857960660823918, exp: -514 },                        // 19^-91
    ExtendedFloat160 { frac: 191962068621409314298003076822671000161, exp: -484 },                        // 19^-84
    ExtendedFloat160 { frac: 319610290416807823632571053703933063036, exp: -455 },                        // 19^-77
    ExtendedFloat160 { frac: 266070110813870135824430347248350305665, exp: -425 },                        // 19^-70
    ExtendedFloat160 { frac: 221498825260546806854707890357129976717, exp: -395 },                        // 19^-63
    ExtendedFloat160 { frac: 184393990898599943457278294759382590034, exp: -365 },                        // 19^-56
    ExtendedFloat160 { frac: 307009699392470911375170613720126268120, exp: -336 },                        // 19^-49
    ExtendedFloat160 { frac: 255580333886495993526502175170654921405, exp: -306 },                        // 19^-42
    ExtendedFloat160 { frac: 212766265035907553834903279232863277537, exp: -276 },                        // 19^-35
    ExtendedFloat160 { frac: 177124283582141234157272719875613305569, exp: -246 },                        // 19^-28
    ExtendedFloat160 { frac: 294905884907949220984741236292709278206, exp: -217 },                        // 19^-21
    ExtendedFloat160 { frac: 245504114948215358970059027289947453449, exp: -187 },                        // 19^-14
    ExtendedFloat160 { frac: 204377984777481442195434041001194890061, exp: -157 },                        // 19^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 19^0
    ExtendedFloat160 { frac: 283279261617600303001696824239079817216, exp: -98 },                         // 19^7
    ExtendedFloat160 { frac: 235825149533115640143913831779140632576, exp: -68 },                         // 19^14
    ExtendedFloat160 { frac: 196320411295724208786653789764804673536, exp: -38 },                         // 19^21
    ExtendedFloat160 { frac: 326866782169983241283381922259938632192, exp: -9 },                          // 19^28
    ExtendedFloat160 { frac: 272111016325296008481488740678554545334, exp: 21 },                          // 19^35
    ExtendedFloat160 { frac: 226527775976573799542251126784141726274, exp: 51 },                          // 19^42
    ExtendedFloat160 { frac: 188580506522192103284103254483314456872, exp: 81 },                          // 19^49
    ExtendedFloat160 { frac: 313980104972594785419533005246072709649, exp: 110 },                         // 19^56
    ExtendedFloat160 { frac: 261383077542535819285819131850454151532, exp: 140 },                         // 19^63
    ExtendedFloat160 { frac: 217596950072905364683647533344696965413, exp: 170 },                         // 19^70
    ExtendedFloat160 { frac: 181145746412467300065049161663171614429, exp: 200 },                         // 19^77
    ExtendedFloat160 { frac: 301601483222404786925353682146130769038, exp: 229 },                         // 19^84
    ExtendedFloat160 { frac: 251078086246727305139312855014591629283, exp: 259 },                         // 19^91
    ExtendedFloat160 { frac: 209018220732132084019166621358085321302, exp: 289 },                         // 19^98
    ExtendedFloat160 { frac: 174004100680832526758226172220060851297, exp: 319 },                         // 19^105
    ExtendedFloat160 { frac: 289710886904423785696066284771374024143, exp: 348 },                         // 19^112
    ExtendedFloat160 { frac: 241179367792317286239104434314982328918, exp: 378 },                         // 19^119
    ExtendedFloat160 { frac: 200777706596478115292553836948931709029, exp: 408 },                         // 19^126
    ExtendedFloat160 { frac: 334288026667806625723834320874258952107, exp: 437 },                         // 19^133
    ExtendedFloat160 { frac: 278289075684203471850125042859239363909, exp: 467 },                         // 19^140
    ExtendedFloat160 { frac: 231670904929322723497179168675130067798, exp: 497 },                         // 19^147
    ExtendedFloat160 { frac: 192862073578757583112120353708241916963, exp: 527 },                         // 19^154
    ExtendedFloat160 { frac: 321108767943438131876044073228689471164, exp: 556 },                         // 19^161
    ExtendedFloat160 { frac: 267317567774791732969648063348207624890, exp: 586 },                         // 19^168
    ExtendedFloat160 { frac: 222537311885602256946640972609673172416, exp: 616 },                         // 19^175
    ExtendedFloat160 { frac: 185258513286308067365753274779077551187, exp: 646 },                         // 19^182
    ExtendedFloat160 { frac: 308449099652072036761706748552096805527, exp: 675 },                         // 19^189
    ExtendedFloat160 { frac: 256778610031103646805005837172547824357, exp: 705 },                         // 19^196
    ExtendedFloat160 { frac: 213763809470930565948441961387708938512, exp: 735 },                         // 19^203
    ExtendedFloat160 { frac: 177954722295557497937779873950595261616, exp: 765 },                         // 19^210
    ExtendedFloat160 { frac: 296288537013515928119667265056482304979, exp: 794 },                         // 19^217
    ExtendedFloat160 { frac: 246655149223317717362685108852956230915, exp: 824 },                         // 19^224
    ExtendedFloat160 { frac: 205336201162591117777390249036062770913, exp: 854 },                         // 19^231
    ExtendedFloat160 { frac: 170938882243688352586356584716130516258, exp: 884 },                         // 19^238
];
const BASE19_BIAS: i32 = 287;

// BASE20

const BASE20_STEP: i32 = 7;
const BASE20_SMALL_POWERS: [ExtendedFloat160; BASE20_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 20^0
    ExtendedFloat160 { frac: 212676479325586539664609129644855132160, exp: -123 },                        // 20^1
    ExtendedFloat160 { frac: 265845599156983174580761412056068915200, exp: -119 },                        // 20^2
    ExtendedFloat160 { frac: 332306998946228968225951765070086144000, exp: -115 },                        // 20^3
    ExtendedFloat160 { frac: 207691874341393105141219853168803840000, exp: -110 },                        // 20^4
    ExtendedFloat160 { frac: 259614842926741381426524816461004800000, exp: -106 },                        // 20^5
    ExtendedFloat160 { frac: 324518553658426726783156020576256000000, exp: -102 },                        // 20^6
];
const BASE20_SMALL_INT_POWERS: [u128; BASE20_STEP as usize] = [1, 20, 400, 8000, 160000, 3200000, 64000000];
const BASE20_LARGE_POWERS: [ExtendedFloat160; 74] = [
    ExtendedFloat160 { frac: 308841328899094571460716776609676066664, exp: -1338 },                       // 20^-280
    ExtendedFloat160 { frac: 184083777009901148951480851536796132722, exp: -1307 },                       // 20^-273
    ExtendedFloat160 { frac: 219444962751747547330237450047488370802, exp: -1277 },                       // 20^-266
    ExtendedFloat160 { frac: 261598781051334795153424084243164504531, exp: -1247 },                       // 20^-259
    ExtendedFloat160 { frac: 311850048364799970571308236412006025948, exp: -1217 },                       // 20^-252
    ExtendedFloat160 { frac: 185877113559722882849757812268737570016, exp: -1186 },                       // 20^-245
    ExtendedFloat160 { frac: 221582786512044528543660416923448526878, exp: -1156 },                       // 20^-238
    ExtendedFloat160 { frac: 264147265567832623176169892458258303259, exp: -1126 },                       // 20^-231
    ExtendedFloat160 { frac: 314888078651228693933689466069052580904, exp: -1096 },                       // 20^-224
    ExtendedFloat160 { frac: 187687920720117505749278942387731421532, exp: -1065 },                       // 20^-217
    ExtendedFloat160 { frac: 223741436863085634409521749481834675708, exp: -1035 },                       // 20^-210
    ExtendedFloat160 { frac: 266720577315194170963194071628850311885, exp: -1005 },                       // 20^-203
    ExtendedFloat160 { frac: 317955705303185189918510999237120523316, exp: -975 },                        // 20^-196
    ExtendedFloat160 { frac: 189516368689051383685178160212707831452, exp: -944 },                        // 20^-189
    ExtendedFloat160 { frac: 225921116696657399755928707376370229068, exp: -914 },                        // 20^-182
    ExtendedFloat160 { frac: 269318958159276723570738682003462587676, exp: -884 },                        // 20^-175
    ExtendedFloat160 { frac: 321053216647239593947814323906257853121, exp: -854 },                        // 20^-168
    ExtendedFloat160 { frac: 191362629322552438943275406304751547051, exp: -823 },                        // 20^-161
    ExtendedFloat160 { frac: 228122030881109760932058580285014566244, exp: -793 },                        // 20^-154
    ExtendedFloat160 { frac: 271942652322184754529069161754863937192, exp: -763 },                        // 20^-147
    ExtendedFloat160 { frac: 324180903818827574883781864350871964922, exp: -733 },                        // 20^-140
    ExtendedFloat160 { frac: 193226876150862917234767594546599367214, exp: -702 },                        // 20^-133
    ExtendedFloat160 { frac: 230344386280611654799899571593522271174, exp: -672 },                        // 20^-126
    ExtendedFloat160 { frac: 274591906405224388599276031963255728690, exp: -642 },                        // 20^-119
    ExtendedFloat160 { frac: 327339060789614187001318969682759915221, exp: -612 },                        // 20^-112
    ExtendedFloat160 { frac: 195109284394749514461349826862072894109, exp: -581 },                        // 20^-105
    ExtendedFloat160 { frac: 232588391774594204975783618524161450993, exp: -551 },                        // 20^-98
    ExtendedFloat160 { frac: 277266969412081485957841418414308370343, exp: -521 },                        // 20^-91
    ExtendedFloat160 { frac: 330527984395124299475957654016385519914, exp: -491 },                        // 20^-84
    ExtendedFloat160 { frac: 197010030981972396061395200500718069025, exp: -460 },                        // 20^-77
    ExtendedFloat160 { frac: 234854258277383322788948059678933702737, exp: -430 },                        // 20^-70
    ExtendedFloat160 { frac: 279968092772225526319680285071055534765, exp: -400 },                        // 20^-63
    ExtendedFloat160 { frac: 333747974362642200374222141588992517906, exp: -370 },                        // 20^-56
    ExtendedFloat160 { frac: 198929294563914656862152899258728336040, exp: -339 },                        // 20^-49
    ExtendedFloat160 { frac: 237142198758023568227473377297792835283, exp: -309 },                        // 20^-42
    ExtendedFloat160 { frac: 282695530364541492733327600118866962532, exp: -279 },                        // 20^-35
    ExtendedFloat160 { frac: 336999333339382997433337688587745383420, exp: -249 },                        // 20^-28
    ExtendedFloat160 { frac: 200867255532373784442745261542645325315, exp: -218 },                        // 20^-21
    ExtendedFloat160 { frac: 239452428260295134118491722992235809940, exp: -188 },                        // 20^-14
    ExtendedFloat160 { frac: 285449538541191976211657193889899027276, exp: -158 },                        // 20^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 20^0
    ExtendedFloat160 { frac: 202824096036516704239472512860160000000, exp: -97 },                         // 20^7
    ExtendedFloat160 { frac: 241785163922925834941235200000000000000, exp: -67 },                         // 20^14
    ExtendedFloat160 { frac: 288230376151711744000000000000000000000, exp: -37 },                         // 20^21
    ExtendedFloat160 { frac: 171798691840000000000000000000000000000, exp: -6 },                          // 20^28
    ExtendedFloat160 { frac: 204800000000000000000000000000000000000, exp: 24 },                          // 20^35
    ExtendedFloat160 { frac: 244140625000000000000000000000000000000, exp: 54 },                          // 20^42
    ExtendedFloat160 { frac: 291038304567337036132812500000000000000, exp: 84 },                          // 20^49
    ExtendedFloat160 { frac: 173472347597680709441192448139190673828, exp: 115 },                         // 20^56
    ExtendedFloat160 { frac: 206795153138256918717852173017490713391, exp: 145 },                         // 20^63
    ExtendedFloat160 { frac: 246519032881566189191165176650870696772, exp: 175 },                         // 20^70
    ExtendedFloat160 { frac: 293873587705571876992184134305561419454, exp: 205 },                         // 20^77
    ExtendedFloat160 { frac: 175162308040602133865466197911239516410, exp: 236 },                         // 20^84
    ExtendedFloat160 { frac: 208809742975952784854729411496209521782, exp: 266 },                         // 20^91
    ExtendedFloat160 { frac: 248920611114445668285762562151204969623, exp: 296 },                         // 20^98
    ExtendedFloat160 { frac: 296736492054993710858538820923811161069, exp: 326 },                         // 20^105
    ExtendedFloat160 { frac: 176868732008334225927912486150152183216, exp: 357 },                         // 20^112
    ExtendedFloat160 { frac: 210843958864610464486971481025400380154, exp: 387 },                         // 20^119
    ExtendedFloat160 { frac: 251345585423243599518503524095297312920, exp: 417 },                         // 20^126
    ExtendedFloat160 { frac: 299627286700300692937974362486955300474, exp: 447 },                         // 20^133
    ExtendedFloat160 { frac: 178591779887855465971216179422709524914, exp: 478 },                         // 20^140
    ExtendedFloat160 { frac: 212897992000407535995502685812365442412, exp: 508 },                         // 20^147
    ExtendedFloat160 { frac: 253794183731564922327402455583054354682, exp: 538 },                         // 20^154
    ExtendedFloat160 { frac: 302546243347602990063908643225496238091, exp: 568 },                         // 20^161
    ExtendedFloat160 { frac: 180331613628627651967947866455016278082, exp: 599 },                         // 20^168
    ExtendedFloat160 { frac: 214972035442146840057310898846407268146, exp: 629 },                         // 20^175
    ExtendedFloat160 { frac: 256266636183436918326986907537468991453, exp: 659 },                         // 20^182
    ExtendedFloat160 { frac: 305493636349960468205197939321361769978, exp: 689 },                         // 20^189
    ExtendedFloat160 { frac: 182088396757817547443627082897044283139, exp: 720 },                         // 20^196
    ExtendedFloat160 { frac: 217066284129402097992452481862359384464, exp: 750 },                         // 20^203
    ExtendedFloat160 { frac: 258763175164940474024358370140027266101, exp: 780 },                         // 20^210
    ExtendedFloat160 { frac: 308469742733169167070816004443201143863, exp: 810 },                         // 20^217
    ExtendedFloat160 { frac: 183862294395666818064937594201088633455, exp: 841 },                         // 20^224
    ExtendedFloat160 { frac: 219180934900840303975269310714112083263, exp: 871 },                         // 20^231
];
const BASE20_BIAS: i32 = 280;

// BASE21

const BASE21_STEP: i32 = 7;
const BASE21_SMALL_POWERS: [ExtendedFloat160; BASE21_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 21^0
    ExtendedFloat160 { frac: 223310303291865866647839586127097888768, exp: -123 },                        // 21^1
    ExtendedFloat160 { frac: 293094773070573949975289456791815979008, exp: -119 },                        // 21^2
    ExtendedFloat160 { frac: 192343444827564154671283706019629236224, exp: -114 },                        // 21^3
    ExtendedFloat160 { frac: 252450771336177953006059864150763372544, exp: -110 },                        // 21^4
    ExtendedFloat160 { frac: 331341637378733563320453571697876926464, exp: -106 },                        // 21^5
    ExtendedFloat160 { frac: 217442949529793900929047656426731732992, exp: -101 },                        // 21^6
];
const BASE21_SMALL_INT_POWERS: [u128; BASE21_STEP as usize] = [1, 21, 441, 9261, 194481, 4084101, 85766121];
const BASE21_LARGE_POWERS: [ExtendedFloat160; 74] = [
    ExtendedFloat160 { frac: 188930289708596676209799452254388021966, exp: -1357 },                       // 21^-280
    ExtendedFloat160 { frac: 316910613181035688810196803289889249327, exp: -1327 },                       // 21^-273
    ExtendedFloat160 { frac: 265792046637109917415346664544311644566, exp: -1296 },                       // 21^-266
    ExtendedFloat160 { frac: 222919047571269915259553685343512655124, exp: -1265 },                       // 21^-259
    ExtendedFloat160 { frac: 186961582932271121624867982904669033286, exp: -1234 },                       // 21^-252
    ExtendedFloat160 { frac: 313608315425491608924946770674906010971, exp: -1204 },                       // 21^-245
    ExtendedFloat160 { frac: 263022418727710133477288744271513099136, exp: -1173 },                       // 21^-238
    ExtendedFloat160 { frac: 220596168374913987040298637494305910349, exp: -1142 },                       // 21^-231
    ExtendedFloat160 { frac: 185013390634471631571140317809259206000, exp: -1111 },                       // 21^-224
    ExtendedFloat160 { frac: 310340428541697166126551714455108139315, exp: -1081 },                       // 21^-217
    ExtendedFloat160 { frac: 260281651120390792657949798157903984485, exp: -1050 },                       // 21^-210
    ExtendedFloat160 { frac: 218297494233351046265168261721480533031, exp: -1019 },                       // 21^-203
    ExtendedFloat160 { frac: 183085499048559996047315841365348324923, exp: -988 },                        // 21^-196
    ExtendedFloat160 { frac: 307106593958686861611740529249331599561, exp: -958 },                        // 21^-189
    ExtendedFloat160 { frac: 257569443082684061423017949935163592863, exp: -927 },                        // 21^-182
    ExtendedFloat160 { frac: 216022772923099798407562907443165238977, exp: -896 },                        // 21^-175
    ExtendedFloat160 { frac: 181177696635406520735133253192775193924, exp: -865 },                        // 21^-168
    ExtendedFloat160 { frac: 303906456841905544859103298218510174905, exp: -835 },                        // 21^-161
    ExtendedFloat160 { frac: 254885497015839035366704878377106222377, exp: -804 },                        // 21^-154
    ExtendedFloat160 { frac: 213771754848918589809660748287013123679, exp: -773 },                        // 21^-147
    ExtendedFloat160 { frac: 179289774060178761532557823153782969422, exp: -742 },                        // 21^-140
    ExtendedFloat160 { frac: 300739666054273966520895709937608788747, exp: -712 },                        // 21^-133
    ExtendedFloat160 { frac: 252229518422167527105381200194678741672, exp: -681 },                        // 21^-126
    ExtendedFloat160 { frac: 211544193016418411141172507362190098875, exp: -650 },                        // 21^-119
    ExtendedFloat160 { frac: 177421524169372127003871154186445625381, exp: -619 },                        // 21^-112
    ExtendedFloat160 { frac: 297605874117660039208334869188863219128, exp: -589 },                        // 21^-105
    ExtendedFloat160 { frac: 249601215872730120248972738434931034589, exp: -558 },                        // 21^-98
    ExtendedFloat160 { frac: 209339843004961281067680970866486691183, exp: -527 },                        // 21^-91
    ExtendedFloat160 { frac: 175572741968079828414917198043029764862, exp: -496 },                        // 21^-84
    ExtendedFloat160 { frac: 294504737174751578579986057113044461622, exp: -466 },                        // 21^-77
    ExtendedFloat160 { frac: 247000300975358943781849899984000214119, exp: -435 },                        // 21^-70
    ExtendedFloat160 { frac: 207158462940841036386486635842909509778, exp: -404 },                        // 21^-63
    ExtendedFloat160 { frac: 173743224597499683284324187350601048284, exp: -373 },                        // 21^-56
    ExtendedFloat160 { frac: 291435914951326341195521446658582144907, exp: -343 },                        // 21^-49
    ExtendedFloat160 { frac: 244426488343013658131803347774089798564, exp: -312 },                        // 21^-42
    ExtendedFloat160 { frac: 204999813470743585875109991683322467328, exp: -281 },                        // 21^-35
    ExtendedFloat160 { frac: 171932771312675304370424739613867083896, exp: -250 },                        // 21^-28
    ExtendedFloat160 { frac: 288399070718915219190771894363169343679, exp: -220 },                        // 21^-21
    ExtendedFloat160 { frac: 241879495562467180767563428674995339169, exp: -189 },                        // 21^-14
    ExtendedFloat160 { frac: 202863657735483715761934442388454879916, exp: -158 },                        // 21^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 21^0
    ExtendedFloat160 { frac: 285393871257854494969375049060085399552, exp: -97 },                         // 21^7
    ExtendedFloat160 { frac: 239359043163317715346200180609446313984, exp: -66 },                         // 21^14
    ExtendedFloat160 { frac: 200749761344015565073276401119215484928, exp: -35 },                         // 21^21
    ExtendedFloat160 { frac: 336736528915531181897146582027467512352, exp: -5 },                          // 21^28
    ExtendedFloat160 { frac: 282419986820723101796180638245767114566, exp: 26 },                          // 21^35
    ExtendedFloat160 { frac: 236864854587323684235605162154192799609, exp: 57 },                          // 21^42
    ExtendedFloat160 { frac: 198657892347713919139770232958888414539, exp: 88 },                          // 21^49
    ExtendedFloat160 { frac: 333227639539799771559276379603105082665, exp: 118 },                         // 21^56
    ExtendedFloat160 { frac: 279477091096160878422591085338554667498, exp: 149 },                         // 21^63
    ExtendedFloat160 { frac: 234396656158058199668034426556911280906, exp: 180 },                         // 21^70
    ExtendedFloat160 { frac: 196587821214923499260650393397155679946, exp: 211 },                         // 21^77
    ExtendedFloat160 { frac: 329755313778627116894330266634709348105, exp: 241 },                         // 21^84
    ExtendedFloat160 { frac: 276564861173063847678827937275753188756, exp: 272 },                         // 21^91
    ExtendedFloat160 { frac: 231954177050879743842271364358265802922, exp: 303 },                         // 21^98
    ExtendedFloat160 { frac: 194539320805773455930561882331086488568, exp: 334 },                         // 21^105
    ExtendedFloat160 { frac: 326319170628861950820780822968026849107, exp: 364 },                         // 21^112
    ExtendedFloat160 { frac: 273682977505152590337309001442167333935, exp: 395 },                         // 21^119
    ExtendedFloat160 { frac: 229537149263215762988196018745023784859, exp: 426 },                         // 21^126
    ExtendedFloat160 { frac: 192512166347254302133159603748230049238, exp: 457 },                         // 21^133
    ExtendedFloat160 { frac: 322918833057513041780316846850363342758, exp: 487 },                         // 21^140
    ExtendedFloat160 { frac: 270831123875909826489436864951365328546, exp: 518 },                         // 21^147
    ExtendedFloat160 { frac: 227145307585155914742230901032159391581, exp: 549 },                         // 21^154
    ExtendedFloat160 { frac: 190506135408554552002160365053046193282, exp: 580 },                         // 21^161
    ExtendedFloat160 { frac: 319553927960379009120613001977483142041, exp: 610 },                         // 21^168
    ExtendedFloat160 { frac: 268008987363883357189628558586004761583, exp: 641 },                         // 21^175
    ExtendedFloat160 { frac: 224778389570351742159039695242233732277, exp: 672 },                         // 21^182
    ExtendedFloat160 { frac: 188521007876654358646847770858984158583, exp: 703 },                         // 21^189
    ExtendedFloat160 { frac: 316224086121109227441755855330741121363, exp: 733 },                         // 21^196
    ExtendedFloat160 { frac: 265216258308350559209803685525363124313, exp: 764 },                         // 21^203
    ExtendedFloat160 { frac: 222436135507219581307712678266362793773, exp: 795 },                         // 21^210
    ExtendedFloat160 { frac: 186556565932173473138426942632517671964, exp: 826 },                         // 21^217
    ExtendedFloat160 { frac: 312928942170691327838033505846831903751, exp: 856 },                         // 21^224
    ExtendedFloat160 { frac: 262452630275340665419193194878653363790, exp: 887 },                         // 21^231
];
const BASE21_BIAS: i32 = 280;

// BASE22

const BASE22_STEP: i32 = 7;
const BASE22_SMALL_POWERS: [ExtendedFloat160; BASE22_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 22^0
    ExtendedFloat160 { frac: 233944127258145193631070042609340645376, exp: -123 },                        // 22^1
    ExtendedFloat160 { frac: 321673174979949641242721308587843387392, exp: -119 },                        // 22^2
    ExtendedFloat160 { frac: 221150307798715378354370899654142328832, exp: -114 },                        // 22^3
    ExtendedFloat160 { frac: 304081673223233645237259987024445702144, exp: -110 },                        // 22^4
    ExtendedFloat160 { frac: 209056150340973131100616241079306420224, exp: -105 },                        // 22^5
    ExtendedFloat160 { frac: 287452206718838055263347331484046327808, exp: -101 },                        // 22^6
];
const BASE22_SMALL_INT_POWERS: [u128; BASE22_STEP as usize] = [1, 22, 484, 10648, 234256, 5153632, 113379904];
const BASE22_LARGE_POWERS: [ExtendedFloat160; 72] = [
    ExtendedFloat160 { frac: 253485022519018253919924370237113401813, exp: -1345 },                       // 22^-273
    ExtendedFloat160 { frac: 294429420218882528916536957614299023600, exp: -1314 },                       // 22^-266
    ExtendedFloat160 { frac: 170993699408656992611557257796811971606, exp: -1282 },                       // 22^-259
    ExtendedFloat160 { frac: 198613611477559667549559644399735817047, exp: -1251 },                       // 22^-252
    ExtendedFloat160 { frac: 230694854843066209807480559654047740794, exp: -1220 },                       // 22^-245
    ExtendedFloat160 { frac: 267958050080955588510051414081700394572, exp: -1189 },                       // 22^-238
    ExtendedFloat160 { frac: 311240216657766425412953324616068813788, exp: -1158 },                       // 22^-231
    ExtendedFloat160 { frac: 180756787183491671385793896073455879749, exp: -1126 },                       // 22^-224
    ExtendedFloat160 { frac: 209953690842110549054787911387661891558, exp: -1095 },                       // 22^-217
    ExtendedFloat160 { frac: 243866650791248218437107825156701479612, exp: -1064 },                       // 22^-210
    ExtendedFloat160 { frac: 283257432291885574825362738029166670391, exp: -1033 },                       // 22^-203
    ExtendedFloat160 { frac: 329010845428281790557809997379933231677, exp: -1002 },                       // 22^-196
    ExtendedFloat160 { frac: 191077310017213090920037889613280023761, exp: -970 },                        // 22^-189
    ExtendedFloat160 { frac: 221941245467987325560887198530024987097, exp: -939 },                        // 22^-182
    ExtendedFloat160 { frac: 257790506028392555656294807759853932728, exp: -908 },                        // 22^-175
    ExtendedFloat160 { frac: 299430350849140679073407301022315056449, exp: -877 },                        // 22^-168
    ExtendedFloat160 { frac: 173898054647064197545039940705011138943, exp: -845 },                        // 22^-161
    ExtendedFloat160 { frac: 201987095324676314922450395008827008097, exp: -814 },                        // 22^-154
    ExtendedFloat160 { frac: 234613243722037545951821958217296505941, exp: -783 },                        // 22^-147
    ExtendedFloat160 { frac: 272509360270263083361596599896866808472, exp: -752 },                        // 22^-140
    ExtendedFloat160 { frac: 316526681344939644363801942778159729297, exp: -721 },                        // 22^-133
    ExtendedFloat160 { frac: 183826970023851061892407973678416422311, exp: -689 },                        // 22^-126
    ExtendedFloat160 { frac: 213519787744680626489456759979414929795, exp: -658 },                        // 22^-119
    ExtendedFloat160 { frac: 248008764723795396613310192753363562635, exp: -627 },                        // 22^-112
    ExtendedFloat160 { frac: 288068604926548520494484698905215058937, exp: -596 },                        // 22^-105
    ExtendedFloat160 { frac: 334599147077506390730919115808572688418, exp: -565 },                        // 22^-98
    ExtendedFloat160 { frac: 194322788582847037606425160078121997271, exp: -533 },                        // 22^-91
    ExtendedFloat160 { frac: 225710952896522753756036626884027523975, exp: -502 },                        // 22^-84
    ExtendedFloat160 { frac: 262169118861406100263284327408804984594, exp: -471 },                        // 22^-77
    ExtendedFloat160 { frac: 304516223083230613247976703037939646270, exp: -440 },                        // 22^-70
    ExtendedFloat160 { frac: 176851740822108453297561032060281913714, exp: -408 },                        // 22^-63
    ExtendedFloat160 { frac: 205417878332621336576645714920472857997, exp: -377 },                        // 22^-56
    ExtendedFloat160 { frac: 238598187060653380400994399373084663429, exp: -346 },                        // 22^-49
    ExtendedFloat160 { frac: 277137975188549744820727493204211809261, exp: -315 },                        // 22^-42
    ExtendedFloat160 { frac: 321902937477411396463387072352875080544, exp: -284 },                        // 22^-35
    ExtendedFloat160 { frac: 186949300409097210175717446748311788922, exp: -252 },                        // 22^-28
    ExtendedFloat160 { frac: 217146455371525722164937611369356262209, exp: -221 },                        // 22^-21
    ExtendedFloat160 { frac: 252221233121680633945860233133208134270, exp: -190 },                        // 22^-14
    ExtendedFloat160 { frac: 292961496095243353908740467141073506540, exp: -159 },                        // 22^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 22^0
    ExtendedFloat160 { frac: 197623392119201162993551290395281850368, exp: -96 },                         // 22^7
    ExtendedFloat160 { frac: 229544689406569328704727047276706398208, exp: -65 },                         // 22^14
    ExtendedFloat160 { frac: 266622103131276669014944105065242165248, exp: -34 },                         // 22^21
    ExtendedFloat160 { frac: 309688479667831891620434363534947647488, exp: -3 },                          // 22^28
    ExtendedFloat160 { frac: 179855595827611186243917689814617910464, exp: 29 },                          // 22^35
    ExtendedFloat160 { frac: 208906933736774069538597567757512183241, exp: 60 },                          // 22^42
    ExtendedFloat160 { frac: 242650815297018604365857953016443636978, exp: 91 },                          // 22^49
    ExtendedFloat160 { frac: 281845207868958552685905724295461354592, exp: 122 },                         // 22^56
    ExtendedFloat160 { frac: 327370510177191550022527822209865447333, exp: 153 },                         // 22^63
    ExtendedFloat160 { frac: 190124664073591590972067520727552350138, exp: 185 },                         // 22^70
    ExtendedFloat160 { frac: 220834722526051754798575401303998056567, exp: 216 },                         // 22^77
    ExtendedFloat160 { frac: 256505250966711193359488809204377717494, exp: 247 },                         // 22^84
    ExtendedFloat160 { frac: 297937493800295372760917121026044042670, exp: 278 },                         // 22^91
    ExtendedFloat160 { frac: 173031058579617612943075122066206208260, exp: 310 },                         // 22^98
    ExtendedFloat160 { frac: 200980056932689281584830175015332323410, exp: 341 },                         // 22^105
    ExtendedFloat160 { frac: 233443542542281837469979323535186565406, exp: 372 },                         // 22^112
    ExtendedFloat160 { frac: 271150722048713022289973166238934106646, exp: 403 },                         // 22^119
    ExtendedFloat160 { frac: 314948587854906379895896832699151980516, exp: 434 },                         // 22^126
    ExtendedFloat160 { frac: 182910471789153927982518189465009158829, exp: 466 },                         // 22^133
    ExtendedFloat160 { frac: 212455251303071888680280266327561854998, exp: 497 },                         // 22^140
    ExtendedFloat160 { frac: 246772278069909496307705991140052073087, exp: 528 },                         // 22^147
    ExtendedFloat160 { frac: 286632393646709698979385809127728921005, exp: 559 },                         // 22^154
    ExtendedFloat160 { frac: 332930950470730393610585733499849696474, exp: 590 },                         // 22^161
    ExtendedFloat160 { frac: 193353961796034982060992467369794830962, exp: 622 },                         // 22^168
    ExtendedFloat160 { frac: 224585635486054469192295827945894724481, exp: 653 },                         // 22^175
    ExtendedFloat160 { frac: 260862033537650797156363766298784568546, exp: 684 },                         // 22^182
    ExtendedFloat160 { frac: 302998009619470597652934458975103615488, exp: 715 },                         // 22^189
    ExtendedFloat160 { frac: 175970018688269509849993361647138860754, exp: 747 },                         // 22^196
    ExtendedFloat160 { frac: 204393735233038880379975139336743154541, exp: 778 },                         // 22^203
    ExtendedFloat160 { frac: 237408618319925875163860351801153447367, exp: 809 },                         // 22^210
    ExtendedFloat160 { frac: 275756260280259146490263920764281465178, exp: 840 },                         // 22^217
    ExtendedFloat160 { frac: 320298039817924375055421761218548035863, exp: 871 },                         // 22^224
];
const BASE22_BIAS: i32 = 273;

// BASE23

const BASE23_STEP: i32 = 7;
const BASE23_SMALL_POWERS: [ExtendedFloat160; BASE23_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 23^0
    ExtendedFloat160 { frac: 244577951224424520614300499091583401984, exp: -123 },                        // 23^1
    ExtendedFloat160 { frac: 175790402442555124191528483722075570176, exp: -118 },                        // 23^2
    ExtendedFloat160 { frac: 252698703511172991025322195350483632128, exp: -114 },                        // 23^3
    ExtendedFloat160 { frac: 181627193148655587299450327908160110592, exp: -109 },                        // 23^4
    ExtendedFloat160 { frac: 261089090151192406742959846367980158976, exp: -105 },                        // 23^5
    ExtendedFloat160 { frac: 187657783546169542346502389576985739264, exp: -100 },                        // 23^6
];
const BASE23_SMALL_INT_POWERS: [u128; BASE23_STEP as usize] = [1, 23, 529, 12167, 279841, 6436343, 148035889];
const BASE23_LARGE_POWERS: [ExtendedFloat160; 71] = [
    ExtendedFloat160 { frac: 282696120152370340326324393350129313583, exp: -1331 },                       // 23^-266
    ExtendedFloat160 { frac: 224106699149813515160027793248965839768, exp: -1299 },                       // 23^-259
    ExtendedFloat160 { frac: 177660070384959299518117426820409879479, exp: -1267 },                       // 23^-252
    ExtendedFloat160 { frac: 281679224484842507661590842703493245235, exp: -1236 },                       // 23^-245
    ExtendedFloat160 { frac: 223300557447880794264720046366338636067, exp: -1204 },                       // 23^-238
    ExtendedFloat160 { frac: 177021003404592607294928857171705382014, exp: -1172 },                       // 23^-231
    ExtendedFloat160 { frac: 280665986726726659567026238400885338462, exp: -1141 },                       // 23^-224
    ExtendedFloat160 { frac: 222497315545222527698408099502330410230, exp: -1109 },                       // 23^-217
    ExtendedFloat160 { frac: 176384235233432227183413281705333529169, exp: -1077 },                       // 23^-210
    ExtendedFloat160 { frac: 279656393720034524921314175179274359595, exp: -1046 },                       // 23^-203
    ExtendedFloat160 { frac: 221696963010873772243653930801015609536, exp: -1014 },                       // 23^-196
    ExtendedFloat160 { frac: 175749757602354687973874432000535316950, exp: -982 },                        // 23^-189
    ExtendedFloat160 { frac: 278650432354108872233513992865613552171, exp: -951 },                        // 23^-182
    ExtendedFloat160 { frac: 220899489451391157505367921930602163472, exp: -919 },                        // 23^-175
    ExtendedFloat160 { frac: 175117562271981659025201047697761937562, exp: -887 },                        // 23^-168
    ExtendedFloat160 { frac: 277648089565453253577594524944387660343, exp: -856 },                        // 23^-161
    ExtendedFloat160 { frac: 220104884510717915806439152820624263438, exp: -824 },                        // 23^-154
    ExtendedFloat160 { frac: 174487641032572953025011241897191693675, exp: -792 },                        // 23^-147
    ExtendedFloat160 { frac: 276649352337562360960754521131809259049, exp: -761 },                        // 23^-140
    ExtendedFloat160 { frac: 219313137870049397588841701361789229875, exp: -729 },                        // 23^-133
    ExtendedFloat160 { frac: 173859985703919913633146478806481352851, exp: -697 },                        // 23^-126
    ExtendedFloat160 { frac: 275654207700752992922523689325318624601, exp: -666 },                        // 23^-119
    ExtendedFloat160 { frac: 218524239247699070573788964393354746146, exp: -634 },                        // 23^-112
    ExtendedFloat160 { frac: 173234588135239186624038395745277208596, exp: -602 },                        // 23^-105
    ExtendedFloat160 { frac: 274662642731995629169562809047557585825, exp: -571 },                        // 23^-98
    ExtendedFloat160 { frac: 217738178398965000940790150805824878955, exp: -539 },                        // 23^-91
    ExtendedFloat160 { frac: 172611440205066873148451849294509762826, exp: -507 },                        // 23^-84
    ExtendedFloat160 { frac: 273674644554746611058977369746265857416, exp: -476 },                        // 23^-77
    ExtendedFloat160 { frac: 216954945115996814791722634413562231827, exp: -444 },                        // 23^-70
    ExtendedFloat160 { frac: 171990533821153063740069798803230423676, exp: -412 },                        // 23^-63
    ExtendedFloat160 { frac: 272690200338780925750826785583705218666, exp: -381 },                        // 23^-56
    ExtendedFloat160 { frac: 216174529227663138172270874689475697712, exp: -349 },                        // 23^-49
    ExtendedFloat160 { frac: 171371860920356751697330024074687407971, exp: -317 },                        // 23^-42
    ExtendedFloat160 { frac: 271709297300025591858350535820526217684, exp: -286 },                        // 23^-35
    ExtendedFloat160 { frac: 215396920599419513929297198496715326537, exp: -254 },                        // 23^-28
    ExtendedFloat160 { frac: 170755413468541124475850272054650469150, exp: -222 },                        // 23^-21
    ExtendedFloat160 { frac: 270731922700393644432243678371210997948, exp: -191 },                        // 23^-14
    ExtendedFloat160 { frac: 214622109133176793688901966303396671549, exp: -159 },                        // 23^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 23^0
    ExtendedFloat160 { frac: 269758063847618717123097185016917000192, exp: -96 },                         // 23^7
    ExtendedFloat160 { frac: 213850084767170003246100602438595641344, exp: -64 },                         // 23^14
    ExtendedFloat160 { frac: 339058325839400057321133061640411938816, exp: -33 },                         // 23^21
    ExtendedFloat160 { frac: 268787708095090219373873551177395072962, exp: -1 },                          // 23^28
    ExtendedFloat160 { frac: 213080837475827679663192730864754774513, exp: 31 },                          // 23^35
    ExtendedFloat160 { frac: 337838687796969586566185112723027994705, exp: 62 },                          // 23^42
    ExtendedFloat160 { frac: 267820842841689106502015241773782572538, exp: 94 },                          // 23^49
    ExtendedFloat160 { frac: 212314357269641678380024305538561269739, exp: 126 },                         // 23^56
    ExtendedFloat160 { frac: 336623436955327832661614051077606366471, exp: 157 },                         // 23^63
    ExtendedFloat160 { frac: 266857455531624240538482847903341038248, exp: 189 },                         // 23^70
    ExtendedFloat160 { frac: 211550634195037448645447237257455979321, exp: 221 },                         // 23^77
    ExtendedFloat160 { frac: 335412557533128124785597638278283462337, exp: 252 },                         // 23^84
    ExtendedFloat160 { frac: 265897533654269339698691446779528819034, exp: 284 },                         // 23^91
    ExtendedFloat160 { frac: 210789658334244775585362676204225495858, exp: 316 },                         // 23^98
    ExtendedFloat160 { frac: 334206033805791401974785682123232789907, exp: 347 },                         // 23^105
    ExtendedFloat160 { frac: 264941064744000514367957679290425962065, exp: 379 },                         // 23^112
    ExtendedFloat160 { frac: 210031419805168987228793890184514974036, exp: 411 },                         // 23^119
    ExtendedFloat160 { frac: 333003850105302012456495057986285588216, exp: 442 },                         // 23^126
    ExtendedFloat160 { frac: 263988036380034387491686584861354377998, exp: 474 },                         // 23^133
    ExtendedFloat160 { frac: 209275908761262624819472661494385533289, exp: 506 },                         // 23^140
    ExtendedFloat160 { frac: 331805990820004247517955628678622148639, exp: 537 },                         // 23^147
    ExtendedFloat160 { frac: 263038436186266797268116360982380732098, exp: 569 },                         // 23^154
    ExtendedFloat160 { frac: 208523115391397574746439388866966551936, exp: 601 },                         // 23^161
    ExtendedFloat160 { frac: 330612440394399607270379273154482344971, exp: 632 },                         // 23^168
    ExtendedFloat160 { frac: 262092251831112080049001043331284413642, exp: 664 },                         // 23^175
    ExtendedFloat160 { frac: 207773029919737660433151703156509162570, exp: 696 },                         // 23^182
    ExtendedFloat160 { frac: 329423183328944795675128680336602321085, exp: 727 },                         // 23^189
    ExtendedFloat160 { frac: 261149471027342931361145730086041639640, exp: 759 },                         // 23^196
    ExtendedFloat160 { frac: 207025642605611692530569464171386341133, exp: 791 },                         // 23^203
    ExtendedFloat160 { frac: 328238204179850442208732821518504518810, exp: 822 },                         // 23^210
    ExtendedFloat160 { frac: 260210081531930842969216498623209392426, exp: 854 },                         // 23^217
    ExtendedFloat160 { frac: 206280943743386975765635578931435897048, exp: 886 },                         // 23^224
];
const BASE23_BIAS: i32 = 266;

// BASE24

const BASE24_STEP: i32 = 7;
const BASE24_SMALL_POWERS: [ExtendedFloat160; BASE24_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 24^0
    ExtendedFloat160 { frac: 255211775190703847597530955573826158592, exp: -123 },                        // 24^1
    ExtendedFloat160 { frac: 191408831393027885698148216680369618944, exp: -118 },                        // 24^2
    ExtendedFloat160 { frac: 287113247089541828547222325020554428416, exp: -114 },                        // 24^3
    ExtendedFloat160 { frac: 215334935317156371410416743765415821312, exp: -109 },                        // 24^4
    ExtendedFloat160 { frac: 323002402975734557115625115648123731968, exp: -105 },                        // 24^5
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -100 },                        // 24^6
];
const BASE24_SMALL_INT_POWERS: [u128; BASE24_STEP as usize] = [1, 24, 576, 13824, 331776, 7962624, 191102976];
const BASE24_LARGE_POWERS: [ExtendedFloat160; 70] = [
    ExtendedFloat160 { frac: 224498717373391335032231518045098273537, exp: -1347 },                       // 24^-266
    ExtendedFloat160 { frac: 239735690866995532087641762678041955188, exp: -1315 },                       // 24^-259
    ExtendedFloat160 { frac: 256006814417050404626793229969178591795, exp: -1283 },                       // 24^-252
    ExtendedFloat160 { frac: 273382276918988884237693747042282021609, exp: -1251 },                       // 24^-245
    ExtendedFloat160 { frac: 291937031065346039954998156631577529912, exp: -1219 },                       // 24^-238
    ExtendedFloat160 { frac: 311751116669878803408975082301396512655, exp: -1187 },                       // 24^-231
    ExtendedFloat160 { frac: 332910005936047335476283449703688561122, exp: -1155 },                       // 24^-224
    ExtendedFloat160 { frac: 177752486079622930343415992310050508587, exp: -1122 },                       // 24^-217
    ExtendedFloat160 { frac: 189816741726628588213403698819375225722, exp: -1090 },                       // 24^-210
    ExtendedFloat160 { frac: 202699811599676133995465766268541805984, exp: -1058 },                       // 24^-203
    ExtendedFloat160 { frac: 216457269515865090355509585365869594574, exp: -1026 },                       // 24^-196
    ExtendedFloat160 { frac: 231148461148045387015380597263260157877, exp: -994 },                        // 24^-189
    ExtendedFloat160 { frac: 246836760024792608106756526472045881483, exp: -962 },                        // 24^-182
    ExtendedFloat160 { frac: 263589840905381559535877208688654464260, exp: -930 },                        // 24^-175
    ExtendedFloat160 { frac: 281479971709018296242657937208050445965, exp: -898 },                        // 24^-168
    ExtendedFloat160 { frac: 300584325257628424747408646813479651038, exp: -866 },                        // 24^-161
    ExtendedFloat160 { frac: 320985312176969416466104839150917967197, exp: -834 },                        // 24^-154
    ExtendedFloat160 { frac: 171385468196052762160979317193129295473, exp: -801 },                        // 24^-147
    ExtendedFloat160 { frac: 183017587375374702561553597022155160742, exp: -769 },                        // 24^-140
    ExtendedFloat160 { frac: 195439191206027575440487166351295574484, exp: -737 },                        // 24^-133
    ExtendedFloat160 { frac: 208703862874796048578293668364396201854, exp: -705 },                        // 24^-126
    ExtendedFloat160 { frac: 222868822317958475703480592144987545632, exp: -673 },                        // 24^-119
    ExtendedFloat160 { frac: 237995173051452727716558620615765508935, exp: -641 },                        // 24^-112
    ExtendedFloat160 { frac: 254148165753675349373102394182948812519, exp: -609 },                        // 24^-105
    ExtendedFloat160 { frac: 271397479737933588417468230506889186025, exp: -577 },                        // 24^-98
    ExtendedFloat160 { frac: 289817523528740604428224130917268871991, exp: -545 },                        // 24^-91
    ExtendedFloat160 { frac: 309487755838552588810803796052767101096, exp: -513 },                        // 24^-84
    ExtendedFloat160 { frac: 330493028329548101430287061507520336961, exp: -481 },                        // 24^-77
    ExtendedFloat160 { frac: 176461975819512133258798291874254633040, exp: -448 },                        // 24^-70
    ExtendedFloat160 { frac: 188438643123668474334468683754392032451, exp: -416 },                        // 24^-63
    ExtendedFloat160 { frac: 201228179937237770199942876645925476060, exp: -384 },                        // 24^-56
    ExtendedFloat160 { frac: 214885756602899904017224155871405769601, exp: -352 },                        // 24^-49
    ExtendedFloat160 { frac: 229470287934835004924643178169318563534, exp: -320 },                        // 24^-42
    ExtendedFloat160 { frac: 245044687360099685434665347000146337133, exp: -288 },                        // 24^-35
    ExtendedFloat160 { frac: 261676138308856451194147028266269550444, exp: -256 },                        // 24^-28
    ExtendedFloat160 { frac: 279436384024154813848437280673013431065, exp: -224 },                        // 24^-21
    ExtendedFloat160 { frac: 298402037041419227483658365640566588740, exp: -192 },                        // 24^-14
    ExtendedFloat160 { frac: 318654909672648364505254319167929262488, exp: -160 },                        // 24^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 24^0
    ExtendedFloat160 { frac: 181688851673850688377539127552069599232, exp: -95 },                         // 24^7
    ExtendedFloat160 { frac: 194020272759136452871913121072449323008, exp: -63 },                         // 24^14
    ExtendedFloat160 { frac: 207188640880972374233825193254612631552, exp: -31 },                         // 24^21
    ExtendedFloat160 { frac: 221250760550139932836609227367108313088, exp: 1 },                           // 24^28
    ExtendedFloat160 { frac: 236267291661697281793781435669856387072, exp: 33 },                          // 24^35
    ExtendedFloat160 { frac: 252303011164126931290527343657214803968, exp: 65 },                          // 24^42
    ExtendedFloat160 { frac: 269427092488254686881046533485512097792, exp: 97 },                          // 24^49
    ExtendedFloat160 { frac: 287713403941314941508226937857819803648, exp: 129 },                         // 24^56
    ExtendedFloat160 { frac: 307240827353347547401607574753443315712, exp: 161 },                         // 24^63
    ExtendedFloat160 { frac: 328093598350474163167634651360244400128, exp: 193 },                         // 24^70
    ExtendedFloat160 { frac: 175180834861447020226468989874232056416, exp: 226 },                         // 24^77
    ExtendedFloat160 { frac: 187070549727531559196917812917453861026, exp: 258 },                         // 24^84
    ExtendedFloat160 { frac: 199767232545952890607255496509019333039, exp: 290 },                         // 24^91
    ExtendedFloat160 { frac: 213325653114257310428744028742785781912, exp: 322 },                         // 24^98
    ExtendedFloat160 { frac: 227804298516055047806476167412340090352, exp: 354 },                         // 24^105
    ExtendedFloat160 { frac: 243265625417291205836310243227923719532, exp: 386 },                         // 24^112
    ExtendedFloat160 { frac: 259776329486140560138677002900131432918, exp: 418 },                         // 24^119
    ExtendedFloat160 { frac: 277407633098725295421526662764935275289, exp: 450 },                         // 24^126
    ExtendedFloat160 { frac: 296235592571734482952577544661578831571, exp: 482 },                         // 24^133
    ExtendedFloat160 { frac: 316341426247257477645159711999449660471, exp: 514 },                         // 24^140
    ExtendedFloat160 { frac: 337811864845093800590802876046287308325, exp: 546 },                         // 24^147
    ExtendedFloat160 { frac: 180369762796928745579122531717097251784, exp: 579 },                         // 24^154
    ExtendedFloat160 { frac: 192611655877384358682393055110005707838, exp: 611 },                         // 24^161
    ExtendedFloat160 { frac: 205684419630781050995309380627725821797, exp: 643 },                         // 24^168
    ExtendedFloat160 { frac: 219644446158456132093135554410564634898, exp: 675 },                         // 24^175
    ExtendedFloat160 { frac: 234551954955343535589691141355422293223, exp: 707 },                         // 24^182
    ExtendedFloat160 { frac: 250471252679363433757155530343900661758, exp: 739 },                         // 24^189
    ExtendedFloat160 { frac: 267471010551644448060009348077202513313, exp: 771 },                         // 24^196
    ExtendedFloat160 { frac: 285624560584202347610957248166426707331, exp: 803 },                         // 24^203
    ExtendedFloat160 { frac: 305010211912915299914630616083972269987, exp: 835 },                         // 24^210
    ExtendedFloat160 { frac: 325711588600364141070945877624827809796, exp: 867 },                         // 24^217
];
const BASE24_BIAS: i32 = 266;

// BASE25

const BASE25_STEP: i32 = 7;
const BASE25_SMALL_POWERS: [ExtendedFloat160; BASE25_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 25^0
    ExtendedFloat160 { frac: 265845599156983174580761412056068915200, exp: -123 },                        // 25^1
    ExtendedFloat160 { frac: 207691874341393105141219853168803840000, exp: -118 },                        // 25^2
    ExtendedFloat160 { frac: 324518553658426726783156020576256000000, exp: -114 },                        // 25^3
    ExtendedFloat160 { frac: 253530120045645880299340641075200000000, exp: -109 },                        // 25^4
    ExtendedFloat160 { frac: 198070406285660843983859875840000000000, exp: -104 },                        // 25^5
    ExtendedFloat160 { frac: 309485009821345068724781056000000000000, exp: -100 },                        // 25^6
];
const BASE25_SMALL_INT_POWERS: [u128; BASE25_STEP as usize] = [1, 25, 625, 15625, 390625, 9765625, 244140625];
const BASE25_LARGE_POWERS: [ExtendedFloat160; 69] = [
    ExtendedFloat160 { frac: 201109222516499671628641054110261305647, exp: -1330 },                       // 25^-259
    ExtendedFloat160 { frac: 285793394306920833441610418092098634655, exp: -1298 },                       // 25^-252
    ExtendedFloat160 { frac: 203068420253004570555511362849258201390, exp: -1265 },                       // 25^-245
    ExtendedFloat160 { frac: 288577581746103207017755725657449092679, exp: -1233 },                       // 25^-238
    ExtendedFloat160 { frac: 205046704412910121830119952091883627559, exp: -1200 },                       // 25^-231
    ExtendedFloat160 { frac: 291388892624283530821742192659774598780, exp: -1168 },                       // 25^-224
    ExtendedFloat160 { frac: 207044260935364498850036477975162511299, exp: -1135 },                       // 25^-217
    ExtendedFloat160 { frac: 294227591176883860910658765384315687611, exp: -1103 },                       // 25^-210
    ExtendedFloat160 { frac: 209061277570927374050781655074839937648, exp: -1070 },                       // 25^-203
    ExtendedFloat160 { frac: 297093944213496817569054052050375869453, exp: -1038 },                       // 25^-196
    ExtendedFloat160 { frac: 211097943899216614887176072592734406508, exp: -1005 },                       // 25^-189
    ExtendedFloat160 { frac: 299988221142963048588365030287739055137, exp: -973 },                        // 25^-182
    ExtendedFloat160 { frac: 213154451346726893197828921904416471830, exp: -940 },                        // 25^-175
    ExtendedFloat160 { frac: 302910693998692996157485768413290076965, exp: -908 },                        // 25^-168
    ExtendedFloat160 { frac: 215230993204821882725842221200657943544, exp: -875 },                        // 25^-161
    ExtendedFloat160 { frac: 305861637464235347360161968596028634045, exp: -843 },                        // 25^-154
    ExtendedFloat160 { frac: 217327764647901735884376228537482684576, exp: -810 },                        // 25^-147
    ExtendedFloat160 { frac: 308841328899094571460716776609676066664, exp: -778 },                        // 25^-140
    ExtendedFloat160 { frac: 219444962751747547330237450047488370802, exp: -745 },                        // 25^-133
    ExtendedFloat160 { frac: 311850048364799970571308236412006025948, exp: -713 },                        // 25^-126
    ExtendedFloat160 { frac: 221582786512044528543660416923448526878, exp: -680 },                        // 25^-119
    ExtendedFloat160 { frac: 314888078651228693933689466069052580904, exp: -648 },                        // 25^-112
    ExtendedFloat160 { frac: 223741436863085634409521749481834675708, exp: -615 },                        // 25^-105
    ExtendedFloat160 { frac: 317955705303185189918510999237120523316, exp: -583 },                        // 25^-98
    ExtendedFloat160 { frac: 225921116696657399755928707376370229068, exp: -550 },                        // 25^-91
    ExtendedFloat160 { frac: 321053216647239593947814323906257853121, exp: -518 },                        // 25^-84
    ExtendedFloat160 { frac: 228122030881109760932058580285014566244, exp: -485 },                        // 25^-77
    ExtendedFloat160 { frac: 324180903818827574883781864350871964922, exp: -453 },                        // 25^-70
    ExtendedFloat160 { frac: 230344386280611654799899571593522271174, exp: -420 },                        // 25^-63
    ExtendedFloat160 { frac: 327339060789614187001318969682759915221, exp: -388 },                        // 25^-56
    ExtendedFloat160 { frac: 232588391774594204975783618524161450993, exp: -355 },                        // 25^-49
    ExtendedFloat160 { frac: 330527984395124299475957654016385519914, exp: -323 },                        // 25^-42
    ExtendedFloat160 { frac: 234854258277383322788948059678933702737, exp: -290 },                        // 25^-35
    ExtendedFloat160 { frac: 333747974362642200374222141588992517906, exp: -258 },                        // 25^-28
    ExtendedFloat160 { frac: 237142198758023568227473377297792835283, exp: -225 },                        // 25^-21
    ExtendedFloat160 { frac: 336999333339382997433337688587745383420, exp: -193 },                        // 25^-14
    ExtendedFloat160 { frac: 239452428260295134118491722992235809940, exp: -160 },                        // 25^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 25^0
    ExtendedFloat160 { frac: 241785163922925834941235200000000000000, exp: -95 },                         // 25^7
    ExtendedFloat160 { frac: 171798691840000000000000000000000000000, exp: -62 },                         // 25^14
    ExtendedFloat160 { frac: 244140625000000000000000000000000000000, exp: -30 },                         // 25^21
    ExtendedFloat160 { frac: 173472347597680709441192448139190673828, exp: 3 },                           // 25^28
    ExtendedFloat160 { frac: 246519032881566189191165176650870696772, exp: 35 },                          // 25^35
    ExtendedFloat160 { frac: 175162308040602133865466197911239516410, exp: 68 },                          // 25^42
    ExtendedFloat160 { frac: 248920611114445668285762562151204969623, exp: 100 },                         // 25^49
    ExtendedFloat160 { frac: 176868732008334225927912486150152183216, exp: 133 },                         // 25^56
    ExtendedFloat160 { frac: 251345585423243599518503524095297312920, exp: 165 },                         // 25^63
    ExtendedFloat160 { frac: 178591779887855465971216179422709524914, exp: 198 },                         // 25^70
    ExtendedFloat160 { frac: 253794183731564922327402455583054354682, exp: 230 },                         // 25^77
    ExtendedFloat160 { frac: 180331613628627651967947866455016278082, exp: 263 },                         // 25^84
    ExtendedFloat160 { frac: 256266636183436918326986907537468991453, exp: 295 },                         // 25^91
    ExtendedFloat160 { frac: 182088396757817547443627082897044283139, exp: 328 },                         // 25^98
    ExtendedFloat160 { frac: 258763175164940474024358370140027266101, exp: 360 },                         // 25^105
    ExtendedFloat160 { frac: 183862294395666818064937594201088633455, exp: 393 },                         // 25^112
    ExtendedFloat160 { frac: 261284035326052074402891767876281837538, exp: 425 },                         // 25^119
    ExtendedFloat160 { frac: 185653473271011701515143789632334288014, exp: 458 },                         // 25^126
    ExtendedFloat160 { frac: 263829453602698580304979415177988198613, exp: 490 },                         // 25^133
    ExtendedFloat160 { frac: 187462101736953869352205554703508169192, exp: 523 },                         // 25^140
    ExtendedFloat160 { frac: 266399669239026862544798113253119949479, exp: 555 },                         // 25^147
    ExtendedFloat160 { frac: 189288349786683953755640255602884245064, exp: 588 },                         // 25^154
    ExtendedFloat160 { frac: 268994923809890385876486015494726082500, exp: 620 },                         // 25^161
    ExtendedFloat160 { frac: 191132389069459226417170338759437756337, exp: 653 },                         // 25^168
    ExtendedFloat160 { frac: 271615461243554856334256923502490730495, exp: 685 },                         // 25^175
    ExtendedFloat160 { frac: 192994392906736931318972184714148973580, exp: 718 },                         // 25^182
    ExtendedFloat160 { frac: 274261527844625066050770363850331497104, exp: 750 },                         // 25^189
    ExtendedFloat160 { frac: 194874536308464787773268059716493991903, exp: 783 },                         // 25^196
    ExtendedFloat160 { frac: 276933372317195090450451374005771742621, exp: 815 },                         // 25^203
    ExtendedFloat160 { frac: 196772995989530194869453349330805553038, exp: 848 },                         // 25^210
    ExtendedFloat160 { frac: 279631245788224013707368483964622716141, exp: 880 },                         // 25^217
];
const BASE25_BIAS: i32 = 259;

// BASE26

const BASE26_STEP: i32 = 7;
const BASE26_SMALL_POWERS: [ExtendedFloat160; BASE26_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 26^0
    ExtendedFloat160 { frac: 276479423123262501563991868538311671808, exp: -123 },                        // 26^1
    ExtendedFloat160 { frac: 224639531287650782520743393187378233344, exp: -118 },                        // 26^2
    ExtendedFloat160 { frac: 182519619171216260798104006964744814592, exp: -113 },                        // 26^3
    ExtendedFloat160 { frac: 296594381153226423796919011317710323712, exp: -109 },                        // 26^4
    ExtendedFloat160 { frac: 240982934686996469334996696695639638016, exp: -104 },                        // 26^5
    ExtendedFloat160 { frac: 195798634433184631334684816065207205888, exp: -99 },                         // 26^6
];
const BASE26_SMALL_INT_POWERS: [u128; BASE26_STEP as usize] = [1, 26, 676, 17576, 456976, 11881376, 308915776];
const BASE26_LARGE_POWERS: [ExtendedFloat160; 69] = [
    ExtendedFloat160 { frac: 255415379164208127122894269982044598460, exp: -1345 },                       // 26^-259
    ExtendedFloat160 { frac: 238819960676830402855710568862720060069, exp: -1312 },                       // 26^-252
    ExtendedFloat160 { frac: 223302816785416365261501121183664506010, exp: -1279 },                       // 26^-245
    ExtendedFloat160 { frac: 208793887424582006747864373745206480886, exp: -1246 },                       // 26^-238
    ExtendedFloat160 { frac: 195227664627991173689253365541181602096, exp: -1213 },                       // 26^-231
    ExtendedFloat160 { frac: 182542896759209079117924981191278218615, exp: -1180 },                       // 26^-224
    ExtendedFloat160 { frac: 170682311959929403775713752016508602388, exp: -1147 },                       // 26^-217
    ExtendedFloat160 { frac: 319184719133881733167774634228398630548, exp: -1115 },                       // 26^-210
    ExtendedFloat160 { frac: 298445936660656380797439373851559361096, exp: -1082 },                       // 26^-203
    ExtendedFloat160 { frac: 279054640682520272457995982566385389064, exp: -1049 },                       // 26^-196
    ExtendedFloat160 { frac: 260923279297292454826462160021044868443, exp: -1016 },                       // 26^-189
    ExtendedFloat160 { frac: 243969989220528359050303591757077787596, exp: -983 },                        // 26^-182
    ExtendedFloat160 { frac: 228118226171942658526451569535286060353, exp: -950 },                        // 26^-175
    ExtendedFloat160 { frac: 213296419277190995686832685182416132955, exp: -917 },                        // 26^-168
    ExtendedFloat160 { frac: 199437647924631042854519895051784617004, exp: -884 },                        // 26^-161
    ExtendedFloat160 { frac: 186479339618067826498814972661362929947, exp: -851 },                        // 26^-154
    ExtendedFloat160 { frac: 174362987461285330626576271532417342855, exp: -818 },                        // 26^-147
    ExtendedFloat160 { frac: 326067771997608226855765039505019953527, exp: -786 },                        // 26^-140
    ExtendedFloat160 { frac: 304881768440366443726402955195105316329, exp: -753 },                        // 26^-133
    ExtendedFloat160 { frac: 285072309225356538301642822964643005408, exp: -720 },                        // 26^-126
    ExtendedFloat160 { frac: 266549954439052068809893515776468694564, exp: -687 },                        // 26^-119
    ExtendedFloat160 { frac: 249231075457753005677502945704601914895, exp: -654 },                        // 26^-112
    ExtendedFloat160 { frac: 233037477363483566918311865718592557833, exp: -621 },                        // 26^-105
    ExtendedFloat160 { frac: 217896045893127676516686822732460831572, exp: -588 },                        // 26^-98
    ExtendedFloat160 { frac: 203738417326773735777703283427712394918, exp: -555 },                        // 26^-91
    ExtendedFloat160 { frac: 190500669824811165252338688092226229364, exp: -522 },                        // 26^-84
    ExtendedFloat160 { frac: 178123034820162511238845936350714491622, exp: -489 },                        // 26^-77
    ExtendedFloat160 { frac: 333099254325168111301464298292509503356, exp: -457 },                        // 26^-70
    ExtendedFloat160 { frac: 311456385593267303107050428484389238716, exp: -424 },                        // 26^-63
    ExtendedFloat160 { frac: 291219745668138391592456499213129019442, exp: -391 },                        // 26^-56
    ExtendedFloat160 { frac: 272297965910924348580716754684381379181, exp: -358 },                        // 26^-49
    ExtendedFloat160 { frac: 254605614290044560620651113294031241626, exp: -325 },                        // 26^-42
    ExtendedFloat160 { frac: 238062809654687405271268739306918462570, exp: -292 },                        // 26^-35
    ExtendedFloat160 { frac: 222594861070586991002412082254420837256, exp: -259 },                        // 26^-28
    ExtendedFloat160 { frac: 208131930589681357218231284385128680344, exp: -226 },                        // 26^-21
    ExtendedFloat160 { frac: 194608717931053648412097371491373800423, exp: -193 },                        // 26^-14
    ExtendedFloat160 { frac: 181964165649487446029462142596867446143, exp: -160 },                        // 26^-7
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 26^0
    ExtendedFloat160 { frac: 318172780953925025918862826105961709568, exp: -95 },                         // 26^7
    ExtendedFloat160 { frac: 297499748388299952530789444812804194304, exp: -62 },                         // 26^14
    ExtendedFloat160 { frac: 278169930267914565987548746187341299712, exp: -29 },                         // 26^21
    ExtendedFloat160 { frac: 260096052263752396381159488684908609536, exp: 4 },                           // 26^28
    ExtendedFloat160 { frac: 243196510629429753543644020787720315589, exp: 37 },                          // 26^35
    ExtendedFloat160 { frac: 227395003759435617782759026890197718352, exp: 70 },                          // 26^42
    ExtendedFloat160 { frac: 212620187686592486096128182198450396409, exp: 103 },                         // 26^49
    ExtendedFloat160 { frac: 198805353963305641500281734389939339052, exp: 136 },                         // 26^56
    ExtendedFloat160 { frac: 185888128472231349675004689621278542958, exp: 169 },                         // 26^63
    ExtendedFloat160 { frac: 173810189806491030350550655153103473999, exp: 202 },                         // 26^70
    ExtendedFloat160 { frac: 325034011895830307254166714675296871096, exp: 234 },                         // 26^77
    ExtendedFloat160 { frac: 303915176108832810280521264920190203131, exp: 267 },                         // 26^84
    ExtendedFloat160 { frac: 284168520489679119677023043643028417295, exp: 300 },                         // 26^91
    ExtendedFloat160 { frac: 265704888683728554332237168601547339173, exp: 333 },                         // 26^98
    ExtendedFloat160 { frac: 248440917202145588620033973783642939305, exp: 366 },                         // 26^105
    ExtendedFloat160 { frac: 232298659034884347081172590621418853088, exp: 399 },                         // 26^112
    ExtendedFloat160 { frac: 217205231719130933968079964556427917884, exp: 432 },                         // 26^119
    ExtendedFloat160 { frac: 203092488274228969742684708734280089507, exp: 465 },                         // 26^126
    ExtendedFloat160 { frac: 189896709517356115129460946793154219973, exp: 498 },                         // 26^133
    ExtendedFloat160 { frac: 177558316370753675479515454481338457101, exp: 531 },                         // 26^140
    ExtendedFloat160 { frac: 332043201723146894831623394706398020929, exp: 563 },                         // 26^147
    ExtendedFloat160 { frac: 310468949199606660363664220576467010752, exp: 596 },                         // 26^154
    ExtendedFloat160 { frac: 290296467197293861525395669447395056845, exp: 629 },                         // 26^161
    ExtendedFloat160 { frac: 271434676751037481783657313824389119896, exp: 662 },                         // 26^168
    ExtendedFloat160 { frac: 253798416681617203255579043607175584203, exp: 695 },                         // 26^175
    ExtendedFloat160 { frac: 237308059092157195001470401326248026898, exp: 728 },                         // 26^182
    ExtendedFloat160 { frac: 221889149847346996027560241559399617642, exp: 761 },                         // 26^189
    ExtendedFloat160 { frac: 207472072412249451654170398205728730819, exp: 794 },                         // 26^196
    ExtendedFloat160 { frac: 193991733532924439390933354984357040572, exp: 827 },                         // 26^203
    ExtendedFloat160 { frac: 181387269339713144970498252974494732833, exp: 860 },                         // 26^210
    ExtendedFloat160 { frac: 339203541092472346195720855153810480522, exp: 892 },                         // 26^217
];
const BASE26_BIAS: i32 = 259;

// BASE27

const BASE27_STEP: i32 = 6;
const BASE27_SMALL_POWERS: [ExtendedFloat160; BASE27_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 27^0
    ExtendedFloat160 { frac: 287113247089541828547222325020554428416, exp: -123 },                        // 27^1
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -118 },                        // 27^2
    ExtendedFloat160 { frac: 204399958133082024424731518496078299136, exp: -113 },                        // 27^3
    ExtendedFloat160 { frac: 172462464674787958108367218731066064896, exp: -108 },                        // 27^4
    ExtendedFloat160 { frac: 291030409138704679307869681608673984512, exp: -104 },                        // 27^5
];
const BASE27_SMALL_INT_POWERS: [u128; BASE27_STEP as usize] = [1, 27, 729, 19683, 531441, 14348907];
const BASE27_LARGE_POWERS: [ExtendedFloat160; 79] = [
    ExtendedFloat160 { frac: 200799661340208058549819756838299306108, exp: -1354 },                       // 27^-258
    ExtendedFloat160 { frac: 289804872078663861025541279674153816330, exp: -1326 },                       // 27^-252
    ExtendedFloat160 { frac: 209130989863161741765482805783015549525, exp: -1297 },                       // 27^-246
    ExtendedFloat160 { frac: 301829093537629265639465570217176944359, exp: -1269 },                       // 27^-240
    ExtendedFloat160 { frac: 217807991453958805640698687941213190524, exp: -1240 },                       // 27^-234
    ExtendedFloat160 { frac: 314352208961548438173441007673156326734, exp: -1212 },                       // 27^-228
    ExtendedFloat160 { frac: 226845008347394462887832488874694288292, exp: -1183 },                       // 27^-222
    ExtendedFloat160 { frac: 327394917835133689224334116987590197085, exp: -1155 },                       // 27^-216
    ExtendedFloat160 { frac: 236256977848340413085493360278597848651, exp: -1126 },                       // 27^-210
    ExtendedFloat160 { frac: 170489389240119998671196096475026672256, exp: -1097 },                       // 27^-204
    ExtendedFloat160 { frac: 246059457021648542224892012444309926233, exp: -1069 },                       // 27^-198
    ExtendedFloat160 { frac: 177563121844831037921653709250257605479, exp: -1040 },                       // 27^-192
    ExtendedFloat160 { frac: 256268648406457241006138264113659504394, exp: -1012 },                       // 27^-186
    ExtendedFloat160 { frac: 184930348919702200346046943747485274024, exp: -983 },                        // 27^-180
    ExtendedFloat160 { frac: 266901426797403574706768528236472000862, exp: -955 },                        // 27^-174
    ExtendedFloat160 { frac: 192603247770383575639211190527648274245, exp: -926 },                        // 27^-168
    ExtendedFloat160 { frac: 277975367137008028446553971650055283412, exp: -898 },                        // 27^-162
    ExtendedFloat160 { frac: 200594500948068090486693848039128919647, exp: -869 },                        // 27^-156
    ExtendedFloat160 { frac: 289508773565335211238455692680966173052, exp: -841 },                        // 27^-150
    ExtendedFloat160 { frac: 208917317212507950117664039252872831665, exp: -812 },                        // 27^-144
    ExtendedFloat160 { frac: 301520709674946766164053267333891939739, exp: -784 },                        // 27^-138
    ExtendedFloat160 { frac: 217585453364802351586979201161384846208, exp: -755 },                        // 27^-132
    ExtendedFloat160 { frac: 314031030021154964119856834958393443507, exp: -727 },                        // 27^-126
    ExtendedFloat160 { frac: 226613236986043931067161987739751269180, exp: -698 },                        // 27^-120
    ExtendedFloat160 { frac: 327060412939660347810097743318775450603, exp: -670 },                        // 27^-114
    ExtendedFloat160 { frac: 236015590119408703302029793810763336632, exp: -641 },                        // 27^-108
    ExtendedFloat160 { frac: 170315197362908885300398426895467760677, exp: -612 },                        // 27^-102
    ExtendedFloat160 { frac: 245808053934833671173174941698733239342, exp: -584 },                        // 27^-96
    ExtendedFloat160 { frac: 177381702616012906692133545122052956869, exp: -555 },                        // 27^-90
    ExtendedFloat160 { frac: 256006814417050404626793229969178591795, exp: -527 },                        // 27^-84
    ExtendedFloat160 { frac: 184741402471039290909022270993420155647, exp: -498 },                        // 27^-78
    ExtendedFloat160 { frac: 266628729119434395515123988465075762881, exp: -470 },                        // 27^-72
    ExtendedFloat160 { frac: 192406461791880080316008520325217417399, exp: -441 },                        // 27^-66
    ExtendedFloat160 { frac: 277691355027891684120101092281051616669, exp: -413 },                        // 27^-60
    ExtendedFloat160 { frac: 200389550171752283164939097875653100692, exp: -384 },                        // 27^-54
    ExtendedFloat160 { frac: 289212977580839036146652597763405686112, exp: -356 },                        // 27^-48
    ExtendedFloat160 { frac: 208703862874796048578293668364396201854, exp: -327 },                        // 27^-42
    ExtendedFloat160 { frac: 301212640893244858516269504216828222245, exp: -299 },                        // 27^-36
    ExtendedFloat160 { frac: 217363142646555453321168098187951653993, exp: -270 },                        // 27^-30
    ExtendedFloat160 { frac: 313710179234688236904530296665341569850, exp: -242 },                        // 27^-24
    ExtendedFloat160 { frac: 226381702429392491474935736226666160567, exp: -213 },                        // 27^-18
    ExtendedFloat160 { frac: 326726249813466247246220462666861782844, exp: -185 },                        // 27^-12
    ExtendedFloat160 { frac: 235774449020380624184618955567855082461, exp: -156 },                        // 27^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 27^0
    ExtendedFloat160 { frac: 245556907710782073166015043857318674432, exp: -99 },                         // 27^6
    ExtendedFloat160 { frac: 177200468746272961345336076752392290304, exp: -70 },                         // 27^12
    ExtendedFloat160 { frac: 255745247947835503562868389206950936576, exp: -42 },                         // 27^18
    ExtendedFloat160 { frac: 184552649072141716781794491390137475072, exp: -13 },                         // 27^24
    ExtendedFloat160 { frac: 266356310061270520809673995345359110719, exp: 15 },                          // 27^30
    ExtendedFloat160 { frac: 192209876872921446586714266254161951235, exp: 44 },                          // 27^36
    ExtendedFloat160 { frac: 277407633098725295421526662764935275289, exp: 72 },                          // 27^42
    ExtendedFloat160 { frac: 200184808797092622572327630249651738267, exp: 101 },                         // 27^48
    ExtendedFloat160 { frac: 288917483816076538023589582665008561757, exp: 129 },                         // 27^54
    ExtendedFloat160 { frac: 208490626626972031635281014538153149532, exp: 158 },                         // 27^60
    ExtendedFloat160 { frac: 300904886870600004067510516586852827477, exp: 186 },                         // 27^66
    ExtendedFloat160 { frac: 217141059066909427380630585083218539864, exp: 215 },                         // 27^72
    ExtendedFloat160 { frac: 313389656266867868879861721401276560157, exp: 243 },                         // 27^78
    ExtendedFloat160 { frac: 226150404435492799169987273137391228527, exp: 272 },                         // 27^84
    ExtendedFloat160 { frac: 326392428107359965184387801150473482685, exp: 300 },                         // 27^90
    ExtendedFloat160 { frac: 235533554299270254021060647605641184828, exp: 329 },                         // 27^96
    ExtendedFloat160 { frac: 339934694701922439619874702371784251126, exp: 357 },                         // 27^102
    ExtendedFloat160 { frac: 245306018087052741642305313258629505287, exp: 386 },                         // 27^108
    ExtendedFloat160 { frac: 177019420046226713314377865847118993119, exp: 415 },                         // 27^114
    ExtendedFloat160 { frac: 255483948725482657093998355855298189652, exp: 443 },                         // 27^120
    ExtendedFloat160 { frac: 184364088525767284952804747951506893851, exp: 472 },                         // 27^126
    ExtendedFloat160 { frac: 266084169338241408156670471179837543899, exp: 500 },                         // 27^132
    ExtendedFloat160 { frac: 192013492808081754945415747456910215687, exp: 529 },                         // 27^138
    ExtendedFloat160 { frac: 277124201053027125645172361985060059244, exp: 557 },                         // 27^144
    ExtendedFloat160 { frac: 199980276610139913759598726349951659975, exp: 586 },                         // 27^150
    ExtendedFloat160 { frac: 288622291962264730584478255384696488209, exp: 614 },                         // 27^156
    ExtendedFloat160 { frac: 208277608246209791806511248482402407710, exp: 643 },                         // 27^162
    ExtendedFloat160 { frac: 300597447285417578884462942447710218615, exp: 671 },                         // 27^168
    ExtendedFloat160 { frac: 216919202393792943992865658673403318648, exp: 700 },                         // 27^174
    ExtendedFloat160 { frac: 313069460782756034010893203297842312622, exp: 728 },                         // 27^180
    ExtendedFloat160 { frac: 225919342762644710883872352816958877073, exp: 757 },                         // 27^186
    ExtendedFloat160 { frac: 326058947472506854027112756453181563499, exp: 785 },                         // 27^192
    ExtendedFloat160 { frac: 235292905704349129354647627776916151748, exp: 814 },                         // 27^198
    ExtendedFloat160 { frac: 339587377705461640362820917278613293254, exp: 842 },                         // 27^204
    ExtendedFloat160 { frac: 245055384801472810432512717678228228007, exp: 871 },                         // 27^210
];
const BASE27_BIAS: i32 = 258;

// BASE28

const BASE28_STEP: i32 = 6;
const BASE28_SMALL_POWERS: [ExtendedFloat160; BASE28_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 28^0
    ExtendedFloat160 { frac: 297747071055821155530452781502797185024, exp: -123 },                        // 28^1
    ExtendedFloat160 { frac: 260528687173843511089146183814947536896, exp: -118 },                        // 28^2
    ExtendedFloat160 { frac: 227962601277113072203002910838079094784, exp: -113 },                        // 28^3
    ExtendedFloat160 { frac: 199467276117473938177627546983319207936, exp: -108 },                        // 28^4
    ExtendedFloat160 { frac: 174533866602789695905424103610404306944, exp: -103 },                        // 28^5
];
const BASE28_SMALL_INT_POWERS: [u128; BASE28_STEP as usize] = [1, 28, 784, 21952, 614656, 17210368];
const BASE28_LARGE_POWERS: [ExtendedFloat160; 78] = [
    ExtendedFloat160 { frac: 248507955182154661987298870611539230539, exp: -1339 },                       // 28^-252
    ExtendedFloat160 { frac: 223058413842966566682004736546150046796, exp: -1310 },                       // 28^-246
    ExtendedFloat160 { frac: 200215143815698040798730279921859793515, exp: -1281 },                       // 28^-240
    ExtendedFloat160 { frac: 179711238516029806533278035755362570551, exp: -1252 },                       // 28^-234
    ExtendedFloat160 { frac: 322614250185735942212427179391214768413, exp: -1224 },                       // 28^-228
    ExtendedFloat160 { frac: 289575530396283324168013345552040300667, exp: -1195 },                       // 28^-222
    ExtendedFloat160 { frac: 259920284847963995399800125815215983072, exp: -1166 },                       // 28^-216
    ExtendedFloat160 { frac: 233302014099717072256401710525774728336, exp: -1137 },                       // 28^-210
    ExtendedFloat160 { frac: 209409703497448836012980688794302909957, exp: -1108 },                       // 28^-204
    ExtendedFloat160 { frac: 187964189199610581269006081054389519147, exp: -1079 },                       // 28^-198
    ExtendedFloat160 { frac: 337429792711562885676838629607664070711, exp: -1051 },                       // 28^-192
    ExtendedFloat160 { frac: 302873822652608199592547515371033250847, exp: -1022 },                       // 28^-186
    ExtendedFloat160 { frac: 271856707468083969679745656096547629768, exp: -993 },                        // 28^-180
    ExtendedFloat160 { frac: 244016035285282981482333348801442963368, exp: -964 },                        // 28^-174
    ExtendedFloat160 { frac: 219026508600450572879143037057044702127, exp: -935 },                        // 28^-168
    ExtendedFloat160 { frac: 196596143419909740056291955312532441411, exp: -906 },                        // 28^-162
    ExtendedFloat160 { frac: 176462857644721687377034700397980722042, exp: -877 },                        // 28^-156
    ExtendedFloat160 { frac: 316782817673398770114452445482208770257, exp: -849 },                        // 28^-150
    ExtendedFloat160 { frac: 284341291171704802743493772571841275116, exp: -820 },                        // 28^-144
    ExtendedFloat160 { frac: 255222080727080523208383932871281083497, exp: -791 },                        // 28^-138
    ExtendedFloat160 { frac: 229084950069124576377435007616984162844, exp: -762 },                        // 28^-132
    ExtendedFloat160 { frac: 205624506307086466111975488366169508167, exp: -733 },                        // 28^-126
    ExtendedFloat160 { frac: 184566631641558957302915986868221103411, exp: -704 },                        // 28^-120
    ExtendedFloat160 { frac: 331330561004604641231243330979299081348, exp: -676 },                        // 28^-114
    ExtendedFloat160 { frac: 297399209378286220063892720385616742108, exp: -647 },                        // 28^-108
    ExtendedFloat160 { frac: 266942745850723232302069966588191406954, exp: -618 },                        // 28^-102
    ExtendedFloat160 { frac: 239605309345945263344621501916001364416, exp: -589 },                        // 28^-96
    ExtendedFloat160 { frac: 215067482294014848993159294730496555497, exp: -560 },                        // 28^-90
    ExtendedFloat160 { frac: 193042558474796699288911421705232156812, exp: -531 },                        // 28^-84
    ExtendedFloat160 { frac: 173273193069468359944466704194632400641, exp: -502 },                        // 28^-78
    ExtendedFloat160 { frac: 311056791556242112413125050076207081650, exp: -474 },                        // 28^-72
    ExtendedFloat160 { frac: 279201663740542055384000770694089408486, exp: -445 },                        // 28^-66
    ExtendedFloat160 { frac: 250608799266136415663698628779517553855, exp: -416 },                        // 28^-60
    ExtendedFloat160 { frac: 224944111822980370837543334787608800457, exp: -387 },                        // 28^-54
    ExtendedFloat160 { frac: 201907728667158642949418150287074186439, exp: -358 },                        // 28^-48
    ExtendedFloat160 { frac: 181230486831379296755646484093658378299, exp: -329 },                        // 28^-42
    ExtendedFloat160 { frac: 325341576312636457580643512071759255196, exp: -301 },                        // 28^-36
    ExtendedFloat160 { frac: 292023552792399342330208805478900181690, exp: -272 },                        // 28^-30
    ExtendedFloat160 { frac: 262117606830390855604604612394616145902, exp: -243 },                        // 28^-24
    ExtendedFloat160 { frac: 235274309738072614830216430996812400431, exp: -214 },                        // 28^-18
    ExtendedFloat160 { frac: 211180017596241035935669959185363632952, exp: -185 },                        // 28^-12
    ExtendedFloat160 { frac: 189553206559602063269009666658011215615, exp: -156 },                        // 28^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 28^0
    ExtendedFloat160 { frac: 305434266554881967834492181318207537152, exp: -99 },                         // 28^6
    ExtendedFloat160 { frac: 274154937941858738966065755004164112384, exp: -70 },                         // 28^12
    ExtendedFloat160 { frac: 246078905440687093968343124469641904128, exp: -41 },                         // 28^18
    ExtendedFloat160 { frac: 220878121537715117784741212850409701376, exp: -12 },                         // 28^24
    ExtendedFloat160 { frac: 198258133856129805696540977101424033792, exp: 17 },                          // 28^30
    ExtendedFloat160 { frac: 177954644699400447924746318168681611264, exp: 46 },                          // 28^36
    ExtendedFloat160 { frac: 319460845859371388212562249545703474176, exp: 74 },                          // 28^42
    ExtendedFloat160 { frac: 286745064197610355009611023687762970225, exp: 103 },                         // 28^48
    ExtendedFloat160 { frac: 257379684889104161503034418684704785797, exp: 132 },                         // 28^54
    ExtendedFloat160 { frac: 231021595363755916570056887236303965333, exp: 161 },                         // 28^60
    ExtendedFloat160 { frac: 207362820991138609531788808643065835705, exp: 190 },                         // 28^66
    ExtendedFloat160 { frac: 186126926626483659918254253754028720893, exp: 219 },                         // 28^72
    ExtendedFloat160 { frac: 334131573344103639308512797544978713750, exp: 247 },                         // 28^78
    ExtendedFloat160 { frac: 299913371828921883094842698046640019943, exp: 276 },                         // 28^84
    ExtendedFloat160 { frac: 269199434526831288331795872363961423540, exp: 305 },                         // 28^90
    ExtendedFloat160 { frac: 241630891972710985114650364591580944199, exp: 334 },                         // 28^96
    ExtendedFloat160 { frac: 216885626294688985349681859923056842835, exp: 363 },                         // 28^102
    ExtendedFloat160 { frac: 194674507506895938395726937393857685110, exp: 392 },                         // 28^108
    ExtendedFloat160 { frac: 174738015241079713869620349559402181972, exp: 421 },                         // 28^114
    ExtendedFloat160 { frac: 313686412889065357315780098042512623701, exp: 449 },                         // 28^120
    ExtendedFloat160 { frac: 281561987228284074576142980610683957411, exp: 478 },                         // 28^126
    ExtendedFloat160 { frac: 252727403529513497084111370284014563793, exp: 507 },                         // 28^132
    ExtendedFloat160 { frac: 226845751173734538409794758625366435361, exp: 536 },                         // 28^138
    ExtendedFloat160 { frac: 203614622343740041422835873088956724196, exp: 565 },                         // 28^144
    ExtendedFloat160 { frac: 182762578614186646525232068123189313087, exp: 594 },                         // 28^150
    ExtendedFloat160 { frac: 328091958791815868790390435525895683218, exp: 622 },                         // 28^156
    ExtendedFloat160 { frac: 294492270354449044397893099587906656150, exp: 651 },                         // 28^162
    ExtendedFloat160 { frac: 264333504599995236391965677440014878764, exp: 680 },                         // 28^168
    ExtendedFloat160 { frac: 237263278829077450304247817879793628477, exp: 709 },                         // 28^174
    ExtendedFloat160 { frac: 212965297630021155936008083539885258459, exp: 738 },                         // 28^180
    ExtendedFloat160 { frac: 191155657202715751455043144381591497592, exp: 767 },                         // 28^186
    ExtendedFloat160 { frac: 171579528154314464133715598246382584383, exp: 796 },                         // 28^192
    ExtendedFloat160 { frac: 308016356015425756696586706818979868623, exp: 824 },                         // 28^198
    ExtendedFloat160 { frac: 276472597266073797985814891590470600613, exp: 853 },                         // 28^204
    ExtendedFloat160 { frac: 248159214750338106225838746496027188809, exp: 882 },                         // 28^210
];
const BASE28_BIAS: i32 = 252;

// BASE29

const BASE29_STEP: i32 = 6;
const BASE29_SMALL_POWERS: [ExtendedFloat160; BASE29_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 29^0
    ExtendedFloat160 { frac: 308380895022100482513683237985039941632, exp: -123 },                        // 29^1
    ExtendedFloat160 { frac: 279470186113778562278025434423942447104, exp: -118 },                        // 29^2
    ExtendedFloat160 { frac: 253269856165611822064460549946697842688, exp: -113 },                        // 29^3
    ExtendedFloat160 { frac: 229525807150085713745917373389194919936, exp: -108 },                        // 29^4
    ExtendedFloat160 { frac: 208007762729765178082237619633957896192, exp: -103 },                        // 29^5
];
const BASE29_SMALL_INT_POWERS: [u128; BASE29_STEP as usize] = [1, 29, 841, 24389, 707281, 20511149];
const BASE29_LARGE_POWERS: [ExtendedFloat160; 78] = [
    ExtendedFloat160 { frac: 293939984653516579153014908772647553927, exp: -1352 },                       // 29^-252
    ExtendedFloat160 { frac: 325669269722502242662675110993270300309, exp: -1323 },                       // 29^-246
    ExtendedFloat160 { frac: 180411782640948456163108621039484300353, exp: -1293 },                       // 29^-240
    ExtendedFloat160 { frac: 199886291656678749778798199538954577713, exp: -1264 },                       // 29^-234
    ExtendedFloat160 { frac: 221462972137332420374365077239613292335, exp: -1235 },                       // 29^-228
    ExtendedFloat160 { frac: 245368742505570013761609603669658845854, exp: -1206 },                       // 29^-222
    ExtendedFloat160 { frac: 271855015841791437163383377269432435982, exp: -1177 },                       // 29^-216
    ExtendedFloat160 { frac: 301200343954417841961395964144914808415, exp: -1148 },                       // 29^-210
    ExtendedFloat160 { frac: 333713346863741229059418106803065448261, exp: -1119 },                       // 29^-204
    ExtendedFloat160 { frac: 184867979254122350694373980738345610830, exp: -1089 },                       // 29^-198
    ExtendedFloat160 { frac: 204823511403978168182734748793342650457, exp: -1060 },                       // 29^-192
    ExtendedFloat160 { frac: 226933138952023660037574950865203175761, exp: -1031 },                       // 29^-186
    ExtendedFloat160 { frac: 251429385238135556008900174982942596722, exp: -1002 },                       // 29^-180
    ExtendedFloat160 { frac: 278569873281449391084528542386060643447, exp: -973 },                        // 29^-174
    ExtendedFloat160 { frac: 308640035159552273337407148073177516540, exp: -944 },                        // 29^-168
    ExtendedFloat160 { frac: 170978056925499484035797671737173759761, exp: -914 },                        // 29^-162
    ExtendedFloat160 { frac: 189434244555519246827735852425496159000, exp: -885 },                        // 29^-156
    ExtendedFloat160 { frac: 209882681179121373738148351480252166035, exp: -856 },                        // 29^-150
    ExtendedFloat160 { frac: 232538419662693834656863635067192575556, exp: -827 },                        // 29^-144
    ExtendedFloat160 { frac: 257639726705579546348788818074016917383, exp: -798 },                        // 29^-138
    ExtendedFloat160 { frac: 285450588652016995453946272087193218261, exp: -769 },                        // 29^-132
    ExtendedFloat160 { frac: 316263487792383250937533050996202736450, exp: -740 },                        // 29^-126
    ExtendedFloat160 { frac: 175201239175822924661859658491633525197, exp: -710 },                        // 29^-120
    ExtendedFloat160 { frac: 194113297257345719179697640426306620561, exp: -681 },                        // 29^-114
    ExtendedFloat160 { frac: 215066813165088318898530949380694999931, exp: -652 },                        // 29^-108
    ExtendedFloat160 { frac: 238282151601732438626757341123945812037, exp: -623 },                        // 29^-102
    ExtendedFloat160 { frac: 264003464487880391063907148518553600721, exp: -594 },                        // 29^-96
    ExtendedFloat160 { frac: 292501258667905961095191488633763572010, exp: -565 },                        // 29^-90
    ExtendedFloat160 { frac: 324075240786231793266420435905585066661, exp: -536 },                        // 29^-84
    ExtendedFloat160 { frac: 179528734626566103050967344518404222838, exp: -506 },                        // 29^-78
    ExtendedFloat160 { frac: 198907923224385351394870557131224024014, exp: -477 },                        // 29^-72
    ExtendedFloat160 { frac: 220378993946205681004466278772642662300, exp: -448 },                        // 29^-66
    ExtendedFloat160 { frac: 244167754534112212293116837315364018708, exp: -419 },                        // 29^-60
    ExtendedFloat160 { frac: 270524387495766270724774864712621178351, exp: -390 },                        // 29^-54
    ExtendedFloat160 { frac: 299726081232954872012689080099923610541, exp: -361 },                        // 29^-48
    ExtendedFloat160 { frac: 332079945186715557451765374774991672241, exp: -332 },                        // 29^-42
    ExtendedFloat160 { frac: 183963119828570739334286542177638201257, exp: -302 },                        // 29^-36
    ExtendedFloat160 { frac: 203820977132676910337518249049241331980, exp: -273 },                        // 29^-30
    ExtendedFloat160 { frac: 225822386345870676314424417535913775896, exp: -244 },                        // 29^-24
    ExtendedFloat160 { frac: 250198732693485636863600410972168425001, exp: -215 },                        // 29^-18
    ExtendedFloat160 { frac: 277206378226597610088077970727216566225, exp: -186 },                        // 29^-12
    ExtendedFloat160 { frac: 307129357939822377568583266911848900855, exp: -157 },                        // 29^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 29^0
    ExtendedFloat160 { frac: 188507034973849692637027842793274343424, exp: -98 },                         // 29^6
    ExtendedFloat160 { frac: 208855384169162106382075974754544648192, exp: -69 },                         // 29^12
    ExtendedFloat160 { frac: 231400231309666912826787170353923751936, exp: -40 },                         // 29^18
    ExtendedFloat160 { frac: 256378676868573301455838684793773426688, exp: -11 },                         // 29^24
    ExtendedFloat160 { frac: 284053415075970164879962990673826094314, exp: 18 },                          // 29^30
    ExtendedFloat160 { frac: 314715496631115750914098978917481871688, exp: 47 },                          // 29^36
    ExtendedFloat160 { frac: 174343694817539847357322592625772163155, exp: 77 },                          // 29^42
    ExtendedFloat160 { frac: 193163185467532912371523116182520814756, exp: 106 },                         // 29^48
    ExtendedFloat160 { frac: 214014141773314894388674508400179209532, exp: 135 },                         // 29^54
    ExtendedFloat160 { frac: 237115849834993471687725998227739696222, exp: 164 },                         // 29^60
    ExtendedFloat160 { frac: 262711266541086358656385267533109039869, exp: 193 },                         // 29^66
    ExtendedFloat160 { frac: 291069574706415032602452081981147446763, exp: 222 },                         // 29^72
    ExtendedFloat160 { frac: 322489014023779685975823216414806216349, exp: 251 },                         // 29^78
    ExtendedFloat160 { frac: 178650008801035774670984867305268298071, exp: 281 },                         // 29^84
    ExtendedFloat160 { frac: 197934343538640677425270715935647505996, exp: 310 },                         // 29^90
    ExtendedFloat160 { frac: 219300321421789265371466533424126863110, exp: 339 },                         // 29^96
    ExtendedFloat160 { frac: 242972644948356100526463281459541324974, exp: 368 },                         // 29^102
    ExtendedFloat160 { frac: 269200272076455967884065094333741996545, exp: 397 },                         // 29^108
    ExtendedFloat160 { frac: 298259034474605889482552078350958780837, exp: 426 },                         // 29^114
    ExtendedFloat160 { frac: 330454538398307869859357548505383554011, exp: 455 },                         // 29^120
    ExtendedFloat160 { frac: 183062689350548673356118481539553806163, exp: 485 },                         // 29^126
    ExtendedFloat160 { frac: 202823349890643162760568057855283073783, exp: 514 },                         // 29^132
    ExtendedFloat160 { frac: 224717070457148091679782830215989826885, exp: 543 },                         // 29^138
    ExtendedFloat160 { frac: 248974103731497779640892318688061490897, exp: 572 },                         // 29^144
    ExtendedFloat160 { frac: 275849556968674066644388728598920812956, exp: 601 },                         // 29^150
    ExtendedFloat160 { frac: 305626074918518590345961283073160340970, exp: 630 },                         // 29^156
    ExtendedFloat160 { frac: 338616812354378462414859289927404969896, exp: 659 },                         // 29^162
    ExtendedFloat160 { frac: 187584363735341677353233362159059264795, exp: 689 },                         // 29^168
    ExtendedFloat160 { frac: 207833115392789061354400140358968381064, exp: 718 },                         // 29^174
    ExtendedFloat160 { frac: 230267613961761833964469003400183663813, exp: 747 },                         // 29^180
    ExtendedFloat160 { frac: 255123799397575000468261630468177681638, exp: 776 },                         // 29^186
    ExtendedFloat160 { frac: 282663080140582027146793786682106989268, exp: 805 },                         // 29^192
    ExtendedFloat160 { frac: 313175082306023963280707662359655742783, exp: 834 },                         // 29^198
    ExtendedFloat160 { frac: 173490347817277081445057491729890113081, exp: 864 },                         // 29^204
    ExtendedFloat160 { frac: 192217724118601260264099717636971309833, exp: 893 },                         // 29^210
];
const BASE29_BIAS: i32 = 252;

// BASE30

const BASE30_STEP: i32 = 6;
const BASE30_SMALL_POWERS: [ExtendedFloat160; BASE30_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 30^0
    ExtendedFloat160 { frac: 319014718988379809496913694467282698240, exp: -123 },                        // 30^1
    ExtendedFloat160 { frac: 299076299051606071403356588563077529600, exp: -118 },                        // 30^2
    ExtendedFloat160 { frac: 280384030360880691940646801777885184000, exp: -113 },                        // 30^3
    ExtendedFloat160 { frac: 262860028463325648694356376666767360000, exp: -108 },                        // 30^4
    ExtendedFloat160 { frac: 246431276684367795650959103125094400000, exp: -103 },                        // 30^5
];
const BASE30_SMALL_INT_POWERS: [u128; BASE30_STEP as usize] = [1, 30, 900, 27000, 810000, 24300000];
const BASE30_LARGE_POWERS: [ExtendedFloat160; 76] = [
    ExtendedFloat160 { frac: 318577830472690336137106460436953992313, exp: -1335 },                       // 30^-246
    ExtendedFloat160 { frac: 216293370737313530448778168911616746705, exp: -1305 },                       // 30^-240
    ExtendedFloat160 { frac: 293697914606894485088361958296166003399, exp: -1276 },                       // 30^-234
    ExtendedFloat160 { frac: 199401546035358756435491021348075025228, exp: -1246 },                       // 30^-228
    ExtendedFloat160 { frac: 270761041082025567891957153683056483774, exp: -1217 },                       // 30^-222
    ExtendedFloat160 { frac: 183828919146951883093674448351327494412, exp: -1187 },                       // 30^-216
    ExtendedFloat160 { frac: 249615464467793559236989678532104461317, exp: -1158 },                       // 30^-210
    ExtendedFloat160 { frac: 338944929832632662139396137837142035924, exp: -1129 },                       // 30^-204
    ExtendedFloat160 { frac: 230121290169646228383872457298707723793, exp: -1099 },                       // 30^-198
    ExtendedFloat160 { frac: 312474408249691315911417866816293319027, exp: -1070 },                       // 30^-192
    ExtendedFloat160 { frac: 212149548916169413644283660602828329029, exp: -1040 },                       // 30^-186
    ExtendedFloat160 { frac: 288071150257973936473359891361485890787, exp: -1011 },                       // 30^-180
    ExtendedFloat160 { frac: 195581343526079319127909243854250027224, exp: -981 },                        // 30^-174
    ExtendedFloat160 { frac: 265573709142416387133758214804806317848, exp: -952 },                        // 30^-168
    ExtendedFloat160 { frac: 180307062310000458937613050074040708794, exp: -922 },                        // 30^-162
    ExtendedFloat160 { frac: 244833246663194772909432488501026624276, exp: -893 },                        // 30^-156
    ExtendedFloat160 { frac: 332451307806128616353452734867573546427, exp: -864 },                        // 30^-150
    ExtendedFloat160 { frac: 225712548373888955751123878842649157480, exp: -834 },                        // 30^-144
    ExtendedFloat160 { frac: 306487917461553679299669913344627685478, exp: -805 },                        // 30^-138
    ExtendedFloat160 { frac: 208085115840167395965624010961720331305, exp: -775 },                        // 30^-132
    ExtendedFloat160 { frac: 282552185370553343852869987471204477403, exp: -746 },                        // 30^-126
    ExtendedFloat160 { frac: 191834329753307055373436045708607941890, exp: -716 },                        // 30^-120
    ExtendedFloat160 { frac: 260485757869036576463347072380734960806, exp: -687 },                        // 30^-114
    ExtendedFloat160 { frac: 176852678401887104140389725347567150767, exp: -657 },                        // 30^-108
    ExtendedFloat160 { frac: 240142648210703765821352805529490956868, exp: -628 },                        // 30^-102
    ExtendedFloat160 { frac: 326082092794781635112624979076905003855, exp: -599 },                        // 30^-96
    ExtendedFloat160 { frac: 221388270750079128888532155889145795079, exp: -569 },                        // 30^-90
    ExtendedFloat160 { frac: 300616117896153936087600778123712697276, exp: -540 },                        // 30^-84
    ExtendedFloat160 { frac: 204098550552778150334824777443135675335, exp: -510 },                        // 30^-78
    ExtendedFloat160 { frac: 277138954685954882938577351637270128950, exp: -481 },                        // 30^-72
    ExtendedFloat160 { frac: 188159102542382767109407940268116001044, exp: -451 },                        // 30^-66
    ExtendedFloat160 { frac: 255495283293346012426090218975127795266, exp: -422 },                        // 30^-60
    ExtendedFloat160 { frac: 173464474753336275982316368848893943009, exp: -392 },                        // 30^-54
    ExtendedFloat160 { frac: 235541913835671069456466721167496749113, exp: -363 },                        // 30^-48
    ExtendedFloat160 { frac: 319834901366763207379289417958082873567, exp: -334 },                        // 30^-42
    ExtendedFloat160 { frac: 217146839104937741700096042539405091508, exp: -304 },                        // 30^-36
    ExtendedFloat160 { frac: 294856812260112936979848919457246198708, exp: -275 },                        // 30^-30
    ExtendedFloat160 { frac: 200188361236473853754168248068850933442, exp: -245 },                        // 30^-24
    ExtendedFloat160 { frac: 271829432512427567293473804075629134624, exp: -216 },                        // 30^-18
    ExtendedFloat160 { frac: 184554286581985369843377175900278276895, exp: -186 },                        // 30^-12
    ExtendedFloat160 { frac: 250600417923680198594596164731872945757, exp: -157 },                        // 30^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 30^0
    ExtendedFloat160 { frac: 231029321891594808422774159179776000000, exp: -98 },                         // 30^6
    ExtendedFloat160 { frac: 313707395752840890251476992000000000000, exp: -69 },                         // 30^12
    ExtendedFloat160 { frac: 212986666247081951232000000000000000000, exp: -39 },                         // 30^18
    ExtendedFloat160 { frac: 289207845356544000000000000000000000000, exp: -10 },                         // 30^24
    ExtendedFloat160 { frac: 196353084654473304748535156250000000000, exp: 20 },                          // 30^30
    ExtendedFloat160 { frac: 266621631967110632288608940143603831529, exp: 49 },                          // 30^36
    ExtendedFloat160 { frac: 181018532909474941844489348460628831000, exp: 79 },                          // 30^42
    ExtendedFloat160 { frac: 245799330046413899594233809090775284541, exp: 108 },                         // 30^48
    ExtendedFloat160 { frac: 333763121820680299409099755486799743829, exp: 137 },                         // 30^54
    ExtendedFloat160 { frac: 226603183715861233202026897808422346833, exp: 167 },                         // 30^60
    ExtendedFloat160 { frac: 307697282971558792524556831461079215336, exp: 196 },                         // 30^66
    ExtendedFloat160 { frac: 208906195393080226844550976655564036201, exp: 226 },                         // 30^72
    ExtendedFloat160 { frac: 283667103278554017412807162817392837984, exp: 255 },                         // 30^78
    ExtendedFloat160 { frac: 192591285603182277357146536646298485706, exp: 285 },                         // 30^84
    ExtendedFloat160 { frac: 261513604232519641878753574965804062932, exp: 314 },                         // 30^90
    ExtendedFloat160 { frac: 177550518406095745907734479894927853604, exp: 344 },                         // 30^96
    ExtendedFloat160 { frac: 241090223040513319460187919146199534233, exp: 373 },                         // 30^102
    ExtendedFloat160 { frac: 327368774631124380764508606973252185538, exp: 402 },                         // 30^108
    ExtendedFloat160 { frac: 222261842997828194477899721342605392688, exp: 432 },                         // 30^114
    ExtendedFloat160 { frac: 301802313971178147521594347169174498450, exp: 461 },                         // 30^120
    ExtendedFloat160 { frac: 204903899584886496461222208185427085841, exp: 491 },                         // 30^126
    ExtendedFloat160 { frac: 278232512618195742220116760148063945729, exp: 520 },                         // 30^132
    ExtendedFloat160 { frac: 188901556375124208702207652989717774500, exp: 550 },                         // 30^138
    ExtendedFloat160 { frac: 256503437827277086943219190518379691262, exp: 579 },                         // 30^144
    ExtendedFloat160 { frac: 174148945301850322989380722761059920238, exp: 609 },                         // 30^150
    ExtendedFloat160 { frac: 236471334705220322048773145103478211637, exp: 638 },                         // 30^156
    ExtendedFloat160 { frac: 321096932515698698858834100478208840422, exp: 667 },                         // 30^162
    ExtendedFloat160 { frac: 218003675159015088778073023304915283497, exp: 697 },                         // 30^168
    ExtendedFloat160 { frac: 296020282788056879712669614682501632104, exp: 726 },                         // 30^174
    ExtendedFloat160 { frac: 200978281118435287206001718625001320433, exp: 756 },                         // 30^180
    ExtendedFloat160 { frac: 272902039690575235287053981567967613406, exp: 785 },                         // 30^186
    ExtendedFloat160 { frac: 185282516232160242762660936045505469826, exp: 815 },                         // 30^192
    ExtendedFloat160 { frac: 251589257890814574386886921482483833102, exp: 844 },                         // 30^198
    ExtendedFloat160 { frac: 170812540689859376035668482781137073721, exp: 874 },                         // 30^204
];
const BASE30_BIAS: i32 = 246;

// BASE31

const BASE31_STEP: i32 = 6;
const BASE31_SMALL_POWERS: [ExtendedFloat160; BASE31_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 31^0
    ExtendedFloat160 { frac: 329648542954659136480144150949525454848, exp: -123 },                        // 31^1
    ExtendedFloat160 { frac: 319347025987326038465139646232352784384, exp: -118 },                        // 31^2
    ExtendedFloat160 { frac: 309367431425222099763104032287591759872, exp: -113 },                        // 31^3
    ExtendedFloat160 { frac: 299699699193183909145507031278604517376, exp: -108 },                        // 31^4
    ExtendedFloat160 { frac: 290334083593396911984709936551148126208, exp: -103 },                        // 31^5
];
const BASE31_SMALL_INT_POWERS: [u128; BASE31_STEP as usize] = [1, 31, 961, 29791, 923521, 28629151];
const BASE31_LARGE_POWERS: [ExtendedFloat160; 76] = [
    ExtendedFloat160 { frac: 204831846702799730753125349709599684516, exp: -1346 },                       // 31^-246
    ExtendedFloat160 { frac: 338608432439644772651060763973150404254, exp: -1317 },                       // 31^-240
    ExtendedFloat160 { frac: 279877549230888994466722529922465951128, exp: -1287 },                       // 31^-234
    ExtendedFloat160 { frac: 231333407733284590292354000091386191754, exp: -1257 },                       // 31^-228
    ExtendedFloat160 { frac: 191209140142019782313183919746595976478, exp: -1227 },                       // 31^-222
    ExtendedFloat160 { frac: 316088675925298444140273004034743674741, exp: -1198 },                       // 31^-216
    ExtendedFloat160 { frac: 261263794643914746305034655542823358647, exp: -1168 },                       // 31^-210
    ExtendedFloat160 { frac: 215948167684024592485185159022754788335, exp: -1138 },                       // 31^-204
    ExtendedFloat160 { frac: 178492435929157837038046463019461592108, exp: -1108 },                       // 31^-198
    ExtendedFloat160 { frac: 295066635902569136770934561228011804883, exp: -1079 },                       // 31^-192
    ExtendedFloat160 { frac: 243887980937787207068127465341221873783, exp: -1049 },                       // 31^-186
    ExtendedFloat160 { frac: 201586150409601608540557644579125879351, exp: -1019 },                       // 31^-180
    ExtendedFloat160 { frac: 333242957530806000015262416296943861408, exp: -990 },                        // 31^-174
    ExtendedFloat160 { frac: 275442704070282164877682412643443854572, exp: -960 },                        // 31^-168
    ExtendedFloat160 { frac: 227667776650720372831068985136241988697, exp: -930 },                        // 31^-162
    ExtendedFloat160 { frac: 188179304658062925738530341045324738580, exp: -900 },                        // 31^-156
    ExtendedFloat160 { frac: 311080041475689584245114254221154550706, exp: -871 },                        // 31^-150
    ExtendedFloat160 { frac: 257123896754632869764136622554477725012, exp: -841 },                        // 31^-144
    ExtendedFloat160 { frac: 212526325921342359594316086083656739425, exp: -811 },                        // 31^-138
    ExtendedFloat160 { frac: 175664105047096554749814041961690625344, exp: -781 },                        // 31^-132
    ExtendedFloat160 { frac: 290391109602281582919653498207198252857, exp: -752 },                        // 31^-126
    ExtendedFloat160 { frac: 240023414326551697067636257878891518439, exp: -722 },                        // 31^-120
    ExtendedFloat160 { frac: 198391884324143423870507706735824676989, exp: -692 },                        // 31^-114
    ExtendedFloat160 { frac: 327962502126029666246881442269148110223, exp: -663 },                        // 31^-108
    ExtendedFloat160 { frac: 271078131968920728852328597367236335349, exp: -633 },                        // 31^-102
    ExtendedFloat160 { frac: 224060229920801636254083864004341099802, exp: -603 },                        // 31^-96
    ExtendedFloat160 { frac: 185197478924335716894196793983277050828, exp: -573 },                        // 31^-90
    ExtendedFloat160 { frac: 306150772156692797734166574102932652557, exp: -544 },                        // 31^-84
    ExtendedFloat160 { frac: 253049598289706899575201136976026000492, exp: -514 },                        // 31^-78
    ExtendedFloat160 { frac: 209158705508044155113481306826330986049, exp: -484 },                        // 31^-72
    ExtendedFloat160 { frac: 172880590941369685133685946960988620150, exp: -454 },                        // 31^-66
    ExtendedFloat160 { frac: 285789670113326703641484035227164931190, exp: -425 },                        // 31^-60
    ExtendedFloat160 { frac: 236220084333189890382264913587684321799, exp: -395 },                        // 31^-54
    ExtendedFloat160 { frac: 195248233500715771863466694882341429463, exp: -365 },                        // 31^-48
    ExtendedFloat160 { frac: 322765719034956327944217102842371688313, exp: -336 },                        // 31^-42
    ExtendedFloat160 { frac: 266782719403631527651786656524762917448, exp: -306 },                        // 31^-36
    ExtendedFloat160 { frac: 220509847158485190531810693342620280466, exp: -276 },                        // 31^-30
    ExtendedFloat160 { frac: 182262902194543738831736462131829646508, exp: -246 },                        // 31^-24
    ExtendedFloat160 { frac: 301299610375241462753468658335350302125, exp: -217 },                        // 31^-18
    ExtendedFloat160 { frac: 249039859782804352657429715423616325073, exp: -187 },                        // 31^-12
    ExtendedFloat160 { frac: 205844447270932349829274233837837318735, exp: -157 },                        // 31^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 31^0
    ExtendedFloat160 { frac: 281261143481103258485187751033924747264, exp: -98 },                         // 31^6
    ExtendedFloat160 { frac: 232477020622927971041480650211423420416, exp: -68 },                         // 31^12
    ExtendedFloat160 { frac: 192154395906963839472434931204572053504, exp: -38 },                         // 31^18
    ExtendedFloat160 { frac: 317651282414350176352409831205456445952, exp: -9 },                          // 31^24
    ExtendedFloat160 { frac: 262555370495753687560337482407159416999, exp: 21 },                          // 31^30
    ExtendedFloat160 { frac: 217015722562838525119352550468059930336, exp: 51 },                          // 31^36
    ExtendedFloat160 { frac: 179374825776921533841673613411506136043, exp: 81 },                          // 31^42
    ExtendedFloat160 { frac: 296525318465664137401900073705767585881, exp: 110 },                         // 31^48
    ExtendedFloat160 { frac: 245093658238625334625948305994309525378, exp: 140 },                         // 31^54
    ExtendedFloat160 { frac: 202582705650977753904076181059390370572, exp: 170 },                         // 31^60
    ExtendedFloat160 { frac: 334890367411416504232175148268475073388, exp: 199 },                         // 31^66
    ExtendedFloat160 { frac: 276804374353098300220638069068074602530, exp: 229 },                         // 31^72
    ExtendedFloat160 { frac: 228793268236589371413606590094648720070, exp: 259 },                         // 31^78
    ExtendedFloat160 { frac: 189109582219266748069833984779993889354, exp: 289 },                         // 31^84
    ExtendedFloat160 { frac: 312617887429839722873414506299686714932, exp: 318 },                         // 31^90
    ExtendedFloat160 { frac: 258395006731549634868442333657079980382, exp: 348 },                         // 31^96
    ExtendedFloat160 { frac: 213576964685944914612871149421565862652, exp: 378 },                         // 31^102
    ExtendedFloat160 { frac: 176532512843220607081571440296564925357, exp: 408 },                         // 31^108
    ExtendedFloat160 { frac: 291826678373921783044839874892818001863, exp: 437 },                         // 31^114
    ExtendedFloat160 { frac: 241209986871908117885122799335937441435, exp: 467 },                         // 31^120
    ExtendedFloat160 { frac: 199372648487594099824157002892036768477, exp: 497 },                         // 31^126
    ExtendedFloat160 { frac: 329583807705824908665978895106558273752, exp: 526 },                         // 31^132
    ExtendedFloat160 { frac: 272418225684124763666228455375305707749, exp: 556 },                         // 31^138
    ExtendedFloat160 { frac: 225167887346958248891898253497228074938, exp: 586 },                         // 31^144
    ExtendedFloat160 { frac: 186113015621359245893427982047443315204, exp: 616 },                         // 31^150
    ExtendedFloat160 { frac: 307664249923018427479642383336104189758, exp: 645 },                         // 31^156
    ExtendedFloat160 { frac: 254300566687046383525258086411660098107, exp: 675 },                         // 31^162
    ExtendedFloat160 { frac: 210192696205470376085866529648531617961, exp: 705 },                         // 31^168
    ExtendedFloat160 { frac: 173735238240724141814430167095519087450, exp: 735 },                         // 31^174
    ExtendedFloat160 { frac: 287202491346848457988114640516635531033, exp: 764 },                         // 31^180
    ExtendedFloat160 { frac: 237387854850570349221700427782261535085, exp: 794 },                         // 31^186
    ExtendedFloat160 { frac: 196213456806330839063706448987155946803, exp: 824 },                         // 31^192
    ExtendedFloat160 { frac: 324361333953874416226256763524449662322, exp: 853 },                         // 31^198
    ExtendedFloat160 { frac: 268101578446229760174200876130811505774, exp: 883 },                         // 31^204
];
const BASE31_BIAS: i32 = 246;

// BASE33

const BASE33_STEP: i32 = 6;
const BASE33_SMALL_POWERS: [ExtendedFloat160; BASE33_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 33^0
    ExtendedFloat160 { frac: 175458095443608895223302531957005484032, exp: -122 },                        // 33^1
    ExtendedFloat160 { frac: 180941160926221673199030736080661905408, exp: -117 },                        // 33^2
    ExtendedFloat160 { frac: 186595572205166100486500446583182589952, exp: -112 },                        // 33^3
    ExtendedFloat160 { frac: 192426683836577541126703585538907045888, exp: -107 },                        // 33^4
    ExtendedFloat160 { frac: 198440017706470589286913072586997891072, exp: -102 },                        // 33^5
];
const BASE33_SMALL_INT_POWERS: [u128; BASE33_STEP as usize] = [1, 33, 1089, 35937, 1185921, 39135393];
const BASE33_LARGE_POWERS: [ExtendedFloat160; 74] = [
    ExtendedFloat160 { frac: 216166617894615765056116969334520642044, exp: -1338 },                       // 33^-240
    ExtendedFloat160 { frac: 259999430717861725140739747707628323854, exp: -1308 },                       // 33^-234
    ExtendedFloat160 { frac: 312720366502509540124188550868577470271, exp: -1278 },                       // 33^-228
    ExtendedFloat160 { frac: 188065849520234218420139418711557486770, exp: -1247 },                       // 33^-222
    ExtendedFloat160 { frac: 226200577540468899968415353242546081790, exp: -1217 },                       // 33^-216
    ExtendedFloat160 { frac: 272068009211510778916881923005514361559, exp: -1187 },                       // 33^-210
    ExtendedFloat160 { frac: 327236130168906522048072439540872358087, exp: -1157 },                       // 33^-204
    ExtendedFloat160 { frac: 196795435814493025178219114364462772750, exp: -1126 },                       // 33^-198
    ExtendedFloat160 { frac: 236700290627603575579431376108202583063, exp: -1096 },                       // 33^-192
    ExtendedFloat160 { frac: 284696783496570703658001718555889571262, exp: -1066 },                       // 33^-186
    ExtendedFloat160 { frac: 171212841180688182412693440011648609110, exp: -1035 },                       // 33^-180
    ExtendedFloat160 { frac: 205930229524469868245369490973155168397, exp: -1005 },                       // 33^-174
    ExtendedFloat160 { frac: 247687376364759110421449812279701734754, exp: -975 },                        // 33^-168
    ExtendedFloat160 { frac: 297911756579516503503766398038992340964, exp: -945 },                        // 33^-162
    ExtendedFloat160 { frac: 179160149400574418614614090847719005026, exp: -914 },                        // 33^-156
    ExtendedFloat160 { frac: 215489039450973655880961058312920063458, exp: -884 },                        // 33^-150
    ExtendedFloat160 { frac: 259184457474862991304220337185522020620, exp: -854 },                        // 33^-144
    ExtendedFloat160 { frac: 311740138466868713435648102307209093287, exp: -824 },                        // 33^-138
    ExtendedFloat160 { frac: 187476353478425047886734580102095167043, exp: -793 },                        // 33^-132
    ExtendedFloat160 { frac: 225491547456297727220699982909970598320, exp: -763 },                        // 33^-126
    ExtendedFloat160 { frac: 271215206775862669602998925081523543654, exp: -733 },                        // 33^-120
    ExtendedFloat160 { frac: 326210402191372960931717472229356291097, exp: -703 },                        // 33^-114
    ExtendedFloat160 { frac: 196178576715647050727159488732861667611, exp: -672 },                        // 33^-108
    ExtendedFloat160 { frac: 235958348989735718000162987087766864735, exp: -642 },                        // 33^-102
    ExtendedFloat160 { frac: 283804395923732956473182174007605293342, exp: -612 },                        // 33^-96
    ExtendedFloat160 { frac: 170676171219393275768862099029406252664, exp: -581 },                        // 33^-90
    ExtendedFloat160 { frac: 205284737238107330416766226733647440005, exp: -551 },                        // 33^-84
    ExtendedFloat160 { frac: 246910995494199118965634147066154669324, exp: -521 },                        // 33^-78
    ExtendedFloat160 { frac: 296977946418022259559609845940558313794, exp: -491 },                        // 33^-72
    ExtendedFloat160 { frac: 178598568447993152099773363289076713958, exp: -460 },                        // 33^-66
    ExtendedFloat160 { frac: 214813584890064968045989415466080479007, exp: -430 },                        // 33^-60
    ExtendedFloat160 { frac: 258372038781252962127704032685119793391, exp: -400 },                        // 33^-54
    ExtendedFloat160 { frac: 310762982974959535779710715380461212334, exp: -370 },                        // 33^-48
    ExtendedFloat160 { frac: 186888705223357570139042022172334279947, exp: -339 },                        // 33^-42
    ExtendedFloat160 { frac: 224784739840635370899218738060963824868, exp: -309 },                        // 33^-36
    ExtendedFloat160 { frac: 270365077465939099085308357449561310080, exp: -279 },                        // 33^-30
    ExtendedFloat160 { frac: 325187889378018709806160016110270251585, exp: -249 },                        // 33^-24
    ExtendedFloat160 { frac: 195563651173574149793648095659417101115, exp: -218 },                        // 33^-18
    ExtendedFloat160 { frac: 235218732982278125085778976978720276897, exp: -188 },                        // 33^-12
    ExtendedFloat160 { frac: 282914805556997697053159518336509943216, exp: -158 },                        // 33^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 33^0
    ExtendedFloat160 { frac: 204641268259797795202129106105341575168, exp: -97 },                         // 33^6
    ExtendedFloat160 { frac: 246137048204490191229033862368809975808, exp: -67 },                         // 33^12
    ExtendedFloat160 { frac: 296047063302535604636168085099824283648, exp: -37 },                         // 33^18
    ExtendedFloat160 { frac: 178038747781766620403928207609098977344, exp: -6 },                          // 33^24
    ExtendedFloat160 { frac: 214140247554538205717741625292909706188, exp: 24 },                          // 33^30
    ExtendedFloat160 { frac: 257562166629747183314707096744136073023, exp: 54 },                          // 33^36
    ExtendedFloat160 { frac: 309788890395834268833431845588643035745, exp: 84 },                          // 33^42
    ExtendedFloat160 { frac: 186302898963108514077454910111746951397, exp: 115 },                         // 33^48
    ExtendedFloat160 { frac: 224080147727111315915564635257325506176, exp: 145 },                         // 33^54
    ExtendedFloat160 { frac: 269517612902775795573733407057825442248, exp: 175 },                         // 33^60
    ExtendedFloat160 { frac: 324168581650849479410768181980047458277, exp: 205 },                         // 33^66
    ExtendedFloat160 { frac: 194950653127503229444535496044715069916, exp: 236 },                         // 33^72
    ExtendedFloat160 { frac: 234481435315497306894208542281087212757, exp: 266 },                         // 33^78
    ExtendedFloat160 { frac: 282028003628468309677795696968404883305, exp: 296 },                         // 33^84
    ExtendedFloat160 { frac: 339215745262040382698593524434347927396, exp: 326 },                         // 33^90
    ExtendedFloat160 { frac: 203999816247443047277506123370388146319, exp: 357 },                         // 33^96
    ExtendedFloat160 { frac: 245365526867526093228395853689144006062, exp: 387 },                         // 33^102
    ExtendedFloat160 { frac: 295119098058174229856749108333084701428, exp: 417 },                         // 33^108
    ExtendedFloat160 { frac: 177480681884243207276442522127875068624, exp: 448 },                         // 33^114
    ExtendedFloat160 { frac: 213469020807909470054649976638820369879, exp: 478 },                         // 33^120
    ExtendedFloat160 { frac: 256754833038160188521579955085771893751, exp: 508 },                         // 33^126
    ExtendedFloat160 { frac: 308817851128733566186038756059234075639, exp: 538 },                         // 33^132
    ExtendedFloat160 { frac: 185718928923909504089695656878457160369, exp: 569 },                         // 33^138
    ExtendedFloat160 { frac: 223377764171191270264346565880758640128, exp: 599 },                         // 33^144
    ExtendedFloat160 { frac: 268672804733672512805858396506074869210, exp: 629 },                         // 33^150
    ExtendedFloat160 { frac: 323152468963460648455199258995519212913, exp: 659 },                         // 33^156
    ExtendedFloat160 { frac: 194339576535660800930047023345366203227, exp: 690 },                         // 33^162
    ExtendedFloat160 { frac: 233746448722509584063702393615930970798, exp: 720 },                         // 33^168
    ExtendedFloat160 { frac: 281143981397731320107155010945727058593, exp: 750 },                         // 33^174
    ExtendedFloat160 { frac: 338152466949356579395454697640846384307, exp: 780 },                         // 33^180
    ExtendedFloat160 { frac: 203360374878824300807265157901557614739, exp: 811 },                         // 33^186
    ExtendedFloat160 { frac: 244596423879111037469845073222731160599, exp: 841 },                         // 33^192
    ExtendedFloat160 { frac: 294194041538814672349638971928150296782, exp: 871 },                         // 33^198
];
const BASE33_BIAS: i32 = 240;

// BASE34

const BASE34_STEP: i32 = 6;
const BASE34_SMALL_POWERS: [ExtendedFloat160; BASE34_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 34^0
    ExtendedFloat160 { frac: 180775007426748558714917760198126862336, exp: -122 },                        // 34^1
    ExtendedFloat160 { frac: 192073445390920343634600120210509791232, exp: -117 },                        // 34^2
    ExtendedFloat160 { frac: 204078035727852865111762627723666653184, exp: -112 },                        // 34^3
    ExtendedFloat160 { frac: 216832912960843669181247791956395819008, exp: -107 },                        // 34^4
    ExtendedFloat160 { frac: 230384970020896398505075778953670557696, exp: -102 },                        // 34^5
];
const BASE34_SMALL_INT_POWERS: [u128; BASE34_STEP as usize] = [1, 34, 1156, 39304, 1336336, 45435424];
const BASE34_LARGE_POWERS: [ExtendedFloat160; 74] = [
    ExtendedFloat160 { frac: 171196178077006380843936555487576333355, exp: -1348 },                       // 34^-240
    ExtendedFloat160 { frac: 246301863245369722310352137071114920922, exp: -1318 },                       // 34^-234
    ExtendedFloat160 { frac: 177178627816250193201809052313908765038, exp: -1287 },                       // 34^-228
    ExtendedFloat160 { frac: 254908880844119689445018585029338745822, exp: -1257 },                       // 34^-222
    ExtendedFloat160 { frac: 183370134237042583472660416437030762483, exp: -1226 },                       // 34^-216
    ExtendedFloat160 { frac: 263816670637481076390123392064426194701, exp: -1196 },                       // 34^-210
    ExtendedFloat160 { frac: 189778002824260993825223275854263893366, exp: -1165 },                       // 34^-204
    ExtendedFloat160 { frac: 273035743108558333663040444930692235847, exp: -1135 },                       // 34^-198
    ExtendedFloat160 { frac: 196409794352921881357331916371145369426, exp: -1104 },                       // 34^-192
    ExtendedFloat160 { frac: 282576976029244796208883094031259474966, exp: -1074 },                       // 34^-186
    ExtendedFloat160 { frac: 203273333809293576669182005378991774735, exp: -1043 },                       // 34^-180
    ExtendedFloat160 { frac: 292451627295127901322196175360309190338, exp: -1013 },                       // 34^-174
    ExtendedFloat160 { frac: 210376719623757394611528557965027181599, exp: -982 },                        // 34^-168
    ExtendedFloat160 { frac: 302671348208910116654395983418902348443, exp: -952 },                        // 34^-162
    ExtendedFloat160 { frac: 217728333226311038599715596529740433091, exp: -921 },                        // 34^-156
    ExtendedFloat160 { frac: 313248197228018957952397977805898800839, exp: -891 },                        // 34^-150
    ExtendedFloat160 { frac: 225336848935989032116058614991615739830, exp: -860 },                        // 34^-144
    ExtendedFloat160 { frac: 324194654192627182361160566056046446660, exp: -830 },                        // 34^-138
    ExtendedFloat160 { frac: 233211244195868906543198111154344051285, exp: -799 },                        // 34^-132
    ExtendedFloat160 { frac: 335523635050871088900625460663893770435, exp: -769 },                        // 34^-126
    ExtendedFloat160 { frac: 241360810165739638162969982622013142483, exp: -738 },                        // 34^-120
    ExtendedFloat160 { frac: 173624253549320755964330470575918052959, exp: -707 },                        // 34^-114
    ExtendedFloat160 { frac: 249795162684930840147804514904540284970, exp: -677 },                        // 34^-108
    ExtendedFloat160 { frac: 179691552375964624115693619162436412029, exp: -646 },                        // 34^-102
    ExtendedFloat160 { frac: 258524253618237975606418771469177669493, exp: -616 },                        // 34^-96
    ExtendedFloat160 { frac: 185970872935167515117533503151908736546, exp: -585 },                        // 34^-90
    ExtendedFloat160 { frac: 267558382598330880564928534158522761469, exp: -555 },                        // 34^-84
    ExtendedFloat160 { frac: 192469624325502244069180532494789364726, exp: -524 },                        // 34^-78
    ExtendedFloat160 { frac: 276908209178500704519431941303594138118, exp: -494 },                        // 34^-72
    ExtendedFloat160 { frac: 199195474556460799929094324231727524305, exp: -463 },                        // 34^-66
    ExtendedFloat160 { frac: 286584765410084542875511011996966308779, exp: -433 },                        // 34^-60
    ExtendedFloat160 { frac: 206156359596095351859662099550265135432, exp: -402 },                        // 34^-54
    ExtendedFloat160 { frac: 296599468859408121472112670217716316867, exp: -372 },                        // 34^-48
    ExtendedFloat160 { frac: 213360492734829033350750838314127106094, exp: -341 },                        // 34^-42
    ExtendedFloat160 { frac: 306964136079605489713373754120772284157, exp: -311 },                        // 34^-36
    ExtendedFloat160 { frac: 220816374276485055706958449270700673524, exp: -280 },                        // 34^-30
    ExtendedFloat160 { frac: 317690996553211397504541477525504659745, exp: -250 },                        // 34^-24
    ExtendedFloat160 { frac: 228532801567968794073232940648013889325, exp: -219 },                        // 34^-18
    ExtendedFloat160 { frac: 328792707121977505492535302517672775182, exp: -189 },                        // 34^-12
    ExtendedFloat160 { frac: 236518879379437072732268269343858967136, exp: -158 },                        // 34^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 34^0
    ExtendedFloat160 { frac: 244784030647202423411643015138274967552, exp: -97 },                         // 34^6
    ExtendedFloat160 { frac: 176086766417174433233313223161344425984, exp: -66 },                         // 34^12
    ExtendedFloat160 { frac: 253338007592048088741599978367647481856, exp: -36 },                         // 34^18
    ExtendedFloat160 { frac: 182240117745863932172015090234506084352, exp: -5 },                          // 34^24
    ExtendedFloat160 { frac: 262190903226072497809728032921351003168, exp: 25 },                          // 34^30
    ExtendedFloat160 { frac: 188608497911442742195268251474898499613, exp: 56 },                          // 34^36
    ExtendedFloat160 { frac: 271353163261640374618023568003458637143, exp: 86 },                          // 34^42
    ExtendedFloat160 { frac: 195199421095732140407812372336079928061, exp: 117 },                         // 34^48
    ExtendedFloat160 { frac: 280835598436492094076350884217966821974, exp: 147 },                         // 34^54
    ExtendedFloat160 { frac: 202020664063606263361049018383687293681, exp: 178 },                         // 34^60
    ExtendedFloat160 { frac: 290649397269553933781951224423558743337, exp: 208 },                         // 34^66
    ExtendedFloat160 { frac: 209080275338955809947349984471742343690, exp: 239 },                         // 34^72
    ExtendedFloat160 { frac: 300806139262500062617960370620227418603, exp: 269 },                         // 34^78
    ExtendedFloat160 { frac: 216386584701305758176605078134277823932, exp: 300 },                         // 34^84
    ExtendedFloat160 { frac: 311317808562643058781928971959145202656, exp: 330 },                         // 34^90
    ExtendedFloat160 { frac: 223948213014292349032159641795245781843, exp: 361 },                         // 34^96
    ExtendedFloat160 { frac: 322196808103274080809106622508050735664, exp: 391 },                         // 34^102
    ExtendedFloat160 { frac: 231774082397596158130210248504430880773, exp: 422 },                         // 34^108
    ExtendedFloat160 { frac: 333455974238137167751959613429477643393, exp: 452 },                         // 34^114
    ExtendedFloat160 { frac: 239873426754333326759263874720553852658, exp: 483 },                         // 34^120
    ExtendedFloat160 { frac: 172554295943652591814138834633669982456, exp: 514 },                         // 34^126
    ExtendedFloat160 { frac: 248255802666326436218178945574152405546, exp: 544 },                         // 34^132
    ExtendedFloat160 { frac: 178584205106164167247426311765412935775, exp: 575 },                         // 34^138
    ExtendedFloat160 { frac: 256931100670110578075784008065054807112, exp: 605 },                         // 34^144
    ExtendedFloat160 { frac: 184824829419575343010846487395966556532, exp: 636 },                         // 34^150
    ExtendedFloat160 { frac: 265909556926979410208640982981191461086, exp: 666 },                         // 34^156
    ExtendedFloat160 { frac: 191283532323968216046404126970062690800, exp: 697 },                         // 34^162
    ExtendedFloat160 { frac: 275201765300840924300371814765015192837, exp: 727 },                         // 34^168
    ExtendedFloat160 { frac: 197967934574808286676526111499839216564, exp: 758 },                         // 34^174
    ExtendedFloat160 { frac: 284818689858133833493377548255268489047, exp: 788 },                         // 34^180
    ExtendedFloat160 { frac: 204885923234829474007514465274285458561, exp: 819 },                         // 34^186
    ExtendedFloat160 { frac: 294771677804553486829405243638525556429, exp: 849 },                         // 34^192
    ExtendedFloat160 { frac: 212045660980140516237478265085420658486, exp: 880 },                         // 34^198
];
const BASE34_BIAS: i32 = 240;

// BASE35

const BASE35_STEP: i32 = 6;
const BASE35_SMALL_POWERS: [ExtendedFloat160; BASE35_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 35^0
    ExtendedFloat160 { frac: 186091919409888222206532988439248240640, exp: -122 },                        // 35^1
    ExtendedFloat160 { frac: 203538036854565243038395456105427763200, exp: -117 },                        // 35^2
    ExtendedFloat160 { frac: 222619727809680734573245030115311616000, exp: -112 },                        // 35^3
    ExtendedFloat160 { frac: 243490327291838303439486751688622080000, exp: -107 },                        // 35^4
    ExtendedFloat160 { frac: 266317545475448144386938634659430400000, exp: -102 },                        // 35^5
];
const BASE35_SMALL_INT_POWERS: [u128; BASE35_STEP as usize] = [1, 35, 1225, 42875, 1500625, 52521875];
const BASE35_LARGE_POWERS: [ExtendedFloat160; 74] = [
    ExtendedFloat160 { frac: 333759367161047558555453110261936949560, exp: -1359 },                       // 35^-240
    ExtendedFloat160 { frac: 285701067966365984586376747532679438728, exp: -1328 },                       // 35^-234
    ExtendedFloat160 { frac: 244562724730167186648778719030613161790, exp: -1297 },                       // 35^-228
    ExtendedFloat160 { frac: 209347927024496598038547144935237941328, exp: -1266 },                       // 35^-222
    ExtendedFloat160 { frac: 179203738418473223738234323205585013488, exp: -1235 },                       // 35^-216
    ExtendedFloat160 { frac: 306800075067366652359189609572260396844, exp: -1205 },                       // 35^-210
    ExtendedFloat160 { frac: 262623667597658781475957209306702595701, exp: -1174 },                       // 35^-204
    ExtendedFloat160 { frac: 224808259148244964176155115402974869951, exp: -1143 },                       // 35^-198
    ExtendedFloat160 { frac: 192437923982884034851744022831878031683, exp: -1112 },                       // 35^-192
    ExtendedFloat160 { frac: 329457242604437106875016361913696434196, exp: -1082 },                       // 35^-186
    ExtendedFloat160 { frac: 282018410035885035080273518999343445370, exp: -1051 },                       // 35^-180
    ExtendedFloat160 { frac: 241410335891937127541744115564170038883, exp: -1020 },                       // 35^-174
    ExtendedFloat160 { frac: 206649453374488156391879990784284316065, exp: -989 },                        // 35^-168
    ExtendedFloat160 { frac: 176893820317164915550880607303262990177, exp: -958 },                        // 35^-162
    ExtendedFloat160 { frac: 302845453064861578599747045789577351117, exp: -928 },                        // 35^-156
    ExtendedFloat160 { frac: 259238475028744868618971960605253012775, exp: -897 },                        // 35^-150
    ExtendedFloat160 { frac: 221910503377561727015952290044860359194, exp: -866 },                        // 35^-144
    ExtendedFloat160 { frac: 189957418565367403947310019950147953923, exp: -835 },                        // 35^-138
    ExtendedFloat160 { frac: 325210571999150993462493010696415973233, exp: -805 },                        // 35^-132
    ExtendedFloat160 { frac: 278383221194439940371876875109029372360, exp: -774 },                        // 35^-126
    ExtendedFloat160 { frac: 238298581027663491439386725539968731964, exp: -743 },                        // 35^-120
    ExtendedFloat160 { frac: 203985762777473298078638682440591083627, exp: -712 },                        // 35^-114
    ExtendedFloat160 { frac: 174613676827044081042213382537397413381, exp: -681 },                        // 35^-108
    ExtendedFloat160 { frac: 298941805740831610317914779329181594758, exp: -651 },                        // 35^-102
    ExtendedFloat160 { frac: 255896917250379178747915691007066092001, exp: -620 },                        // 35^-96
    ExtendedFloat160 { frac: 219050099386294168656703063834650082992, exp: -589 },                        // 35^-90
    ExtendedFloat160 { frac: 187508886612326915552639247886565481996, exp: -558 },                        // 35^-84
    ExtendedFloat160 { frac: 321018640549353575448221161417409055741, exp: -528 },                        // 35^-78
    ExtendedFloat160 { frac: 274794889570264049674062490664049677426, exp: -497 },                        // 35^-72
    ExtendedFloat160 { frac: 235226936369523138128743288428339150410, exp: -466 },                        // 35^-66
    ExtendedFloat160 { frac: 201356406883409564599485887013038683702, exp: -435 },                        // 35^-60
    ExtendedFloat160 { frac: 172362924156377643998457025163202983813, exp: -404 },                        // 35^-54
    ExtendedFloat160 { frac: 295088476036816041433113722500648405442, exp: -374 },                        // 35^-48
    ExtendedFloat160 { frac: 252598431814515573631538400793788490787, exp: -343 },                        // 35^-42
    ExtendedFloat160 { frac: 216226565713775509518577464877951003673, exp: -312 },                        // 35^-36
    ExtendedFloat160 { frac: 185091915989032531210622402180350455682, exp: -281 },                        // 35^-30
    ExtendedFloat160 { frac: 316880742672855387564028424008808371736, exp: -251 },                        // 35^-24
    ExtendedFloat160 { frac: 271252811178556028546302550648486766486, exp: -220 },                        // 35^-18
    ExtendedFloat160 { frac: 232194884901007304160476988045002650981, exp: -189 },                        // 35^-12
    ExtendedFloat160 { frac: 198760943121441293094365080225590768425, exp: -158 },                        // 35^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 35^0
    ExtendedFloat160 { frac: 291284815363771407923214131658752000000, exp: -97 },                         // 35^6
    ExtendedFloat160 { frac: 249342463523006462279286784000000000000, exp: -66 },                         // 35^12
    ExtendedFloat160 { frac: 213439427105318371328000000000000000000, exp: -35 },                         // 35^18
    ExtendedFloat160 { frac: 182706099873129286194801330566406250000, exp: -4 },                          // 35^24
    ExtendedFloat160 { frac: 312796181882350172840701732490664710439, exp: 26 },                          // 35^30
    ExtendedFloat160 { frac: 267756389819817671992755772413469185401, exp: 57 },                          // 35^36
    ExtendedFloat160 { frac: 229201916269897841937750664282591869365, exp: 88 },                          // 35^42
    ExtendedFloat160 { frac: 196198934625406481835921498002300001382, exp: 119 },                         // 35^48
    ExtendedFloat160 { frac: 335896161555784742544555459149306885413, exp: 149 },                         // 35^54
    ExtendedFloat160 { frac: 287530183492901554386658282699185698307, exp: 180 },                         // 35^60
    ExtendedFloat160 { frac: 246128464334152340394473578644658105769, exp: 211 },                         // 35^66
    ExtendedFloat160 { frac: 210688214432220328999897334535237907298, exp: 242 },                         // 35^72
    ExtendedFloat160 { frac: 180351036686161310983217274120661055754, exp: 273 },                         // 35^78
    ExtendedFloat160 { frac: 308764270668182755993104788402294107778, exp: 303 },                         // 35^84
    ExtendedFloat160 { frac: 264305036979502132888830858069861274896, exp: 334 },                         // 35^90
    ExtendedFloat160 { frac: 226247526702365186299998179464172492810, exp: 365 },                         // 35^96
    ExtendedFloat160 { frac: 193669950160303864708173829536684422103, exp: 396 },                         // 35^102
    ExtendedFloat160 { frac: 331566493935091266452974274160244175815, exp: 426 },                         // 35^108
    ExtendedFloat160 { frac: 283823948447894890028153679891537227671, exp: 457 },                         // 35^114
    ExtendedFloat160 { frac: 242955893270455896832007072848205740580, exp: 488 },                         // 35^120
    ExtendedFloat160 { frac: 207972464612800582882401068589523181243, exp: 519 },                         // 35^126
    ExtendedFloat160 { frac: 178026330025978501184229413909552478799, exp: 550 },                         // 35^132
    ExtendedFloat160 { frac: 304784330382628027085206214062708598478, exp: 580 },                         // 35^138
    ExtendedFloat160 { frac: 260898171728955674614898641290082042559, exp: 611 },                         // 35^144
    ExtendedFloat160 { frac: 223331218918173589647399860035052277272, exp: 642 },                         // 35^150
    ExtendedFloat160 { frac: 191173564049707817673529979461919468076, exp: 673 },                         // 35^156
    ExtendedFloat160 { frac: 327292635293038256953486831534470016773, exp: 703 },                         // 35^162
    ExtendedFloat160 { frac: 280165486398550695538293601153410149006, exp: 734 },                         // 35^168
    ExtendedFloat160 { frac: 239824216327565160313568077123618791086, exp: 765 },                         // 35^174
    ExtendedFloat160 { frac: 205291720534454460326603808102056989101, exp: 796 },                         // 35^180
    ExtendedFloat160 { frac: 175731588600014458664842906141703880790, exp: 827 },                         // 35^186
    ExtendedFloat160 { frac: 300855691125661557416025651977765954086, exp: 857 },                         // 35^192
    ExtendedFloat160 { frac: 257535220627636274500677259708261311842, exp: 888 },                         // 35^198
];
const BASE35_BIAS: i32 = 240;

// BASE36

const BASE36_STEP: i32 = 6;
const BASE36_SMALL_POWERS: [ExtendedFloat160; BASE36_STEP as usize] = [
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 36^0
    ExtendedFloat160 { frac: 191408831393027885698148216680369618944, exp: -122 },                        // 36^1
    ExtendedFloat160 { frac: 215334935317156371410416743765415821312, exp: -117 },                        // 36^2
    ExtendedFloat160 { frac: 242251802231800917836718836736092798976, exp: -112 },                        // 36^3
    ExtendedFloat160 { frac: 272533277510776032566308691328104398848, exp: -107 },                        // 36^4
    ExtendedFloat160 { frac: 306599937199623036637097277744117448704, exp: -102 },                        // 36^5
];
const BASE36_SMALL_INT_POWERS: [u128; BASE36_STEP as usize] = [1, 36, 1296, 46656, 1679616, 60466176];
const BASE36_LARGE_POWERS: [ExtendedFloat160; 73] = [
    ExtendedFloat160 { frac: 200594500948068090486693848039128919647, exp: -1337 },                       // 36^-234
    ExtendedFloat160 { frac: 203331264836010463860204821197057175037, exp: -1306 },                       // 36^-228
    ExtendedFloat160 { frac: 206105367118290399407064648402758144682, exp: -1275 },                       // 36^-222
    ExtendedFloat160 { frac: 208917317212507950117664039252872831665, exp: -1244 },                       // 36^-216
    ExtendedFloat160 { frac: 211767631486382365261996259087726574961, exp: -1213 },                       // 36^-210
    ExtendedFloat160 { frac: 214656833352574406771088703014069554755, exp: -1182 },                       // 36^-204
    ExtendedFloat160 { frac: 217585453364802351586979201161384846208, exp: -1151 },                       // 36^-198
    ExtendedFloat160 { frac: 220554029315269330081435801781477974040, exp: -1120 },                       // 36^-192
    ExtendedFloat160 { frac: 223563106333419891448609016293621894840, exp: -1089 },                       // 36^-186
    ExtendedFloat160 { frac: 226613236986043931067161987739751269180, exp: -1058 },                       // 36^-180
    ExtendedFloat160 { frac: 229704981378746362247969882824709232796, exp: -1027 },                       // 36^-174
    ExtendedFloat160 { frac: 232838907258801165579649662968151663564, exp: -996 },                        // 36^-168
    ExtendedFloat160 { frac: 236015590119408703302029793810763336632, exp: -965 },                        // 36^-162
    ExtendedFloat160 { frac: 239235613305375443823879271798297650114, exp: -934 },                        // 36^-156
    ExtendedFloat160 { frac: 242499568120235502703106353919523432682, exp: -903 },                        // 36^-150
    ExtendedFloat160 { frac: 245808053934833671173174941698733239342, exp: -872 },                        // 36^-144
    ExtendedFloat160 { frac: 249161678297389871677290466673500998400, exp: -841 },                        // 36^-138
    ExtendedFloat160 { frac: 252561057045065251911260457800735557729, exp: -810 },                        // 36^-132
    ExtendedFloat160 { frac: 256006814417050404626793229969178591795, exp: -779 },                        // 36^-126
    ExtendedFloat160 { frac: 259499583169196479959998361450291137700, exp: -748 },                        // 36^-120
    ExtendedFloat160 { frac: 263040004690210240376322725691803307553, exp: -717 },                        // 36^-114
    ExtendedFloat160 { frac: 266628729119434395515123988465075762881, exp: -686 },                        // 36^-108
    ExtendedFloat160 { frac: 270266415466234845327287688358055741312, exp: -655 },                        // 36^-102
    ExtendedFloat160 { frac: 273953731731016754981191818978678705632, exp: -624 },                        // 36^-96
    ExtendedFloat160 { frac: 277691355027891684120101092281051616669, exp: -593 },                        // 36^-90
    ExtendedFloat160 { frac: 281479971709018296242657937208050445965, exp: -562 },                        // 36^-84
    ExtendedFloat160 { frac: 285320277490639481303204301467482637509, exp: -531 },                        // 36^-78
    ExtendedFloat160 { frac: 289212977580839036146652597763405686112, exp: -500 },                        // 36^-72
    ExtendedFloat160 { frac: 293158786809041363160730749526943361727, exp: -469 },                        // 36^-66
    ExtendedFloat160 { frac: 297158429757277967604640789526650060843, exp: -438 },                        // 36^-60
    ExtendedFloat160 { frac: 301212640893244858516269504216828222245, exp: -407 },                        // 36^-54
    ExtendedFloat160 { frac: 305322164705175286969651759320250334279, exp: -376 },                        // 36^-48
    ExtendedFloat160 { frac: 309487755838552588810803796052767101096, exp: -345 },                        // 36^-42
    ExtendedFloat160 { frac: 313710179234688236904530296665341569850, exp: -314 },                        // 36^-36
    ExtendedFloat160 { frac: 317990210271190550439415903835536554761, exp: -283 },                        // 36^-30
    ExtendedFloat160 { frac: 322328634904349856025836233807108654402, exp: -252 },                        // 36^-24
    ExtendedFloat160 { frac: 326726249813466247246220462666861782844, exp: -221 },                        // 36^-18
    ExtendedFloat160 { frac: 331183862547146446042592332649497399781, exp: -190 },                        // 36^-12
    ExtendedFloat160 { frac: 335702291671596630919115661345637412333, exp: -159 },                        // 36^-6
    ExtendedFloat160 { frac: 170141183460469231731687303715884105728, exp: -127 },                        // 36^0
    ExtendedFloat160 { frac: 172462464674787958108367218731066064896, exp: -96 },                         // 36^6
    ExtendedFloat160 { frac: 174815415743320440759790006808579407872, exp: -65 },                         // 36^12
    ExtendedFloat160 { frac: 177200468746272961345336076752392290304, exp: -34 },                         // 36^18
    ExtendedFloat160 { frac: 179618061658836457920697688990341398528, exp: -3 },                          // 36^24
    ExtendedFloat160 { frac: 182068638431613361423174859113151594496, exp: 28 },                          // 36^30
    ExtendedFloat160 { frac: 184552649072141716781794491390137475072, exp: 59 },                          // 36^36
    ExtendedFloat160 { frac: 187070549727531559196917812917453861026, exp: 90 },                          // 36^42
    ExtendedFloat160 { frac: 189622802768228720381105803326920695033, exp: 121 },                         // 36^48
    ExtendedFloat160 { frac: 192209876872921446586714266254161951235, exp: 152 },                         // 36^54
    ExtendedFloat160 { frac: 194832247114605420104007752175098574688, exp: 183 },                         // 36^60
    ExtendedFloat160 { frac: 197490395047822988635051696441052554380, exp: 214 },                         // 36^66
    ExtendedFloat160 { frac: 200184808797092622572327630249651738267, exp: 245 },                         // 36^72
    ExtendedFloat160 { frac: 202915983146544838776512848181734408257, exp: 276 },                         // 36^78
    ExtendedFloat160 { frac: 205684419630781050995309380627725821797, exp: 307 },                         // 36^84
    ExtendedFloat160 { frac: 208490626626972031635281014538153149532, exp: 338 },                         // 36^90
    ExtendedFloat160 { frac: 211335119448212897232599978727666183358, exp: 369 },                         // 36^96
    ExtendedFloat160 { frac: 214218420438151760708217936124820030498, exp: 400 },                         // 36^102
    ExtendedFloat160 { frac: 217141059066909427380630585083218539864, exp: 431 },                         // 36^108
    ExtendedFloat160 { frac: 220103572028307748788051030668660629356, exp: 462 },                         // 36^114
    ExtendedFloat160 { frac: 223106503338424488684979682521025988628, exp: 493 },                         // 36^120
    ExtendedFloat160 { frac: 226150404435492799169987273137391228527, exp: 524 },                         // 36^126
    ExtendedFloat160 { frac: 229235834281163651816744244429413474808, exp: 555 },                         // 36^132
    ExtendedFloat160 { frac: 232363359463149818964276081092475750857, exp: 586 },                         // 36^138
    ExtendedFloat160 { frac: 235533554299270254021060647605641184828, exp: 617 },                         // 36^144
    ExtendedFloat160 { frac: 238747000942913976797497733353022683918, exp: 648 },                         // 36^150
    ExtendedFloat160 { frac: 242004289489942830549695955106475311593, exp: 679 },                         // 36^156
    ExtendedFloat160 { frac: 245306018087052741642305313258629505287, exp: 710 },                         // 36^162
    ExtendedFloat160 { frac: 248652793041613380567795520750960012282, exp: 741 },                         // 36^168
    ExtendedFloat160 { frac: 252045228933006394543323172270604972624, exp: 772 },                         // 36^174
    ExtendedFloat160 { frac: 255483948725482657093998355855298189652, exp: 803 },                         // 36^180
    ExtendedFloat160 { frac: 258969583882559258973487053363982248701, exp: 834 },                         // 36^186
    ExtendedFloat160 { frac: 262502774482977247520692697766891651596, exp: 865 },                         // 36^192
    ExtendedFloat160 { frac: 266084169338241408156670471179837543899, exp: 896 },                         // 36^198
];
const BASE36_BIAS: i32 = 234;

// HIGH LEVEL
// ----------

pub(crate) const BASE3_POWERS: Powers<u128> = Powers {
    small: &BASE3_SMALL_POWERS,
    small_int: &BASE3_SMALL_INT_POWERS,
    large: &BASE3_LARGE_POWERS,
    step: BASE3_STEP,
    bias: BASE3_BIAS,
};

pub(crate) const BASE5_POWERS: Powers<u128> = Powers {
    small: &BASE5_SMALL_POWERS,
    small_int: &BASE5_SMALL_INT_POWERS,
    large: &BASE5_LARGE_POWERS,
    step: BASE5_STEP,
    bias: BASE5_BIAS,
};

pub(crate) const BASE6_POWERS: Powers<u128> = Powers {
    small: &BASE6_SMALL_POWERS,
    small_int: &BASE6_SMALL_INT_POWERS,
    large: &BASE6_LARGE_POWERS,
    step: BASE6_STEP,
    bias: BASE6_BIAS,
};

pub(crate) const BASE7_POWERS: Powers<u128> = Powers {
    small: &BASE7_SMALL_POWERS,
    small_int: &BASE7_SMALL_INT_POWERS,
    large: &BASE7_LARGE_POWERS,
    step: BASE7_STEP,
    bias: BASE7_BIAS,
};

pub(crate) const BASE9_POWERS: Powers<u128> = Powers {
    small: &BASE9_SMALL_POWERS,
    small_int: &BASE9_SMALL_INT_POWERS,
    large: &BASE9_LARGE_POWERS,
    step: BASE9_STEP,
    bias: BASE9_BIAS,
};

pub(crate) const BASE10_POWERS: Powers<u128> = Powers {
    small: &BASE10_SMALL_POWERS,
    small_int: &BASE10_SMALL_INT_POWERS,
    large: &BASE10_LARGE_POWERS,
    step: BASE10_STEP,
    bias: BASE10_BIAS,
};

pub(crate) const BASE11_POWERS: Powers<u128> = Powers {
    small: &BASE11_SMALL_POWERS,
    small_int: &BASE11_SMALL_INT_POWERS,
    large: &BASE11_LARGE_POWERS,
    step: BASE11_STEP,
    bias: BASE11_BIAS,
};

pub(crate) const BASE12_POWERS: Powers<u128> = Powers {
    small: &BASE12_SMALL_POWERS,
    small_int: &BASE12_SMALL_INT_POWERS,
    large: &BASE12_LARGE_POWERS,
    step: BASE12_STEP,
    bias: BASE12_BIAS,
};

pub(crate) const BASE13_POWERS: Powers<u128> = Powers {
    small: &BASE13_SMALL_POWERS,
    small_int: &BASE13_SMALL_INT_POWERS,
    large: &BASE13_LARGE_POWERS,
    step: BASE13_STEP,
    bias: BASE13_BIAS,
};

pub(crate) const BASE14_POWERS: Powers<u128> = Powers {
    small: &BASE14_SMALL_POWERS,
    small_int: &BASE14_SMALL_INT_POWERS,
    large: &BASE14_LARGE_POWERS,
    step: BASE14_STEP,
    bias: BASE14_BIAS,
};

pub(crate) const BASE15_POWERS: Powers<u128> = Powers {
    small: &BASE15_SMALL_POWERS,
    small_int: &BASE15_SMALL_INT_POWERS,
    large: &BASE15_LARGE_POWERS,
    step: BASE15_STEP,
    bias: BASE15_BIAS,
};

pub(crate) const BASE17_POWERS: Powers<u128> = Powers {
    small: &BASE17_SMALL_POWERS,
    small_int: &BASE17_SMALL_INT_POWERS,
    large: &BASE17_LARGE_POWERS,
    step: BASE17_STEP,
    bias: BASE17_BIAS,
};

pub(crate) const BASE18_POWERS: Powers<u128> = Powers {
    small: &BASE18_SMALL_POWERS,
    small_int: &BASE18_SMALL_INT_POWERS,
    large: &BASE18_LARGE_POWERS,
    step: BASE18_STEP,
    bias: BASE18_BIAS,
};

pub(crate) const BASE19_POWERS: Powers<u128> = Powers {
    small: &BASE19_SMALL_POWERS,
    small_int: &BASE19_SMALL_INT_POWERS,
    large: &BASE19_LARGE_POWERS,
    step: BASE19_STEP,
    bias: BASE19_BIAS,
};

pub(crate) const BASE20_POWERS: Powers<u128> = Powers {
    small: &BASE20_SMALL_POWERS,
    small_int: &BASE20_SMALL_INT_POWERS,
    large: &BASE20_LARGE_POWERS,
    step: BASE20_STEP,
    bias: BASE20_BIAS,
};

pub(crate) const BASE21_POWERS: Powers<u128> = Powers {
    small: &BASE21_SMALL_POWERS,
    small_int: &BASE21_SMALL_INT_POWERS,
    large: &BASE21_LARGE_POWERS,
    step: BASE21_STEP,
    bias: BASE21_BIAS,
};

pub(crate) const BASE22_POWERS: Powers<u128> = Powers {
    small: &BASE22_SMALL_POWERS,
    small_int: &BASE22_SMALL_INT_POWERS,
    large: &BASE22_LARGE_POWERS,
    step: BASE22_STEP,
    bias: BASE22_BIAS,
};

pub(crate) const BASE23_POWERS: Powers<u128> = Powers {
    small: &BASE23_SMALL_POWERS,
    small_int: &BASE23_SMALL_INT_POWERS,
    large: &BASE23_LARGE_POWERS,
    step: BASE23_STEP,
    bias: BASE23_BIAS,
};

pub(crate) const BASE24_POWERS: Powers<u128> = Powers {
    small: &BASE24_SMALL_POWERS,
    small_int: &BASE24_SMALL_INT_POWERS,
    large: &BASE24_LARGE_POWERS,
    step: BASE24_STEP,
    bias: BASE24_BIAS,
};

pub(crate) const BASE25_POWERS: Powers<u128> = Powers {
    small: &BASE25_SMALL_POWERS,
    small_int: &BASE25_SMALL_INT_POWERS,
    large: &BASE25_LARGE_POWERS,
    step: BASE25_STEP,
    bias: BASE25_BIAS,
};

pub(crate) const BASE26_POWERS: Powers<u128> = Powers {
    small: &BASE26_SMALL_POWERS,
    small_int: &BASE26_SMALL_INT_POWERS,
    large: &BASE26_LARGE_POWERS,
    step: BASE26_STEP,
    bias: BASE26_BIAS,
};

pub(crate) const BASE27_POWERS: Powers<u128> = Powers {
    small: &BASE27_SMALL_POWERS,
    small_int: &BASE27_SMALL_INT_POWERS,
    large: &BASE27_LARGE_POWERS,
    step: BASE27_STEP,
    bias: BASE27_BIAS,
};

pub(crate) const BASE28_POWERS: Powers<u128> = Powers {
    small: &BASE28_SMALL_POWERS,
    small_int: &BASE28_SMALL_INT_POWERS,
    large: &BASE28_LARGE_POWERS,
    step: BASE28_STEP,
    bias: BASE28_BIAS,
};

pub(crate) const BASE29_POWERS: Powers<u128> = Powers {
    small: &BASE29_SMALL_POWERS,
    small_int: &BASE29_SMALL_INT_POWERS,
    large: &BASE29_LARGE_POWERS,
    step: BASE29_STEP,
    bias: BASE29_BIAS,
};

pub(crate) const BASE30_POWERS: Powers<u128> = Powers {
    small: &BASE30_SMALL_POWERS,
    small_int: &BASE30_SMALL_INT_POWERS,
    large: &BASE30_LARGE_POWERS,
    step: BASE30_STEP,
    bias: BASE30_BIAS,
};

pub(crate) const BASE31_POWERS: Powers<u128> = Powers {
    small: &BASE31_SMALL_POWERS,
    small_int: &BASE31_SMALL_INT_POWERS,
    large: &BASE31_LARGE_POWERS,
    step: BASE31_STEP,
    bias: BASE31_BIAS,
};

pub(crate) const BASE33_POWERS: Powers<u128> = Powers {
    small: &BASE33_SMALL_POWERS,
    small_int: &BASE33_SMALL_INT_POWERS,
    large: &BASE33_LARGE_POWERS,
    step: BASE33_STEP,
    bias: BASE33_BIAS,
};

pub(crate) const BASE34_POWERS: Powers<u128> = Powers {
    small: &BASE34_SMALL_POWERS,
    small_int: &BASE34_SMALL_INT_POWERS,
    large: &BASE34_LARGE_POWERS,
    step: BASE34_STEP,
    bias: BASE34_BIAS,
};

pub(crate) const BASE35_POWERS: Powers<u128> = Powers {
    small: &BASE35_SMALL_POWERS,
    small_int: &BASE35_SMALL_INT_POWERS,
    large: &BASE35_LARGE_POWERS,
    step: BASE35_STEP,
    bias: BASE35_BIAS,
};

pub(crate) const BASE36_POWERS: Powers<u128> = Powers {
    small: &BASE36_SMALL_POWERS,
    small_int: &BASE36_SMALL_INT_POWERS,
    large: &BASE36_LARGE_POWERS,
    step: BASE36_STEP,
    bias: BASE36_BIAS,
};

/// Get powers from base.
pub(crate) fn get_powers(base: u32) -> &'static Powers<u128> {
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

    const POW2: [u32; 5] = [2, 4, 8, 16, 32];
    const BASEN: [u32; 30] = [
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
