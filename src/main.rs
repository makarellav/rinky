use lexer::Lexer;
use std::{env, error::Error, fs, process};

mod lexer;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let filename = args.next().unwrap_or_else(|| {
        eprintln!("Not enough arguments. Usage: rinky <filename>");

        process::exit(1);
    });

    let contents = fs::read_to_string(&filename)?;

    let mut lexer = Lexer::new(contents);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }

    // TODO: tokenize

    Ok(())
}
