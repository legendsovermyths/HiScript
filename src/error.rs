pub struct Error {
    pub line: usize,
    pub message: String,
    pub why: String,
}

pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    pub fn new(message: &str) -> Self {
        ErrorMessage {
            message: message.to_owned(),
        }
    }
    pub fn get_message(self)->String{
        return self.message;
    }
}

impl Error {
    pub fn new(line: usize, message: String, why: String) -> Self {
        Error { line, message, why }
    }
}

pub struct ErrorManager {
    errors: Vec<Error>,
    has_error: bool,
    has_runtime_error: bool,
}

impl ErrorManager {
    pub fn new() -> Self {
        ErrorManager {
            errors: vec![],
            has_error: false,
            has_runtime_error: false,
        }
    }
    pub fn add_error(&mut self, line: usize, message: String, why: String) {
        self.has_error = true;
        self.errors.push(Error::new(line, message, why));
    }
    pub fn add_runtime_error(&mut self, error: Error) {
        self.has_runtime_error = true;
        self.errors.push(error);
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
    pub fn clear_errors(&mut self) {
        self.has_error = false;
        self.errors = vec![];
    }
}
