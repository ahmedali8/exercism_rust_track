use std::collections::HashSet;

// 1. convert the word to lowercase then a sorted vector
// 2. convert each possible anagram to lowercase then a sorted vector
// 3. iterate each possible anagram to original word, if equal then skip, if not equal then add in return list

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // convert word to lowercase
    let word_lowercase: String = word.clone().to_lowercase();

    // convert to sorted vec
    let words_vec: Vec<char> = to_lowercase_char_sorted_vec(word);

    // println!(
    //     "word: {}, word_lowercase: {}, words_vec: {:?}",
    //     word, word_lowercase, words_vec
    // );

    // iterate possible_anagrams and if original word found skip.
    // if not then sort and compare
    // if equal add to return list
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for &item in possible_anagrams {
        let item_lowercase: String = item.to_lowercase();
        if item_lowercase != word_lowercase {
            let item_vec: Vec<char> = to_lowercase_char_sorted_vec(item);

            if item_vec == words_vec {
                anagrams.insert(item);
            }
        }
    }

    return anagrams;
}

pub fn to_lowercase_char_sorted_vec(word: &str) -> Vec<char> {
    let mut vector: Vec<char> = word.to_lowercase().chars().collect::<Vec<char>>();
    vector.sort_unstable();
    return vector;
}
