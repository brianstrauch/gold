use gold::lint;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: gold [packages]");
        return ExitCode::FAILURE;
    }

    if lint(String::from(&args[1])) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
