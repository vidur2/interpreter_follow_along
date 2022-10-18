use std::{
    collections::{HashSet, VecDeque},
    fs::ReadDir,
};

use crate::{
    ast::expr_types::{ExprPossibilities, Literal, Scope, Stmt},
    interpreter::{environment::Environment, interpreter::Interpreter},
    lib_functions::BUILTINS,
    parser::parser::Parser,
    scanner::{
        scanner::Scanner,
        token::{Func, Token, TokenType},
    },
};

pub struct Importer {
    parser: Parser,
}

impl Importer {
    pub fn new() -> Self {
        return Self {
            parser: Parser::new(Vec::new()),
        };
    }

    pub fn import_files(
        &mut self,
        files: HashSet<String>,
        global_interp: &mut Interpreter,
        base_filename: String,
    ) {
        let mut files = files;

        let mut dir_string = String::new();
        let mut split_dir: Vec<&str> = base_filename.split("/").collect();
        split_dir.pop();

        for dir in split_dir.iter() {
            dir_string += (dir.to_string() + "/").as_str();
        }
        let dir_paths = std::fs::read_dir("./".to_string() + &dir_string).unwrap();
        let lib_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string()
            + "/vmod_lib";

        for builtin in BUILTINS.iter() {
            if files.contains(&builtin.to_string()) {
                files.remove(&builtin.to_string());
                let env = crate::lib_functions::import_lib(&builtin);
                global_interp.globals.define_env(builtin, env.vars);
            }
        }

        for path in dir_paths.chain(std::fs::read_dir(lib_path).unwrap()) {
            let mut interpreter = Interpreter::new();
            let path = path.unwrap().path().into_os_string().into_string().unwrap();
            let split_path: Vec<&str> = path.split("/").collect();
            let file_name = split_path.last().unwrap().to_string();
            let split_file: Vec<&str> = file_name.split(".").collect();
            if split_file.last().unwrap() == &super::FileExtenstion && files.contains(split_file[0])
            {
                let mut scanned = Scanner::input_file(&path).unwrap();
                scanned.tokenize_buff();
                scanned.token.push(Token {
                    tok: TokenType::EOF,
                    lexeme: String::new(),
                    line: usize::MAX,
                    literal: None,
                });
                self.parser = Parser::new(scanned.token);
                while !self.parser.is_at_end() {
                    let expr = self.parser.parse().unwrap();
                    if let ExprPossibilities::Scope(scope) = expr {
                        if TokenType::FUNC == scope.stmt || TokenType::CLOS == scope.stmt {
                            interpreter.interpret(&ExprPossibilities::Scope(scope.clone()));
                        }
                    }
                }
                global_interp
                    .globals
                    .define_env(split_file[0], interpreter.globals.vars);
            }
        }
    }
}
