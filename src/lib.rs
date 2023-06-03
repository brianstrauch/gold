mod error;
mod regexp;

use error::Error;
use go_parser::{
    ast::{self, Node},
    parse,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    vec::Vec,
};

pub struct Linter {
    filename: String,
    cumulative_line_len: Vec<usize>,
    source_file: Node,
    variables: HashMap<String, String>,
}

impl Linter {
    pub fn new(filename: String) -> Linter {
        let content: String = fs::read_to_string(&filename).unwrap();

        let mut cumulative_line_len = Vec::with_capacity(content.lines().count());
        let mut sum = 0;

        for line in content.lines() {
            cumulative_line_len.push(sum);
            sum += line.len() + 1;
        }

        Linter {
            filename,
            cumulative_line_len,
            source_file: ast::Node::SourceFile(parse(&content).expect("Failed to parse")),
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> bool {
        let errors = self.walk(self.source_file.clone());

        for error in errors.iter() {
            println!("{}", error.to_string());
        }

        errors.len() == 0
    }

    fn walk(&mut self, node: Node) -> Vec<Error> {
        let mut errors = Vec::new();
        match node {
            Node::ConstDecl(const_decl) => {
                self.variables.insert(
                    const_decl.const_spec.identifier_list[0].clone(),
                    const_decl.const_spec.expression_list[0]
                        .unary_expr
                        .clone()
                        .unwrap()
                        .primary_expr
                        .clone()
                        .operand
                        .unwrap()
                        .literal
                        .unwrap()
                        .basic_lit
                        .string_lit
                        .raw_string_lit
                        .unwrap(),
                );
                errors.extend(
                    self.walk(Node::PrimaryExpr(
                        const_decl.const_spec.expression_list[0]
                            .unary_expr
                            .clone()
                            .unwrap()
                            .primary_expr
                            .clone(),
                    )),
                );
            }
            Node::Declaration(declaration) => {
                if let Some(const_decl) = declaration.const_decl {
                    errors.extend(self.walk(Node::ConstDecl(const_decl)));
                }
                if let Some(var_decl) = declaration.var_decl {
                    errors.extend(self.walk(Node::VarDecl(var_decl)));
                }
            }
            Node::ExpressionStmt(expression_stmt) => {
                errors.extend(self.walk(Node::PrimaryExpr(
                    expression_stmt.expression.unary_expr.unwrap().primary_expr,
                )));
            }
            Node::FunctionDecl(function_decl) => {
                for statement in function_decl.function_body.block.statement_list.statements {
                    errors.extend(self.walk(Node::Statement(statement)));
                }
            }
            Node::PrimaryExpr(primary_expr) => errors.extend(self.sa1000(primary_expr)),
            Node::ShortVarDecl(short_var_decl) => {
                errors.extend(
                    self.walk(Node::PrimaryExpr(
                        short_var_decl.expression_list[0]
                            .unary_expr
                            .clone()
                            .unwrap()
                            .primary_expr
                            .clone(),
                    )),
                );
            }
            Node::SimpleStmt(simple_stmt) => {
                if let Some(expression_stmt) = simple_stmt.expression_stmt {
                    errors.extend(self.walk(Node::ExpressionStmt(expression_stmt)));
                }
                if let Some(short_var_decl) = simple_stmt.short_var_decl {
                    errors.extend(self.walk(Node::ShortVarDecl(short_var_decl)));
                }
            }
            Node::SourceFile(source_file) => {
                for top_level_decl in source_file.top_level_decls.iter() {
                    errors.extend(self.walk(Node::TopLevelDecl(top_level_decl.to_owned())));
                }
            }
            Node::Statement(statement) => {
                if let Some(simple_stmt) = statement.simple_stmt {
                    errors.extend(self.walk(Node::SimpleStmt(simple_stmt)));
                }
            }
            Node::TopLevelDecl(top_level_decl) => {
                if let Some(declaration) = top_level_decl.declaration {
                    errors.extend(self.walk(Node::Declaration(declaration)));
                }
                if let Some(function_decl) = top_level_decl.function_decl {
                    errors.extend(self.walk(Node::FunctionDecl(function_decl)));
                }
            }
            Node::VarDecl(var_decl) => {
                errors.extend(
                    self.walk(Node::PrimaryExpr(
                        var_decl.var_spec.expression_list[0]
                            .unary_expr
                            .clone()
                            .unwrap()
                            .primary_expr
                            .clone(),
                    )),
                );
            }
        }
        return errors;
    }

    // SA1000 - Invalid regular expression
    // https://staticcheck.io/docs/checks/#SA1000
    fn sa1000(&self, primary_expr: ast::PrimaryExpr) -> Option<Error> {
        let method = primary_expr.method_expr?;

        if method.receiver_type._type.type_name.identifier.as_str() != "regexp" {
            return None;
        }

        let functions: HashSet<&str> = HashSet::from([
            "MustCompile",
            "Compile",
            "Match",
            "MatchReader",
            "MatchString",
        ]);

        if !functions.contains(method.method_name.as_str()) {
            return None;
        }

        let operand = primary_expr.arguments?.expression_list[0]
            .unary_expr
            .clone()?
            .primary_expr
            .clone()
            .operand?;

        let mut re = String::from("");
        if let Some(literal) = operand.literal {
            if let Some(raw_string_lit) = literal.basic_lit.string_lit.raw_string_lit {
                re = raw_string_lit;
            }
            if let Some(interpreted_string_lit) =
                literal.basic_lit.string_lit.interpreted_string_lit
            {
                re = interpreted_string_lit;
            }
        };
        if let Some(operand_name) = operand.operand_name {
            re = self
                .variables
                .get(&operand_name.identifier)
                .unwrap()
                .clone();
        };

        let err = unsafe { regexp::compile(re) };
        if err == "" {
            return None;
        }

        let (line, char) = self.get_line_and_char(operand.loc);

        Some(Error {
            filename: String::from(&self.filename),
            line,
            char,
            check: String::from("SA1000"),
            message: err,
        })
    }

    fn get_line_and_char(&self, loc: usize) -> (usize, usize) {
        let line = self.cumulative_line_len.partition_point(|&x| x < loc) - 1;
        let char = loc - self.cumulative_line_len.get(line).unwrap();

        (line + 1, char + 1)
    }
}
