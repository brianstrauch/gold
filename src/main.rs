use gold::Linter;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: gold [packages]");
        return ExitCode::FAILURE;
    }

    if Linter::new(String::from(&args[1])).run() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
