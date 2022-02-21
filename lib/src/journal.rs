use super::config::Config;
use super::data::Data;
use super::page::Page;
use walkdir::WalkDir;
use anyhow::Result;


pub struct Journal {
    pub config: Config,
    pub data: Data,
}

impl Journal {
    pub fn from_config( config: Config ) -> Result<Self> {
        let path = config.data_dir.to_path_buf();
        let journal= Journal {
            config: config,
            data: Data::new(path.as_path()).unwrap(),
        };
        Ok(journal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_pages() {
        for entry in walkdir::WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            //pages.push(Page.from_path(file));
            println!("{}", entry.path().display());
        }
    }


}