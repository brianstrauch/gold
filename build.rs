fn main() {
    println!("cargo:rerun-if-changed=lib/tree-sitter-go/src/parser.c");

    cc::Build::new()
        .warnings(false)
        .include("lib/tree-sitter-go/src")
        .file("lib/tree-sitter-go/src/parser.c")
        .compile("tree-sitter-go");
}
