extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rustc-env=LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu/libclang-6.0.so.1");
    // println!("cargo:rustc-link-lib=kvm");
    let bindings = bindgen::Builder::default()
        .header("resources/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}