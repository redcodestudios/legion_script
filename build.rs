extern crate cc;
extern crate cbindgen;

use std::env;
use std::process::Command;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest not found");
    
    let output = Command::new("python3.7-config")
    .arg("--includes")
    .output()
    .expect("failed to execute process");
    
    
    let python_includes = String::from_utf8_lossy(&output.stdout);
    let formatted_python_includes = String::from(python_includes).trim_end().replace("-I", "");
    eprintln!("argghhhh {}",formatted_python_includes);
    let formatted_python_includes = String::from(formatted_python_includes.split_whitespace().next().unwrap());

    cc::Build::new()
        .include(formatted_python_includes)
        .flag("-lpython3.7")
        .file("c_drivers/python_vm.c")
        .compile("python_vm");

    // cbindgen::generate(&crate_dir)
        // .expect("cbindgen: Failed to generate binds")
        // .write_to_file("gen/ecs.h");

    println!("cargo:rustc-flags=-l python3.7m");
}
