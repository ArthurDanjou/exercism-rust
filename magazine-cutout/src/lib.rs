// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map = magazine
        .iter()
        .fold(HashMap::new(), |mut words, str| {
            *words.entry(str).or_insert(0) += 1;
            words
        });

    let note_map = note
        .iter()
        .fold(HashMap::new(), |mut words, str| {
            *words.entry(str).or_insert(0) += 1;
            words
        });

    note_map
        .iter()
        .all(&|(word, count)| magazine_map.get(word).unwrap_or(&0) >= count)
}
