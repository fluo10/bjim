mod front_matter;
mod task;

use front_matter::FrontMatter;
use task::Task;

pub struct Page {
    path: String;
    front_matter: FrontMatter;
    content: Content;
}

impl Page {
    pub fn new(path: String) => Journal {
        todo!
    }
    pub fn write($self) {
        todo!
    }
    pub fn migrate($self, path: String) {
        
    }
}