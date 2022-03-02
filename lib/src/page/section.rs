use std::iter::IntoIterator;

pub struct Section {
    heading: Option<String>,
    content: Vec<SectionContent>,
    children: Vec<Section>
}
pub enum SectionContent {
//    List(Vec<Entry>),
    Text(String),
}

impl From<&str> for Section {
    fn from(s: &str) -> Section {
        todo!(); 
    }
}

/*
impl IntoIterator for Section {
    type Item = Section;
    type IntoIter<'a> = SectionIter<'a>;
    fn into_iter(&self) -> SectionIter<'_> {
        SectionIter{
            children: std::slice::from_ref(self),
            parent: None,
        }
    }
}
struct SectionIter<'a>{
    cur: Option<Section>,
    children: &'a [Section],
    parent: Option<Box<SectionIter<'a>>>,
    used: bool,
}
impl <'a> Iterator for SectionIter<'a> {
    type Item= &'a Section;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.take() {
            Some(x) => {
                Some(x)
            },
            None => match self.children.get(0) {
                None => match self.parent.take() {
                    Some(parent) => {
                        *self = *parent;
                        self.next()
                    }
                    None => None,
                },
                Some(x) => {
                    self.children = &self.children[1..];
                    *self = SectionIter {
                        cur: x,
                        children: self.children.as_slice(),
                        parent: Some(Box::new(std::mem::take(self))),
                    };
                    self.next()
                }
            }
        }
    }
}
impl Default for SectionIter<'_> {
    fn default() -> Self {
        SectionIter{ cur: None, children: &[], parent: None }
    }
}
*/