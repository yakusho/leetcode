struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
      let rows = matrix.len();
      let cols = matrix[0].len();      
      let mut dp = vec![vec![0; cols]; rows];

      for j in 0..cols {
          dp[0][j] = matrix[0][j];
      }

      for i in 1..rows {
          for j in 0..cols {
              dp[i][j] = matrix[i][j] + dp[i-1][j];
              
              if j > 0 {
                  dp[i][j] = dp[i][j].min(matrix[i][j] + dp[i-1][j-1]);
              }
              
              if j < cols - 1 {
                  dp[i][j] = dp[i][j].min(matrix[i][j] + dp[i-1][j+1]);
              }
          }
      }

      dp[rows - 1].iter().cloned().min().unwrap_or(0)
    }
}

#[test]
fn test() {
    let t1 = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    assert_eq!(Solution::min_falling_path_sum(t1), 13);

    let t2 = vec![vec![-19, 57], vec![-40, -5]];
    assert_eq!(Solution::min_falling_path_sum(t2), -59);
}
