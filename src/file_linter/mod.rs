#![allow(non_snake_case)]

pub mod F001;
pub mod F002;

use std::fs::{self, File};

use tree_sitter::{Node, Parser, Tree};

use crate::configuration::Configuration;

pub struct FileLinter<'a> {
    pub path: String,
    pub fix: bool,
    pub configuration: &'a Configuration,
    pub source: String,
    pub tree: Tree,
}

extern "C" {
    fn tree_sitter_go() -> tree_sitter::Language;
}

impl<'a> FileLinter<'a> {
    pub fn new(path: String, fix: bool, configuration: &'a Configuration) -> Self {
        let mut parser = Parser::new();
        parser.set_language(unsafe { tree_sitter_go() }).unwrap();

        let source = fs::read_to_string(&path).unwrap();
        let tree = parser.parse(&source, None).unwrap();

        FileLinter {
            fix,
            path,
            configuration,
            source,
            tree,
        }
    }

    pub fn run(&mut self) -> bool {
        let mut all_errors = vec![];
        let mut all_editors = vec![];

        let rules = vec![F001::run, F002::run];
        for rule in rules {
            let (errors, editors) = &mut rule(self);
            all_errors.append(errors);
            all_editors.append(editors);
        }

        if self.fix {
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
