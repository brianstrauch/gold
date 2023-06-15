use tree_sitter::{Node, Query, QueryCursor};

use crate::{error::Error, go, tree_sitter_go, Linter};

// SA1002 - Invalid format in time.Parse
// https://staticcheck.io/docs/checks#SA1002
pub fn run(linter: &Linter, call_expression: Node) -> Option<Error> {
    let query = Query::new(
        unsafe { tree_sitter_go() },
        r#"
        (call_expression
            function: (selector_expression
                operand: (identifier) @package
                field: (field_identifier) @a (.eq? @a "Parse")
            )
            arguments: (argument_list . [(identifier) (interpreted_string_literal) (raw_string_literal)] @expr)
        )
        "#,
    )
    .unwrap();

    let mut cursor = QueryCursor::new();
    cursor.set_max_start_depth(1);

    let captures = cursor
        .matches(&query, call_expression, linter.source.as_bytes())
        .next()?
        .captures;

    let idx = query.capture_index_for_name("package")? as usize;
    if linter.eval(captures[idx].node)? != "time" {
        return None;
    }

    let idx = query.capture_index_for_name("expr")? as usize;
    let node = captures[idx].node;
    let expr = linter.eval(node)?.replace("_", " ").replace("Z", "-");
    let err = go::time_parse(expr)?;

    Some(Error {
        filename: linter.filename.clone(),
        line: node.start_position().row + 1,
        char: node.start_position().column + 1,
        check: String::from("SA1002"),
        message: err,
    })
}
