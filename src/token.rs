#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ListType {
    Unordered,
    Ordered,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Heading(String),
    Text(String),
    Strong(String),
    Em(String),

    Code(String),

    List(ListType),

    NewLine,
}
