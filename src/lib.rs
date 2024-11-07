type Result<T> = std::result::Result<T, std::io::Error>;

use crate::word::Word;

mod word;
mod anagram;

pub fn get_anagrams(target_anagram: &str, word_list: Vec<String>) 
    -> Result<Vec<(String, String)>> {
        let target_anagram = Word::from(target_anagram.to_string());
        let _words: Vec<Word> = word_list
            .into_iter()
            .map(Word::from)
            .filter(|w| anagram::is_compatible(w, &target_anagram))
            .collect();
        Ok(vec![("to".to_string(), "do".to_string())])
}

