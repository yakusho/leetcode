use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut seen_set = HashSet::new();
        let mut repeated = 0;
        let mut missing = 0;

        for &num in &nums {
            if !seen_set.insert(num) {
                repeated = num;
            }
        }

        for i in 1..=nums.len() as i32 {
            if !seen_set.contains(&i) {
                missing = i;
                break;
            }
        }

        vec![repeated, missing]
    }
}

#[test]
fn test() {
    let t1 = vec![1, 2, 2, 4];
    assert_eq!(Solution::find_error_nums(t1), vec![2, 3]);

    let t2 = vec![1, 1];
    assert_eq!(Solution::find_error_nums(t2), vec![1, 2]);
}
