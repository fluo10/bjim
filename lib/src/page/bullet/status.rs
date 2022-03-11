use std::fmt::{Display,Formatter};


pub enum Key {
    Task(TaskStatus),
    Note,
}

pub enum TaskStatus {
    Open,
    Closed,
    Migrated,
    Scheduled,
    InProgress,
    Canceled,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, " "),
            Self::Closed => write!(f, "x"),
            Self::Migrated => write!(f, ">"),
            Self::Scheduled => write!(f, "<"),
            Self::InProgress => write!(f, "/"),
            Self::Canceled => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fmt() {
        assert_eq!(format!{"{}", TaskStatus::Open} , " ");
        assert_eq!(format!{"{}", TaskStatus::Closed} , "x");
        assert_eq!(format!{"{}", TaskStatus::Migrated} , ">");
        assert_eq!(format!{"{}", TaskStatus::Scheduled} , "<");
        assert_eq!(format!{"{}", TaskStatus::InProgress} , "/");
        assert_eq!(format!{"{}", TaskStatus::Canceled} , "-");
    }
}