struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut freq_s = vec![0; 26];
        let mut freq_t = vec![0; 26];

        for &c in s.as_bytes() {
            freq_s[(c - b'a') as usize] += 1;
        }

        for &c in t.as_bytes() {
            freq_t[(c - b'a') as usize] += 1;
        }

        let steps: i32 = freq_s
            .iter()
            .zip(&freq_t)
            .map(|(&count_s, &count_t)| i32::abs(count_s - count_t))
            .sum();

        steps / 2
    }
}

#[test]
fn test() {
    let s1: String = "bab".to_string();
    let t1: String = "aba".to_string();
    assert_eq!(Solution::min_steps(s1, t1), 1);

    let s2: String = "leetcode".to_string();
    let t2: String = "practice".to_string();
    assert_eq!(Solution::min_steps(s2, t2), 5);

    let s3: String = "anagram".to_string();
    let t3: String = "mangaar".to_string();
    assert_eq!(Solution::min_steps(s3, t3), 0);

    let s4: String = "xxyyzz".to_string();
    let t4: String = "xxyyzz".to_string();
    assert_eq!(Solution::min_steps(s4, t4), 0);
}
