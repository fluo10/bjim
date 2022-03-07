
use std::default::Default;
use anyhow::{bail, Result};
use serde::{Serialize, Deserialize};
use toml::Value;

/// Assigned role for tag
/// The tags assigned are used with migration, filtering entry, etc.
#[derive(Debug, Deserialize, PartialEq)]
pub enum AssignedTag {
    /// Entry including this tag will not be deleted but reopen by each migration
    Repeat,
    StartDate,
    CloseDate,
    DueDate,
    Date,
}

/// The type of value trailing tag.
#[derive(Debug, Deserialize, PartialEq)]
pub enum TagValueType {
    None,
    Date,
    Time,
    DateTime,
    Number,
}

#[derive(PartialEq, Deserialize, Debug, )]
#[serde(default)]
pub struct TagConfig {

    /// If false, this tags will not inherit to child entries
    inherit: bool,

    /// If false, this tag will not be copied with migration
    migrate: bool,

    /// The type of value trailing tag.
    /// If `None`, it's not checked.
    value_type: Option<TagValueType>,

    /// Assigned role of tag.
    /// This referred when migration, filtering, etc.
    assigned: Option<AssignedTag>,
}

impl TagConfig {
    pub const fn new() -> Self {
        Self{
            inherit: true,
            migrate: true,
            value_type: None,
            assigned: None,
        }
    }
}

impl Default for TagConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Default for &'a TagConfig {
    fn default() -> Self {
        static DEFAULT: TagConfig = TagConfig::new();
        &DEFAULT
    }
}