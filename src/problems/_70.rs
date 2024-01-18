struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut ways = vec![0; (n + 1) as usize];
        ways[1] = 1;
        ways[2] = 2;

        for i in 3..=n as usize {
            ways[i] = ways[i - 1] + ways[i - 2];
        }

        ways[n as usize]
    }
}

#[test]
fn test() {
    let t1 = 2;
    assert_eq!(Solution::climb_stairs(t1), 2);

    let t2 = 3;
    assert_eq!(Solution::climb_stairs(t2), 3);
}
