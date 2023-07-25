use tree_sitter::{Node, Query, QueryCursor};
use tree_sitter_edit::{NodeId, Replace};

use crate::{error::Error, file_linter::FileLinter, query};

lazy_static! {
    static ref QUERY: Query = query::new(
        r#"
        (parameter_list
            (parameter_declaration
                name: (identifier) @name
                type: (_) @kind
            ) @decl
        ) @list
        "#,
    );
}

struct Parameter<'a> {
    list: Node<'a>,
    decl: Node<'a>,
    name: Node<'a>,
    kind: Node<'a>,
}

// G0000 - Redundant parameter types
pub fn run(linter: &mut FileLinter) -> (Vec<Error>, Vec<Replace>) {
    if !linter
        .module_linter
        .configuration
        .is_enabled(String::from("G0000"))
    {
        return (vec![], vec![]);
    }

    let mut cursor = QueryCursor::new();

    let mut errors = vec![];
    let mut editors = vec![];

    let mut last: Option<Parameter> = None;
    let mut parameters = vec![];

    for m in cursor.matches(&QUERY, linter.tree.root_node(), linter.source.as_bytes()) {
        let curr = Parameter {
            list: m.captures[0].node,
            decl: m.captures[1].node,
            name: m.captures[2].node,
            kind: m.captures[3].node,
        };

        if let Some(last) = last {
            let last_kind = linter.text(last.kind);
            let curr_kind = linter.text(curr.kind);

            if last.list.id() == curr.list.id() {
                if last.decl.id() != curr.decl.id() {
                    if last_kind == curr_kind {
                        if linter.module_linter.fix {
                            editors.push(Replace {
                                id: NodeId::new(&last.decl),
                                bytes: parameters.join(", ").as_bytes().to_vec(),
                            });
                        } else {
                            errors.push(Error {
                                filename: linter.path.clone(),
                                position: last.kind.start_position(),
                                rule: String::from("G0000"),
                                message: format!(r#"redundant parameter type "{}""#, last_kind),
                            });
                        }
                    }
                    parameters = vec![];
                }
            } else {
                parameters = vec![];
            }
        }

        parameters.push(linter.text(curr.name));
        last = Some(curr);
    }

    (errors, editors)
}
