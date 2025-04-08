use std::{
    error::Error,
    fs,
    io::{stdin, stdout, Write},
};

use crate::{error::ErrorManager, interpreter::Interpreter, lexer::Lexer, parser::Parser};
pub struct HiScript {}

impl HiScript {
    pub fn new() -> Self {
        HiScript {}
    }

    pub fn run(&self, source: String) {
        let mut error_manager = ErrorManager::new();
        let mut lexer = Lexer::new(&source, &mut error_manager);
        let tokens = lexer.scan_tokens();
        let mut parser = Parser::new(tokens, &mut error_manager);
        let result = parser.parse();
        let mut interpreter = Interpreter::new(&mut error_manager);
        match result {
            Some(expr)=>{
                let literal = interpreter.interpret(expr);
                if let Some(val) = literal{
                    val.print();
                }
            }
            None=>{

            }
        }
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
            if bytes_read == 0 {
                break;
            }
            self.run(line);
        }
        return Ok(());
    }
}
