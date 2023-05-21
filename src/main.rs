mod compiler;
mod interpreter;
mod logging;
mod scanner;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use scanner::Scanner;

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
fn run_file(mut file: File) -> Result<(), Box<dyn std::error::Error>> {
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    let scanner = Scanner::new(source);
    scanner.tokenize();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_repl(),
        2 => {
            let path = Path::new(args[1].as_str());
            let file = File::open(path)?;
            run_file(file)
        }
        _ => Ok(println!(
            "Usage:
           - script mode: clox [file path]
           - repl mode: clox"
        )),
    }
}
