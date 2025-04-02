use std::{
    error::Error,
    fs,
    io::{stdin, stdout, Write},
};

use crate::{
    error::ErrorManager,
    lexer::Lexer, parser::Parser,
};
pub struct HiScript {}

impl HiScript {
    pub fn new() -> Self {
        HiScript {}
    }

    pub fn run(&self, source: String) {
        let mut error_manager = ErrorManager::new();
        let mut lexer = Lexer::new(&source, &mut error_manager);
        let tokens = lexer.scan_tokens();
        lexer.print_tokens();
        println!();
        let mut parser = Parser::new(tokens, &mut error_manager);
        let result = parser.parse();
        println!("{:?}",result);
        error_manager.report_errors();
    }
    pub fn run_file(self, path: &String) -> Result<(), Box<dyn Error>> {
        let file_data = fs::read_to_string(path)?;
        self.run(file_data);
        return Ok(());
    }

    pub fn run_prompt(&self) -> Result<(), Box<dyn Error>> {
        loop {
            print!("> ");
            let _ = stdout().flush();
            let mut line = String::new();
            let bytes_read = stdin().read_line(&mut line)?;
            self.run(line);
            if bytes_read == 0 {
                break;
            }
        }
        return Ok(());
    }
}
