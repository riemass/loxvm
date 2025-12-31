use clap::{Parser, Subcommand};
use loxemu::{Lexer, compiler, vm};
use std::fs;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Lex { filename: PathBuf },
    Parse { filename: PathBuf },
    Run { filename: PathBuf },
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    match args.command {
        Commands::Lex { filename } => {
            let file_contents = fs::read_to_string(filename)?;
            let lexer = Lexer::new(&file_contents);
            let tokens: Result<Vec<_>, _> = lexer.collect();
            let tokens = tokens.unwrap();
            for token in tokens {
                println!("{token}");
            }
        }
        Commands::Parse { filename } => {
            let file_contents = fs::read_to_string(filename)?;
            let mut parser = loxemu::Parser::new(&file_contents);
            let expr = parser.expression().expect("Failed to parse expreesion");
            println!("{expr}");
        }
        Commands::Run { filename } => {
            // TODO: Implement eval loop
            let file_contents = fs::read_to_string(filename)?;
            let mut parser = loxemu::Parser::new(&file_contents);
            let stmt = parser.statement().expect("Failed to parse expreesion");
            let mut chunk = compiler::compile(&stmt);
            // TODO: Manually appending Return. Fix when complete.
            chunk.emit(vm::OpCode::Return);
            let mut vm = vm::VM::new(chunk);
            vm.interpret().unwrap();
            if vm.stack.len() == 1 {
                println!("Top of the stack: {:?}", vm.stack.first());
            }
        }
    }

    Ok(())
}
