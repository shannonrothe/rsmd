type Heading = (String, String);

#[derive(Debug, PartialEq)]
pub enum Token {
    Heading(Heading),
    Paragraph(String),
    Code(String),
}
