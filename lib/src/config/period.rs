use chrono::NaiveDate;

pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl Period {
    pub fn contains(&self, date: NaiveDate) -> bool {
        todo!()
    }
}