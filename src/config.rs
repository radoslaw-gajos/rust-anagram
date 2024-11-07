type Result<T> = std::result::Result<T, std::io::Error>;

pub struct Config {
    pub target_anagram: String,
    pub word_list_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_anagram: "documenting".to_string(),
            word_list_file: "word_list.txt".to_string(),
        }
    }
}

pub fn get_config() -> Result<Config> {
    let config = Config::default();

    Ok(config)
}
