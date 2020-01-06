extern crate rustc_version;

fn version_parse(version: &str) -> rustc_version::Version {
    rustc_version::Version::parse(version).unwrap()
}

fn main() {
    let version = rustc_version::version().unwrap();

    // VERSIONS
    // --------

    // We need at minimum version 1.32.0.
    assert!(version >= version_parse("1.32.0"));

    // TARGET
    // ------

    // We need to optimize limb size for performance.
    // Only have optimized 64-bit instructions on certain architectures.
    // See `lexical-core/src/atof/algorithm/math.rs` for detailed
    // instructions of architecture instruction support for 64-bit
    // mathematical operations.
    let limb_width_64 = cfg!(any(
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "powerpc64",
        target_arch = "x86_64"
    ));
    if limb_width_64 {
        println!("cargo:rustc-cfg=limb_width_64");
    } else {
        println!("cargo:rustc-cfg=limb_width_32");
    }
}
