struct Error {
    pub line: usize,
    pub message: String,
    pub why: String,
}

impl Error {
    fn new(line: usize, message: String, why: String) -> Self {
        Error { line, message, why }
    }
}

pub struct ErrorManager {
    errors: Vec<Error>,
    has_error: bool,
}

impl ErrorManager {
    pub fn new() -> Self {
        ErrorManager {
            errors: vec![],
            has_error: false,
        }
    }
    pub fn add_error(&mut self, line: usize, message: String, why: String) {
        self.has_error = true;
        self.errors.push(Error::new(line, message, why));
    }
    pub fn report_errors(&self) {
        for error in self.errors.iter() {
            println!(
                "[line {}] Error {}: {}",
                error.line, error.why, error.message
            );
        }
    }
    pub fn has_errors(&self) -> bool {
        return self.has_error;
    }
}
