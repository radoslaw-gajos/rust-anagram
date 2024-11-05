use std::collections::BTreeMap;
use std::collections::HashMap;

type CharacterMap = BTreeMap<char, i32>;

#[derive(Debug, PartialEq)]
struct Word {
    value: String,
    characters: CharacterMap,
}

impl Word {
    fn from(text: String) -> Word {
        let characters = map_characters(&text);
        Word {
            value: text,
            characters,
        }
    }
}

fn get_two_word_anagrams(target_word: &str, word_list: &str) -> Vec<(String, String)> {
    let target_word = Word::from(target_word.to_string());

    let words = word_list_to_vec(word_list)
        .into_iter()
        .map(Word::from)
        .inspect(|word_map| println!("{word_map:#?}"))
        //.filter(|w| has_common_letters(target_word, &w))
        .filter(|w| has_common_letters(&w.value, &target_word))
        .collect();
    println!("{words:#?}");
    let word_map = prepare_map(words);
    println!("{word_map:#?}");

    let mut pairs = Vec::new();
    let length = target_word.value.len();

    for left_len in 1..=length/2 {
        let right_len = length - left_len;

        for left in word_map.get(&left_len).or(Some(&Vec::new())).into_iter().into_iter() {
            for right in word_map.get(&right_len) {
                // todo: add pairs which are anagrams
                pairs.push((left.value.clone(), right.value.clone()))
            }
        }
    }

    pairs
}

fn word_list_to_vec(word_list: &str) -> Vec<String> {
    word_list
        .split_whitespace()
        .map(String::from)
        .collect()
}

// todo HashMap<usize, Word> -> HashMap<usize, Vec<Word>>
fn prepare_map(words: Vec<Word>) -> HashMap<usize, Vec<Word>> {
    let single_map = HashMap::from_iter(words.into_iter().map(map_by_length));

    let mut multi_map = HashMap::new();
    for (size, word) in single_map.into_iter() {
        if !multi_map.contains_key(size) {
            let list = Vec::new();
            multi_map.insert(size, list);
        }

        multi_map.get(size).unwrap().push(word);
    }

    multi_map
}

fn map_by_length(w: Word) -> (usize, Word) {
    (w.value.len(), w)
}

fn has_common_letters(s: &str, w: &Word) -> bool {
    for c in s.to_lowercase().chars() {
        if !w.characters.contains_key(&c) {
            println!("{w:#?} doesn't contain {c}");
            return false;
        }
    }
    true
}

fn is_anagram(left: Vec<&Word>, right: Vec<&Word>) -> bool {
    merge_recursive(left.iter().map(|word| &word.characters).collect())
        ==  merge_recursive(right.iter().map(|word| &word.characters).collect())
        
}

fn merge_recursive(ch: Vec<&CharacterMap>) -> CharacterMap {
    if ch.len() == 1 {
        return (*ch.get(0).unwrap()).clone();
    }
    if ch.len() == 2 {
        return merge(ch.get(0).unwrap(), ch.get(1).unwrap());
    }
    merge(ch.get(0).unwrap(), &merge_recursive(ch[1..ch.len()].to_vec()))
}

fn merge(left_map: &CharacterMap, right_map: &CharacterMap) -> CharacterMap {
    let mut merged = left_map.clone();

    for (ch, count) in right_map.iter() {
        let dup = merged.get_mut(ch);
        match dup {
            Some(dup_count) => {
                *dup_count += *count;
            }
            None => {
                merged.insert(*ch, *count);
            }
        };
    }

    merged
}

fn map_characters(text: &str) -> CharacterMap {
    let mut characters = BTreeMap::new();

    for c in text.to_lowercase().chars() {
        let count = match characters.get(&c) {
            Some(count) => count + 1,
            None => 1,
        };

        characters.insert(c, count);
    }

    characters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_characters_in_word() {
        // given
        let text = "AaaBbc".to_string();

        // when
        let word = Word::from(text);

        // then
        assert_eq!(word.characters.get(&'a').unwrap(), &3);
        assert_eq!(word.characters.get(&'b').unwrap(), &2);
        assert_eq!(word.characters.get(&'c').unwrap(), &1);
    }

    #[test]
    fn should_merge_character_maps() {
        // given
        let left = Word::from("AaB".to_string());
        let right = Word::from("BCc".to_string());

        // when
        let map = merge(&left.characters, &right.characters);

        // then
        assert_eq!(map.get(&'a').unwrap(), &2);
        assert_eq!(map.get(&'b').unwrap(), &2);
        assert_eq!(map.get(&'c').unwrap(), &2);
    }

    #[test]
    fn should_recognize_anagram() {
        // given
        let left = vec![Word::from("boat".to_string())];
        let right = vec![Word::from("a".to_string()), Word::from("bot".to_string())];

        // when
        let is_anagram = is_anagram(left.iter().collect(), right.iter().collect());

        // then
        assert!(is_anagram);
    }

    #[test]
    fn should_tell_if_has_common_letters() {
        // when
        let has_common = has_common_letters("dog", &Word::from("god".to_string()));

        assert!(has_common);
    }

    #[test]
    fn should_tell_letters_dont_match() {
        // when
        let has_common = has_common_letters("great", &Word::from("greet".to_string()));

        assert!(!has_common);
    }

    #[test]
    fn should_map_by_length() {
        // given
        let word = Word::from("four".to_string());

        // when
        let (len, _) = map_by_length(word);

        // then
        assert_eq!(len, 4);
    }

    #[test]
    fn should_prepare_map() {
        // given
        let words = vec![
            Word::from("a".to_string()),
            Word::from("at".to_string()),
            Word::from("cat".to_string()),
        ];

        // when
        let mapped_words = prepare_map(words.into_iter()/*.map(|w| *w)*/.collect());

        // then
        assert_eq!(mapped_words, HashMap::from([
            (1usize, Word::from("a".to_string())),
            (2usize, Word::from("at".to_string())),
            (3usize, Word::from("cat".to_string())),
        ]));
    }

    #[test]
    fn should_get_two_word_anagrams() {
        // given
        let target = "abcd";
        let word_list = "a bcd ab cd xyz abc d";

        // when
        let anagrams = get_two_word_anagrams(&target, &word_list);

        // then
        assert_eq!(anagrams, vec![
            ("a".to_string(), "bcd".to_string()),
            ("ab".to_string(), "cd".to_string()),
            ("abc".to_string(), "d".to_string()),
        ]);
    }

    #[test]
    fn should_turn_word_list_to_vector() {
        // given
        let word_list = "a quick brown fox";

        // when
        let result = word_list_to_vec(&word_list);

        // then
        assert_eq!(result, vec![
            "a".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
        ]);
    }
}
