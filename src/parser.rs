use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub text: String,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag_name = self.name.as_str();
        let mut t = String::from(format!("<{}>", tag_name).as_str());
        t.push_str(self.text.as_str());
        t.push_str(format!("</{}>", tag_name).as_str());

        write!(f, "{}", t)
    }
}

pub type Program = Vec<Tag>;

fn heading_pattern_to_name(pattern: &str) -> String {
    match pattern {
        "#" => "h1".to_string(),
        "##" => "h2".to_string(),
        "###" => "h3".to_string(),
        "####" => "h4".to_string(),
        "#####" => "h5".to_string(),
        "######" => "h6".to_string(),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut program = Vec::new();

        for token in self.tokens.iter() {
            match token {
                Token::Heading((pattern, text)) => {
                    program.push(Tag {
                        name: heading_pattern_to_name(pattern),
                        text: text.clone(),
                    });
                }
                Token::Paragraph(text) => {
                    program.push(Tag {
                        name: "p".to_string(),
                        text: text.clone(),
                    });
                }
            }
        }

        Ok(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tags() {
        assert_eq!(
            Parser::new(vec![
                Token::Heading(("#".to_string(), "Hello World".to_string())),
                Token::Paragraph("This is the body".to_string())
            ])
            .parse(),
            Ok(vec![
                Tag {
                    name: "h1".to_string(),
                    text: "Hello World".to_string()
                },
                Tag {
                    name: "p".to_string(),
                    text: "This is the body".to_string()
                }
            ]),
        )
    }
}
