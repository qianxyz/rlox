mod expr;
mod parser;
mod scanner;
mod token;

use std::io::{self, Write};
use std::{env, error, fs, result};

use crate::scanner::Scanner;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

fn main() {
    let mut args = env::args();

    // skip the first arg (binary name)
    args.next();

    match args.next() {
        Some(path) => run_file(&path),
        None => run_prompt(),
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).unwrap();
    run(source);
}

fn run_prompt() {
    loop {
        // print prompt
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => run(buffer),
            Err(_) => todo!(),
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
