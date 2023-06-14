mod error;
mod go;

mod sa1000;
mod sa1001;

use error::Error;
use std::{collections::HashMap, fs};
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

extern "C" {
    fn tree_sitter_go() -> Language;
}

pub struct Linter {
    filename: String,
    source: String,
    variables: HashMap<String, String>,
}

impl Linter {
    pub fn new(filename: String) -> Linter {
        let source = fs::read_to_string(&filename).expect("failed to read file");

        Linter {
            filename,
            source,
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> bool {
        let mut parser = Parser::new();
        parser
            .set_language(unsafe { tree_sitter_go() })
            .expect("failed to set language");

        let tree = parser.parse(&self.source, None).expect("failed to parse");

        let errors = self.walk(tree.root_node());

        for error in errors.iter() {
            println!("{}", error.to_string());
        }

        errors.is_empty()
    }

    fn walk(&mut self, node: Node) -> Vec<Error> {
        let mut errors = Vec::new();

        match node.kind() {
            "const_declaration" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                let query = Query::new(
                    unsafe { tree_sitter_go() },
                    r#"
                    (const_declaration (const_spec
                        name: (identifier) @k
                        value: (expression_list [(interpreted_string_literal) (raw_string_literal)] @v)
                    ))
                    "#,
                )
                .unwrap();

                for m in cursor.matches(&query, node, self.source.as_bytes()) {
                    let k = m.captures[0]
                        .node
                        .utf8_text(&self.source.as_bytes())
                        .unwrap();
                    if let Some(v) = self.eval(m.captures[1].node) {
                        self.variables.insert(String::from(k), v);
                    }
                }
            }
            "call_expression" => {
                if let Some(error) = sa1000::run(&self, node) {
                    errors.push(error);
                }
                if let Some(error) = sa1001::run(&self, node) {
                    errors.push(error);
                }
            }
            "import_spec" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                let query = Query::new(
                    unsafe { tree_sitter_go() },
                    r#"
                    (import_spec
                        name: (package_identifier)? @k
                        path: (interpreted_string_literal) @v
                    )
                    "#,
                )
                .unwrap();

                for m in cursor.matches(&query, node, self.source.as_bytes()) {
                    match m.captures.len() {
                        1 => {
                            if let Some(v) = self.eval(m.captures[0].node) {
                                let k = v.split("/").last().unwrap().to_string();
                                self.variables.insert(k, v);
                            }
                        }
                        2 => {
                            let k = m.captures[0]
                                .node
                                .utf8_text(&self.source.as_bytes())
                                .unwrap();
                            if let Some(v) = self.eval(m.captures[1].node) {
                                self.variables.insert(String::from(k), v);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        for child in node.children(&mut node.walk()) {
            errors.append(&mut self.walk(child));
        }

        return errors;
    }

    fn eval(&self, node: Node) -> Option<String> {
        let text = node.utf8_text(&self.source.as_bytes()).ok()?;

        match node.kind() {
            "identifier" => self.variables.get(text).cloned(),
            "interpreted_string_literal" => Some(text.trim_matches('"').to_string()),
            "raw_string_literal" => Some(text.trim_matches('`').to_string()),
            _ => None,
        }
    }
}
