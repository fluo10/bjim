use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CoreConfig {
    #[serde(default,)]
    pub data_dir: PathBuf,

    #[serde(default, skip_deserializing, skip_serializing_if = "std::ops::Not::not")]
    pub read_only: bool,

    //#[serde(default)]
    //pub allow_duplicate_file_name: bool,

    /// Like index.html, Ignore this file name and use parent dir as file name in matching
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub index_file_names: HashSet<OsString>,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("."),
            read_only: false,
            //allow_duplicate_file_name: true 
            index_file_names: HashSet::from([OsString::from("index.md")])
        }
    }
}

impl CoreConfig {
    fn canonicalize_file_name(&self, p: &dyn AsRef<Path>) -> Option<PathBuf> {
        const EXTENSION: &str = ".md";
        let path: &Path = p.as_ref();
        let file_name = path.file_name()?;
        let extension = path.extension()?;
        let mut buf: PathBuf;
        if self.index_file_names.contains(file_name) {
            buf = path.to_path_buf();
            buf.pop();
            buf.set_extension(".md");
            return Some(buf);
        } else if extension == EXTENSION {
            return Some(path.to_path_buf())
        }
        None
    }
}