#[macro_use]
extern crate lazy_static;
extern crate simple_error;

mod configuration;
mod error;
mod file_linter;
mod module_linter;

use module_linter::ModuleLinter;
use simple_error::{bail, SimpleError};
use std::{env, process::ExitCode};
use walkdir::WalkDir;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let result;
    if args.len() == 1 {
        result = lint(".", false);
    } else if args.len() == 2 {
        if args[1] == "--fix" {
            result = lint(".", true);
        } else {
            result = lint(&args[1], false);
        }
    } else if args.len() == 3 && args[2] == "--fix" {
        result = lint(&args[1], true);
    } else {
        eprintln!("Usage: gold [path] [--fix]");
        return ExitCode::FAILURE;
    }

    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

pub fn lint(path: &str, fix: bool) -> Result<bool, SimpleError> {
    let mut go_mod_files = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| {
                    !(e.path().join("..").join("go.mod").is_file()
                        || s.starts_with('.') && s != "." && s != "..")
                })
                .unwrap_or(false)
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s == "go.mod")
                .unwrap_or(false)
        })
        .peekable();

    if go_mod_files.peek().is_none() {
        bail!("no go.mod file found in {}", path);
    }

    let mut exit = true;

    for file in go_mod_files {
        let module_linter = ModuleLinter::new(fix);

        let mut dir = file.path().to_path_buf();
        dir.pop();

        exit &= module_linter.run(dir.to_str().unwrap());
    }

    Ok(exit)
}
