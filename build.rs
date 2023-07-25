fn main() {
    println!("cargo:rerun-if-changed=lib/tree-sitter-go/src/parser.c");
    println!("cargo:rerun-if-changed=lib/std.go");

    cc::Build::new()
        .warnings(false)
        .include("lib/tree-sitter-go/src")
        .file("lib/tree-sitter-go/src/parser.c")
        .compile("tree-sitter-go");
}
