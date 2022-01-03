use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub text: String,
}

impl Tag {
    pub fn as_html(&self) -> String {
        let tag_name = &self.name;
        let opening_tag = format!("<{}>", tag_name);
        let closing_tag = format!("</{}>", tag_name);
        let mut html = String::from(&opening_tag);
        html.push_str(&self.text);
        html.push_str(&closing_tag);

        html
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_html())
    }
}

pub type Program = Vec<Tag>;

fn heading_pattern_to_name(pattern: &str) -> String {
    let count = pattern.chars().count().to_string();
    let name = format!("h{}", count);
    name
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
                        text: String::from(text),
                    });
                }
                Token::Paragraph(text) => {
                    program.push(Tag {
                        name: String::from("p"),
                        text: String::from(text),
                    });
                }
                Token::Code(code) => program.push(Tag {
                    name: String::from("code"),
                    text: String::from(code),
                }),
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
