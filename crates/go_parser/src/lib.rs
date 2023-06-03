use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;

pub mod ast;
lalrpop_mod!(pub go);

pub fn parse(
    input: &str,
) -> Result<ast::SourceFile, ParseError<usize, go::Token<'_>, &'static str>> {
    go::SourceFileParser::new().parse(input)
}
