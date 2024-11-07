type Result<T> = std::result::Result<T, std::io::Error>;

use crate::word::Word;

mod word;
mod anagram;
mod word_map;

pub fn get_anagrams(target_anagram: &str, word_list: Vec<String>) 
    -> Result<Vec<(String, String)>> {
        let target_anagram = Word::from(target_anagram.to_string());
        let words: Vec<Word> = word_list
            .into_iter()
            .map(Word::from)
            .filter(|w| anagram::is_compatible(w, &target_anagram))
            .collect();
        let _word_map = word_map::map_by_length(words);

        Ok(vec![("to".to_string(), "do".to_string())])
}
