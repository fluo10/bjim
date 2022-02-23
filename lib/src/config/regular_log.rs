use std::convert::{AsRef, TryFrom};
use std::path::{Path, PathBuf};
use anyhow::{Error, Result};
use chrono::{Date, Local, NaiveDate};


enum FormatKind {
    YearMonthDay,
    YearMOnth,
    Year,
    YearWeekDay,
    YearWeek,
    IsoYearWeek,
}

impl TryFrom<&str> for FormatKind {
    type Error = Error;
    fn try_from(s: &str) -> Result<FormatKind> {
        todo!();
    } 
}

impl TryFrom<PathBuf> for FormatKind {
    type Error = Error;
    fn try_from(s: PathBuf) -> Result<FormatKind> {
        todo!();
    } 
}

pub struct RegularLogTemplate {
    format: String,
    format_kind: FormatKind,
}

impl RegularLogTemplate {
    pub fn get_path(date: NaiveDate) -> PathBuf {
        todo!();
    }
    
}