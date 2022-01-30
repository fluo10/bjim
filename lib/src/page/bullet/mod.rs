mod status;
pub use status::TaskStatus;

/*
pub struct Bullet{
    line: i64,
    raw_content: String,
    children: Vec<Bullet>,
    section: String,
    task_status: Option<TaskStatus>,
}


impl Bullet {
    pub fn is_task(&self) -> bool {
        match self.task_status {
            Some => true,
            None => false,
        }
    }
    pub fn is_open(&self) -> bool {
        match self.task_status {
            Some(x) => {
                match x {
                    TaskStatus::Open => true,
                    _ => false,
                }
            },
            None => false,
        }
    }
    pub fn migrate(&mut self){
        

    }
}

*/