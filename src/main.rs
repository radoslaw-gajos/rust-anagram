#![warn(clippy::all)]

use crate::config::Config;
use std::fs;

type Result<T> = std::result::Result<T, std::io::Error>;

mod config;

fn main() -> Result<()> {
    let config = config::get_config()?;
    let word_list = get_word_list(&config)?;
    let anagrams = get_anagrams(&config.target_anagram, word_list)?;
    save_anagrams(anagrams);
    Ok(())
}


fn get_word_list(config: &Config) -> Result<Vec<String>> {
    let word_list = fs::read_to_string(&config.word_list_file)?
        .split_whitespace()
        .map(String::from)
        .collect();
    Ok(word_list)
}

fn get_anagrams(_target_anagram: &str, _word_list: Vec<String>) -> Result<Vec<(String, String)>> {
    todo!();
}

fn save_anagrams(_anagrams: Vec<(String, String)>) {
}
