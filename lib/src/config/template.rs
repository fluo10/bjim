mod format;

pub use format::RegularPathFormat;

use crate::{Config, Page};
use std::fs::remove_file;
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink;
#[cfg(unix)]
use std::os::unix::fs::symlink;

use anyhow::{anyhow, bail, Result};
use log::{info, trace, warn,};
use serde::{Deserialize, Serialize};


/// Preset template for regularly log like daily log
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct RegularLogTemplate {



    /// If true, this log is automatically created with update command
    /// Auto migration require `path_format` including `.md` extension 
    pub auto_migration: bool,

    /// If set, soft link to the latest file will be created or updated by each `update`
    pub link_path: Option<PathBuf>,
    
    pub path_format: Option<RegularPathFormat>,
}


impl RegularLogTemplate {

    pub fn is_valid(&self) -> bool {
        todo!();
    }
    pub fn update_link(&self) -> Result<()> {
        todo!();
    }

    pub fn regular_migration(&self, exists: &[&Path]) -> Result<()> {
        if self.auto_migration && self.path_format.is_some() {
            let format: &RegularPathFormat = &self.path_format.as_ref().unwrap();
            let today_path: PathBuf = Config::global().data_dir.join(format.get_today_path());
            let latest_path: PathBuf =  format.find_latest_path(exists).ok_or(anyhow::anyhow!("Latest page is not found"))?;
            if latest_path < today_path {
                let mut latest_page = Page::new(latest_path);
                let mut today_page = Page::new(today_path);
                latest_page.read();
                latest_page.migrate_to(&mut today_page);
                if ! Config::global().dry_run {
                    latest_page.write();
                    today_page.write();
                }
                if let Some(x) = &self.link_path {
                    let link_path = Config::global().data_dir.join(x);
                    trace!("Updating link {:?}", &link_path);
                    match (link_path.is_symlink(), link_path.exists()) {
                        (true, _) => {
                            remove_file(&link_path);
                            symlink(&today_page.path, &link_path);
                            info!("Complete updating link {:?}", &link_path);
                        },
                        (false, true) => {
                            warn!("Skip updating link: {:?}", &link_path);
                        }
                        (false, false) => {
                            symlink(&today_page.path, &link_path);
                            info!("Create new link {:?}", &link_path);
                        }
                    }
                }
                return Ok(());
            } else {
                bail!("Today file is exists");
            };

        }
        bail!("This template is not target of auto migration");
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