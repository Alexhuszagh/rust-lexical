extern crate rustc_version;

fn version_parse(version: &str) -> rustc_version::Version {
    rustc_version::Version::parse(version).unwrap()
}

fn main() {
    let version = rustc_version::version().unwrap();

    // VERSIONS
    // --------

    // We need at minimum version 1.20.0.
    assert!(version >= version_parse("1.20.0"));

    if version >= version_parse("1.26.0") {
        println!("cargo:rustc-cfg=has_i128");
    }
}
