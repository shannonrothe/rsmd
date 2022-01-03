use crate::html::Tag;
use crate::token::Token;

pub type Program = Vec<Tag>;

fn heading_pattern_to_name(pattern: &str) -> String {
    format!("h{}", pattern.chars().count())
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
