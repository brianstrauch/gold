use tree_sitter::{Node, Query, QueryCursor};

use crate::{error::Error, tree_sitter_go, Linter};

// G0000 - Redundant parameter types
pub fn run(linter: &Linter, parameter_list: Node) -> Option<Error> {
    let query = Query::new(
        unsafe { tree_sitter_go() },
        r#"
        (parameter_list (parameter_declaration
            name: (identifier) @name .
            type: (type_identifier) @type
        ))
        "#,
    )
    .unwrap();

    let mut cursor = QueryCursor::new();
    cursor.set_max_start_depth(1);

    let mut last: Option<Node> = None;
    for m in cursor.matches(&query, parameter_list, linter.source.as_bytes()) {
        let node = m.captures[1].node;

        if let Some(last) = last {
            let last_type = last.utf8_text(linter.source.as_bytes()).unwrap();
            let node_type = node.utf8_text(linter.source.as_bytes()).unwrap();

            if last_type == node_type {
                return Some(Error {
                    filename: linter.filename.clone(),
                    position: last.start_position(),
                    rule: String::from("G0000"),
                    message: format!(r#"redundant parameter type "{last_type}""#),
                });
            }
        }

        last = Some(node);
    }

    None
}
