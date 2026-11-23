//! Hands the build's optimization level to the code as `XTASK_OPT_LEVEL`, so that `cargo xtask
//! synthetic` can tell the dev profile's unoptimized build from `report-check`'s optimized one and
//! fly the census in the second (ADR-372).

#![expect(
    clippy::disallowed_methods,
    reason = "a build script reads what cargo hands it in its environment"
)]

fn main() {
    let level = std::env::var("OPT_LEVEL").unwrap_or_default();
    println!("cargo:rustc-env=XTASK_OPT_LEVEL={level}");
    println!("cargo:rerun-if-changed=build.rs");
}
