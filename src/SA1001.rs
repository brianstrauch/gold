use tree_sitter::{Node, Query, QueryCursor};

use crate::{error::Error, go, tree_sitter_go, Linter};

impl Linter {
    // SA1001 - Invalid template
    // https://staticcheck.io/docs/checks#SA1001
    pub fn sa1001(&self, call_expression: Node) -> Option<Error> {
        let query = Query::new(
            unsafe { tree_sitter_go() },
            r#"
            (call_expression
                function: (selector_expression
                    operand: (call_expression
                        function: (selector_expression
                            operand: (identifier) @package
                            field: (field_identifier) @a (.eq? @a "New")
                        )
                        arguments: (argument_list [(identifier) (interpreted_string_literal) (raw_string_literal)])
                    )
                    field: (field_identifier) @b (.eq? @b "Parse")
                )
                arguments: (argument_list [(identifier) (interpreted_string_literal) (raw_string_literal)] @expr)
            )
            "#,
        )
        .unwrap();

        let mut cursor = QueryCursor::new();
        cursor.set_max_start_depth(1);

        let captures = cursor
            .matches(&query, call_expression, self.source.as_bytes())
            .next()?
            .captures;

        let idx = query.capture_index_for_name("package")? as usize;
        let package = self.eval(captures[idx].node)?;

        let idx = query.capture_index_for_name("expr")? as usize;
        let node = captures[idx].node;
        let expr = self.eval(node)?;

        let err = match package.as_str() {
            "html/template" => go::html_template_new_parse(expr),
            "text/template" => go::text_template_new_parse(expr),
            _ => None,
        }?;

        if !(err.contains("bad character") || err.contains("unexpected")) {
            return None;
        }

        Some(Error {
            filename: self.filename.clone(),
            line: node.start_position().row + 1,
            char: node.start_position().column + 1,
            check: String::from("SA1001"),
            message: err,
        })
    }
}
