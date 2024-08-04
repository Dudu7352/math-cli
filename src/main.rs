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
        if buffer.trim() == "exit" {
            return;
        }
        let source = buffer.chars().collect();
        let scanner = TokenScanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(error) => {
                println!("Error while scanning the input: {:?}", error);
                continue;
            }
        };
        let mut parser = ExpressionParser::new(tokens);
        match parser.parse() {
            Ok(result) => println!("Result: {:?}", result),
            Err(error) => println!("Error encountered while parsing: {:?}", error),
        }
    }
}
