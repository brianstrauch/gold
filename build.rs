use std::path::PathBuf;

fn main() {
    for lib in ["tree-sitter-go", "tree-sitter-go-mod"] {
        let src: PathBuf = ["lib", lib, "src"].iter().collect();

        cc::Build::new()
            .warnings(false)
            .include(&src)
            .file(src.join("parser.c"))
            .compile(lib);
    }
}
