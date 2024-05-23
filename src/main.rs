use std::io::{self, Write};
use std::{env, fs};

use anyhow::{bail, Result};

use rlox::scanner::Scanner;

fn main() -> Result<()> {
    // usage: ./rlox [file.lox]
    // run script if provided, otherwise run REPL

    let mut args = env::args();

    // skip the first arg (binary name)
    args.next();

    match args.next() {
        Some(path) => run_file(path),
        None => run_repl(),
    }
}

fn run_file(path: String) -> Result<()> {
    let source = fs::read_to_string(path)?;
    run(source)
}

fn run_repl() -> Result<()> {
    loop {
        // print prompt
        print!("> ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        // if read 0 bytes, exit
        if io::stdin().read_line(&mut buffer)? == 0 {
            return Ok(());
        }

        // ignore error
        let _ = run(buffer);
    }
}

fn run(source: String) -> Result<()> {
    let scanner = Scanner::new(source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            bail!("scanner error")
        }
    };

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
