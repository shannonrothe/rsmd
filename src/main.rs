use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod token;

fn parse_file(filename: &String) -> io::Result<()> {
    let input_filename = Path::new(filename);
    let mut file = File::open(&input_filename).expect("[ERROR] Failed to read file");

    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut output_filename = String::from(&filename[..]);
    output_filename.push_str(".html");

    let mut out_file = File::create(output_filename)?;
    let mut lexer = Lexer::new(input.as_str());
    let mut parser = Parser::new(lexer.lex());

    if let Ok(program) = parser.parse() {
        for tag in program.iter() {
            out_file.write_all(&format!("{}", tag).as_bytes())?;
        }
    }

    Ok(())
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("): A basic markdown-to-html compiler.");

    title
}

fn print_help() {
    println!("{}\n", get_title());
    println!("Usage: `rsmd file.md`");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => parse_file(&args[1]),
        _ => {
            print_help();
            Ok(())
        }
    }
}
