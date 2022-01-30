use super::bullet::TaskStatus;

pub struct PageContent {
    pub raw: String,

}
impl PageContent {
    pub fn from_str(raw: &str) -> Self {
        Self{
            raw: String::from(raw),
        }
    }
    pub fn replace_task_status(&mut self, from: TaskStatus, to: TaskStatus) {

    }
    pub fn filter_open_tasks(&mut self) {

    }
    
}

