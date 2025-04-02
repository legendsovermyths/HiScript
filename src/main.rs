mod hiscript;
mod token;
mod token_type;
mod literal;
mod lexer;
mod error;
mod expr;
mod parser;
use std::error::Error;

use crate::hiscript::HiScript;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let interpreter = HiScript::new();
    if args.len() > 2 {
        println!("Usage: hiscript [script]");
        return Err("".into());
    } else if args.len() == 2 {
        return interpreter.run_file(&args[1]);
    } else {
        return interpreter.run_prompt();
    }
}
