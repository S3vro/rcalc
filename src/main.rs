use clap::Parser;

use rcalc::lexer::Lexer;
use rcalc::parser::{TokenSource, self};


#[derive(Parser, Debug)]
#[command()]
struct Args {
    input_file: String
}

fn main() {
    let args = Args::parse();
    let lexer = Lexer::new("3*10-1+3");
    let tokens = lexer.into_iter().collect::<Vec<_>>();
    let mut token_src = TokenSource::new(tokens);

    let root = parser::parse_expr(&mut token_src);
    println!("Parsed to: {:?}", root);
}
