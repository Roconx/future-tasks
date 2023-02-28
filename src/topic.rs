use std::fmt;

#[derive(PartialEq)]
pub enum Topic {
    All,
    Topic(String),
    Late,
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Topic::All => write!(f, "All topics"),
            Topic::Topic(topic) => write!(f, "{}", topic),
            Topic::Late => write!(f, "Late"),
        }
    }
}
