mod note;
mod task;

pub use note::Note;
pub use task::{Task, TaskStatus};
pub struct Bullet{
    line: i64,
    raw_content: String,
    content: BulletContent,
    children: Vec<Bullet>
}

pub enum BulletContent {
    Note(Note),
    Task(Task),
}

impl HasTask for Task{

    fn is_open(&self) -> bool {
        match self.content{
            Task(task) -> {
                if Task.is_open() {
                    return true;
                }
            }
        }
        for bullet in children {
            if bullet.is_open() {
                return true;
            }
        }
        return false;
    }
}
