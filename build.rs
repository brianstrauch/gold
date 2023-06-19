use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=lib/tree-sitter-go/src/parser.c");
    println!("cargo:rerun-if-changed=lib/std.go");

    cc::Build::new()
        .warnings(false)
        .include("lib/tree-sitter-go/src")
        .file("lib/tree-sitter-go/src/parser.c")
        .compile("tree-sitter-go");

    gobuild::Build::new().file("lib/std.go").compile("std");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header(out_dir.join("libstd.h").to_str().unwrap())
        .allowlist_function("GoFree")
        .allowlist_function("RegexpCompile")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=std");
}
