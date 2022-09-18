#![feature(let_chains)]

mod ast;
mod error_reporting;
mod scanner;
mod parser;

use std::env;

use ast::ast_printer::AstPrinter;

static PRINTER: AstPrinter = AstPrinter;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Error: format: interpreter_follow_along [filepath]")
    } else if args.len() == 1 {
        scanner::scanner::Scanner::accept_input();
    } else if let Ok(scanner) = scanner::scanner::Scanner::input_file(&args[0]) {
        // TODO interpret file here
    } else {
        println!("Please enter a valid filepath");
    }
}
