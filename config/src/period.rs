use chrono::NaiveDate;

#[derive(PartialEq, Debug)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl Period {
    pub fn contains(&self, date: NaiveDate) -> bool {
        (self.start <= date) && (date <= self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let period = Period{
            start: NaiveDate::from_ymd(2022, 06, 17),
            end: NaiveDate::from_ymd(2022, 06, 21)
        };
        assert_eq!(period.contains(NaiveDate::from_ymd(2022, 06, 16)), false);
        assert_eq!(period.contains(NaiveDate::from_ymd(2022, 06, 17)), true);
        assert_eq!(period.contains(NaiveDate::from_ymd(2022, 06, 20)), true);
        assert_eq!(period.contains(NaiveDate::from_ymd(2022, 06, 21)), true);
        assert_eq!(period.contains(NaiveDate::from_ymd(2022, 06, 22)), false);
    }
}