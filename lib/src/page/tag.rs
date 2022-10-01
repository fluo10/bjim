use config::TagConfig;
use std::collections::HashMap;
use std::convert::{From, Into};
use regex::Regex;
use once_cell::sync::OnceCell;

static TAG_REGEX: OnceCell<Regex> = OnceCell::new();


fn tag_regex() -> &'static Regex {
    TAG_REGEX.get_or_init(|| Regex::new(r##"[^#]#(?P<tag>[^:[:space:]]*)(?::(?P<value>\S*))?"##).unwrap())
}

pub struct TagValue {
    name: String,
    value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct TagValues {
    pub map: HashMap<String, Option<String>>,
}

impl From<&str> for TagValues {
    fn from(s: &str) -> Self {
        let mut result: HashMap<String, Option<String>> = HashMap::new();
        for caps in tag_regex().captures_iter(s) {
            let tag:String = caps["tag"].to_string();
            let value = match &caps.name("value") {
                None => None,
                Some(x) => Some(x.as_str().to_string()),
            };
            result.insert(tag, value);
        }
        Self{ 
            map: result
        }
    }
}

impl From<HashMap<String, Option<String>>> for TagValues {
    fn from(map: HashMap<String, Option<String>>) -> Self {
        Self{ 
            map: map,
        }
    }
}

impl Into<HashMap<String, Option<String>>> for TagValues {
    fn into(self) -> HashMap<String, Option<String>> {
        self.map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tag() {
        let mut set: HashMap<String, Option<String>> = HashMap::new();
        set.insert( "hash".to_string(),  None);
        set.insert( "tag".to_string(), None);
        set.insert( "with".to_string(), Some("value".to_string()));
        assert_eq!(
            TagValues::from("test str with #hash #tag #with:value"),
            TagValues::from(set)
        );
    }
}