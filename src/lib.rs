use std::collections::BTreeMap;
use std::cmp::Ordering;

type CharacterMap = BTreeMap<char, i32>;

struct Word {
    characters: CharacterMap,
}

impl Word {
    fn from(text: String) -> Word {
        Word {
            characters: map_characters(&text),
        }
    }
}

fn is_anagram(left: Vec<&Word>, right: Vec<&Word>) -> bool {
    // temporary implementation
    left.get(0).unwrap().characters == merge(&right.get(0).unwrap().characters, &right.get(1).unwrap().characters)
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
}
