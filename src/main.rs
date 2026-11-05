use std::{env, error::Error, fs, process};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let filename = args.next().unwrap_or_else(|| {
        eprintln!("Not enough arguments. Usage: rinky <filename>");

        process::exit(1);
    });

    let contents = fs::read_to_string(&filename)?;

    println!("{:?}", contents);

    // TODO: tokenize

    Ok(())
}
