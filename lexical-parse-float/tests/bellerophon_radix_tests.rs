#![cfg(feature = "radix")]

mod bellerophon;

use bellerophon::bellerophon_test;
use lexical_util::format::NumberFormatBuilder;

const BASE3: u128 = NumberFormatBuilder::from_radix(3);

#[test]
fn bellerophon_radix_test() {
    // Checking the exact rounding of the digits close to 5e-324.
    bellerophon_test::<f64, { BASE3 }>(5, -640, false, 4172256988254845, 10);
    bellerophon_test::<f64, { BASE3 }>(2, -679, false, 0, 0);
    bellerophon_test::<f64, { BASE3 }>(3, -679, false, 1, 0);
    bellerophon_test::<f64, { BASE3 }>(6, -680, false, 0, 0);
    bellerophon_test::<f64, { BASE3 }>(7, -680, false, 1, 0);
    bellerophon_test::<f64, { BASE3 }>(20, -681, false, 0, 0);
    bellerophon_test::<f64, { BASE3 }>(21, -681, false, 1, 0);
    bellerophon_test::<f64, { BASE3 }>(61, -682, false, 0, 0);
    bellerophon_test::<f64, { BASE3 }>(62, -682, false, 1, 0);
    bellerophon_test::<f64, { BASE3 }>(184, -683, false, 0, 0);
    bellerophon_test::<f64, { BASE3 }>(185, -683, false, 1, 0);
    bellerophon_test::<f64, { BASE3 }>(554, -684, false, 0, 0);
    bellerophon_test::<f64, { BASE3 }>(555, -684, false, 1, 0);
}
