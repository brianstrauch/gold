fn main() {
    println!("cargo:rerun-if-changed=lib");

    for dir in ["tree-sitter-go", "tree-sitter-go-mod"] {
        cc::Build::new()
            .warnings(false)
            .include(format!("lib/{dir}/src"))
            .file(format!("lib/{dir}/src/parser.c"))
            .compile(dir);
    }
}
