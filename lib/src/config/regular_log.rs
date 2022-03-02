use std::convert::{AsRef, TryFrom};
use std::path::{Path, PathBuf};
use anyhow::{Error, Result};
use chrono::{Date, Local, NaiveDate};

/// Preset template for regularly log like daily log
pub struct RegularLogTemplate {

    /// Use to generate path from date
    path_format: Option<String>,

    /// If true, this log is automatically created with update command
    /// Auto migration require `path_format` including `.md` extension 
    auto_migration: bool,

    /// If set, soft link to the latest file will be created or updated by each `update`
    link_path: Option<PathBuf>,
}

impl RegularLogTemplate {
    pub fn get_path(&self, date: NaiveDate) -> Result<PathBuf> {
        Ok(PathBuf::from(date.format(self.path_format.as_ref().unwrap().as_str()).to_string()))
    }
    pub fn get_today_path(&self, date: NaiveDate) -> Result<PathBuf> {
        todo!();
    }
    pub fn get_last_path(&self) -> Result<PathBuf> {
        todo!();
    }
    pub fn is_valid(&self) -> bool {
        todo!();
    }
    pub fn update_link(&self) -> Result<()> {
        todo!();
    }
    
}

impl Default for RegularLogTemplate {
    fn default() -> Self {
        RegularLogTemplate{
            path_format: None,
            auto_migration: false,
            link_path: None,
        }
    }
}


impl TryFrom<&str> for RegularLogTemplate {
    type Error = Error;
    fn try_from(s: &str) -> Result<RegularLogTemplate> {
        Ok(RegularLogTemplate{
            path_format: Some(s.to_string()),
            ..RegularLogTemplate::default()
        })
    } 
}

impl TryFrom<PathBuf> for RegularLogTemplate {
    type Error = Error;
    fn try_from(s: PathBuf) -> Result<RegularLogTemplate> {
        todo!();
    } 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_path() {
        fn test_format(format: &str, date: NaiveDate, path: PathBuf) {
            let template = RegularLogTemplate::try_from(format).unwrap();
            assert_eq!(template.get_path(date).unwrap(), path);
        }
        test_format(
            "%Y/%m/%d",
            NaiveDate::from_ymd(2022, 01, 01),
            PathBuf::from("2022/01/01"),
        );
    }
}