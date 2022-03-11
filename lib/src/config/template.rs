mod format;

use format::RegularPathFormat;

use std::path::{PathBuf};
use anyhow::{Result};



use serde::{Deserialize, Serialize};


/// Preset template for regularly log like daily log
#[derive(Deserialize, Serialize, Debug,)]
pub struct RegularLogTemplate {



    /// If true, this log is automatically created with update command
    /// Auto migration require `path_format` including `.md` extension 
    auto_migration: bool,

    /// If set, soft link to the latest file will be created or updated by each `update`
    link_path: Option<PathBuf>,
    
    path_format: Option<RegularPathFormat>,
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


#[cfg(test)]
mod tests {
    use super::*;


}