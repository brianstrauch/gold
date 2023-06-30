use crate::{
    configuration::{golangci::GolangciConfiguration, gold::Configuration},
    file_linter::FileLinter,
};
use std::{fs::File, path::Path};
use walkdir::WalkDir;

pub struct ModuleLinter {
    pub configuration: Configuration,
}

impl ModuleLinter {
    pub fn new() -> Self {
        Self {
            configuration: Configuration::default(),
        }
    }

    pub fn run(mut self, path: &str) -> bool {
        if let Ok(file) = File::open(Path::new(path).join(".gold.yml")) {
            self.configuration = serde_yaml::from_reader(&file).unwrap();
        } else if let Ok(file) = File::open(Path::new(path).join(".golangci.yml")) {
            eprintln!("Using configuration from .golangci.yml");
            let golangci_configuration: GolangciConfiguration =
                serde_yaml::from_reader(&file).unwrap();
            self.configuration = Configuration::from(golangci_configuration);
        } else {
            eprintln!("Could not find .gold.yml, using default configuration");
        }

        let mut exit = false;

        for file in WalkDir::new(path)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(is_source_file)
        {
            let mut file_linter = FileLinter::new(&self, file.path().display().to_string());
            exit &= file_linter.run();
        }

        exit
    }
}

fn is_source_file(entry: &walkdir::DirEntry) -> bool {
    entry.metadata().unwrap().is_file() && entry.path().display().to_string().ends_with(".go")
}
