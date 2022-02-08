use regex::Regex;
use chrono::{DateTime, NaiveDate, Local};

#[derive(Clone)]
pub struct FrontMatter {
    //date: String,
    //categories: Vec<String>,
    //tags: Vec<String>,
    //spent_time: f32,
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

impl AsRef<str> for FrontMatter{
    fn as_ref(&self) -> &str {
        self.raw.as_str()
    }
}

impl From<&str> for FrontMatter{
    fn from(s: &str) -> Self {
        FrontMatter{
            raw: String::from(s),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_date() {
        const BEFORE: &str = r"date: 2022-01-02
name: test-name";
        const AFTER: &str = r"date: 2022-02-03
name: test-name";
        let mut fm = FrontMatter::from(BEFORE);

        fm.update_date(NaiveDate::parse_from_str("2022-02-03", "%Y-%m-%d").unwrap());
        assert_eq!(fm.raw.as_str(), AFTER);

    }
}