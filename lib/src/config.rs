use std::path::PathBuf;
use std::dirs;

pub struct Config {
    config_path: Path
    document_root: String,
}

impl Config {
    pub fn new( path: Option<String> ) => Config {
        
        println!(dirs::config_dir)

        .config/sbjo/sbjo.conf
        todo!;
    }
}

trait ToConfig {
    fn to_config(&self);
}

impl ToConfig for 