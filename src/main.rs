use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod interpreter;

fn print_help() {
    println!("BrainrotLang CLI");
    println!("Usage:");
    println!("  brl <file.brl>           Run the specified .brl file");
    println!("  brl --run <file.brl>     Run the specified .brl file");
    println!("  brl --tokens <file.brl>  Print tokens for a .brl file");
    println!("  brl --ast <file.brl>     Print parsed AST from .brl");
    println!("  brl --version            Show version");
    println!("  brl --help               Show this help message");
}

fn print_version() {
    println!("BrainrotLang version 0.1.0");
}

fn read_code(file: &str) -> String {
    fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("Failed to read {}: {}", file, err);
        process::exit(1);
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "--help" | "-h" => print_help(),
        "--version" | "-v" => print_version(),
        "--tokens" => {
            if args.len() != 3 {
                eprintln!("Usage: brl --tokens <file.brl>");
                process::exit(1);
            }
            let code = read_code(&args[2]);
            let tokens = lexer::tokenize(&code);
            println!("{:#?}", tokens);
        }
        "--ast" => {
            if args.len() != 3 {
                eprintln!("Usage: brl --ast <file.brl>");
                process::exit(1);
            }
            let code = read_code(&args[2]);
            let tokens = lexer::tokenize(&code);
            let statements = parser::parse(&tokens);
            println!("{:#?}", statements);
        }
        "--run" => {
            if args.len() != 3 {
                eprintln!("Usage: brl --run <file.brl>");
                process::exit(1);
            }
            let code = read_code(&args[2]);
            let tokens = lexer::tokenize(&code);
            let statements = parser::parse(&tokens);
            let mut interp = interpreter::Interpreter::new();
            interp.run(&statements);
        }
        file if file.ends_with(".brl") => {
            let code = read_code(file);
            let tokens = lexer::tokenize(&code);
            let statements = parser::parse(&tokens);
            let mut interp = interpreter::Interpreter::new();
            interp.run(&statements);
        }
        _ => {
            eprintln!("Unknown command or missing .brl file.");
            print_help();
            process::exit(1);
        }
    }
}
