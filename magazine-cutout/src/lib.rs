use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // Map the available words in magazine
    let mut words = HashMap::new();
    for word in magazine {
        words.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }
    // Check if we can construct note
    for word in note.iter() {
        if !words.contains_key(word) || *words.get(word).unwrap() <= 0{
            return false;
        }
        words.entry(word).and_modify(|c| *c -= 1);
    }
    true
}
