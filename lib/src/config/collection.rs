pub use crate::config::{PeriodFormat, Period};

use crate::{Config, Page};
use std::fs::{remove_file, rename};
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink;
#[cfg(unix)]
use std::os::unix::fs::symlink;

use anyhow::{bail, Result};
use chrono::{NaiveDate, Local};
use log::{info, trace, warn,};
use serde::{Deserialize, Serialize};


/// Preset collection for regularly log like daily log
/// This is named from the same term in Bullet Journal 
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct CollectionConfig {

    /// Path of template file used to create new file without migration
    pub archetype_path: Option<PathBuf>,

    /// If true, this log is automatically created with update command
    /// Auto migration require `path_format` including `.md` extension 
    #[serde(default)]
    pub auto_migration: bool,

    /// Path format used to generate file name from date
    pub path: Option<PeriodFormat>,

    /// Path format used to generate file name for archive 
    pub archive_path: Option<PeriodFormat>,

}

impl CollectionConfig {

    pub fn is_valid(&self) -> bool {
        todo!();
    }
    pub fn update_link(&self) -> Result<()> {
        todo!();
    }
    pub fn get_path(&self, date: NaiveDate) -> Option<PathBuf> {
        if let Some(x) = self.get_working_path(date) {
            Some(x)
        } else {
            self.get_archive_path(date)
        }
    }
    pub fn get_working_path(&self, date: NaiveDate) -> Option<PathBuf> {
        Some(self.path.as_ref()?.get_path(date))
    }
    pub fn get_archive_path(&self, date: NaiveDate) -> Option<PathBuf> {
        Some(self.archive_path.as_ref()?.get_path(date))
    }
    pub fn get_latest_path_period(&self, exists: &[&Path]) -> Option<(PathBuf, Period)> {
        if let Some(x) = self.get_latest_working_path_period(exists) {
            Some(x)
        } else {
            self.get_latest_archive_path_period(exists)
        }
    }
    pub fn get_latest_working_path_period(&self, exists: &[&Path]) -> Option<(PathBuf, Period)> {
        let fmt = self.path.as_ref()?;
        if let Some(x) = fmt.find_latest_path(exists) {
            let period = fmt.get_period(&x.to_str()?)?;
            Some((x, period))
        } else {
            None
        }
    }
    pub fn get_latest_archive_path_period(&self, exists: &[&Path]) -> Option<(PathBuf, Period)> {
        let fmt = self.archive_path.as_ref()?;
        if let Some(x) = fmt.find_latest_path(exists) {
            let period = fmt.get_period(&x.to_str()?)?;
            Some((x, period))
        } else {
            None
        }
    }

    pub fn migrate(&self, config: &Config,  exists: &[&Path]) -> Result<()> {
        let date: NaiveDate = Local::today().naive_local();
        if let Some((latest_path, period)) = self.get_latest_path_period(&exists) {
            if period.contains(date) {
                Err(anyhow::anyhow!("Migration is not needed"))
            } else {
                let mut latest_page = Page::new(&latest_path);
                let today_path = config.data_dir.join(self.get_path(date).unwrap());
                let mut today_page = Page::new(today_path);
                latest_page.read();
                latest_page.migrate_to(&mut today_page);
                if !config.dry_run {
                    latest_page.write();
                    today_page.write();
                }

                if let Some(archive) = &self.archive_path {
                    let archive_path = archive.get_path(period.start);
                    if archive_path != latest_path {
                        rename(latest_path, archive_path)?;
                    }
                }

                /*
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
                */

                Ok(())
            }

        } else {
            bail!("Latest page is not found");
        }
        
    }
    
}



#[cfg(test)]
mod tests {
    


}