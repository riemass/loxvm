use clap::Parser;
use loxemu::Lexer;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    filename: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    println!("Reading file: {}", args.filename);
    let file_contents = fs::read_to_string(&args.filename)?;

    let lexer = Lexer::new(&file_contents);
    let tokens: Result<Vec<_>, _> = lexer.collect();

    let tokens = tokens.unwrap();

    for token in tokens {
        println!("{token}");
    }

    Ok(())
}
