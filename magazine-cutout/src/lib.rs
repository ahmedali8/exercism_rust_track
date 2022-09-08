// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // For each word in magazine, create a hashmap with the word as the key
    // and the number of times it occurs as the value
    // Do this by looping over the magazine vector, creating the map / updating
    // the map as we go.
    let mut magazine_word_map = 
        magazine
            .iter()
            .fold(
                HashMap::new(),
                |mut acc_map: HashMap<&str, u32>, curr_word| {
                    *acc_map
                        .entry(curr_word)
                        .or_insert(0) += 1;
                    acc_map
                }
            );
    // println!("Magazine word map: {:?}", magazine_word_map);
    
    // Loop over the note, and check if there is an entry for that word.
    // If so, decrement the word count in the magazine and continue.
    // In not return false.
    for word in note {
        let instances_found: &mut u32 = magazine_word_map
            .entry(word)
            .or_insert(0);
        if (*instances_found == 0) {
            return false;
        } else {
            // the word was found and used, so decrement the number 
            // of instances available for use by 1.
            *instances_found -= 1;
        }
    }
    true
}
