#[macro_use]
extern crate lazy_static;

mod configuration;
mod error;
mod file_linter;
mod module_linter;

use module_linter::ModuleLinter;
use std::{env, io, process::ExitCode};
use walkdir::WalkDir;

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
    let go_mod_files = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with('.') && !e.path().join("..").join("go.mod").is_file())
                .unwrap_or(false)
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s == "go.mod")
                .unwrap_or(false)
        });

    let mut exit = true;

    for file in go_mod_files {
        let module_linter = ModuleLinter::new(fix);

        let mut dir = file.path().to_path_buf();
        dir.pop();

        exit &= module_linter.run(dir.to_str().unwrap());
    }

    Ok(exit)
}
