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
        let escaped = regex::escape(format!("{}", before).as_str());
        let pattern = format!(r"(?m)^(\s*- \[){}(\] .*)$", escaped);
        //let pattern = format!(r"(?m)^(- \[)[{}](\] .*)$", before);
        let replacement = "${1}".to_string() + format!("{}", after).as_str() + "${2}";
        let re = Regex::new(pattern.as_str()).unwrap();
        self.raw = re.replace_all(self.raw.as_str(), replacement.as_str()).to_string();
    }

    /// Delete other lines, leaving only the active task, heading and blank lines
    pub fn filter_open_tasks(&mut self) {
        let pattern = r##"(?m)(?:^\s*- \[ |/\] .*$)|(?:^\s*$)|(?:^#+ .*$)"##;
        let re = Regex::new(pattern).unwrap();
        let mut result = String::new();
        for caps in re.captures_iter(self.raw.as_str()) {
            result = result + "\n" + &caps[0];
        }
        self.raw = result;
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replace_task_status() {

        fn test_by_status(before: TaskStatus, after:TaskStatus) {
            const origin:&str = r"
- [ ] Open task
- [>] Migrated task
- [<] Scheduled task
- [/] Task in progress
- [x] Closed task";
            let result = match (& before, & after) {
                (TaskStatus::Closed, TaskStatus::InProgress) => r"
- [ ] Open task
- [>] Migrated task
- [<] Scheduled task
- [/] Task in progress
- [/] Closed task",
                (TaskStatus::InProgress, TaskStatus::Migrated) => r"
- [ ] Open task
- [>] Migrated task
- [<] Scheduled task
- [>] Task in progress
- [x] Closed task",
                (TaskStatus::Migrated, TaskStatus::Open,) => r"
- [ ] Open task
- [ ] Migrated task
- [<] Scheduled task
- [/] Task in progress
- [x] Closed task",
                (TaskStatus::Open, TaskStatus::Scheduled) => r"
- [<] Open task
- [>] Migrated task
- [<] Scheduled task
- [/] Task in progress
- [x] Closed task",
                (TaskStatus::Scheduled, TaskStatus::Closed) => r"
- [ ] Open task
- [>] Migrated task
- [x] Scheduled task
- [/] Task in progress
- [x] Closed task",
                _ => panic!(),
            };
            let mut content = PageContent::from_str(origin.clone());
            content.replace_task_status(before, after);
            assert_eq!(content.raw, String::from(result));
        }
        test_by_status(TaskStatus::Closed, TaskStatus::InProgress);
        test_by_status(TaskStatus::InProgress, TaskStatus::Migrated);
        test_by_status(TaskStatus::Migrated, TaskStatus::Open,);
        test_by_status(TaskStatus::Open, TaskStatus::Scheduled);
        test_by_status(TaskStatus::Scheduled, TaskStatus::Closed);

    }

    #[test]
    fn filter_open_tasks() {
        let mut content = PageContent::from_str(r##"## Section1

- [ ] Open task
- [>] Migrated task
- [<] Scheduled task
- [/] Task in progress
- [x] Closed task"##);
        content.filter_open_tasks();
        assert_eq!(content.raw, r##"## Section1

- [ ] Open task"##);
    }

}