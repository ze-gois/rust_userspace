use std::env;
use std::path::PathBuf;

use userspace_build::info;

fn main() {
    if env::var_os("CARGO_FEATURE_HOST_TESTS").is_some() {
        return;
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let linker_script = PathBuf::from(&manifest_dir).join("linker.ld");

    info!("cargo:rerun-if-changed={}\n", linker_script.display());
    info!("cargo:rerun-if-changed=./build.rs\n");
    info!("cargo:rerun-if-changed=./src/\n");
    info!("cargo:rerun-if-changed=./crates/\n");

    info!("cargo:rustc-link-arg=-static\n");
    info!("cargo:rustc-link-arg=--no-dynamic-linker\n");
    info!("cargo:rustc-link-arg=-n\n");
    info!("cargo:rustc-link-arg=-pie\n");
    info!("cargo:rustc-link-arg=--hash-style=sysv\n");
    info!("cargo:rustc-link-arg=-T{}\n", linker_script.display());

    // Compile assembly startup code
    cc::Build::new()
        .file("src/start.s")
        .flag("-fPIC")
        .compile("start");
}
