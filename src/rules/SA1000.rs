use tree_sitter::{Node, QueryCursor};

use crate::{error::Error, go, Cache, Linter};

// SA1000 - Invalid regular expression
// https://staticcheck.io/docs/checks/#SA1000
pub fn run(linter: &Linter, call_expression: Node, cache: &Cache) -> Option<Error> {
    let query = cache.queries.get("SA1000").unwrap();

    let mut cursor = QueryCursor::new();
    cursor.set_max_start_depth(1);

    let captures = cursor
        .matches(&query, call_expression, linter.source.as_bytes())
        .next()?
        .captures;

    if linter.eval(captures[0].node)? != "regexp" {
        return None;
    }

    let node = captures[2].node;
    let expr = linter.eval(node)?;
    let err = go::regexp_compile(expr)?;

    Some(Error {
        filename: linter.filename.clone(),
        position: node.start_position(),
        rule: String::from("SA1000"),
        message: err,
    })
}
