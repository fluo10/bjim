pub enum BlockKind {
    Section(PageSection),
    List(PageList),
    Paragraph(String),
}

trait Block {
    fn has_task(&self) -> bool;
    fn is_task(&self) -> bool;
    fn get_note(&self) -> bool;
    fn has_note(&self) -> bool;
    fn is_note(&self) -> bool;

_
impl hasTask for BlockKind {
    fn has_task(&self) -> bool {
        match self {
            BlockKind::Section => 
        }
    }
}
