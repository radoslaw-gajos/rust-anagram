use crate::word::Word;
use std::collections::HashMap;

pub fn map_by_length(words: Vec<Word>) -> HashMap<usize, Vec<Word>> {
    let mut map = HashMap::new();

    for word in words {
        let length = word.value.len();
        map.entry(length).or_insert(Vec::new());
        map.get_mut(&length).unwrap().push(word);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_map_by_length() {
        // given
        let words = vec![
            Word::from("a".to_string()),
            Word::from("to".to_string()),
            Word::from("in".to_string()),
            Word::from("cat".to_string()),
            Word::from("dog".to_string()),
            Word::from("god".to_string()),
        ];

        // when
        let map = map_by_length(words);

        // then
        assert_eq!(map.keys().len(), 3);
        assert!(map.contains_key(&1usize));
        assert!(map.contains_key(&2usize));
        assert!(map.contains_key(&3usize));
        assert_eq!(map.get(&1usize).unwrap().len(), 1usize);
        assert_eq!(map.get(&2usize).unwrap().len(), 2usize);
        assert_eq!(map.get(&3usize).unwrap().len(), 3usize);
    }
}
