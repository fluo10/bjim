use super::Inline;

pub struct ListItem {
    indent: Option<String>,
    bullet: Option<String>,
    space: Option<String>,
    checkbox: Option<String>,
    content: Vec<Inline>,
}