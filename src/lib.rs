#![feature(let_chains)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(iterator_try_collect)]

pub mod ast;
pub mod error_reporting;
pub mod import_sys;
pub mod interpreter;
pub mod lib_functions;
pub mod parser;
pub mod scanner;
