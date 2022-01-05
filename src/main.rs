use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use clap::{App, Arg, ArgMatches};

use crate::lexer::Lexer;
use crate::parser::Parser;

mod html;
mod lexer;
mod parser;
mod token;

fn parse_file(filename: &str, classes: HashMap<String, String>) -> io::Result<()> {
    let input_filename = Path::new(filename);
    let mut file = File::open(&input_filename).expect("[ERROR] Failed to read file");

    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut out_file = File::create(format!("{}.html", filename))?;
    let mut parser = Parser::new(Lexer::new(&input));
    for tag in parser.parse().into_iter() {
        out_file.write_all(tag.as_html(&classes).as_bytes())?;
    }

    Ok(())
}

fn parse_classes_from_args(matches: &ArgMatches) -> HashMap<String, String> {
    let mut classes = HashMap::new();
    let tags = vec![
        "h1", "h2", "h3", "h4", "h5", "h6", "p", "code", "ul", "ol", "li",
    ];

    for tag in tags.iter() {
        classes.insert(
            tag.to_string(),
            matches.value_of(tag).unwrap_or("").to_string(),
        );
    }

    classes
}

fn main() -> io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Shannon Rothe <shannon.michael.rothe@gmail.com>")
        .about("A basic markdown-to-html transpiler")
        .arg(Arg::new("filename"))
        .arg(Arg::new("h1").long("h1").takes_value(true))
        .arg(Arg::new("h2").long("h2").takes_value(true))
        .arg(Arg::new("h3").long("h3").takes_value(true))
        .arg(Arg::new("h4").long("h4").takes_value(true))
        .arg(Arg::new("h5").long("h5").takes_value(true))
        .arg(Arg::new("h6").long("h6").takes_value(true))
        .arg(Arg::new("p").short('p').long("paragraph").takes_value(true))
        .arg(Arg::new("code").short('c').long("code").takes_value(true))
        .arg(Arg::new("ul").short('u').long("ul").takes_value(true))
        .arg(Arg::new("ol").short('o').long("ol").takes_value(true))
        .arg(Arg::new("li").short('l').long("li").takes_value(true))
        .get_matches();

    let classes = parse_classes_from_args(&matches);

    if let Some(filename) = matches.value_of("filename") {
        parse_file(filename, classes)
    } else {
        Ok(())
    }
}
