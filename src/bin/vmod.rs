#![feature(let_chains)]

use std::{collections::VecDeque, env};

use vmod::ast::expr_types::ExprPossibilities;
use vmod::import_sys::import_sys::Importer;
use vmod::interpreter::interpreter::Interpreter;
// use ast::ast_printer::AstPrinter;
use vmod::parser::parser::Parser;
use vmod::scanner::scanner;
use vmod::scanner::token::{Token, TokenType};

// static PRINTER: AstPrinter = AstPrinter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    let mut importer = Importer::new();
    if args.len() > 2 {
        println!("Error: format: interpreter_follow_along [filepath]")
    } else if args.len() == 1 {
        vmod::scanner::scanner::Scanner::accept_input();
    } else if let Ok(mut scanner) = vmod::scanner::scanner::Scanner::input_file(&args[1]) {
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

        importer.import_files(parser.imports, &mut interpreter, args[1].clone());

        for expr in expressions.iter() {
            interpreter.interpret(expr);
        }
    } else if let Err(err) = vmod::scanner::scanner::Scanner::input_file(&args[1]) {
        println!("{}", err);
    }
}
