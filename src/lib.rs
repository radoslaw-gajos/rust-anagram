type Result<T> = std::result::Result<T, std::io::Error>;

use crate::word::Word;

mod word {
    use std::collections::HashMap;

    pub struct Word {
        value: String,
        characters: HashMap<char, u8>,
    }

    impl Word {
        pub fn from(string: String) -> Self {
            let characters = map_characters(&string);
            Self {
                value: string,
                characters,
            }
        }
    }

    fn map_characters(string: &str) -> HashMap<char, u8> {
        let mut map = HashMap::new();

        for c in string.to_lowercase().chars() {
            map.entry(c).or_insert(0u8);
            let count = map.get_mut(&c).unwrap();
            *count += 1;
        }

        map
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_map_characters() {
            // given
            let string = "AaaBbC".to_string();

            // when
            let word = Word::from(string);

            // then
            assert_eq!(word.characters.len(), 3);
            assert!(word.characters.contains_key(&'a'));
            assert_eq!(word.characters.get(&'a'), Some(&3u8));
            assert!(word.characters.contains_key(&'b'));
            assert_eq!(word.characters.get(&'b'), Some(&2u8));
            assert!(word.characters.contains_key(&'c'));
            assert_eq!(word.characters.get(&'c'), Some(&1u8));
        }
    }
}

pub fn get_anagrams(_target_anagram: &str, word_list: Vec<String>) 
    -> Result<Vec<(String, String)>> {
        let _words: Vec<Word> = word_list
            .into_iter()
            .map(Word::from)
            .collect();
        Ok(vec![("to".to_string(), "do".to_string())])
}
