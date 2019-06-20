extern crate rustc_version;

fn version_parse(version: &str) -> rustc_version::Version {
    rustc_version::Version::parse(version).unwrap()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let version = rustc_version::version().unwrap();

    // VERSIONS
    // --------

    // We need at minimum version 1.20.0.
    assert!(version >= version_parse("1.20.0"));

    if version >= version_parse("1.28.0") {
        println!("cargo:rustc-cfg=has_range_bounds");
        println!("cargo:rustc-cfg=has_slice_index");
    }

    if version >= version_parse("1.27.0") {
        println!("cargo:rustc-cfg=has_full_range_inclusive");
    }

    if version >= version_parse("1.26.0") {
        println!("cargo:rustc-cfg=has_const_index");
        println!("cargo:rustc-cfg=has_i128");
        println!("cargo:rustc-cfg=has_ops_bound");
        println!("cargo:rustc-cfg=has_pointer_methods");
        println!("cargo:rustc-cfg=has_range_inclusive");
    }

    // TARGET
    // ------

    // We need to optimize limb size for performance.
    // Only have optimized 64-bit instructions on certain architectures.
    // See `lexical-core/src/atof/algorithm/math.rs` for detailed
    // instructions of architecture instruction support for 64-bit
    // mathematical operations.
    let has_i128 = version >= version_parse("1.26.0");
    let limb_width_64 = cfg!(any(
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "powerpc64",
        target_arch = "x86_64"
    ));
    if has_i128 && limb_width_64 {
        println!("cargo:rustc-cfg=limb_width_64");
    } else {
        println!("cargo:rustc-cfg=limb_width_32");
    }
}
