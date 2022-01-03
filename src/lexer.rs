use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    pointer: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut s = Self {
            source: input.chars().collect(),
            pointer: 0,
            char: '\0',
        };

        s.char = s.source[s.pointer];
        s
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pointer < self.source.len() {
            match self.char {
                '#' => {
                    let mut buffer = String::from(self.char);
                    self.read();
                    buffer.push_str(&self.consume_until(|b, c| b.len() < 7 && c == '#'));

                    // TODO: This is a weird way to approach it?
                    if buffer.len() == 7 {
                        let remainder = self.consume_until(|_, c| c != '\n');
                        buffer.push_str(&remainder);
                        tokens.push(Token::Paragraph(buffer));
                    } else {
                        match self.char {
                            _ if self.char.is_ascii_whitespace() => {
                                self.read();
                                tokens.push(Token::Heading((
                                    buffer,
                                    self.consume_until(|_, c| c != '\n'),
                                )));
                            }
                            // Treat as string
                            _ => todo!(),
                        }
                    }
                }
                _ if self.char.is_ascii_alphanumeric() => {
                    let buffer = self.consume_until(|_b, c| c != '\n');
                    tokens.push(Token::Paragraph(buffer));
                }
                '\n' => {
                    self.read();
                }
                _ => {
                    self.read();
                }
            }
        }

        tokens
    }

    fn consume_until(&mut self, accept: impl Fn(&str, char) -> bool) -> String {
        let mut buffer = String::new();

        while self.pointer < self.source.len() && accept(&buffer, self.char) {
            buffer.push(self.char);
            self.read();
        }

        buffer
    }

    fn read(&mut self) {
        self.pointer += 1;
        if self.pointer < self.source.len() {
            self.char = self.source[self.pointer];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_h1() {
        assert_eq!(
            Lexer::new("# Hello World").lex(),
            vec![Token::Heading(("#".to_string(), "Hello World".to_string()))]
        );
    }

    #[test]
    fn tokenizes_h2() {
        assert_eq!(
            Lexer::new("## Hello World").lex(),
            vec![Token::Heading((
                "##".to_string(),
                "Hello World".to_string()
            ))]
        );
    }

    #[test]
    fn tokenizes_h3() {
        assert_eq!(
            Lexer::new("### Hello World").lex(),
            vec![Token::Heading((
                "###".to_string(),
                "Hello World".to_string()
            ))]
        );
    }

    #[test]
    fn tokenizes_h4() {
        assert_eq!(
            Lexer::new("#### Hello World").lex(),
            vec![Token::Heading((
                "####".to_string(),
                "Hello World".to_string()
            ))]
        );
    }

    #[test]
    fn tokenizes_h5() {
        assert_eq!(
            Lexer::new("##### Hello World").lex(),
            vec![Token::Heading((
                "#####".to_string(),
                "Hello World".to_string()
            ))]
        );
    }

    #[test]
    fn tokenizes_h6() {
        assert_eq!(
            Lexer::new("###### Hello World").lex(),
            vec![Token::Heading((
                "######".to_string(),
                "Hello World".to_string()
            ))]
        );
    }

    #[test]
    fn parses_h7_as_paragraph() {
        assert_eq!(
            Lexer::new("####### Hello World").lex(),
            vec![Token::Paragraph("####### Hello World".to_string())]
        );
    }

    #[test]
    fn parses_h1_with_body() {
        assert_eq!(
            Lexer::new("# Hello World\nThis is the body.").lex(),
            vec![
                Token::Heading(("#".to_string(), "Hello World".to_string())),
                Token::Paragraph("This is the body.".to_string())
            ],
        );
    }
}
