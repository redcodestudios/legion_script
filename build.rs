extern crate cc;
extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest not found");

    let python_includes = "/usr/include/python3.8/";

    cc::Build::new()
        .include(python_includes)
        .flag("-lpython3.8")
        .file("c_drivers/python_vm.c")
        .compile("python_vm");

    cbindgen::generate(&crate_dir)
        .expect("cbindgen: Failed to generate binds")
        .write_to_file("gen/ecs.h");

    println!("cargo:rustc-flags=-L {} -l python3.8", python_includes);
}
