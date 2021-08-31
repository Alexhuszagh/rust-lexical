pub fn black_box<T>(mut dummy: T) -> T {
    // SAFETY: the inline assembly is a no-op.
    unsafe { llvm_asm!("" : : "r"(&mut dummy) : "memory" : "volatile") };

    dummy
}
