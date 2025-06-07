use clap::{Subcommand, Parser};
use std::path::Path;
use std::fs;

use rcalc::lexer::Lexer;
use rcalc::parser::{TokenSource, self};
use rcalc::evaluator;



#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Debug, Subcommand)]
enum Command {
    File {
        path: Box<Path>,
    },
    Str {
        expr: String,
    }
}

fn calculate_expression(expr_str: &str) -> i32 {
    let lexer = Lexer::new(expr_str);
    let tokens = lexer.into_iter().collect::<Vec<_>>();
    let mut token_src = TokenSource::new(tokens);

    let root = parser::parse_expr(&mut token_src);
    println!("Now evaluating: {}", root);
    evaluator::evaluate(root)
}

fn str_command(expr_str: &str) {
    println!("The result is: {}", calculate_expression(expr_str))
}

fn file_command(path: &Box<Path>) {
    let bytes = fs::read(path).unwrap();
    let str = String::from_utf8(bytes).unwrap();
    println!("Reading: {:?}", *path); 
    println!("The result is: {}", calculate_expression(&str))
}

fn cli_command() {
    todo!("not implemented");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Str{expr}) => str_command(expr),
        Some(Command::File{path}) => file_command(path),
        None => cli_command(),
    }
}
