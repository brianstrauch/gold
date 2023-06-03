use std::{env, path::PathBuf};

extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=lib/regexp.go");

    gobuild::Build::new()
        .file("lib/regexp.go")
        .compile("regexp");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header(out_dir.join("libregexp.h").to_str().unwrap())
        .allowlist_function("RegexpCompile")
        .allowlist_function("GoFree")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=regexp");
}
