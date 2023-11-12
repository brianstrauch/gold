use std::fmt::{self, Display};

use tree_sitter::Point;

pub struct Error {
    pub filename: String,
    pub position: Point,
    pub rule: String,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{}: {} ({})",
            self.filename,
            self.position.row + 1,
            self.position.column + 1,
            self.message,
            self.rule
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let error = Error {
            filename: String::from("main.go"),
            position: Point { row: 0, column: 0 },
            rule: String::from("F001"),
            message: String::from(r#"redundant parameter type "string""#),
        };

        assert_eq!(
            error.to_string(),
            String::from(r#"main.go:1:1: redundant parameter type "string" (F001)"#)
        );
    }
}
