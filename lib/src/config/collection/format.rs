use crate::Config;
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

use anyhow::{bail, Result};
use chrono::{NaiveDate, Local, Duration, Weekday};
use regex::{escape, Captures, Regex,};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq)]
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

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(try_from = "&str", into = "String")]
pub struct RegularPathFormat {
    
    /// Use to generate path from date
    format: String,
    /// Use for filtering old file
    regex: Regex,
    interval: ValidInterval,
}

impl RegularPathFormat {
    fn clean_path(p: &AsRef<Path>) -> PathBuf {
        let mut path = p.as_ref().to_path_buf();
        let config = Config::global();
        let file_name = path.file_name().unwrap();
        if config.index_file_names.contains(file_name) {
            path.pop();
        } else {
            path.set_extension("");
        }
        path
    }
    fn date_format_to_pattern(f: &str) -> String {
        static YEAR_REGEX: OnceCell<Regex> = OnceCell::new();
        static MONTH_REGEX: OnceCell<Regex> = OnceCell::new();
        static DAY_REGEX: OnceCell<Regex> = OnceCell::new();
        static ISO_YEAR_REGEX: OnceCell<Regex> = OnceCell::new();
        static ISO_WEEK_REGEX: OnceCell<Regex> = OnceCell::new();
        static ISO_WEEKDAY_REGEX: OnceCell<Regex> = OnceCell::new();

        let mut f = escape(f);
        f = YEAR_REGEX.get_or_init(|| {
            Regex::new(r"%Y").unwrap()
        }).replace_all(&f, r"(?P<year>\d{4})").to_string();
        f = MONTH_REGEX.get_or_init(|| {
            Regex::new(r"%m").unwrap()
        }).replace_all(&f, r"(?P<month>\d{2})").to_string();
        f = DAY_REGEX.get_or_init(|| {
            Regex::new(r"%d").unwrap()
        }).replace_all(&f, r"(?P<day>\d{2})").to_string();
        f = ISO_YEAR_REGEX.get_or_init(|| {
            Regex::new(r"%G").unwrap()
        }).replace_all(&f, r"(?P<isoyear>\d{4})").to_string();
        f = ISO_WEEK_REGEX.get_or_init(|| {
            Regex::new(r"%V").unwrap()
        }).replace_all(&f, r"(?P<isoweek>\d{2})").to_string();
        //f = ISO_WEEKDAY_REGEX.get_or_init(|| {
        //    Regex::new(r"%u").unwrap()
        //}).replace_all(&f, r"(?P<u>\d{2})").to_string();
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
        self.get_path(Local::today().naive_local())
    }
    pub fn get_interval(&self, s: &str) -> Option<(NaiveDate, NaiveDate)> {
        let caps = self.regex.captures(s).unwrap();
        fn try_parse_caps<T: std::str::FromStr>(caps: &Captures, name: &str) -> Option<T> {
            caps.name(&name)?.as_str().parse::<T>().ok()
        }
        let year: Option<i32> = try_parse_caps(&caps, "year");
        let month: Option<u32> = try_parse_caps(&caps, "month");
        let day: Option<u32> = try_parse_caps(&caps, "day");
        let iso_year: Option<i32> = try_parse_caps(&caps, "isoyear");
        let iso_week: Option<u32> = try_parse_caps(&caps, "isoweek");
        //let iso_weekday: Option<u32> = caps.name("isoweekday").parse().ok();
        let start_date : NaiveDate;
        let end_date: NaiveDate;
        let duration: Duration;
        match (year, month, day, iso_year, iso_week) {
            (Some(y), Some(m), Some(d), _, _) => {
                start_date = NaiveDate::from_ymd(y, m, d);
                end_date= start_date.clone();
            },
            (_, _, _, Some(y), Some(w)) => {
                start_date = NaiveDate::from_isoywd(y, w, Weekday::Mon);
                end_date= NaiveDate::from_isoywd(y, w, Weekday::Sun);
            },
            (Some(y), Some(m), None, _, _) => {
                start_date = NaiveDate::from_ymd(y, m, 1);
                end_date = if m == 12 {
                    NaiveDate::from_ymd(y + 1, 1, 1).pred()
                } else {
                    NaiveDate::from_ymd(y, m+1, 1).pred()
                };
            },
            (Some(y), None, None, _, _) => {
                start_date = NaiveDate::from_ymd(y, 1, 1);
                end_date = NaiveDate::from_ymd(y + 1, 1, 1).pred();
            },
            (_, _, _, Some(y), None) => {
                start_date = NaiveDate::from_isoywd(y, 1, Weekday::Mon);
                end_date = NaiveDate::from_isoywd(y + 1, 1, Weekday::Mon).pred();
            }, 
            _ => return None,
        };
        Some((start_date, end_date))
    }
}
impl Into<String> for RegularPathFormat {
    fn into(self) -> String {
        self.format
    }
}

impl PartialEq for RegularPathFormat {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
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
    
    #[test]
    fn regex() {
        assert!(RegularPathFormat::date_format_to_regex("%Y/%m/%d").unwrap().is_match("2022/02/02"));
    }

    #[test]
    fn convert_path() {
        fn test_format(format: &str, date: NaiveDate, path: PathBuf) {
            let template = RegularPathFormat::try_from(format).unwrap();
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

    #[test]
    fn get_interval() {
        let v: Vec<(&str, &str, (i32, u32, u32), (i32, u32, u32))> = vec![
            ("%Y/%m/%d", "2022/06/15", (2022, 6, 15), (2022, 6, 15)),
            ("%Y/%m", "2022/06", (2022, 6, 1), (2022, 6, 30)),
            ("%Y", "2022", (2022, 1, 1), (2022, 12, 31)),
            ("%G/W%V", "2022/W24", (2022, 6,13), (2022, 6, 19)),
            ("%G", "2022", (2022, 1, 3), (2023, 1, 1))
        ];
        let date = NaiveDate::from_ymd(2022, 6, 15);
        for x in v{
            
            let format = RegularPathFormat::try_from(x.0).unwrap();
            let filename = x.1;
            let syear = x.2.0;
            let smonth = x.2.1;
            let sday = x.2.2;
            let eyear = x.3.0;
            let emonth = x.3.1;
            let eday = x.3.2;

            assert_eq!(format.get_interval(filename).unwrap(), (NaiveDate::from_ymd(syear,smonth,sday), NaiveDate::from_ymd(eyear,emonth,eday)));

        }

    }
}
