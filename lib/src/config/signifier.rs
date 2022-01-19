//use chrono::DateTime;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Eq, PartialEq, Deserialize, Debug,)]
pub struct Signifier {
    pub name: String,
    pub emoji: String,
    pub value: Option<String>,
//    pub aliases: Vec<String>
}

/*
#[derive(Deserialize)]
#[serde(untagged)]
pub enum SignifierValue {
//    Date,
    Number<f64>,
String<String>,
}
*/