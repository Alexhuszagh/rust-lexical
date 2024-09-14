// Optimized black box using the nicer assembly syntax.
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
