extern crate cbindgen;
extern crate cc;

use std::env;
use std::process::Command;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest not found");
    
    let output = Command::new("python3.7-config")
    .arg("--includes")
    .output()
    .expect("failed to execute process");
    
    let ldflags = Command::new("python3.7-config")
    .arg("--ldflags")
    .output()
    .expect("failed to execute process");
    
    eprintln!("{}", String::from_utf8_lossy(&ldflags.stdout));
    
    let python_includes = String::from_utf8_lossy(&output.stdout);
    let formatted_python_includes = String::from(python_includes).trim_end().replace("-I", "");
    eprintln!("{}",formatted_python_includes);
    let formatted_python_includes = String::from(formatted_python_includes.split_whitespace().next().unwrap());

    cc::Build::new()
        .include(formatted_python_includes)
        .flag("-lpython3.7m")
        .file("c_drivers/vm.c")
        .compile("python");

    println!("{}", format!("cargo:rustc-flags={}", String::from_utf8_lossy(&ldflags.stdout)));
}
