#[macro_use]
extern crate lazy_static;

mod configuration;
mod error;
mod module_linter;
mod query;

use module_linter::{file_linter::FileLinter, ModuleLinter};
use std::{env, fs, io, process::ExitCode};
use tree_sitter::Language;

extern "C" {
    fn tree_sitter_go() -> Language;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let fix = args.len() == 3 && args[2] == "--fix";

    if !(args.len() == 2 || fix) {
        eprintln!("Usage: gold <path> [--fix]");
        return ExitCode::FAILURE;
    }

    match lint(&args[1], fix) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

pub fn lint(path: &str, fix: bool) -> io::Result<bool> {
    let module_linter = ModuleLinter::new(fix);

    if fs::metadata(path)?.is_dir() {
        Ok(module_linter.run(path))
    } else {
        let mut linter = FileLinter::new(
            path.to_string(),
            module_linter.fix,
            &module_linter.configuration,
        );
        Ok(linter.run())
    }
}
