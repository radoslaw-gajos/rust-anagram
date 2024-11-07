use crate::word::Word;

pub fn is_compatible(word: &Word, anagram: &Word) -> bool {
    for c in word.characters.keys() {
        if !anagram.characters.contains_key(c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_when_compatible() {
        // given
        let target_anagram = Word::from("Godly".to_string());
        let word = Word::from("Dog".to_string());

        // when
        let is_compatible = is_compatible(&word, &target_anagram);

        // then
        assert!(is_compatible);
    }

    #[test]
    fn should_check_when_incompatible() {
        // given
        let target_anagram = Word::from("Fog".to_string());
        let word = Word::from("Food".to_string());

        // when
        let is_compatible = is_compatible(&word, &target_anagram);

        // then
        assert!(!is_compatible);
    }
}

