use std::{collections::HashMap, fs};

use tree_sitter::{Node, Parser, Query, QueryCursor};

use crate::{error::Error, module_linter::ModuleLinter, query, rules, tree_sitter_go};

lazy_static! {
    static ref CONST_DECLARATION_QUERY: Query = query::new(
        r#"
        (const_declaration (const_spec
            name: (identifier) @k
            value: (expression_list [(identifier) (interpreted_string_literal) (raw_string_literal)] @v)
        ))
        "#
    );
    static ref IMPORT_SPEC_QUERY: Query = query::new(
        r#"
        (import_spec
            name: (package_identifier)? @k
            path: (interpreted_string_literal) @v
        )
        "#,
    );
    static ref SHORT_VAR_DECLARATION_QUERY: Query = query::new(
        r#"
        (short_var_declaration
            left: (expression_list (identifier) @k)
            right: (expression_list [(identifier) (interpreted_string_literal) (raw_string_literal)] @v)
        )
        "#
    );
    static ref VAR_DECLARATION_QUERY: Query = query::new(
        r#"
        (var_declaration (var_spec
            name: (identifier) @k
            value: (expression_list [(identifier) (interpreted_string_literal) (raw_string_literal)] @v)
        ))
        "#
    );
}

pub struct FileLinter<'a> {
    pub path: String,
    pub source: String,
    pub module_linter: &'a ModuleLinter,
    variables: HashMap<String, String>,
}

impl<'a> FileLinter<'a> {
    pub fn new(module_linter: &'a ModuleLinter, path: String) -> Self {
        FileLinter {
            path: path.clone(),
            source: fs::read_to_string(path).unwrap(),
            module_linter,
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
            println!("{error}");
        }

        errors.is_empty()
    }

    fn walk(&mut self, node: Node) -> Vec<Error> {
        let mut errors = Vec::new();
        match node.kind() {
            "const_declaration" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                for m in cursor.matches(&CONST_DECLARATION_QUERY, node, self.source.as_bytes()) {
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
                if let Some(error) = rules::SA1000::run(self, node) {
                    errors.push(error);
                }
            }
            "parameter_list" => {
                if let Some(error) = rules::G0000::run(self, node) {
                    errors.push(error);
                }
            }
            "import_spec" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                for m in cursor.matches(&IMPORT_SPEC_QUERY, node, self.source.as_bytes()) {
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
            "import_spec_list" => {
                if let Some(error) = rules::G0001::run(self, node) {
                    errors.push(error);
                }
            }
            "short_var_declaration" => {
                let mut cursor = QueryCursor::new();
                cursor.set_max_start_depth(1);

                for m in cursor.matches(&SHORT_VAR_DECLARATION_QUERY, node, self.source.as_bytes())
                {
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

                for m in cursor.matches(&VAR_DECLARATION_QUERY, node, self.source.as_bytes()) {
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
            errors.append(&mut self.walk(child));
        }

        errors
    }

    pub fn eval(&self, node: Node) -> Option<String> {
        let text = node.utf8_text(self.source.as_bytes()).ok()?;

        match node.kind() {
            "identifier" => self.variables.get(text).cloned(),
            "interpreted_string_literal" => Some(text.trim_matches('"').to_string()),
            "raw_string_literal" => Some(text.trim_matches('`').to_string()),
            _ => None,
        }
    }
}
