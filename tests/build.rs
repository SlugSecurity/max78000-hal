use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    // Get the out directory.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Add out directory to the linker search path.
    println!("cargo:rustc-link-search={}", out.display());

    // Put the memory.x linker script somewhere the linker can find it.
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // Put the link.x linker script somewhere the linker can find it.
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();

    let flc_asm_path = out.join("flc_asm.s");
    File::create(&flc_asm_path)
        .unwrap()
        .write_all(include_bytes!("flc_asm.s"))
        .unwrap();

    cc::Build::new().file(&flc_asm_path).compile("flc_asm");

    println!("cargo:rustc-link-arg=--nmagic");

    // Only re-run the build script when this file, memory.x, or link.x is changed.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link.x");
}
