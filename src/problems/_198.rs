struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];

        for (i, num) in nums.iter().enumerate() {
            dp[i + 2] = dp[i + 2].max(dp[i] + num);
            dp[i + 1] = dp[i + 1].max(dp[i]);
        }

        *dp.iter().max().unwrap()
    }
}

#[test]
fn test() {
    let t1 = vec![1, 2, 3, 1];
    assert_eq!(Solution::rob(t1), 4);

    let t2 = vec![2, 7, 9, 3, 1];
    assert_eq!(Solution::rob(t2), 12);

    let t3 = vec![1, 2];
    assert_eq!(Solution::rob(t3), 2);

    let t4 = vec![2, 1, 1, 2];
    assert_eq!(Solution::rob(t4), 4);
}
