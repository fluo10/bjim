use std::fmt::{Display,Formatter}
use std::str::FromStr;



pub enum Key {
    Task(TaskStatus),
    Note,
}




impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Key {}
        }
    }
}

pub enum TaskStatus {
    Open,
    Closed,
    Migrated,
    Scheduled,
    InProgress,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, " "),
            Self::Closed => write!(f, "x"),
            Self::Migrated => write!(f, ">"),
            Self::Scheduled => write!(f, "<"),
            Self::InProgress => write!(f, "/"),
        }
    }
}