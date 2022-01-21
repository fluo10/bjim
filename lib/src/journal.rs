use super::config::Config;
use super::content::Content;
use super::page::Page;
use walkdir::WalkDir;


pub struct Journal {
    config: Config,
    pages:Content,
}

impl Journal {
    pub fn from_config( config: Config ) -> Result<Self, u8> {
        let path = config.content_dir.to_path_buf();
        let journal= Journal {
            config: config,
            pages: Content::new(path.as_path()).unwrap(),
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