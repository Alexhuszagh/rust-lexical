// Optimized black box using the nicer assembly syntax.
#[cfg(feature = "asm")]
pub fn black_box(mut dummy: f64) -> f64 {
    // THe `asm!` macro was stabilized in 1.59.0.
    use core::arch::asm;

    unsafe {
        asm!(
            "/* {dummy} */",
            dummy = inout(reg) dummy
        );
        dummy
    }
}

// Optimized black box using the nicer assembly syntax.
#[cfg(not(feature = "asm"))]
pub fn black_box(dummy: f64) -> f64 {
    unsafe {
        let x = core::ptr::read_volatile(&dummy);
        core::mem::forget(dummy);
        x
    }
}
