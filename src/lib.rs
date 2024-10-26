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
    panic!("not implemented yet");
}

fn merge(left_map: &CharacterMap, right_map: &CharacterMap) -> CharacterMap {
    let mut merged = BTreeMap::new();

    let mut left_iter = left_map.iter();
    let mut right_iter = right_map.iter();

    let left = left_iter.next();
    let right = right_iter.next();
    
    if left.is_some() && right.is_some() {
        let left = left.unwrap();
        let right = right.unwrap();

        let (left_key, left_value) = left;
        let (right_key, right_value) = right;

        match left_key.cmp(&right_key) {
            Ordering::Less => {
                merged.insert(*left_key, *left_value);
                merged.insert(*right_key, *right_value);
            }
            Ordering::Equal => {
                let (sum_key, sum_value) = (left_key, left_value + right_value);
                merged.insert(*sum_key, sum_value);
            }
            Ordering::Greater => {
                merged.insert(*left_key, *left_value);
                merged.insert(*right_key, *right_value);
            }
        }
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
}
