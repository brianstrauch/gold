use tree_sitter::{Node, Query, QueryCursor};

use crate::{error::Error, file_linter::FileLinter, go, query};

lazy_static! {
    static ref QUERY: Query = query::new(
        r#"
        (call_expression
            function: (selector_expression
                operand: (identifier) @package
                field: (field_identifier) @f (.match? @f "^(Compile|Match|MatchReader|MatchString|MustCompile)$")
            )
            arguments: (argument_list . [(identifier) (interpreted_string_literal) (raw_string_literal)] @expr)
        )
        "#
    );
}

// SA1000 - Invalid regular expression
// https://staticcheck.io/docs/checks/#SA1000
pub fn run(linter: &FileLinter, node: Node) -> Option<Error> {
    if !linter
        .module_linter
        .configuration
        .is_enabled(String::from("SA1000"))
    {
        return None;
    }

    let mut cursor = QueryCursor::new();
    cursor.set_max_start_depth(1);

    let captures = cursor
        .matches(&QUERY, node, linter.source.as_bytes())
        .next()?
        .captures;

    if linter.eval(captures[0].node)? != "regexp" {
        return None;
    }

    let node = captures[2].node;
    let expr = linter.eval(node)?;
    let err = go::regexp_compile(expr)?;

    Some(Error {
        filename: linter.path.clone(),
        position: node.start_position(),
        rule: String::from("SA1000"),
        message: err,
    })
}
