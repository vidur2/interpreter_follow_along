#![feature(let_chains)]
#![feature(const_for)]
#![feature(const_mut_refs)]

mod ast;
mod error_reporting;
mod import_sys;
mod interpreter;
mod parser;
mod scanner;

use std::{collections::VecDeque, env};

use ast::expr_types::ExprPossibilities;
use import_sys::import_sys::Importer;
use interpreter::interpreter::Interpreter;
// use ast::ast_printer::AstPrinter;
use parser::parser::Parser;
use scanner::token::{Token, TokenType};

// static PRINTER: AstPrinter = AstPrinter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    let mut importer = Importer::new();
    if args.len() > 2 {
        println!("Error: format: interpreter_follow_along [filepath]")
    } else if args.len() == 1 {
        scanner::scanner::Scanner::accept_input();
    } else if let Ok(mut scanner) = scanner::scanner::Scanner::input_file(&args[1]) {
        scanner.tokenize_buff();
        scanner.token.push(Token {
            tok: TokenType::EOF,
            lexeme: String::new(),
            line: usize::MAX,
            literal: None,
        });
        let mut parser = Parser::new(scanner.get_buff());
        let mut expressions: VecDeque<ExprPossibilities> = VecDeque::new();
        while !parser.is_at_end() {
            let expr = parser.parse();
            if let Ok(ExprPossibilities::Scope(scope)) = expr.clone() && let TokenType::FUNC = scope.stmt {
                expressions.push_front(ExprPossibilities::Scope(scope));
            } else if let Ok(expr) = expr {
                expressions.push_back(expr)
            } else {
                break;
            }
        }

        importer.import_files(parser.imports, &mut interpreter);

        for expr in expressions.iter() {
            interpreter.interpret(expr);
        }
    } else if let Err(err) = scanner::scanner::Scanner::input_file(&args[1]) {
        println!("{}", err);
    }
}
