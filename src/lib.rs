type Result<T> = std::result::Result<T, std::io::Error>;

use crate::word::Word;
use std::collections::HashMap;

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
        let word_map = word_map::map_by_length(words);
        let anagrams = find_anagrams(&word_map, &target_anagram);

        Ok(anagrams)
}

fn find_anagrams(word_map: &HashMap<usize, Vec<Word>>, target_anagram: &Word)
    -> Vec<(String, String)> {
        let mut anagrams = Vec::new();

        for (left_len, left_words) in word_map.iter() {
            let avoid_duplicates = *left_len > target_anagram.len() / 2;
            if avoid_duplicates {
                continue;
            }
            for left in left_words {
                if *left_len > target_anagram.len() {
                    continue;
                }

                let right_len = target_anagram.len() - left_len;

                let right_vec = word_map.get(&right_len);

                if right_vec.is_some() {
                    for right in right_vec.unwrap().iter() {
                        if is_anagram(left, right, target_anagram) {
                            anagrams.push((left.value.clone(), right.value.clone()));
                        }
                    }
                }
            }
        }

        anagrams
}

fn is_anagram(left: &Word, right: &Word, target_anagram: &Word) -> bool {
    for c in target_anagram.characters.keys() {
        let left_count = left.characters.get(c).copied().unwrap_or_default();
        let right_count = right.characters.get(c).copied().unwrap_or_default();
        let target_count = target_anagram.characters.get(c).copied().unwrap_or_default();

        if target_count != left_count + right_count {
            return false;
        }
    }
    true
}
