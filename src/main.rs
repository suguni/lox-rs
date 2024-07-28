use std::{env, io};
use std::fs::File;
use std::io::Write;

use crate::scanner::Scanner;

mod token;
mod scanner;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: {} [script]", args[0]);
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).unwrap();
    } else {
        run_prompt().unwrap();
    }
}

fn run_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let source = io::read_to_string(file)?;
    run(source)
}

fn run_prompt() -> io::Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        if line.len() == 0 {
            break
        }

        run(line)?;
    }
    Ok(())
}

fn run(source: String) -> io::Result<()> {
    let scanner = Scanner::new(&source);
    for token in scanner.tokens.iter() {
        println!("{}", token);
    }
    Ok(())
}

