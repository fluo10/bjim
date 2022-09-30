
use std::default::Default;

use serde::{Deserialize, Serialize};


/// Assigned role for tag
/// The tags assigned are used with migration, filtering entry, etc.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum AssignedTag {
    StartDate,
    CloseDate,
    DueDate,
    Date,
}

/// The type of value trailing tag.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum TagValueType {
    None,
    Date,
    Time,
    DateTime,
    Number,
}

#[derive(PartialEq, Deserialize, Serialize, Debug, )]
#[serde(default)]
pub struct TagConfig {
    /// Entry including this tag will not be deleted but reopen by each migration
    pub repeat: bool,
    /// If false, this tags will not inherit to child entries
    pub inherit: bool,

    /// If false, this tag will not be copied with migration
    pub migrate: bool,

    /// The type of value trailing tag.
    /// If `None`, it's not checked.
    pub value_type: Option<TagValueType>,

    /// Assigned role of tag.
    /// This referred when migration, filtering, etc.
    pub assigned: Option<AssignedTag>,
}

impl TagConfig {
    pub const fn new() -> Self {
        Self{
            repeat: false,
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