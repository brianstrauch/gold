use tree_sitter::Query;

use crate::tree_sitter_go;

pub const STRING: &str = "[(identifier) (interpreted_string_literal) (raw_string_literal)]";

pub fn new(source: &str) -> Query {
    let language = unsafe { tree_sitter_go() };
    Query::new(language, source).unwrap()
}
