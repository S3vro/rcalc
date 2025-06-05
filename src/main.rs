use clap::Parser;

use rcalc::lexer::Lexer;


#[derive(Parser, Debug)]
#[command()]
struct Args {
    input_file: String
}

fn main() {
    let args = Args::parse();
    let mut lexer = Lexer::new("10-1+3".to_string());

    while let Some(t) = lexer.next_token() {
        println!("Found token: {:?}", t);
    }

    println!("Now opening file {}", args.input_file);
}
