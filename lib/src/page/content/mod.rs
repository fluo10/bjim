use regex::Regex;
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
    pub fn replace_task_status(&mut self, before: TaskStatus, after: TaskStatus) {
        let pattern = format!(r"(?m)^(- \[)[{}](\] .*)$", before);
        let replacement = format!("$1{}$2", after);
        let re = Regex::new(pattern.as_str()).unwrap();
        self.raw = re.replace_all(self.raw.as_str(), replacement.as_str()).to_string();
    }
    pub fn filter_open_tasks(&mut self) {

    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replace_task_status() {
        let mut before = PageContent::from_str(r"
- [ ] open-task
- [>] Migrated task
- [<] Scheduled task
- [/] Task in progress
- [x] Closed task");
        before.replace_task_status(TaskStatus::Open, TaskStatus::Scheduled);
        assert_eq!(before.raw, String::from(r"
- [x] open-task
- [>] Migrated task
- [<] Scheduled task
- [/] Task in progress
- [x] Closed task"));
    }

}