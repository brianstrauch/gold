use tree_sitter::Query;

use crate::tree_sitter_go;

pub fn new(source: &str) -> Query {
    let language = unsafe { tree_sitter_go() };
    Query::new(language, source).unwrap()
}
