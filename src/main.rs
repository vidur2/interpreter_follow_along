#![feature(let_chains)]
#![feature(const_for)]
#![feature(const_mut_refs)]

mod ast;
mod error_reporting;
mod parser;
mod scanner;
mod interpreter;

use std::env;

use interpreter::interpreter::Interpreter;
// use ast::ast_printer::AstPrinter;
use parser::parser::Parser;

// static PRINTER: AstPrinter = AstPrinter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    if args.len() > 2 {
        println!("Error: format: interpreter_follow_along [filepath]")
    } else if args.len() == 1 {
        scanner::scanner::Scanner::accept_input();
    } else if let Ok(mut scanner) = scanner::scanner::Scanner::input_file(&args[1]) {
        scanner.tokenize_buff();
        let mut parser = Parser::new(scanner.get_buff());
        while !parser.is_at_end() {
            let expr = parser.parse();
            if let Ok(expr) = expr {
                interpreter.interpret(&expr);
            } else {
                break;
            }
        }
    } else if let Err(err) = scanner::scanner::Scanner::input_file(&args[1]) {
        println!("{}", err);
    }
}
