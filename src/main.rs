#![feature(linked_list_cursors)]
use std::{io, process::exit};

use parser::ExpressionParser;
use scanner::TokenScanner;

mod error;
mod parser;
mod scanner;
mod tokens;

fn main() {
    repl();
}

fn repl() {
    let input = io::stdin();
    loop {
        let mut buffer = String::new();
        print!("mathshell > ");
        io::Write::flush(&mut io::stdout()).expect("Cannot flush terminal");
        if let Err(err) = input.read_line(&mut buffer) {
            println!("Error while reading STDIN: {:?}", err);
            exit(1);
        }
        if buffer == "exit" {
            return;
        }
        let source = buffer.chars().collect();
        let scanner = TokenScanner::new(source);
        let mut parser = ExpressionParser::new(scanner.scan_tokens());
        match parser.parse() {
            Ok(val) => println!("Result: {:?}", val),
            Err(err) => println!("Error encountered while parsing: {:?}", err),
        }
    }
}
