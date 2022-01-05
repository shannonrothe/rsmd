use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Heading(u8),
    Paragraph,
    Text(String),
    Strong(String),
    Em(String),

    Code(String),

    OrderedList,
    UnorderedList,
    ListItem(Vec<Node>),
}

impl Node {
    pub fn render(&self, classes: &HashMap<String, String>) -> String {
        let root_classes = &classes.get(&self.to_string());

        match self {
            Node::Text(text) => text.to_string(),
            Node::Code(code) => self.tag(&classes.get(&self.to_string()), code.to_string()),
            Node::ListItem(nodes) => {
                let children: Vec<String> = nodes.iter().map(|n| n.render(classes)).collect();
                self.tag(root_classes, children.join(""))
            }
            Node::Em(text) => self.tag(root_classes, text.to_string()),
            Node::Strong(text) => self.tag(root_classes, text.to_string()),
            _ => "".to_string(),
        }
    }

    fn tag(&self, classes: &Option<&String>, text: String) -> String {
        format!(
            "{}{}{}",
            self.opening_tag(classes),
            text,
            self.closing_tag()
        )
    }

    fn opening_tag(&self, classes: &Option<&String>) -> String {
        let mut tag = format!("<{}", self.to_string());

        if let Some(classes) = classes {
            if !classes.is_empty() {
                tag.push_str(format!(" class=\"{}\"", classes).as_str());
            }
        }

        tag.push('>');
        tag
    }

    fn closing_tag(&self) -> String {
        format!("</{}>", self.to_string())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Node::Text(text) => text.to_string(),
            Node::Paragraph => "p".to_string(),
            Node::Code(_) => "code".to_string(),
            Node::Heading(level) => format!("h{}", level),
            Node::Strong(_) => "strong".to_string(),
            Node::Em(_) => "em".to_string(),
            Node::OrderedList => "ol".to_string(),
            Node::UnorderedList => "ul".to_string(),
            Node::ListItem(_) => "li".to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub node: Node,
    pub children: Vec<Node>,
}

impl Tag {
    pub fn as_html(&self, classes: &HashMap<String, String>) -> String {
        let children: Vec<String> = self.children.iter().map(|n| n.render(classes)).collect();
        vec![
            self.opening_tag(&classes.get(&self.node.to_string())),
            children.join(""),
            self.closing_tag(),
        ]
        .join("")
    }

    pub fn opening_tag(&self, classes: &Option<&String>) -> String {
        let mut tag = format!("<{}", self.node.to_string());

        if let Some(classes) = &classes {
            if !classes.is_empty() {
                tag.push_str(format!(" class=\"{}\"", classes).as_str());
            }
        }

        tag.push('>');
        tag
    }

    pub fn closing_tag(&self) -> String {
        format!("</{}>\n", self.node.to_string())
    }
}
