#![warn(clippy::all)]

use crate::config::Config;
use std::fs;
use std::fs::File;
use std::io::Write;

type Result<T> = std::result::Result<T, std::io::Error>;

mod config;

fn main() -> Result<()> {
    let config = config::get_config()?;
    let word_list = get_word_list(&config)?;
    let anagrams = anagram::get_anagrams(&config.target_anagram, word_list)?;
    save_anagrams(&config, anagrams)?;
    Ok(())
}

fn get_word_list(config: &Config) -> Result<Vec<String>> {
    let word_list = fs::read_to_string(&config.word_list_file)?
        .split_whitespace()
        .map(String::from)
        .collect();
    Ok(word_list)
}

fn save_anagrams(config: &Config, anagrams: Vec<(String, String)>) -> Result<()> {
    let mut file = File::create(&config.target_file)?;
    writeln!(&mut file, "Anagrams of {}:", &config.target_anagram)?;
    for (left, right) in anagrams {
        writeln!(&mut file, "{left} {right}")?;
    }
    Ok(())
}
