#[cfg(feature = "flc-ram")]
fn compile_flc_asm() {
    use std::{env, fs::File, io::Write, path::PathBuf};

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let flc_asm_path = out.join("flc_asm.s");
    File::create(&flc_asm_path)
        .unwrap()
        .write_all(include_bytes!("flc_asm.s"))
        .unwrap();

    cc::Build::new().file(&flc_asm_path).compile("flc_asm");

    println!("cargo:rerun-if-changed=flc_asm.s");
}

fn main() {
    #[cfg(feature = "flc-ram")]
    compile_flc_asm();

    println!("cargo:rerun-if-changed=build.rs");
}
