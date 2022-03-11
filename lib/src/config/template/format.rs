
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

use anyhow::{bail, Error, Result};
use chrono::{NaiveDate, Utc};
use regex::{escape, Regex};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Clone, Debug)]
pub enum ValidInterval {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Preset template for regularly log like daily log
impl TryFrom<&str> for ValidInterval {
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
                    Ok(ValidInterval::Daily)
                } else {
                    Ok(ValidInterval::Monthly)
                }
            } else {
                Ok(ValidInterval::Yearly)
            }
        } else if has_iso_year {
            if has_iso_week {
                if has_iso_weekday {
                    Ok(ValidInterval::Daily)
                } else {
                    Ok(ValidInterval::Weekly)
                }
            } else {
                Ok(ValidInterval::Monthly)
            }
        } else {
            bail!("Invalid date format")
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug,)]
#[serde(try_from = "&str", into = "String")]
pub struct RegularPathFormat {
    
    /// Use to generate path from date
    format: String,
    /// Use for filtering old file
    regex: Regex,
    interval: ValidInterval,
}

impl RegularPathFormat {
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
        let re = Regex::new(Self::date_format_to_pattern(f).as_str())?;
        Ok(re)
    }

    pub fn match_format(&self, s: &str) -> bool {
        self.regex.is_match(s)
    }
    
    pub fn find_latest_path(&self, paths: &[&Path]) -> Option<PathBuf> {
        let mut paths: Vec<&Path> = paths.to_vec();
        paths.sort();
        paths.reverse();
        for path in paths {
            if self.match_format(path.to_str().unwrap()) {
                return Some(path.to_path_buf());
            };
        }
        None
    }
    pub fn get_path(&self, date: NaiveDate) -> PathBuf {
        PathBuf::from(date.format(self.format.as_str()).to_string())
    }
    pub fn get_today_path(&self) -> PathBuf {
        self.get_path(Utc::today().naive_local())
    }
}
impl Into<String> for RegularPathFormat {
    fn into(self) -> String {
        self.format
    }
}
impl TryFrom<&str> for RegularPathFormat {
    type Error = anyhow::Error;
    fn try_from(f: &str) -> Result<Self> {
        let result = RegularPathFormat{
            format: f.to_string(),
            regex: Self::date_format_to_regex(f)?,
            interval: ValidInterval::try_from(f)?,
        };
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::*;
    #[test]
    fn regex() {
        assert!(RegularPathFormat::date_format_to_regex("%Y/%m/%d").unwrap().is_match("2022/02/02"));
    }

    #[test]
    fn convert_path() {
        fn test_format(format: &str, date: NaiveDate, path: PathBuf) {
            let template = RegularPathFormat::new(format).unwrap();
            assert_eq!(template.get_path(date), path);
        }
        test_format(
            "%Y/%m/%d",
            NaiveDate::from_ymd(2022, 01, 01),
            PathBuf::from("2022/01/01"),
        );
    }

    #[test]
    fn find_latest_path() {
        let paths: Vec<&Path> =  vec![
            &Path::new("2022/02/03"),
            &Path::new("2022/09/12"),
            &Path::new("2021/04/02"),
        ];
        let format = RegularPathFormat::try_from("%Y/%m/%d").unwrap();
        assert_eq!(format.find_latest_path(&paths).unwrap(), format.get_path(NaiveDate::from_ymd(2022, 09, 12)));

    }
}
