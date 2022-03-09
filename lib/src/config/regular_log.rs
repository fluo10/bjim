use std::convert::{AsRef, TryFrom};
use std::path::{Path, PathBuf};
use anyhow::{bail, Error, Result};
use chrono::{Date, Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc};
use once_cell::sync::OnceCell;
use regex::{escape, Regex};



pub enum RegularLogInterval {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl TryFrom<&str> for RegularLogInterval {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self> {
        let has_year:bool = s.contains("%Y");
        let has_month:bool = s.contains("%m");
        let has_day:bool = s.contains("%d");
        let has_iso_year:bool = s.contains("%G");
        let has_iso_week: bool = s.contains("%V");
        let has_iso_weekday: bool = s.contains("%u");
        if has_year {
            if has_month {
                if has_day {
                    Ok(RegularLogInterval::Daily)
                } else {
                    Ok(RegularLogInterval::Monthly)
                }
            } else {
                Ok(RegularLogInterval::Yearly)
            }
        } else if has_iso_year {
            if has_iso_week {
                if has_iso_weekday {
                    Ok(RegularLogInterval::Daily)
                } else {
                    Ok(RegularLogInterval::Weekly)
                }
            } else {
                Ok(RegularLogInterval::Monthly)
            }
        } else {
            bail!("Invalid date format")
        }
    }
}

/// Preset template for regularly log like daily log
pub struct RegularLogTemplate {

    /// Use to generate path from date
    path_format: Option<String>,

    /// If true, this log is automatically created with update command
    /// Auto migration require `path_format` including `.md` extension 
    auto_migration: bool,

    /// If set, soft link to the latest file will be created or updated by each `update`
    link_path: Option<PathBuf>,

    /// Use for filtering old file
    regex: Option<Regex>,
}

fn date_format_to_pattern(f: &str) -> String {
    static FOUR_DIGIT_REGEX: OnceCell<Regex> = OnceCell::new();
    static TWO_DIGIT_REGEX: OnceCell<Regex> = OnceCell::new();
    let mut f = escape(f);
    f = FOUR_DIGIT_REGEX.get_or_init(|| {
        Regex::new(r"%[YG]").unwrap()
    }).replace_all(&f, r"[0-9]{4}").to_string();
    f = TWO_DIGIT_REGEX.get_or_init(|| {
        Regex::new(r"%[mdVu]").unwrap()
    }).replace_all(&f, r"[0-9]{2}").to_string();
    f
}
fn date_format_to_regex(f: &str) -> Result<Regex> {
    let re = Regex::new(date_format_to_pattern(f).as_str())?;
    Ok(re)
}

impl RegularLogTemplate {
    pub fn get_path(&self, date: NaiveDate) -> Result<PathBuf> {
        
        Ok(PathBuf::from(date.format(self.path_format.as_ref().unwrap().as_str()).to_string()))
    }
    pub fn get_today_path(&self) -> Result<PathBuf> {
        self.get_path(Utc::today().naive_local())
    }

    pub fn match_format(&mut self, s: &str) -> Result<bool> {
        let re = self.regex.get_or_insert(
            Regex::new(
                date_format_to_pattern(self.path_format.as_ref().ok_or(Error::msg("Format is empty"))?.as_str()).as_str()
            )?
        );
        let result = re.is_match(s);
        
        Ok(result)
    }
    pub fn get_last_path(&self, date: NaiveDate) -> Result<PathBuf> {
        let today = Utc::today().naive_local();
        let last_date = match RegularLogInterval::try_from(self.path_format.as_ref().unwrap().as_str())? {
            RegularLogInterval::Daily => date.checked_sub_signed(Duration::days(1)).unwrap(),
            RegularLogInterval::Weekly => date.checked_sub_signed(Duration::weeks(1)).unwrap(),
            RegularLogInterval::Monthly => NaiveDate::from_ymd(date.year(),date.month(), 1).pred(),
            RegularLogInterval::Yearly => NaiveDate::from_ymd(date.year()-1,1,1),
        };
        self.get_path(last_date)

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
            regex: None,
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
    #[test]
    fn regex () {
        assert!(date_format_to_regex("%Y/%m/%d").unwrap().is_match("2022/02/02"));
    }
}