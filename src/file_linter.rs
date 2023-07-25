use std::fs::{self, File};

use tree_sitter::{Node, Parser, Tree};

use crate::{module_linter::ModuleLinter, rules, tree_sitter_go};

pub struct FileLinter<'a> {
    pub module_linter: &'a ModuleLinter,
    pub path: String,
    pub source: String,
    pub tree: Tree,
}

impl<'a> FileLinter<'a> {
    pub fn new(module_linter: &'a ModuleLinter, path: String) -> Self {
        let mut parser = Parser::new();
        parser.set_language(unsafe { tree_sitter_go() }).unwrap();

        let source = fs::read_to_string(&path).unwrap();
        let tree = parser.parse(&source, None).unwrap();

        FileLinter {
            module_linter,
            path,
            source,
            tree,
        }
    }

    pub fn run(&mut self) -> bool {
        let mut all_errors = vec![];
        let mut all_editors = vec![];

        let rules = vec![rules::G0000::run, rules::G0001::run];
        for rule in rules {
            let (errors, editors) = &mut rule(self);
            all_errors.append(errors);
            all_editors.append(editors);
        }

        if self.module_linter.fix {
            for editor in all_editors.iter().rev() {
                let source = fs::read_to_string(&self.path).unwrap();
                let mut w = File::create(&self.path).unwrap();
                tree_sitter_edit::render(&mut w, &self.tree, source.as_bytes(), editor).unwrap();
            }
        } else {
            for error in all_errors.iter() {
                println!("{error}");
            }
        }

        all_errors.is_empty()
    }

    pub fn text(&self, node: Node) -> &str {
        node.utf8_text(self.source.as_bytes()).unwrap()
    }
}
