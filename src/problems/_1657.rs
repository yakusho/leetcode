use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut freq_word1: HashMap<char, usize> = HashMap::new();
        for char in word1.chars() {
            *freq_word1.entry(char).or_insert(0) += 1;
        }

        let mut freq_word2: HashMap<char, usize> = HashMap::new();
        for char in word2.chars() {
            *freq_word2.entry(char).or_insert(0) += 1;
        }
        
        let keys1: HashSet<_> = freq_word1.keys().collect();
        let keys2: HashSet<_> = freq_word2.keys().collect();
        let exclusive: Vec<&char> = keys1.symmetric_difference(&keys2).cloned().collect();
        if exclusive.len() > 0 {
          return false
        }

        let mut freq_dist_word1: Vec<usize> = freq_word1.values().cloned().collect();
        let mut freq_dist_word2: Vec<usize> = freq_word2.values().cloned().collect();
        freq_dist_word1.sort();
        freq_dist_word2.sort();

        freq_dist_word1 == freq_dist_word2
    }
}

#[test]
fn test() {
    let word11: String = "abc".to_string();
    let word12: String = "bca".to_string();
    assert_eq!(Solution::close_strings(word11, word12), true);

    let word21: String = "a".to_string();
    let word22: String = "aa".to_string();
    assert_eq!(Solution::close_strings(word21, word22), false);

    let word31: String = "cabbba".to_string();
    let word32: String = "abbccc".to_string();
    assert_eq!(Solution::close_strings(word31, word32), true);

    let word41: String = "uau".to_string();
    let word42: String = "ssx".to_string();
    assert_eq!(Solution::close_strings(word41, word42), false);
}
