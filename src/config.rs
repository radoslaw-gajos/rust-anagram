use std::env;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct Config {
    pub target_anagram: String,
    pub word_list_file: String,
    pub target_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_anagram: "documenting".to_string(),
            word_list_file: "word_list.txt".to_string(),
            target_file: "anagrams.txt".to_string(),
        }
    }
}

pub fn get_config() -> Result<Config> {
    let mut config = Config::default();

    //let args: Vec<&str> = env::args().collect();
    let args: Vec<String> = env::args().collect();

    if args.len() > 4 {
        panic!("Invalid number of arguments");
    }

    if args.len() > 1 {
        if let Some(target_anagram) = &args.get(1) {
            config.target_anagram = target_anagram.to_string();
        }
        if let Some(word_list_file) = &args.get(2) {
            config.word_list_file = word_list_file.to_string();
        }
        if let Some(target_file) = &args.get(3) {
            config.target_file = target_file.to_string();
        }
    }

    Ok(config)
}
