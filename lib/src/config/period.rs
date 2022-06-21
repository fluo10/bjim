use chrono::NaiveDate;

#[derive(PartialEq, Debug)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl Period {
    pub fn contains(&self, date: NaiveDate) -> bool {
        todo!()
    }
}