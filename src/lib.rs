type Result<T> = std::result::Result<T, std::io::Error>;

pub fn get_anagrams(_target_anagram: &str, _word_list: Vec<String>) 
    -> Result<Vec<(String, String)>> {
        Ok(vec![("to".to_string(), "do".to_string())])
}
