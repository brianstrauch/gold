use crate::{
    configuration::{golangci::GolangciConfiguration, Configuration},
    file_linter::FileLinter,
};
use std::{
    collections::HashSet,
    fs::{self, File},
    path::Path,
};
use tree_sitter::{Parser, QueryCursor};
use walkdir::WalkDir;

pub struct ModuleLinter {
    pub configuration: Configuration,
    pub fix: bool,
}

extern "C" {
    fn tree_sitter_gomod() -> tree_sitter::Language;
}

impl ModuleLinter {
    pub fn new(fix: bool) -> Self {
        ModuleLinter {
            configuration: Configuration::default(),
            fix,
        }
    }

    pub fn run(mut self, dir: &str) -> bool {
        let path = Path::new(dir);

        if let Some(module) = get_module(path) {
            eprintln!("Module: {module}");
        }

        if let Ok(file) = File::open(path.join(".gold.yml")) {
            eprintln!("Configuration: .gold.yml");
            self.configuration = serde_yaml::from_reader(&file).unwrap();
        } else if let Ok(file) = File::open(path.join(".golangci.yml")) {
            eprintln!("Configuration: .golangci.yml");
            let gc: GolangciConfiguration = serde_yaml::from_reader(&file).unwrap();
            self.configuration = Configuration::from(gc);
        } else {
            eprintln!("Configuration: default");
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

fn get_module(path: &Path) -> Option<String> {
    let mut cursor = QueryCursor::new();

    let query = tree_sitter::Query::new(
        unsafe { tree_sitter_gomod() },
        "(module_directive (module_path) @path)",
    )
    .ok()?;

    let mut parser = Parser::new();
    parser.set_language(unsafe { tree_sitter_gomod() }).ok()?;

    let source = fs::read_to_string(path.join("go.mod")).ok()?;
    let tree = parser.parse(&source, None)?;

    let module = cursor
        .matches(&query, tree.root_node(), source.as_bytes())
        .next()?
        .captures[0]
        .node
        .utf8_text(source.as_bytes())
        .ok()?
        .to_string();

    Some(module)
}

fn is_source_file(entry: &walkdir::DirEntry) -> bool {
    entry.metadata().unwrap().is_file() && entry.path().display().to_string().ends_with(".go")
}
