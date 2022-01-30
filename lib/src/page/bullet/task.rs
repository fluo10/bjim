pub struct Task{
    status: TaskStatus;
}

impl Task {
    pub fn read(line: String) => Task{
        todo!
    }
}

pub enum TaskStatus {
    Open,
    Closed,
    Migrated,
    Scheduled,
    InProgress
}

impl HasTask for Task{
    fn is_open(&self) -> bool {
        match self.
