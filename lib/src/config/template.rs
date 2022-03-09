mod format;

use format::RegularPathFormat;
use std::convert::{AsRef, TryFrom};
use std::path::{Path, PathBuf};
use anyhow::{bail, Error, Result};
use chrono::{Date, Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc};
use once_cell::sync::OnceCell;
use regex::{escape, Regex};

/// Preset template for regularly log like daily log
pub struct RegularLogTemplate {

    path_format: Option<RegularPathFormat>,

    /// If true, this log is automatically created with update command
    /// Auto migration require `path_format` including `.md` extension 
    auto_migration: bool,

    /// If set, soft link to the latest file will be created or updated by each `update`
    link_path: Option<PathBuf>,
}


impl RegularLogTemplate {

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
            path_format: Some(RegularPathFormat::new(s)?),
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


}