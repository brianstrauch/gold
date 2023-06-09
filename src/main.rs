#[macro_use]
extern crate lazy_static;

mod configuration;
mod error;
mod file_linter;
mod go;
mod module_linter;
mod query;
mod rules;

use file_linter::FileLinter;
use module_linter::ModuleLinter;
use std::{env, fs, io, process::ExitCode};
use tree_sitter::Language;

extern "C" {
    fn tree_sitter_go() -> Language;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: gold [packages]");
        return ExitCode::FAILURE;
    }

    match lint(&args[1]) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

pub fn lint(path: &str) -> io::Result<bool> {
    let module_linter = ModuleLinter::new();

    if fs::metadata(path)?.is_dir() {
        Ok(module_linter.run(path))
    } else {
        let mut linter = FileLinter::new(&module_linter, path.to_string());
        Ok(linter.run())
    }
}
