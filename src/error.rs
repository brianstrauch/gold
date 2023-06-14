use std::fmt::{self, Display};

pub struct Error {
    pub filename: String,
    pub line: usize,
    pub char: usize,
    pub check: String,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{}: {} ({})",
            self.filename, self.line, self.char, self.message, self.check
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
            line: 1,
            char: 1,
            check: String::from("SA1000"),
            message: String::from("error parsing regexp: missing closing ): `(`"),
        };

        assert_eq!(
            error.to_string(),
            String::from("main.go:1:1: error parsing regexp: missing closing ): `(` (SA1000)")
        );
    }
}
