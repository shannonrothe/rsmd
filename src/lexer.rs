use std::str;

use crate::token::{ListType, Token};

#[derive(Clone, Debug)]
pub struct Lexer {
    source: String,
    pointer: usize,
    slice: u8,
}

const MAX_HEADING_LEVEL: u8 = 7;

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut s = Self {
            source: input.to_string(),
            pointer: 0,
            slice: b'\0',
        };

        s.slice = s.source.as_bytes()[s.pointer];
        s
    }

    pub fn next_token(&mut self) -> Token {
        match self.slice {
            _ if self.slice.is_ascii_digit() => {
                self.advance();

                if self.slice == b'.' {
                    self.advance();
                    self.advance();
                    Token::List(ListType::Ordered)
                } else {
                    let buffer = self.consume_until_one_of(&[b'\n', b'*', b'_', b'`']);
                    Token::Text(buffer)
                }
            }
            b'#' => {
                let mut heading = self.consume_until_not_one_of(&[b'#']);
                if heading.len() >= MAX_HEADING_LEVEL.into() {
                    let remainder = self.consume_until_one_of(&[b'\n']);
                    heading.push_str(&remainder);
                    Token::Text(heading)
                } else {
                    self.advance();
                    Token::Heading(heading)
                }
            }
            b'`' => {
                self.advance();
                let buffer = self.consume_until_one_of(&[b'`']);
                self.advance();
                Token::Code(buffer)
            }
            b'_' => {
                self.advance();
                let buffer = self.consume_until_one_of(&[b'_']);
                self.advance();
                Token::Em(buffer)
            }
            b'-' => {
                self.advance();

                match self.slice {
                    _ if self.slice.is_ascii_whitespace() => {
                        self.advance();
                        Token::List(ListType::Unordered)
                    }
                    _ => {
                        let buffer = self.consume_until_one_of(&[b'\n', b'*', b'_']);
                        Token::Text(buffer)
                    }
                }
            }
            b'*' => {
                self.advance();

                match self.slice {
                    _ if self.slice.is_ascii_whitespace() => {
                        self.advance();
                        Token::List(ListType::Unordered)
                    }
                    b'*' => {
                        self.advance();
                        let buffer = self.consume_until_one_of(&[b'*']);
                        self.advance();
                        self.advance();
                        // println!("strong! {}", buffer);
                        Token::Strong(buffer)
                    }
                    _ => {
                        let buffer = self.consume_until_one_of(&[b'\n', b'*']);
                        match self.slice {
                            b'\n' => {
                                self.advance();
                                Token::Text(buffer)
                            }
                            b'*' => {
                                self.advance();
                                Token::Em(buffer)
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            b'\n' => {
                self.advance();
                Token::NewLine
            }
            _ => {
                let buffer = self.consume_until_one_of(&[b'\n', b'*', b'_', b'`']);
                Token::Text(buffer)
            }
        }
    }

    fn consume_until_one_of(&mut self, one_of: &[u8]) -> String {
        let mut buffer = String::new();

        while self.pointer < self.source.len() && !one_of.contains(&self.slice) {
            buffer.push_str(&self.source[self.pointer..self.pointer + 1]);
            self.advance();
        }

        buffer
    }

    fn consume_until_not_one_of(&mut self, one_of: &[u8]) -> String {
        let mut buffer = String::new();

        while self.pointer < self.source.len() && one_of.contains(&self.slice) {
            buffer.push_str(&self.source[self.pointer..self.pointer + 1]);
            self.advance();
        }

        buffer
    }

    fn advance(&mut self) {
        self.pointer += 1;
        if self.pointer < self.source.len() {
            self.slice = self.source.as_bytes()[self.pointer];
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer >= self.source.len() {
            None
        } else {
            Some(self.next_token())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_h1() {
        let mut iter = Lexer::new("# Hello World").into_iter();
        assert_eq!(Some(Token::Heading("#".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello World".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_h2() {
        let mut iter = Lexer::new("## Hello World").into_iter();
        assert_eq!(Some(Token::Heading("##".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello World".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_h3() {
        let mut iter = Lexer::new("### Hello World").into_iter();
        assert_eq!(Some(Token::Heading("###".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello World".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_h4() {
        let mut iter = Lexer::new("#### Hello World").into_iter();
        assert_eq!(Some(Token::Heading("####".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello World".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_h5() {
        let mut iter = Lexer::new("##### Hello World").into_iter();
        assert_eq!(Some(Token::Heading("#####".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello World".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_h6() {
        let mut iter = Lexer::new("###### Hello World").into_iter();
        assert_eq!(Some(Token::Heading("######".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello World".to_string())), iter.next());
    }

    #[test]
    fn parses_h7_as_paragraph() {
        let mut iter = Lexer::new("####### Hello World").into_iter();
        assert_eq!(
            Some(Token::Text("####### Hello World".to_string())),
            iter.next()
        );
    }

    #[test]
    fn h1_with_body() {
        let mut iter = Lexer::new("# Hello World\nThis is the body.").into_iter();
        assert_eq!(Some(Token::Heading("#".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello World".to_string())), iter.next());
        assert_eq!(Some(Token::NewLine), iter.next());
        assert_eq!(
            Some(Token::Text("This is the body.".to_string())),
            iter.next()
        );
    }

    #[test]
    fn tokenizes_code() {
        let mut iter = Lexer::new("# Hello world\n`struct Foo {}`").into_iter();
        assert_eq!(Some(Token::Heading("#".to_string())), iter.next());
        assert_eq!(Some(Token::Text("Hello world".to_string())), iter.next());
        assert_eq!(Some(Token::NewLine), iter.next());
        assert_eq!(Some(Token::Code("struct Foo {}".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_bold_within_paragraph() {
        let mut iter = Lexer::new("hello **world**").into_iter();
        assert_eq!(Some(Token::Text("hello ".to_string())), iter.next());
        assert_eq!(Some(Token::Strong("world".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_paragraph_starting_with_bold() {
        let mut iter = Lexer::new("**bold** text!").into_iter();
        assert_eq!(Some(Token::Strong("bold".to_string())), iter.next());
        assert_eq!(Some(Token::Text(" text!".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_em_within_paragraph() {
        let mut iter = Lexer::new("hello *world*").into_iter();
        assert_eq!(Some(Token::Text("hello ".to_string())), iter.next());
        assert_eq!(Some(Token::Em("world".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_em_and_strong_within_paragraph() {
        let mut iter = Lexer::new("**bold** text, and *italic* text!").into_iter();
        assert_eq!(Some(Token::Strong("bold".to_string())), iter.next());
        assert_eq!(Some(Token::Text(" text, and ".to_string())), iter.next());
        assert_eq!(Some(Token::Em("italic".to_string())), iter.next());
        assert_eq!(Some(Token::Text(" text!".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_em_with_underscores() {
        let mut iter = Lexer::new("**bold** text, and _italic_ text!").into_iter();
        assert_eq!(Some(Token::Strong("bold".to_string())), iter.next());
        assert_eq!(Some(Token::Text(" text, and ".to_string())), iter.next());
        assert_eq!(Some(Token::Em("italic".to_string())), iter.next());
        assert_eq!(Some(Token::Text(" text!".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_unordered_list() {
        let mut iter = Lexer::new("some text followed by\n* item 1\n* item 2").into_iter();
        assert_eq!(
            Some(Token::Text("some text followed by".to_string())),
            iter.next()
        );
        assert_eq!(Some(Token::NewLine), iter.next());
        assert_eq!(Some(Token::List(ListType::Unordered)), iter.next());
        assert_eq!(Some(Token::Text("item 1".to_string())), iter.next());
        assert_eq!(Some(Token::NewLine), iter.next());
        assert_eq!(Some(Token::List(ListType::Unordered)), iter.next());
        assert_eq!(Some(Token::Text("item 2".to_string())), iter.next());
    }

    #[test]
    fn tokenizes_ordered_list() {
        let mut iter = Lexer::new("some text followed by\n1. item 1\n2. item 2").into_iter();
        assert_eq!(
            Some(Token::Text("some text followed by".to_string())),
            iter.next()
        );
        assert_eq!(Some(Token::NewLine), iter.next());
        assert_eq!(Some(Token::List(ListType::Ordered)), iter.next());
        assert_eq!(Some(Token::Text("item 1".to_string())), iter.next());
        assert_eq!(Some(Token::NewLine), iter.next());
        assert_eq!(Some(Token::List(ListType::Ordered)), iter.next());
        assert_eq!(Some(Token::Text("item 2".to_string())), iter.next());
    }
}
