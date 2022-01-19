use super::{Config, Signifier};
use std::path::PathBuf;


#[test]
fn parse_string_all() {
    let fromtoml = Config::from_toml(r##"data_dir = "/home/test/"
[[signifiers]]
name = "tag"
emoji = "🏷️"
[[signifiers]]
name = "date"
emoji = "📅"
value = "date"
[[signifiers]]
name = "time"
emoji = "⏰"
value = "date"
[[signifiers]]
name = "hours"
emoji = "⌛"
value = "float""##);

    let config = Config{
        data_dir : PathBuf::from("/home/test/"),
        signifiers : vec![
            Signifier{ 
                name: "tag".to_string(),
                emoji: "🏷️".to_string(),
                value: None,
            },
            Signifier{
                name: "date".to_string(),
                emoji: "📅".to_string(),
                value: Some("date".to_string()),
            },
            Signifier {
                name: "time".to_string(),
                emoji: "⏰".to_string(),
                value: Some("date".to_string()),
            },
            Signifier {
                name: "hours".to_string(),
                emoji: "⌛".to_string(),
                value: Some("float".to_string()),
            },
        ],
    };
    assert_eq!(fromtoml, config);
}

fn parse_string_min() {
    let fromtoml = Config::from_toml(r##"data_dir = "/home/test/"##);

    let config = Config{
        data_dir : PathBuf::from("/home/test/"),
        signifiers : Vec::new(),
    };
    assert_eq!(fromtoml, config);
}
