use std::fmt::{self, Display};

use tree_sitter::Point;

pub struct Error {
    pub filename: String,
    pub point: Point,
    pub check: String,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{}: {} ({})",
            self.filename,
            self.point.row + 1,
            self.point.column + 1,
            self.message,
            self.check
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
            point: Point { row: 0, column: 0 },
            check: String::from("G0000"),
            message: String::from("error parsing regexp: missing closing ): `(`"),
        };

        assert_eq!(
            error.to_string(),
            String::from("main.go:1:1: error parsing regexp: missing closing ): `(` (G0000)")
        );
    }
}
