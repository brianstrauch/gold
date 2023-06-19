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
            rule: String::from("SA1000"),
            message: String::from("error parsing regexp: missing closing ): `(`"),
        };

        assert_eq!(
            error.to_string(),
            String::from("main.go:1:1: error parsing regexp: missing closing ): `(` (SA1000)")
        );
    }
}
