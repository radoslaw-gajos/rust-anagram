#![warn(clippy::all)]

type Result<T> = std::result::Result<T, Box<dyn std::any::Any>>;

struct Config {
    target_anagram: String,
}

fn main() -> Result<()> {
    let config = get_config()?;
    let word_list = get_word_list(&config)?;
    let anagrams = get_anagrams(&config.target_anagram, word_list)?;
    save_anagrams(anagrams);
    Ok(())
}

fn get_config() -> Result<Config> {
    Err(Box::new("Not implemented yet"))
}

fn get_word_list(_config: &Config) -> Result<Vec<String>> {
    Err(Box::new("Not implemented yet"))
}

fn get_anagrams(_target_anagram: &str, _word_list: Vec<String>) -> Result<Vec<(String, String)>> {
    Err(Box::new("Not implemented yet"))
}

fn save_anagrams(_anagrams: Vec<(String, String)>) {
}
