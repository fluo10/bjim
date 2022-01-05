mod config;
//pub mod journal;

pub use config::Config;
//pub use journal::Journal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let config = super::Config::new();
        config.show();
    }
}
