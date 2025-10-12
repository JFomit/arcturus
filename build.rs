use nasm_rs::*;

fn main() {
    Build::new()
        .debug(true)
        .flag("-felf32")
        .file("src/interrupts.nasm")
        .compile("dbrt")
        .unwrap();
    
    println!("cargo::rerun-if-changed=src/interrupts.nasm");
    println!("cargo::rerun-if-changed=build.rs");
}
