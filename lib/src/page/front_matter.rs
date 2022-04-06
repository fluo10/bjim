use std::collections::{HashMap};
use std::convert::{From, TryFrom};
use std::str::FromStr;
use std::default::Default;

use anyhow::{anyhow, bail, Error, Result};
use chrono::{Local, NaiveDate, NaiveDateTime};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;

const DATE_FORMAT: &'static str = "%Y-%m-%d";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrontMatter {
    title: Option<String>,
    date: Option<String>,
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    //spent_time: f32,
    #[serde(skip)]
    pub raw: String,
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    extra: HashMap<String, String>,
}


impl FrontMatter {

    /// Check value in front matter
    /// For now, check only parsing `FrontMatter.date` 
    pub fn is_valid(&self) -> bool {
        if let Some(x) = &self.date{
            NaiveDate::parse_from_str(x, DATE_FORMAT).is_ok()
        } else {
            true
        }
    }

    /// Update date in front matter
    /// Used with migration
    pub fn update_date(&mut self, date: NaiveDate) {
        const DATE_PATTERN:&str = r"\d{4}-\d{2}-\d{2}";
        let replacement = date.format("%F").to_string();
        let re = Regex::new(DATE_PATTERN).unwrap();
        self.title = match &self.title {
            Some(x) => {
                Some(re.replace_all(&x, &replacement).to_string())
            },
            None => None
        };
        self.date = match &self.date {
            Some(x) => {
                Some(date.format(DATE_FORMAT).to_string())
            },
            None => None,
        }
    }

    pub fn to_string(&self)-> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }
}

impl Default for FrontMatter{
    fn default() -> Self {
        FrontMatter{
            title: None,
            date: None,
            tags: Vec::new(),
            categories: Vec::new(),
            extra: HashMap::new(),
            raw: String::new(),            
        }
    }
}


impl From<&str> for FrontMatter {
    fn from(s: &str) -> Self{
        let mut f: FrontMatter = match serde_yaml::from_str(s) {
            Ok(x) => x,
            Err(e) => {
                panic!("{}",e);
                FrontMatter{
                    raw: String::from(s),
                    ..FrontMatter::default()
                }
            },
        };
        f
    }
}

impl FromStr for FrontMatter {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(s)?)
    }
}

mod date_format {
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const DATE_FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.unwrap().format(DATE_FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D> (
        deserializer: D,
    ) -> Result<Option<NaiveDateTime>, D::Error>
    where 
        D: Deserializer<'de>,
    {
        
        let s = String::deserialize(deserializer)?;
        
        match NaiveDate::parse_from_str(&s, DATE_FORMAT){
            Ok(x) => Ok(Some(x.and_hms(0, 0, 0))),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frontmatter_check() {
        assert!(FrontMatter::from_str(r"---
date: 2022-01-01
").unwrap().is_valid());
        assert!(!FrontMatter::from_str(r"---
date: 2022-01
").unwrap().is_valid());
    }
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
        assert_eq!(fm, FrontMatter::from(AFTER));

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
        /*
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
        */
    }
}