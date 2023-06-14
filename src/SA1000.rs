use tree_sitter::{Node, Query, QueryCursor};

use crate::{error::Error, go, tree_sitter_go, Linter};

impl Linter {
    // SA1000 - Invalid regular expression
    // https://staticcheck.io/docs/checks#SA1000
    pub fn sa1000(&self, call_expression: Node) -> Option<Error> {
        let query = Query::new(
            unsafe { tree_sitter_go() },
            r#"
            (call_expression
                function: (selector_expression
                    operand: (identifier) @package
                    field: (field_identifier) @a (.match? @a "^(Compile|Match|MatchReader|MatchString|MustCompile)$")
                )
                arguments: (argument_list . [(identifier) (interpreted_string_literal) (raw_string_literal)] @expr)
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
        if self.eval(captures[idx].node)? != "regexp" {
            return None;
        }

        let idx = query.capture_index_for_name("expr")? as usize;
        let node = captures[idx].node;
        let expr = self.eval(node)?;
        let err = go::regexp_compile(expr)?;

        Some(Error {
            filename: self.filename.clone(),
            line: node.start_position().row + 1,
            char: node.start_position().column + 1,
            check: String::from("SA1000"),
            message: err,
        })
    }
}
