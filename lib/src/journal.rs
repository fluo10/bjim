use super::config::Config;
//use super::page::Page;
use walkdir::WalkDir;

pub struct Journal {
    config: Config,
//    pages: Vec<Page>,
}

impl Journal {
    pub fn from_config( config:Config ) -> Result<Self, u8> {
        let journal= Journal {
            config: config,
            //pages: Vec::new(),
        };
        Ok(journal)
    }
    pub fn load_pages( &self ) {
        for entry in WalkDir::new(self.config.data_dir.as_path()).into_iter().filter_map(|e| e.ok()) {
            //pages.push(Page.from_path(file));
            println!("{}", entry.path().display());
        }
    }
}