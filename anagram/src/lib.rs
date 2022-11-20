use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut valid_anagrams = HashSet::new();
    for candidate in possible_anagrams.iter() {
        if is_valid_anagram(&word.to_lowercase(), &candidate.to_lowercase()) {
            valid_anagrams.insert(*candidate);
        }
    }
    valid_anagrams
}

fn is_valid_anagram(word: &str, candidate: &str) -> bool {
    if word == candidate {
        return false;
    }
    // count available letters
    let mut letters = HashMap::new();
    for c in word.chars() {
        letters.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }
    // check if we can write candidate with note letters
    for c in candidate.chars() {
        if !letters.contains_key(&c) || letters.get(&c).expect("oops") <= &0 {
            return false;
        }
        letters.entry(c).and_modify(|c| *c -= 1);
    }
    // check if there are still some candidate letters left
    !letters.into_values().any(|c| c > 0)
}
