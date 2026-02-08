use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut sorted_word: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word.sort();

    let mut hashset = HashSet::new();

    for &candidate in possible_anagrams {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut sorted_candidate: Vec<char> = candidate.to_lowercase().chars().collect();
        sorted_candidate.sort();

        if sorted_candidate == sorted_word {
            hashset.insert(candidate);
        }
    }

    hashset
}
