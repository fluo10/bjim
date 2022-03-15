use std::collections::{HashMap};
use std::convert::{From};
use std::default::Default;


use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;
use serde::Deserialize;
use serde_yaml;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrontMatter {
    title: Option<String>,
    #[serde(with = "date_format")]
    date: Option<NaiveDateTime>,
    categories: Vec<String>,
    tags: Vec<String>,
    //spent_time: f32,
    #[serde(skip)]
    pub raw: String,
    #[serde(flatten)]
    extra: HashMap<String, String>,
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
            extra: HashMap::new(),
            raw: String::new(),            
        }
    }
}


impl From<&str> for FrontMatter {
    fn from(s: &str) -> Self{
        let mut f: FrontMatter = match serde_yaml::from_str(s) {
            Ok(x) => x,
            Err(e) => FrontMatter::default(),
        };
        f.raw = String::from(s);
        f
    }
}

mod date_format {
    use chrono::{NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M";

    pub fn serialize<S>(
        date: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.unwrap().format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D> (
        deserializer: D,
    ) -> Result<Option<NaiveDateTime>, D::Error>
    where 
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match NaiveDateTime::parse_from_str(&s, FORMAT){
            Ok(x) => Ok(Some(x)),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
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