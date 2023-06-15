use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=tree-sitter-go/src/parser.c");
    println!("cargo:rerun-if-changed=lib/go.go");

    cc::Build::new()
        .warnings(false)
        .include("tree-sitter-go/src")
        .file("tree-sitter-go/src/parser.c")
        .compile("tree-sitter-go");

    gobuild::Build::new().file("lib/go.go").compile("go");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header(out_dir.join("libgo.h").to_str().unwrap())
        .allowlist_function("GoFree")
        .allowlist_function("HtmlTemplateNewParse")
        .allowlist_function("RegexpCompile")
        .allowlist_function("TextTemplateNewParse")
        .allowlist_function("TimeParse")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=go");
}
