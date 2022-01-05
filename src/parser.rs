use crate::html::{Node, Tag};
use crate::lexer::Lexer;
use crate::token::{ListType, Token};

pub type Program = Vec<Tag>;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    ast: Program,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            ast: Program::new(),
        }
    }

    pub fn parse(&mut self) -> Program {
        while let Some(token) = self.lexer.next() {
            match token {
                Token::Heading(text) => self.parse_heading(text),
                Token::Text(text) => self.parse_text(Node::Text(text)),
                Token::Code(text) => self.parse_code(text),
                Token::Strong(text) => self.parse_text(Node::Strong(text)),
                Token::Em(text) => self.parse_text(Node::Em(text)),
                Token::List(list_type) => self.parse_list(list_type),
                Token::NewLine => {}
            }
        }

        self.ast.to_vec()
    }

    fn parse_heading(&mut self, text: String) {
        let level = text.chars().count() as u8;

        match self.lexer.next() {
            Some(Token::Text(text)) => self.ast.push(Tag {
                node: Node::Heading(level),
                children: vec![Node::Text(text)],
            }),
            Some(_) => panic!(""),
            None => panic!(""),
        }
    }

    fn parse_code(&mut self, text: String) {
        let t = text.clone();
        self.ast.push(Tag {
            node: Node::Code(text),
            children: vec![Node::Text(t)],
        });
    }

    fn parse_text(&mut self, initial: Node) {
        let mut parent = Tag {
            node: Node::Paragraph,
            children: vec![initial],
        };

        while let Some(token) = self.lexer.next() {
            match token {
                Token::NewLine => break,
                initial => {
                    let mut nodes = self.parse_inline_nodes(&initial);
                    parent.children.append(&mut nodes);
                }
            }
        }

        self.ast.push(parent)
    }

    fn parse_list(&mut self, list_type: ListType) {
        let tag = match list_type {
            ListType::Ordered => Tag {
                node: Node::OrderedList,
                children: self.parse_list_items(),
            },
            ListType::Unordered => Tag {
                node: Node::UnorderedList,
                children: self.parse_list_items(),
            },
        };

        self.ast.push(tag);
    }

    fn parse_list_items(&mut self) -> Vec<Node> {
        let mut list_items = Vec::new();
        let mut nodes = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                Token::NewLine => {
                    list_items.push(Node::ListItem(nodes.clone()));
                    nodes.clear();

                    if let Some(token) = self.lexer.next() {
                        if token == Token::NewLine {
                            break;
                        }
                    }
                }
                Token::List(_) => {
                    list_items.append(&mut self.parse_list_items());
                }
                _ => {
                    if let Some(inline_node) = self.parse_inline_node(&token) {
                        nodes.push(inline_node);
                    }
                }
            }
        }

        if !nodes.is_empty() {
            list_items.push(Node::ListItem(nodes.clone()));
            nodes.clear();
        }

        list_items
    }

    /// Parse all inline tokens into nodes until a new line is hit.
    fn parse_inline_nodes(&mut self, initial: &Token) -> Vec<Node> {
        let mut nodes = Vec::new();

        if let Some(node) = self.parse_inline_node(initial) {
            nodes.push(node);
        }

        while let Some(token) = self.lexer.next() {
            if let Some(node) = self.parse_inline_node(&token) {
                nodes.push(node);
            }

            if token == Token::NewLine {
                break;
            }
        }

        nodes
    }

    fn parse_inline_node(&mut self, token: &Token) -> Option<Node> {
        match token {
            Token::Text(text) => Some(Node::Text(text.to_string())),
            Token::Code(text) => Some(Node::Code(text.to_string())),
            Token::Strong(text) => Some(Node::Strong(text.to_string())),
            Token::Em(text) => Some(Node::Em(text.to_string())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_list() {
        assert_eq!(
            Parser::new(Lexer::new("some text followed by\n* item 1\n* item 2").into_iter())
                .parse(),
            vec![
                Tag {
                    node: Node::Paragraph,
                    children: vec![Node::Text("some text followed by".to_string())]
                },
                Tag {
                    node: Node::UnorderedList,
                    children: vec![
                        Node::ListItem(vec![Node::Text("item 1".to_string())]),
                        Node::ListItem(vec![Node::Text("item 2".to_string())])
                    ]
                }
            ]
        );
    }
}
