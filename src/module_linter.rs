use crate::{configuration::Configuration, file_linter::FileLinter, query};
use query::STRING;
use std::{collections::HashMap, path::Path};
use tree_sitter::Query;
use walkdir::WalkDir;

pub struct ModuleLinter {
    pub configuration: Configuration,
    pub queries: HashMap<String, Query>,
}

impl ModuleLinter {
    pub fn new() -> Self {
        let queries = HashMap::from([
            (
                String::from("const_declaration"),
                query::new(
                    format!(r#"
                    (const_declaration (const_spec
                        name: (identifier) @k
                        value: (expression_list {STRING} @v)
                    ))
                    "#).as_str(),
                ),
            ),
            (
                String::from("import_spec"),
                query::new(
                    r#"
                    (import_spec
                        name: (package_identifier)? @k
                        path: (interpreted_string_literal) @v
                    )
                    "#,
                ),
            ),
            (
                String::from("short_var_declaration"),
                query::new(
                    format!(r#"
                    (short_var_declaration
                        left: (expression_list (identifier) @k)
                        right: (expression_list {STRING} @v)
                    )
                    "#).as_str(),
                ),
            ),
            (
                String::from("var_declaration"),
                query::new(
                    format!(r#"
                    (var_declaration (var_spec
                        name: (identifier) @k
                        value: (expression_list {STRING} @v)
                    ))
                    "#).as_str(),
                ),
            ),
            (
                String::from("G0000"),
                query::new(
                    r#"
                    (parameter_list (parameter_declaration
                        name: (identifier) @name .
                        type: (_) @type
                    ))
                    "#,
                ),
            ),
            (
                String::from("SA1000"),
                query::new(
                    format!(r#"
                    (call_expression
                        function: (selector_expression
                            operand: (identifier) @package
                            field: (field_identifier) @f (.match? @f "^(Compile|Match|MatchReader|MatchString|MustCompile)$")
                        )
                        arguments: (argument_list . {STRING} @expr)
                    )
                    "#).as_str(),
                ),
            ),
        ]);

        Self {
            configuration: Configuration::new(),
            queries,
        }
    }

    pub fn run(mut self, path: &str) -> bool {
        if let Ok(file) = std::fs::File::open(Path::new(path).join(".gold.yml")) {
            self.configuration = serde_yaml::from_reader(&file).unwrap();
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
