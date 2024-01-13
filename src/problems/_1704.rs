use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mid = s.len() / 2;
        let valid_vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        let mut freq_a: HashMap<char, i32> = HashMap::new();
        let mut freq_b: HashMap<char, i32> = HashMap::new();

        let (f_half, l_half) = s.split_at(mid);
        for i in 0..mid {
            let f_char = f_half.chars().nth(i).unwrap();
            let l_char = l_half.chars().nth(i).unwrap();
            *freq_a.entry(f_char).or_insert(0) += 1;
            *freq_b.entry(l_char).or_insert(0) += 1;
        }

        let sum_a: i32 = valid_vowels
            .iter()
            .map(|&vowel| *freq_a.entry(vowel).or_insert(0))
            .sum();

        let sum_b: i32 = valid_vowels
            .iter()
            .map(|&vowel| *freq_b.entry(vowel).or_insert(0))
            .sum();

        return sum_a == sum_b;
    }
}

#[test]
fn test() {
    let s1: String = "book".to_string();
    assert_eq!(Solution::halves_are_alike(s1), true);

    let s2: String = "textbook".to_string();
    assert_eq!(Solution::halves_are_alike(s2), false);

    let s3: String = "AbCdEfGh".to_string();
    assert_eq!(Solution::halves_are_alike(s3), true);
}
