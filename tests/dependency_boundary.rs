use std::{fs, path::Path};
#[test]
fn only_ethos_authority_and_strict_projection_remain() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(root.join("ethos/interface.ethos").is_file());
    assert!(!root.join("schema").exists());
    let manifest = fs::read_to_string(root.join("Cargo.toml")).expect("manifest");
    let build = fs::read_to_string(root.join("build.rs")).expect("build");
    for retired in [
        "branch =",
        "nota-text",
        concat!("CargoSchema", "Metadata"),
        concat!("Generation", "Driver"),
        concat!("Generation", "Plan"),
        concat!("Module", "Emission"),
        concat!("ContractCrate", "Build"),
        concat!("Dependency", "Schema"),
    ] {
        assert!(!manifest.contains(retired), "manifest contains {retired}");
        assert!(!build.contains(retired), "build contains {retired}");
    }
    assert!(manifest.contains("664335240a40728826cfaa09e3100cd867031912"));
    assert!(manifest.contains("8aa0bcaeb29fe9e461a11706a469638d2fd109ac"));
}
#[test]
fn crate_root_has_no_readable_wire_shadow() {
    let lib = include_str!("../src/lib.rs");
    assert!(!lib.contains("pub use schema::lib::{"));
    assert!(!lib.contains("type Input"));
    assert!(!lib.contains("type Output"));
}
