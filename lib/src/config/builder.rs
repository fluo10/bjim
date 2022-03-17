use super::buf::ConfigBuf;
use super::tag::TagConfig;
use super::Config;
use super::template::{RegularLogTemplate, RegularPathFormat};

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::convert::{AsRef, TryFrom};


use std::default::Default;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

use anyhow::{bail,Error, Result};
use dirs::{config_dir, home_dir};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Hash, Eq)]
pub enum Scope {
    User,
    Local,
    Argument,
}

#[derive(Debug, PartialEq)]
pub struct ConfigBuilder {
    items: HashMap<Scope, ConfigBuf>
}

impl ConfigBuilder {
    pub fn new() -> Self {
        todo!();
    }
    pub fn build(self) -> Result<Config> {
        todo!();
    }
}