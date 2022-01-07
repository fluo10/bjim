mod bullet;
mod note;
mod section;
mod task;

pub struct JournalContent {
    sections: Vec<Section>
}

trait HasTask{
    pub fn is_open(&self) => bool;
}