use std::fmt::Display;

trait HtmlTag {
    fn opening_tag(&self) -> String;
    fn closing_tag(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub text: String,
}

impl HtmlTag for Tag {
    fn opening_tag(&self) -> String {
        format!("<{}>", self.name)
    }

    fn closing_tag(&self) -> String {
        format!("</{}>", self.name)
    }
}

impl Tag {
    pub fn as_html(&self) -> String {
        vec![self.opening_tag(), self.text.clone(), self.closing_tag()].join("")
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_html())
    }
}