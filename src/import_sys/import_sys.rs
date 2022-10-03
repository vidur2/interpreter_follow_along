use std::collections::{HashSet, VecDeque};

use crate::{
    ast::expr_types::ExprPossibilities,
    parser::parser::Parser,
    scanner::{scanner::Scanner, token::TokenType},
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
        parsed: &mut VecDeque<ExprPossibilities>,
    ) {
        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            let path = path.unwrap().path().into_os_string().into_string().unwrap();
            let split_path: Vec<&str> = path.split("/").collect();
            let file_name = split_path.last().unwrap().to_string();
            let split_file: Vec<&str> = file_name.split(".").collect();
            if split_file.last().unwrap() == &super::FileExtenstion && files.contains(split_file[0])
            {
                let mut scanned = Scanner::input_file(&path).unwrap();
                scanned.tokenize_buff();
                self.parser = Parser::new(scanned.token);

                while !self.parser.is_at_end() {
                    let expr = self.parser.parse().unwrap();
                    if let ExprPossibilities::Scope(scope) = expr {
                        if TokenType::FUNC == scope.stmt || TokenType::CLOS == scope.stmt {
                            parsed.push_front(ExprPossibilities::Scope(scope));
                        }
                    }
                }
            }
        }
    }
}
