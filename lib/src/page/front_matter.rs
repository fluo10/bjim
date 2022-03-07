use std::convert::{From, TryFrom};
use std::default::Default;

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;
use serde::Deserialize;
use serde_yaml;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrontMatter {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    date: Option<NaiveDateTime>,
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    //spent_time: f32,
    #[serde(skip)]
    pub raw: String,
}


impl FrontMatter {
    pub fn update_date(&mut self, date: NaiveDate) {
        const DATE_PATTERN:&str = r"\d{4}-\d{2}-\d{2}";
        let replacement = date.format("%F").to_string();
        let re = Regex::new(DATE_PATTERN).unwrap();
        self.raw = re.replace_all(self.raw.as_str(), replacement.as_str()).to_string();
    }
}

impl Default for FrontMatter{
    fn default() -> Self {
        FrontMatter{
            title: None,
            date: None,
            tags: Vec::new(),
            categories: Vec::new(),
            raw: String::new(),            
        }
    }
}


impl From<&str> for FrontMatter {
    fn from(s: &str) -> Self{
        let mut f: FrontMatter = serde_yaml::from_str(s).unwrap();
        f.raw = String::from(s);
        f
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_date() {
        const BEFORE: &str = r"---
        date: 2022-01-02
title: test-title
";
        const AFTER: &str = r"---
        date: 2022-02-03
title: test-title
";
        let mut fm = FrontMatter::from(BEFORE);

        fm.update_date(NaiveDate::parse_from_str("2022-02-03", "%Y-%m-%d").unwrap());
        assert_eq!(fm.raw.as_str(), AFTER);

    }
    #[test]
    fn parse_date() {
        /*
        assert_eq!(
            FrontMatter::from(r"---
date: 2022-01-02
---
"),
            FrontMatter{
                date: Some(NaiveDate::from_ymd(2022, 01, 02).and_hms(0,0,0)),
                raw: String::from(r"---
                date: 2022-01-02
                "),
                ..Default::default()
            }

        );
        */
        assert_eq!(
            FrontMatter::from(r"---
date: 2022-03-07T09:15:00
"),
            FrontMatter{
                date: Some(NaiveDate::from_ymd(2022, 03, 07).and_hms(9,15,0)),
                raw: String::from(r"---
                date: 2022-03-07T09:15:00
                "),
                ..Default::default()
            }
        );
    }
}