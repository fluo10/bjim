mod bullet;
mod note;
mod section;
mod task;

pub enum PageContent {
    sections: Vec<Section>
    
}


trait HasTask{
    pub fn is_open(&self) => bool;
}