fn main() -> Result<(), std::env::VarError> {
    // TARGET
    // ------

    // We need to optimize limb size for performance.
    // Only have optimized 64-bit instructions on certain architectures.
    // See `lexical-core/src/atof/algorithm/math.rs` for detailed
    // instructions of architecture instruction support for 64-bit
    // mathematical operations.
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")?;
    let limb_width_64 = ["aarch64", "mips64", "powerpc64", "x86_64"].contains(&&*target_arch);
    if limb_width_64 {
        println!("cargo:rustc-cfg=limb_width_64");
    } else {
        println!("cargo:rustc-cfg=limb_width_32");
    }
    Ok(())
}
