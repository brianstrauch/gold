mod error;
mod go;
mod query;
mod rules;

use error::Error;
use query::STRING;
use std::{collections::HashMap, fs};
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};
use walkdir::WalkDir;

extern "C" {
    fn tree_sitter_go() -> Language;
}

pub struct Cache {
    queries: HashMap<String, Query>,
}

pub struct Linter {
    filename: String,
    source: String,
    variables: HashMap<String, String>,
}

pub fn lint(path: String) -> bool {
    let mut cache = Cache {
        queries: HashMap::new(),
    };

    cache.queries.insert(
        String::from("const_declaration"),
        query::new(
            format!(
                r#"
                    (const_declaration (const_spec
                        name: (identifier) @k
                        value: (expression_list {STRING} @v)
                    ))
                    "#
            )
            .as_str(),
        ),
    );

    cache.queries.insert(
        String::from("import_spec"),
        query::new(
            r#"
                    (import_spec
                        name: (package_identifier)? @k
                        path: (interpreted_string_literal) @v
                    )
                    "#,
        ),
    );

    cache.queries.insert(
        String::from("short_var_declaration"),
        query::new(
            format!(
                r#"
                    (short_var_declaration
                        left: (expression_list (identifier) @k)
                        right: (expression_list {STRING} @v)
                    )
                    "#
            )
            .as_str(),
        ),
    );

    cache.queries.insert(
        String::from("var_declaration"),
        query::new(
            format!(
                r#"
                    (var_declaration (var_spec
                        name: (identifier) @k
                        value: (expression_list {STRING} @v)
                    ))
                    "#
            )
            .as_str(),
        ),
    );

    cache.queries.insert(
        String::from("G0000"),
        query::new(
            r#"
        (parameter_list (parameter_declaration
            name: (identifier) @name .
            type: (_) @type
        ))
        "#,
        ),
    );

    cache.queries.insert(
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
    );

    if fs::metadata(&path).unwrap().is_dir() {
        let mut exit = false;

        for file in WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| is_source_file(e))
        {
            let source = fs::read_to_string(file.path()).expect("failed to read file");
            let mut linter = Linter {
                filename: file.path().display().to_string(),
                source,
                variables: HashMap::new(),
            };
            exit &= linter.run(&cache);
        }

        exit
    } else {
        let source = fs::read_to_string(&path).expect("failed to read file");

        let mut linter = Linter {
            filename: path,
            source,
            variables: HashMap::new(),
        };

        linter.run(&cache)
    }
}

fn is_source_file(entry: &walkdir::DirEntry) -> bool {
    entry.metadata().unwrap().is_file() && entry.path().display().to_string().ends_with(".go")
}

impl Linter {
    pub fn run(&mut self, cache: &Cache) -> bool {
        let mut parser = Parser::new();
        parser
            .set_language(unsafe { tree_sitter_go() })
            .expect("failed to set language");

        let tree = parser.parse(&self.source, None).expect("failed to parse");

        let errors = self.walk(tree.root_node(), cache);

        for error in errors.iter() {
            println!("{error}");
        }

        errors.is_empty()
    }

    fn walk(&mut self, node: Node, cache: &Cache) -> Vec<Error> {
        let mut errors = Vec::new();

        match node.kind() {
            "const_declaration" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                let query = cache.queries.get("const_declaration").unwrap();

                for m in cursor.matches(&query, node, self.source.as_bytes()) {
                    let k = m.captures[0]
                        .node
                        .utf8_text(self.source.as_bytes())
                        .unwrap();
                    if let Some(v) = self.eval(m.captures[1].node) {
                        self.variables.insert(String::from(k), v);
                    }
                }
            }
            "call_expression" => {
                if let Some(error) = rules::SA1000::run(self, node, cache) {
                    errors.push(error);
                }
            }
            "parameter_list" => {
                if let Some(error) = rules::G0000::run(self, node, cache) {
                    errors.push(error);
                }
            }
            "import_spec" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                let query = cache.queries.get("import_spec").unwrap();

                for m in cursor.matches(&query, node, self.source.as_bytes()) {
                    match m.captures.len() {
                        1 => {
                            if let Some(v) = self.eval(m.captures[0].node) {
                                let k = v.split('/').last().unwrap().to_string();
                                self.variables.insert(k, v);
                            }
                        }
                        2 => {
                            let k = m.captures[0]
                                .node
                                .utf8_text(self.source.as_bytes())
                                .unwrap();
                            if let Some(v) = self.eval(m.captures[1].node) {
                                self.variables.insert(String::from(k), v);
                            }
                        }
                        _ => {}
                    }
                }
            }
            "short_var_declaration" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                let query = cache.queries.get("short_var_declaration").unwrap();

                for m in cursor.matches(&query, node, self.source.as_bytes()) {
                    let k = m.captures[0]
                        .node
                        .utf8_text(self.source.as_bytes())
                        .unwrap();
                    if let Some(v) = self.eval(m.captures[1].node) {
                        self.variables.insert(String::from(k), v);
                    }
                }
            }
            "var_declaration" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                let query = cache.queries.get("var_declaration").unwrap();

                for m in cursor.matches(&query, node, self.source.as_bytes()) {
                    let k = m.captures[0]
                        .node
                        .utf8_text(self.source.as_bytes())
                        .unwrap();
                    if let Some(v) = self.eval(m.captures[1].node) {
                        self.variables.insert(String::from(k), v);
                    }
                }
            }
            _ => {}
        }

        for child in node.children(&mut node.walk()) {
            errors.append(&mut self.walk(child, cache));
        }

        errors
    }

    fn eval(&self, node: Node) -> Option<String> {
        let text = node.utf8_text(self.source.as_bytes()).ok()?;

        match node.kind() {
            "identifier" => self.variables.get(text).cloned(),
            "interpreted_string_literal" => Some(text.trim_matches('"').to_string()),
            "raw_string_literal" => Some(text.trim_matches('`').to_string()),
            _ => None,
        }
    }
}
