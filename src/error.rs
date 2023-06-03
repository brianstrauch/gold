pub struct Error {
    pub filename: String,
    pub line: usize,
    pub char: usize,
    pub check: String,
    pub message: String,
}

impl Error {
    pub fn to_string(&self) -> String {
        format!(
            "{}:{}:{}: {} ({})",
            self.filename, self.line, self.char, self.message, self.check
        )
    }
}
