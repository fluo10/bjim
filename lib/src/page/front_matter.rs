#[derive(Clone)]

pub struct FrontMatter {
    //date: String,
    //categories: Vec<String>,
    //tags: Vec<String>,
    //spent_time: f32,
    pub raw: String,
}

impl FrontMatter {
    pub fn from_str(raw: &str) -> Self {
        FrontMatter{
            raw: String::from(raw),
        }
    }
}