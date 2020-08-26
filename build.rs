extern crate cc;
extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest not found");

    let lua_includes = "/usr/include/lua5.2/";

    cc::Build::new()
        .include(lua_includes)
        .include("c_drivers/include")
        .flag("-llua5.2")
        .file("c_drivers/lua_vm.c")
        .compile("lua_vm");

    cbindgen::generate(&crate_dir)
        .expect("cbindgen: Failed to generate binds")
        .write_to_file("c_drivers/include/gen_ecs.h");

    println!("cargo:rustc-flags=-L {} -l lua5.2", lua_includes);
}
