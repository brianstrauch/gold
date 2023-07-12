use tree_sitter::{Node, Query, QueryCursor};

use crate::{error::Error, file_linter::FileLinter, query};

lazy_static! {
    static ref QUERY: Query = query::new(
        r#"
        (parameter_list (parameter_declaration
            name: (identifier) @name .
            type: (_) @type
        ))
        "#,
    );
}

// G0000 - Redundant parameter types
pub fn run(linter: &FileLinter, node: Node) -> Option<Error> {
    if !linter
        .module_linter
        .configuration
        .is_enabled(String::from("G0000"))
    {
        return None;
    }

    let mut cursor = QueryCursor::new();
    cursor.set_max_start_depth(1);

    let mut last: Option<Node> = None;
    for m in cursor.matches(&QUERY, node, linter.source.as_bytes()) {
        let node = m.captures[1].node;

        if let Some(last) = last {
            let last_type = last.utf8_text(linter.source.as_bytes()).unwrap();
            let node_type = node.utf8_text(linter.source.as_bytes()).unwrap();

            if last_type == node_type {
                return Some(Error {
                    filename: linter.path.clone(),
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
