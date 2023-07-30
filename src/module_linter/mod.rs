use crate::{
    configuration::{golangci::GolangciConfiguration, Configuration},
    file_linter::FileLinter,
};
use std::{collections::HashSet, fs::File, path::Path};
use walkdir::WalkDir;

pub struct ModuleLinter {
    pub configuration: Configuration,
    pub fix: bool,
}

impl ModuleLinter {
    pub fn new(fix: bool) -> Self {
        ModuleLinter {
            configuration: Configuration::default(),
            fix,
        }
    }

    pub fn run(mut self, dir: &str) -> bool {
        eprintln!("Linting {}", dir);

        if let Ok(file) = File::open(Path::new(dir).join(".gold.yml")) {
            self.configuration = serde_yaml::from_reader(&file).unwrap();
        } else if let Ok(file) = File::open(Path::new(dir).join(".golangci.yml")) {
            eprintln!("Could not find .gold.yml, using configuration from .golangci.yml");
            let gc: GolangciConfiguration = serde_yaml::from_reader(&file).unwrap();
            self.configuration = Configuration::from(gc);
        } else {
            eprintln!("Could not find .gold.yml or .golangci.yml, using default configuration");
        }

        let mut ignore = HashSet::new();
        if let Some(patterns) = &self.configuration.ignore {
            for pattern in patterns {
                ignore.insert(Path::new(dir).join(pattern));
            }
        }

        let walk_dir = WalkDir::new(dir)
            .sort_by_file_name()
            .into_iter()
            .filter_entry(|entry| !ignore.contains(entry.path()))
            .filter_map(|e| e.ok())
            .filter(is_source_file);

        let mut exit = true;

        for file in walk_dir {
            let mut file_linter = FileLinter::new(
                file.path().display().to_string(),
                self.fix,
                &self.configuration,
            );
            exit &= file_linter.run();
        }

        exit
    }
}

fn is_source_file(entry: &walkdir::DirEntry) -> bool {
    entry.metadata().unwrap().is_file() && entry.path().display().to_string().ends_with(".go")
}
